/** term: Minimal interactive serial console.
 * Usage: term /dev/ttyS1
 *
 * Copyright (c) 2020 W.J. van der Laan
 * Distributed under the MIT software license,
 */
#include <errno.h>
#include <fcntl.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <termios.h>
#include <unistd.h>
#include <poll.h>

int set_raw(int fd)
{
    struct termios tty;

    if (tcgetattr(fd, &tty) < 0) {
        printf("Error from tcgetattr: %s\n", strerror(errno));
        return -1;
    }

    tty.c_iflag &= ~(ICRNL | IXON);
    tty.c_lflag &= ~(ECHO | ICANON);

    if (tcsetattr(fd, TCSANOW, &tty) != 0) {
        printf("Error from tcsetattr: %s\n", strerror(errno));
        return -1;
    }
    return 0;
}

int set_interface_attribs(int fd, int speed)
{
    struct termios tty;

    if (tcgetattr(fd, &tty) < 0) {
        printf("Error from tcgetattr: %s\n", strerror(errno));
        return -1;
    }

    cfsetospeed(&tty, (speed_t)speed);
    cfsetispeed(&tty, (speed_t)speed);

    tty.c_cflag |= (CLOCAL | CREAD);    /* ignore modem controls */
    tty.c_cflag &= ~CSIZE;
    tty.c_cflag |= CS8;         /* 8-bit characters */
    tty.c_cflag &= ~PARENB;     /* no parity bit */
    tty.c_cflag &= ~CSTOPB;     /* only need 1 stop bit */
    tty.c_cflag &= ~CRTSCTS;    /* no hardware flowcontrol */

    /* setup for non-canonical mode */
    tty.c_iflag &= ~(IGNBRK | BRKINT | PARMRK | ISTRIP | INLCR | IGNCR | ICRNL | IXON);
    tty.c_lflag &= ~(ECHO | ECHONL | ICANON | ISIG | IEXTEN);
    tty.c_oflag &= ~OPOST;

    /* fetch bytes as they become available */
    tty.c_cc[VMIN] = 1;
    tty.c_cc[VTIME] = 1;

    if (tcsetattr(fd, TCSANOW, &tty) != 0) {
        printf("Error from tcsetattr: %s\n", strerror(errno));
        return -1;
    }
    return 0;
}

int main(int argc, char **argv) {
    if (argc < 2) {
        fprintf(stderr, "Pass serial device on command line\n");
        exit(1);
    }
    const char *portname = argv[1];
    int fd = open(portname, O_RDWR | O_NOCTTY | O_SYNC);
    if (fd < 0) {
        fprintf(stderr, "Error opening %s: %s\n", portname, strerror(errno));
        exit(1);
    }

    set_raw(STDIN_FILENO);
    /*baudrate 115200, 8 bits, no parity, 1 stop bit */
    set_interface_attribs(fd, B115200);

    struct pollfd fds[2] = {
        {STDIN_FILENO, POLLIN, 0},
        {fd, POLLIN, 0},
    };
    do {
        for (int i=0; i<2; ++i) {
            fds[i].revents = 0;
        }

        if (poll(fds, 2, -1) < 0) {
            fprintf(stderr, "Poll error: %s\n", strerror(errno));
            exit(1);
        }

        for (int i=0; i<2; ++i) {
            if (fds[i].revents & POLLIN) {
                char ch;
                int rdlen = read(fds[i].fd, &ch, 1);
                if (rdlen < 0) {
                    fprintf(stderr, "Error from read: %d: %s\n", rdlen, strerror(errno));
                    exit(1);
                } else if (rdlen == 0) {
                    /* EOF */
                    exit(0);
                }

                if (i == 1 && ch == '\r') {
                    /* Don't print \r */
                    continue;
                }
                if (i == 0 && ch == '\n') {
                    /* Add extra \r before newline */
                    char cr = '\r';
                    write(fds[1-i].fd, &cr, 1);
                }

                int wlen = write(fds[1-i].fd, &ch, 1);
                if (wlen <= 0) {
                    fprintf(stderr, "Error from write: %d: %s\n", wlen, strerror(errno));
                    exit(1);
                }
            }
        }
    } while (1);
}
