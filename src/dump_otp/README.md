otp-dump
========

Dump the contents of OTP (One-Time Programmable memory) in hexadecimal format
to serial.

Output is in Intel HEX format. Make sure your parser ignores non-':' prefixed lines
as there can be warnings, or do

    grep "^:" /tmp/otp1.txt > /tmp/otp1.ihx
    objcopy -I ihex -O binary /tmp/otp1.ihx kendryte_otp1.dat

Note that the OTP contains a serial number (at least 0x3d9c..0x3d9f seem to
differ between boards) so it'd be wise to treat the output as
privacy-sensitive.
