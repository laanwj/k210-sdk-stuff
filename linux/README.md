esptun
======

A tool to tunnel IP packets over UDP over WIFI through an UART connected to a ESP8285 with the
standard AT firmware.

Usage
-----

    esptun <ifname> <uart> <ssid> <passwd> <host> <port>

This will create tun interface `ifname`. Then, with the with the ESP WIFI
device on `uart` it connects to the AP `ssid` with password `passwd`. 
It creates a UDP over IP tunnel with the other endpoint `host:port`.

The new tun device will not be configured, use the `ip` utility to assign an
IP address and switch the interface on.

For example:

    /root/esptun tun0 /dev/ttyS1 "accesspointname" "secretpassword" 192.168.122.21 23232
    /sbin/ip link set dev tun0 mtu 1472
    /sbin/ip addr add 10.0.1.2/24 dev tun0
    /sbin/ip link set tun0 up
    /sbin/ip route add default via 10.0.1.1 dev tun0

(setting the MTU to `1500-20-8` the typical network MTU minus IP and UDP header overhead, because
otherwise the ESP's network stack will drop the oversized packets)

### Baudrate note

If the tunnel hangs intermittently, the problem may be the `BAUDRATE` setting. The ESP8285 can handle
baudrates up to `115200*40 = 4608000` baud and `esptun` will try switching to a higher baudrate.

However, these higher baudrates are unstable as it seems that the UART cannot
keep up reading in some cases and loses data. This may be a misconfiguration or something that could
be improved in the `8250_dw` Linux driver for this peripheral (DMA support?).

Which values work may vary, if you want to be completely safe use `115200`. The value can be changed
at the top of `esptun.c`.

Host side
---------

On the other endpoint the tunnel is expected to be a host running `socat` or similar to unwrap
the tunnel. For example:

    sudo socat UDP:192.168.2.127:23232,bind=192.168.122.21:23232 \
        TUN:10.0.1.1/24,tun-name=tundudp,iff-no-pi,tun-type=tun,su=$USER,iff-up &
    sudo ip link set dev tundudp mtu 1472

Optionally, enable forwarding and masquerading:

    sudo iptables -t nat -F
    sudo iptables -t nat -A POSTROUTING -o ens3 -j MASQUERADE
    echo 1 | sudo tee /proc/sys/net/ipv4/ip_forward

Linux kernel
------------

Until there is a proper solution, you can use the following kernel branch
to get the UART working on Kendryte K210 under Linux:

https://github.com/laanwj/linux/tree/kendryte-5.6-rc1-wifi

This is a hack that configures the FPIOA and GPIOHS manually, and configures UART1
from the device tree.

To enable TUN/TAP, enable the following kernel settings:
```
CONFIG_NET=y
CONFIG_INET=y
CONFIG_TUN=y
```

While building the root filesystem, make sure you enable at least `ip` and `ping`
(and possibly other network tools) for busybox.

Author
------

Copyright (c) 2020 W.J. van der Laan
Distributed under the MIT software license,
