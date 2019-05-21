# `uart-passthrough`

Pass through UART from host to the ESP8285 WIFI chip.

Any input from the host is forwarded to the ESP8285, and any input from the ESP8285
is forwarded to the host.

Allows setting the baudrate through an out-of-band protocol. Pulling DTR low will
reset the baudrate to the safe default of 115200, accepting two commands:

```
0x23   <baudrate 32-bit LE>
  Set baudrate (both to host and ESP)

0x42
  Reset ESP8285 MCU (toggle WIFI_EN pin)

others: no op
```

The connection from the K210 to the ESP can handle up to `115200*40=4608000` baud,
however the connection to the host seems to get stuck at a lower number. Use
`AT+UART_CUR=` (not `UART_DEF` !) to set the baudrate at the ESP side so that
it is always possible to reset the MCU to get back to 115200 baud.

There's a demo in `demo/weather.py`.
