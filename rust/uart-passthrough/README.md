# `uart-passthrough`

Pass through UART from host to the ESP8285 WIFI chip.

Any input from the host is forwarded to the ESP8285, and any input from the ESP8285
is forwarded to the host. Currently only supports a fixed baudrate of 115200.
