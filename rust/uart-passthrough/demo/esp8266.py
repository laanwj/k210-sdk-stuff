# based on https://github.com/rogerdahl/python-esp8266/blob/master/esp8266.py

# https://github.com/espressif/esp8266_at/wiki/AT_Description

import logging
import re

logging.basicConfig(level=logging.DEBUG)

MAX_CIPSEND_BUFFER_SIZE = 2048


class ESP8266Exception(Exception):
    pass


class ESP8266ExceptionUnresolvable(Exception):
    pass

CLOSED = object()

class ESP8266(object):
    _successResponse_list = (b'ok', b'ready', b'no change', b'send ok')
    _unresolvableFailureResponse_list = (
        b'error', b'type error', b'alreay connect')

    def __init__(self, serial):
        assert serial.isOpen(), "Need a connected pySerial object"
        self._serial = serial
        self.sendCmd(b'AT')

    # General

    def sendCmd(self, cmd, retries=3):
        '''Send an AT command with automatic retries. If retries are exhausted, the final exception is
        forwarded to the client. If successful, the response lines are returned in a list.'''
        for i in range(retries):
            try:
                return self._sendCmd(cmd)
            except ESP8266Exception:
                if i == retries - 1:
                    raise

    def closeCurrent(self):
        currentStatus_str, currentProtocol_str, currentHost_str, currentPort_int = self.getCipStatus()
        if currentStatus_str in (b'CONNECTED'):
            self.closeCip()

    def sendBuffer(self, buf):
        while buf:
            self._sendBuffer(buf[:MAX_CIPSEND_BUFFER_SIZE])
            buf = buf[MAX_CIPSEND_BUFFER_SIZE:]

    def recvBuffer(self):
        '''
        State machine for parsing incoming received data.
        '''
        STATE_IGNORE_TO_EOL = -2
        STATE_RECV_DATA = -3
        STATE_EXPECT_LF = -4
        STATE_EXPECT_CR = -5
        STATE_EXPECT = 0
        STATE_NUMBER = 1

        expect = [b'+IPD,', b'CLOSED'] # prefixes
        ofs = 0
        state = STATE_EXPECT
        state_n = 3 # bit field of expect[N]
        recv_size = 0
        data = None
        rv = None
        line = b''
        while True:
            ch = self._serial.read(1)
            line += ch # for debugging only
            if ch == b'':
                return None

            if state != STATE_RECV_DATA and state != STATE_EXPECT_LF:
                if ch == b'\r':
                    if state == STATE_IGNORE_TO_EOL:
                        print('Ignored unknown response: %s' % line)
                        line = b''
                    state = STATE_EXPECT_LF
                    continue
            elif state == STATE_RECV_DATA:
                data += ch
                if len(data) == recv_size:
                    return data
            if state == STATE_EXPECT_LF:
                if ch == b'\n':
                    return rv
                else:
                    raise IOError('Improperly terminated line')
            if state == STATE_EXPECT_CR:
                if ch == b'\r':
                    state = STATE_EXPECT_LF
                else:
                    raise IOError('Improperly terminated line')
            elif state == STATE_EXPECT:
                new_state_n = 0
                terminate = None
                for n,e in enumerate(expect):
                    if (state_n & (1<<n)) and ord(ch) == e[ofs]:
                        if ofs == len(e) - 1:
                            terminate = n
                        else:
                            new_state_n |= (1<<n)

                if terminate is not None:
                    if terminate == 0:
                        state = STATE_NUMBER
                        recv_size = 0
                    else: # Closed
                        state = STATE_EXPECT_CR
                        rv = CLOSED
                elif new_state_n:
                    ofs = ofs + 1
                    state_n = new_state_n
                else:
                    state = STATE_IGNORE_TO_EOL
            elif state == STATE_NUMBER:
                if ord(ch) >= 0x30 and ord(ch) <= 0x39:
                    recv_size = recv_size * 10 + ord(ch) - 0x30
                elif ch == b':':
                    state = STATE_RECV_DATA
                    data = b''
                else:
                    raise IOError('Length must end with :')

    # Access Point

    def scanForAccessPoints(self):
        return self.sendCmd(b'AT+CWLAP')

    def connectToAccessPoint(self, ssid_str, password_str):
        '''Call is ignored if already connected to the given access point. If
        already connected to another access point, the old access point is
        automatically disconnected first.'''
        current_ssid_str = self.getConnectedAccessPoint()
        if current_ssid_str == ssid_str:
            logging.info(
                'Already connected to access point: {}'.format(ssid_str))
            return
        if current_ssid_str != b'<NOT CONNECTED>':
            self.disconnectFromAccessPoint()
        self.setDeviceMode(1)
        self.sendCmd(b'AT+CWJAP_CUR="' + ssid_str +
                     b'","' + password_str + b'"')

    def disconnectFromAccessPoint(self):
        self.sendCmd(b'AT+CWQAP')

    def getConnectedAccessPoint(self):
        try:
            responseLines_list = self.sendCmd(b'AT+CWJAP_CUR?')
        except ESP8266ExceptionUnresolvable:
            return b'<NOT CONNECTED>'
        else:
            if responseLines_list[0] == b'No AP':
                return None
            m = re.match(rb'\+CWJAP_CUR:"(.*?)"', responseLines_list[0])
            return m.group(1)

    # CIP

    def startCip(self, protocol_str, host_str, port_int):
        self.sendCmd(b'AT+CIPMUX=0') # can only one connection at a time
        cmd = b'AT+CIPSTART="%s","%s",%d' % (protocol_str, host_str, port_int)
        self.sendCmd(cmd)

    def getCipStatus(self):
        responseLines_list = self._sendCmd(b'AT+CIPSTATUS')
        m = re.match(rb'STATUS:(\d)', responseLines_list[0])
        status_int = int(m.group(1)) - 1
        status_str = (b'<INVALID>', b'GOTIP', b'CONNECTED',
                      b'DISCONNECTED', b'<INVALID>')[status_int]
        protocol_str, host_str, port_int = None, None, None
        if len(responseLines_list) >= 2:
            m = re.match(rb'\+CIPSTATUS:0,"(.+)","(.+)",(\d+),0', responseLines_list[1])
            if m:
                protocol_str, host_str, port_int = m.group(
                    1), m.group(2), int(m.group(3))
        return status_str, protocol_str, host_str, port_int

    def closeCip(self):
        return self._sendCmd(b'AT+CIPCLOSE')

    # Device Mode

    def getDeviceMode(self):
        responseLines_list = self.sendCmd(b'AT+CWMODE?')
        m = re.match(rb'\+CWMODE:(\d)', responseLines_list[0])
        return int(m.group(1))

    def setDeviceMode(self, deviceMode_int):
        '''1=client, 2=AP, 3=both'''
        currentDeviceMode_int = self.getDeviceMode()
        if currentDeviceMode_int != deviceMode_int:
            self.sendCmd(b'AT+CWMODE=%d' % (deviceMode_int))

    # Misc

    def getIPAddress(self):
        responseLines_list = self.sendCmd(b'AT+CIFSR')
        return responseLines_list[0]

    #
    # Private.
    #

    def _sendCmd(self, cmd):
        self._sendStr(cmd)
        return self._getResponse()

    def _sendStr(self, s):
        logging.debug('> %s' % s.decode())
        self._serial.flushInput()
        self._serial.write(s + b'\r\n')
        # eat echo
        self._serial.readline()

    def _getResponse(self):
        responseLines_list = []

        while True:
            r = self._serial.readline()
            if not r:
                raise ESP8266Exception('Timeout. Possible partial response: {}'.format(
                    ' / '.format(responseLines_list)))
            r = r.strip()
            if r:
                logging.debug('< {}'.format(r.decode()))
                responseLines_list.append(r)
                if (r.lower() in self._successResponse_list):
                    return responseLines_list
                if (r.lower() in self._unresolvableFailureResponse_list):
                    raise ESP8266ExceptionUnresolvable(
                        'Failed with unresolvable response: {}'.format(' / '.format(responseLines_list)))

        raise ESP8266Exception('Failed with unknown response: {}'.format(
            ' / '.format(responseLines_list)))

    def _sendBuffer(self, s):
        assert len(s) <= MAX_CIPSEND_BUFFER_SIZE
        self._sendCmd(b'AT+CIPSEND=%d' % (len(s)))
        # eat the send prompt ("> ")
        #sendPrompt_str = self._serial.read(2)
        #assert sendPrompt_str == b'> '
        # send the buffer
        self._serial.write(s)
        return self._getResponse()
