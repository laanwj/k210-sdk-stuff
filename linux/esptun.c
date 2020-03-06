/** esptun: UDP tunnel over UART, using ESP8285 AT command set.
 * Based on "simpletun" by Davide Brini.
 *
 * Copyright (c) 2020 W.J. van der Laan
 * Distributed under the MIT software license,
 */
#include <errno.h>
#include <fcntl.h>
#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <poll.h>
#include <string.h>
#include <stdarg.h>
#include <asm/termbits.h>
#include <net/if.h>
#include <linux/if_tun.h>
#include <sys/types.h>
#include <sys/socket.h>
#include <sys/ioctl.h>
#include <sys/stat.h>
#include <arpa/inet.h>
#include <sys/select.h>
#include <sys/time.h>

/* buffer for reading from UART, must be >= 1500 */
#define ESP_BUFSIZE 2000
/* buffer for reading from tun/tap interface, must be >= 1500 */
#define TAP_BUFSIZE 2000
/* baud rate (max 115200*40 = 4608000). TODO: make this a command line argument. */
#define BAUDRATE (115200 * 4)

/** Macro for writing static strings without needing strlen. */
#define S(s) (const uint8_t*)(s), (sizeof(s)-1)

bool debug = false;
char *progname;
uint8_t esp_buffer[ESP_BUFSIZE];
size_t esp_end;
uint8_t tap_buffer[TAP_BUFSIZE];
size_t tap2net;
size_t net2tap;
int esp_fd = -1;
int tun_fd = -1;

#define INFO1 0
#define INFO2 1
#define WARNING 2
#define DEBUG 3

/**
 * Prints debug/warning/info.
 */
static __attribute__ ((format (printf, 2, 3))) void logprintf(int cls, const char *msg, ...)
{
    va_list argp;
    int attr = 0;
    switch (cls) {
        case INFO1: attr = 95; break;
        case INFO2: attr = 35; break;
        case WARNING: attr = 91; break;
        case DEBUG:
            if (!debug) {
                return;
            }
            break;
    }
    fprintf(stderr, "\x1b[%dm", attr);
    va_start(argp, msg);
    vfprintf(stderr, msg, argp);
    va_end(argp);
    fprintf(stderr, "\x1b[0m");
}

/**
 * Log a raw response for debugging.
 */
static void debug_response(const uint8_t *esp_buffer, size_t n) {
    static const char hexchars[16] = "0123456789abcdef";
    if (debug) {
        for (size_t i = 0; i < n; ++i) {
            if (esp_buffer[i] < 32 || esp_buffer[i] >= 127 || esp_buffer[i] == '\\') {
                fputc('\\', stderr);
                fputc('x', stderr);
                fputc(hexchars[esp_buffer[i] >> 4], stderr);
                fputc(hexchars[esp_buffer[i] & 0xf], stderr);
            } else {
                fputc(esp_buffer[i], stderr);
            }
        }
    }
}

/**
 * Prints error message on stderr and exits the program.
 */
static __attribute__ ((format (printf, 1, 2))) void my_err(char *msg, ...)
{
    va_list argp;

    fprintf(stderr, "error: ");
    va_start(argp, msg);
    vfprintf(stderr, msg, argp);
    va_end(argp);
    fprintf(stderr, "\n");
    exit(1);
}

/**
 * Allocates or reconnects to a tun/tap device. *dev specifies the name of the
 * interface (e.g. tunX).  This will be overwritten with the actual name. The
 * caller must reserve enough space (IFNAMSIZ) in *dev.
 */
int tun_alloc(char *dev, int flags)
{
    struct ifreq ifr;
    int fd, err;
    static const char *clonedev = "/dev/net/tun";

    if ((fd = open(clonedev, O_RDWR)) < 0) {
        my_err("Opening /dev/net/tun (%s)", strerror(errno));
    }

    memset(&ifr, 0, sizeof(ifr));

    ifr.ifr_flags = flags;

    if (*dev) {
        strncpy(ifr.ifr_name, dev, IFNAMSIZ);
    }

    if ((err = ioctl(fd, TUNSETIFF, (void *)&ifr)) < 0) {
        close(fd);
        my_err("ioctl(TUNSETIFF) (%s)", strerror(errno));
    }

    strcpy(dev, ifr.ifr_name);

    return fd;
}

/**
 * Ensures we write exactly n bytes. Exit in case of error or EOF.
 */
static void write_all(int fd, const uint8_t *buf, int n)
{
    int nwrite, left = n;

    while (left > 0) {
        if ((nwrite = write(fd, buf, left)) <= 0) {
            my_err("Writing data (%s)", strerror(errno));
        }
        left -= nwrite;
        buf += nwrite;
    }
}

/**
 * Ensures we write exactly n bytes, escaping special characters. Exit in case
 * of error or EOF.
 */
static void write_esc(int fd, const char *buf, int n)
{
    for (int i = 0; i < n; ++i) {
        if (buf[i] == '\\' || buf[i] == ',' || buf[i] == '"') {
            write_all(fd, S("\\"));
        }
        write_all(fd, (const uint8_t*)&buf[i], 1);
    }
}

/**
 * Write a 32-bit unsigned integer (as ASCII text).
 */
static void write_uint(int fd, uint32_t x)
{
    char buf[11];
    size_t ptr = sizeof(buf);
    do {
        ptr -= 1;
        buf[ptr] = '0' + (x % 10);
        x /= 10;
    } while (x != 0);
    write_all(fd, (const uint8_t*)&buf[ptr], sizeof(buf) - ptr);
}

/**
 * Prints usage and exits.
 */
static void usage(void)
{
    fprintf(stderr, "Usage:\n");
    fprintf(stderr, "%s <ifname> <uart> <ssid> <passwd> <host> <port>\n", progname);
    exit(1);
}

/**
 * Set UART attributes and speed.
 */
static int setup_uart(int fd, int speed)
{
    struct termios2 tty;

    if (ioctl(fd, TCGETS2, &tty) != 0) {
        my_err("Error from TCGETS2 (%s)", strerror(errno));
    }

    tty.c_cflag |= (CLOCAL | CREAD);    /* ignore modem controls */
    tty.c_cflag &= ~CSIZE;
    tty.c_cflag |= CS8;         /* 8-bit characters */
    tty.c_cflag &= ~PARENB;     /* no parity bit */
    tty.c_cflag &= ~CSTOPB;     /* only need 1 stop bit */
    tty.c_cflag &= ~CRTSCTS;    /* no hardware flowcontrol */

    /* custom baudrate setting */
    tty.c_cflag &= ~CBAUD;
    tty.c_cflag |= BOTHER;
    tty.c_ispeed = speed;
    tty.c_ospeed = speed;

    /* setup for non-canonical mode */
    tty.c_iflag &= ~(IGNBRK | BRKINT | PARMRK | ISTRIP | INLCR | IGNCR | ICRNL | IXON);
    tty.c_lflag &= ~(ECHO | ECHONL | ICANON | ISIG | IEXTEN);
    tty.c_oflag &= ~OPOST;

    /* fetch bytes as they become available */
    tty.c_cc[VMIN] = 1;
    tty.c_cc[VTIME] = 0;

    if (ioctl(fd, TCSETS2, &tty) != 0) {
        my_err("Error from TCSETS2 (%s)", strerror(errno));
    }
    return 0;
}

/** Prefix for receive packet. */
static const uint8_t pktprefix[] = {'+', 'I', 'P', 'D', ','};

struct packet_info {
    /* header: part after +IPD, and before : */
    const uint8_t *hdr;
    size_t hdr_len;
    /* packet payload */
    const uint8_t *payload;
    size_t payload_len;
};

/** Determine if a esp_buffer[0..end] contains a complete ESP8285 response.
 * If so, return the size (including \r\n line terminators).
 * If not, return 0.
 */
static size_t esp_response_is_complete(const uint8_t *b, size_t end, struct packet_info *pinfo)
{
    size_t i;
    /* Count number of bytes matching prefix. */
    for (i = 0; i < sizeof(pktprefix) && i < end && b[i] == pktprefix[i]; ++i)
        ;
    if (i == sizeof(pktprefix)) {
        /* Received packet.
         * Syntax is +IPD,<len>[,<remote IP>,<remote port>]:<data>
         * Look for end of header (':') first.
         */
        for (i = 5; i < end && b[i] != ':'; ++i)
            ;
        if (i != end) {
            size_t hdr_end = i + 1;
            /* Parse length. */
            size_t length = atoi((const char*)&b[5]);
            if ((hdr_end + length) <= end) {
                if (pinfo != NULL) {
                    pinfo->hdr = &b[5];
                    pinfo->hdr_len = i - 5;
                    pinfo->payload = &b[hdr_end];
                    pinfo->payload_len = length;
                }
                return hdr_end + length;
            }
        }
    } else if (i == end) {
        /* Possive start of received packet response. These are not "\r\n"
         * terminated and may contain any character.
         */
        return 0;
    } else {
        /** Other response - look for line terminator. */
        for (i = 0; i < end && b[i] != '\n'; ++i)
            ;
        if (i != end) {
            return i + 1;
        }
    }
    return 0;
}

/** Check if a buffer starts with a specified prefix. */
static bool is_prefix(const uint8_t *esp_buffer, size_t len, const uint8_t *prefix, size_t prefix_len)
{
    if (len < prefix_len) {
        return false;
    }
    for (size_t i = 0; i < prefix_len; ++i) {
        if (esp_buffer[i] != prefix[i]) {
            return false;
        }
    }
    return true;
}

/** Send a packet to tun interface. */
static void tun_tx_packet(int fd, const uint8_t *esp_buffer, size_t size) {
    if (write(fd, esp_buffer, size) <= 0) {
        logprintf(WARNING, "warning: error writing to tun (%s)\n", strerror(errno));
    }
}

/** Get OK/FAIL status after running a command on the ESP. */
static bool esp_read_responses(int fd, bool early_terminate)
{
    bool finished = false;
    bool retval = false;
    /** TODO handle timeouts. */
    while (!finished) {
        ssize_t n = read(fd, &esp_buffer[esp_end], ESP_BUFSIZE - esp_end);
        if (n <= 0) {
            my_err("Reading from UART (%s)", strerror(errno));
        }
        esp_end += n;

        size_t ptr = 0;
        while (ptr < esp_end) {
            const uint8_t *resp = &esp_buffer[ptr];
            struct packet_info pkt_info = {};
            size_t len = esp_response_is_complete(resp, esp_end - ptr, &pkt_info);
            if (len) {
                logprintf(DEBUG, "<-: ");
                debug_response(resp, len);
                logprintf(DEBUG, "\n");
                /* Check for final response */
                if (is_prefix(resp, len, S("OK"))
                 || is_prefix(resp, len, S("SEND OK"))) {
                    finished = true;
                    retval = true;
                }
                if (is_prefix(resp, len, S("FAIL"))
                  || is_prefix(resp, len, S("SEND FAIL"))
                  || is_prefix(resp, len, S("ERROR"))
                  || is_prefix(resp, len, S("ALREADY CONNECTED"))) {
                    finished = true;
                    retval = false;
                }
                /* Log interesting info responses */
                if (is_prefix(resp, len, S("+CIPSTA_CUR"))) {
                    logprintf(INFO2, "  %.*s\n", (int)(len - 11 - 2), resp + 11);
                }
                if (is_prefix(resp, len, S("WIFI "))) {
                    logprintf(INFO2, "  %.*s\n", (int)(len - 2), resp);
                }
                /* Did we receive a packet? */
                if (is_prefix(resp, len, pktprefix, sizeof(pktprefix)) && tun_fd != -1) {
                    logprintf(DEBUG, "NET2TUN %lu: Read %ld bytes from the esp interface\n", net2tap, pkt_info.payload_len);
                    net2tap++;
                    tun_tx_packet(tun_fd, pkt_info.payload, pkt_info.payload_len);
                }

                /* On non-final response, keep reading. */
            } else {
                break;
            }
            ptr += len;
        }

        if (ptr == 0 && esp_end == ESP_BUFSIZE) {
            /* Buffer full there was no progress processing responses - this isn't good,
             * exit to prevent looping forever. */
            my_err("Buffer full with unterminated command");
        }

        /* Remove processed responses from esp_buffer by shifting bytes. */
        memmove(esp_buffer, &esp_buffer[ptr], esp_end - ptr);
        esp_end -= ptr;

        if (early_terminate && esp_end == 0) {
            /* All responses processed, back to select so that packets from tun get a chance. */
            finished = true;
            retval = true;
        }
    }
    return retval;
}

/** Send a packet to ESP interface. */
static void esp_tx_packet(int fd, const uint8_t *buffer, size_t size) {
    write_all(fd, S("AT+CIPSEND="));
    write_uint(fd, size);
    write_all(fd, S("\r\n"));
    if (esp_read_responses(fd, false)) {
        write_all(fd, buffer, size);
        esp_read_responses(fd, false);
    }
}

int main(int argc, char **argv)
{
    if (argc < 7) {
        usage();
    }
    char if_name[IFNAMSIZ] = "";
    const char *portname = argv[2];
    const char *ssid = argv[3];
    const char *passwd = argv[4];

    strncpy(if_name, argv[1], IFNAMSIZ-1);
    const char *host = argv[5];
    int port = atoi(argv[6]);

    esp_fd = open(portname, O_RDWR | O_NOCTTY | O_SYNC);
    if (esp_fd < 0) {
        my_err("Error opening %s (%s)", portname, strerror(errno));
    }

    /* initial: baudrate 115200, 8 bits, no parity, 1 stop bit */
    setup_uart(esp_fd, 115200);

    /* initialize tun/tap interface */
    if ((tun_fd = tun_alloc(if_name, IFF_TUN | IFF_NO_PI)) < 0) {
        my_err("Error connecting to tun interface %s!\n", if_name);
    }

    logprintf(INFO1, "Successfully connected to interface %s\n", if_name);

    logprintf(INFO1, "Initializing device\n");
    write_all(esp_fd, S("ATE0\r\n"));
    if (!esp_read_responses(esp_fd, false)) {
        my_err("Could not disable echo");
    }

    write_all(esp_fd, S("AT+CWMODE_CUR=1\r\n"));
    if (!esp_read_responses(esp_fd, false)) {
        my_err("Could not switch to station mode");
    }

    write_all(esp_fd, S("AT+CIPMUX=0\r\n"));
    if (!esp_read_responses(esp_fd, false)) {
        my_err("Could not switch to single-connection mode");
    }

    logprintf(INFO1, "Changing baudrate to %d\n", BAUDRATE);
    write_all(esp_fd, S("AT+UART_CUR="));
    write_uint(esp_fd, BAUDRATE);
    write_all(esp_fd, S(",8,1,0,0\r\n"));
    if (!esp_read_responses(esp_fd, false)) {
        my_err("Could not set faster baudrate");
    }
    setup_uart(esp_fd, BAUDRATE);

    write_all(esp_fd, S("AT\r\n"));
    if (!esp_read_responses(esp_fd, false)) {
        my_err("AT test unsuccesful: new baudrate unstable?");
    }

    logprintf(INFO1, "Connecting to AP\n");
    write_all(esp_fd, S("AT+CWJAP_CUR=\""));
    write_esc(esp_fd, ssid, strlen(ssid));
    write_all(esp_fd, S("\",\""));
    write_esc(esp_fd, passwd, strlen(passwd));
    write_all(esp_fd, S("\"\r\n"));
    if (!esp_read_responses(esp_fd, false)) {
        my_err("Could not connect to AP");
    }

    write_all(esp_fd, S("AT+CIPSTA_CUR?\r\n"));
    if (!esp_read_responses(esp_fd, false)) {
        my_err("Could not query IP");
    }

    logprintf(INFO1, "Opening UDP connection to %s:%d from port %d\n", host, port, port);
    write_all(esp_fd, S("AT+CIPSTART=\"UDP\",\""));
    write_esc(esp_fd, host, strlen(host));
    write_all(esp_fd, S("\","));
    write_uint(esp_fd, port);
    write_all(esp_fd, S(","));
    write_uint(esp_fd, port); /* local port is same as remote port for now */
    write_all(esp_fd, S("\r\n"));
    if (!esp_read_responses(esp_fd, false)) {
        my_err("Could not open UDP connection");
    }

    logprintf(INFO1, "Starting packet loop, moving to background\n");

    /* Process to background */
    if (daemon(0, 0) < 0) {
        my_err("Could not go to background\n");
    }
    debug = false; /* no use to log anything after this */

    struct pollfd fds[2] = {
        {esp_fd, POLLIN, 0},
        {tun_fd, POLLIN, 0},
    };
    do {
        for (int i=0; i<2; ++i) {
            fds[i].revents = 0;
        }

        if (poll(fds, 2, -1) < 0) {
            my_err("poll (%s)", strerror(errno));
        }

        /* Handle input from UART */
        if (fds[0].revents & POLLIN) {
            /* Read all data available from the UART, before processing data from TUN, to make sure we don't miss any.
             * TODO: There can still be data loss at high baudrates, unfortunately, this needs logic to re-sync with the device in this case.
             */
            esp_read_responses(esp_fd, true);
        }

        /* Handle input from TUN */
        if (fds[1].revents & POLLIN) {
            ssize_t nread;
            /* data from tun/tap: just read it and write it to the network */
            if ((nread = read(tun_fd, tap_buffer, TAP_BUFSIZE)) <= 0) {
                my_err("Reading data from tap (%s)", strerror(errno));
            }

            logprintf(DEBUG, "TUN2NET %lu: Read %ld bytes from the tap interface\n", tap2net, nread);

            tap2net++;
            esp_tx_packet(esp_fd, tap_buffer, nread);
        }
    } while (true);

    return 0;
}
