# `cryptest`

Test and benchmark the cryptographic acceleration engines of the K210 compared
to rust CPU implementations.

Example benchmark output:

```
SHA256 hw (4194240 bytes): MATCH (12542 kB/s)
SHA256 hw, 32bit (4194240 bytes): MATCH (18965 kB/s)
SHA256 sw (4194240 bytes): MATCH (5087 kB/s)
AES128 hw, 32bit (262144 bytes): (6247 kB/s)
AES128 sw (262144 bytes): (394 kB/s)
```
