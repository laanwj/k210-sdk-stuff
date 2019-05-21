#!/usr/bin/env python3
import argparse
import os
import serial
import struct
import sys
import time

from esp8266 import ESP8266, CLOSED

def expect(a, b):
    if a == b:
        return
    print(f'Mismatch: {a} versus {b}', file=sys.stderr)
    assert(0)

def sanity_check(ser):
    '''Check sanity of current rate'''
    retries = 0
    ser.write(b'AT\r\n')
    while retries < 5: 
        line1 = ser.readline()
        if line1 == b'AT\r\n':
            expect(ser.readline(), b'\r\n')
            expect(ser.readline(), b'OK\r\n')
            return
        retries += 1
    raise IOError('Basic connection check failed')

def send_oob(ser, buf):
    ser.baudrate = 115200
    # Toggle DTR for OoB command
    ser.dtr = False
    time.sleep(0.01)
    ser.dtr = True
    ser.write(buf)
    time.sleep(0.01)

def reset_esp8266(ser):
    send_oob(ser, b'\x42') # reset

    retries = 0
    while retries < 5:
        line = ser.readline()
        # print('Reset: ', line)
        if line == b'ready\r\n':
            return
        retries += 1
    raise IOError('Reset failed')

def set_rate(ser, newbaud):
    '''Set baudrate'''
    # Need to do three things here:
    # - Set WIFI module baudrate
    # - Set K210<->WIFI baudrate
    # - Set Host<->K210 baudrate

    sanity_check(ser)
    # Set WIFI module to new rate
    cmd = b'AT+UART_CUR=%d,8,1,0,0\r\n' % newbaud
    ser.write(cmd)
    try:
        expect(ser.readline(), cmd)
        expect(ser.readline(), b'\r\n')
        expect(ser.readline(), b'OK\r\n')
    except IOError as e:
        reset_esp8266(ser)
        raise

    send_oob(ser, b'\x23' + struct.pack('<I', newbaud))
    ser.baudrate = newbaud

    sanity_check(ser)

def parse_args():
    parser = argparse.ArgumentParser(description='Fetch the current weather from wttr.in through ESP8266 passthrough')
    parser.add_argument('--skip-reset', help='Skip reset and baudrate setting', action='store_true')
    parser.add_argument('--baud', help='Set baudrate (default 1382400)', type=int, default=12*115200)

    return parser.parse_args()

ser = serial.Serial('/dev/ttyUSB1', 115200, timeout=1)
try:
    args = parse_args()
    TARGET_BAUD = args.baud
    wifi_ap = os.getenv('WIFI_AP')
    wifi_pass = os.getenv('WIFI_PASS')

    if wifi_ap is None or wifi_pass is None:
        print('Set environment variables WIFI_AP and (optionally) WIFI_PASS')
        exit(1)

    if not args.skip_reset:
        print('\x1b[95mNote: if reset fails, the only way to reset the ESP8285 is to power off and on the board\x1b[0m')
        print('\x1b[35mReset ESP8266\x1b[0m')
        # first, try resetting the chip
        reset_esp8266(ser)
        sanity_check(ser)

        set_rate(ser, TARGET_BAUD)
        print('\x1b[35mSwitching to rate %d was successful\x1b[0m' % TARGET_BAUD)
    else:
        ser.baud = TARGET_BAUD

    ser.timeout = 5 # longer timeout while using the device, allow some time to actually connect to AP
    esp = ESP8266(ser)
    #print(esp.scanForAccessPoints())
    esp.connectToAccessPoint(wifi_ap.encode(), wifi_pass.encode())
    print('\x1b[35mIP address: {}\x1b[0m'.format(esp.getIPAddress()))
    esp.closeCurrent()
    #esp.startCip(b'TCP', b'192.168.1.110', 8000)
    esp.startCip(b'TCP', b'wttr.in', 80)
    esp.sendBuffer(b'GET /?0qA HTTP/1.1\r\nHost: wttr.in\r\nConnection: close\r\nUser-Agent: Weather-Spy\r\n\r\n')
    data = b''
    while True:
        rv = esp.recvBuffer()
        if rv is CLOSED:
            break
        elif rv is not None:
            data += rv
            print('Received %d bytes' % len(data))
        else:
            print('(timeout or empty)')
    idx = data.find(b'\r\n\r\n')
    print()
    print(data[idx+4:].decode())

finally:
    # always try to switch back before exiting so next invocation won't be messed up
    set_rate(ser, 115200)
