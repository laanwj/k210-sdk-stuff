/** ESP8285 serial WiFi network handler, for connecting to AP and making connections */
use core::{fmt, str};

use crate::response::{
    CmdResponse, ConnectionType, GenResponse, IPAddress, MACAddress, Response, Status,
};
use crate::traits::Write;
use crate::util::{write_num_u32, write_qstr};

/** Handler state */
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum State {
    Initial,
    SetStationMode,
    // QueryCurrentAP,
    ConnectingToAP,
    QueryIP,
    SetMux,
    MakeConnection(u32),
    Error,
    Idle,
    Sending(u32),
    RequestListen(u16),
}

/** Wifi network state */
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum WifiState {
    Unknown,
    Disconnected,
    Connected,
    GotIP,
}

/** Event type for callback */
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum NetworkEvent<'a> {
    /** Device initialization error */
    InitError,
    /** Network handler became idle */
    Ready,
    /** Error connecting to AP or querying IP information */
    Error,
    ConnectionEstablished(u32),
    ConnectionFailed(u32),
    Data(u32, &'a [u8]),
    ConnectionClosed(u32),
    SendComplete(u32),
    SendFailed(u32),
    ListenSuccess(IPAddress, u16),
    ListenFailed(u16),
}

/** Max CIPSEND buffer size */
const TX_BUFFER_SIZE: usize = 2048;
/** Max link_id */
const MAX_NUM_LINKS: usize = 5;

/** ESP8285 serial WiFi network handler */
pub struct SerialNetworkHandler<'a, S>
where
    S: Write,
{
    /** Serial port */
    port: &'a mut S,
    /** Handler state */
    state: State,
    /** Current AP connction state */
    wifistate: WifiState,
    /** Current IP */
    ip: Option<IPAddress>,
    /** Current MAC */
    mac: Option<MACAddress>,
    /** Access point name to connect to */
    apname: &'a [u8],
    /** Access point password */
    appass: &'a [u8],
    /** Send buffer */
    txbuf: [u8; TX_BUFFER_SIZE],
    /** Send buffer size */
    txn: usize,
    /** Connection slots (in use) */
    links: [bool; MAX_NUM_LINKS],
}

impl<'a, S> SerialNetworkHandler<'a, S>
where
    S: Write,
{
    pub fn new(port: &'a mut S, apname: &'a [u8], appass: &'a [u8]) -> Self {
        Self {
            port,
            state: State::Initial,
            wifistate: WifiState::Unknown,
            ip: None,
            mac: None,
            apname,
            appass,
            txbuf: [0; TX_BUFFER_SIZE],
            txn: 0,
            links: [false; MAX_NUM_LINKS],
        }
    }

    /** Start off network handling by checking liveness of the link and ESP device. */
    pub fn start(&mut self, echo: bool) -> Result<(), S::Error> {
        assert!(self.state == State::Initial || self.state == State::Idle);
        if echo {
            self.port.write_all(b"AT\r\n")?;
        } else {
            // Disable echo as very first thing to avoid excess serial traffic
            self.port.write_all(b"ATE0\r\n")?;
        }
        Ok(())
    }

    /** Handle an incoming message */
    pub fn message<F>(
        &mut self,
        resp: &Response,
        mut on_event: F,
        debug: &mut dyn fmt::Write,
    ) -> Result<(), S::Error>
    where
        F: FnMut(&mut Self, NetworkEvent, &mut dyn fmt::Write),
    {
        match resp {
            Response::Echo(data) => {
                writeln!(debug, "→ {}", str::from_utf8(data).unwrap_or("???")).unwrap();
            }
            Response::Data(link, d) => {
                writeln!(debug, "← Data({}, [...] {})", link, d.len()).unwrap();
            }
            _ => {
                writeln!(debug, "← {:?}", resp).unwrap();
            }
        }
        match self.state {
            State::Initial => match resp {
                Response::Gen(GenResponse::OK) => {
                    writeln!(debug, "Initial AT confirmed - configuring station mode").unwrap();
                    // Set station mode so that we're sure we can connect to an AP
                    self.port.write_all(b"AT+CWMODE_CUR=1\r\n").unwrap();
                    self.state = State::SetStationMode;
                }
                Response::Gen(GenResponse::FAIL) | Response::Gen(GenResponse::ERROR) => {
                    // TODO: retry if ERROR
                    writeln!(debug, "Fatal: Initial AT had unexpected result").unwrap();
                    on_event(self, NetworkEvent::InitError, debug);
                    self.state = State::Error;
                }
                _ => {}
            }
            State::SetStationMode => match resp {
                Response::Gen(GenResponse::OK) => {
                    writeln!(debug, "Station mode set - connecting to AP").unwrap();
                    self.port.write_all(b"AT+CWJAP_CUR=").unwrap();
                    write_qstr(self.port, self.apname)?;
                    self.port.write_all(b",")?;
                    write_qstr(self.port, self.appass)?;
                    self.port.write_all(b"\r\n")?;
                    self.state = State::ConnectingToAP;
                }
                Response::Gen(GenResponse::FAIL) | Response::Gen(GenResponse::ERROR) => {
                    writeln!(debug, "Fatal: failed to set station mode").unwrap();
                    self.state = State::Error;
                    on_event(self, NetworkEvent::Error, debug);
                }
                _ => {}
            }
            State::ConnectingToAP => match resp {
                Response::Gen(GenResponse::FAIL) | Response::Gen(GenResponse::ERROR) => {
                    writeln!(debug, "Fatal: failed to connect to AP").unwrap();
                    self.state = State::Error;
                    on_event(self, NetworkEvent::Error, debug);
                }
                Response::Gen(GenResponse::OK) => {
                    if self.wifistate != WifiState::GotIP {
                        writeln!(debug, "Warning: succesful but did not get IP yet").unwrap();
                    }
                    writeln!(debug, "Succesfully connected to AP").unwrap();
                    self.port.write_all(b"AT+CIFSR\r\n")?;
                    self.state = State::QueryIP;
                }
                _ => {}
            },
            State::QueryIP => match resp {
                Response::Gen(GenResponse::OK) => {
                    writeln!(debug, "Succesfully queried IP").unwrap();

                    // Enable multi-connection mode
                    self.port.write_all(b"AT+CIPMUX=1\r\n")?;
                    self.state = State::SetMux;
                }
                Response::Gen(GenResponse::FAIL) | Response::Gen(GenResponse::ERROR) => {
                    writeln!(debug, "Fatal: failed to query IP").unwrap();
                    self.state = State::Error;
                    on_event(self, NetworkEvent::Error, debug);
                }
                _ => {}
            }
            State::SetMux => match resp {
                Response::Gen(GenResponse::OK) => {
                    writeln!(debug, "Succesfully set multi-connection mode").unwrap();

                    self.state = State::Idle;
                    on_event(self, NetworkEvent::Ready, debug);
                }
                Response::Gen(GenResponse::FAIL) | Response::Gen(GenResponse::ERROR) => {
                    writeln!(debug, "Fatal: failed to set multi-connection mode").unwrap();
                    self.state = State::Error;
                    on_event(self, NetworkEvent::Error, debug);
                }
                _ => {}
            },
            State::MakeConnection(link) => match resp {
                Response::Gen(GenResponse::OK) => {
                    self.state = State::Idle;
                    on_event(self, NetworkEvent::ConnectionEstablished(link), debug);
                }
                Response::Gen(GenResponse::FAIL) | Response::Gen(GenResponse::ERROR) => {
                    self.state = State::Idle;
                    on_event(self, NetworkEvent::ConnectionFailed(link), debug);
                }
                _ => {}
            },
            State::Sending(link) => match resp {
                Response::Gen(GenResponse::OK) => {}
                Response::Gen(GenResponse::FAIL) | Response::Gen(GenResponse::ERROR) => {
                    self.state = State::Idle;
                    on_event(self, NetworkEvent::SendFailed(link), debug);
                }
                Response::RecvPrompt => {
                    // Send queued data
                    self.port.write_all(&self.txbuf[0..self.txn])?;
                    self.txn = 0;
                }
                Response::Status(Status::SEND_OK) => {
                    self.state = State::Idle;
                    on_event(self, NetworkEvent::SendComplete(link), debug);
                }
                _ => {}
            }
            State::RequestListen(port) => match resp {
                Response::Gen(GenResponse::OK) => {
                    self.state = State::Idle;
                    on_event(self, NetworkEvent::ListenSuccess(self.ip.unwrap(), port), debug);
                }
                Response::Gen(GenResponse::FAIL) | Response::Gen(GenResponse::ERROR) => {
                    self.state = State::Idle;
                    on_event(self, NetworkEvent::ListenFailed(port), debug);
                }
                _ => {}
            }
            _ => {}
        }
        match resp {
            Response::Status(Status::WIFI_DISCONNECT) => {
                writeln!(debug, "Disconnected from AP").unwrap();
                self.wifistate = WifiState::Disconnected;
                self.ip = None;
                self.mac = None;
            }
            Response::Status(Status::WIFI_CONNECTED) => {
                writeln!(debug, "Connected to AP").unwrap();
                self.wifistate = WifiState::Connected;
            }
            Response::Status(Status::WIFI_GOT_IP) => {
                writeln!(debug, "Have IP").unwrap();
                self.wifistate = WifiState::GotIP;
            }
            Response::Status(Status::CONNECT(link)) => {
                // Mark connection slot id as connected
                self.links[*link as usize] = true;
            }
            Response::Status(Status::CLOSED(link)) => {
                // Mark connection slot id as closed
                self.links[*link as usize] = false;
                on_event(self, NetworkEvent::ConnectionClosed(*link), debug);
            }
            Response::Cmd(CmdResponse::CIFSR_STAIP(ip)) => {
                self.ip = Some(*ip);
                writeln!(debug, "Queried IP: {}.{}.{}.{}", ip[0], ip[1], ip[2], ip[3]).unwrap();
            }
            Response::Cmd(CmdResponse::CIFSR_STAMAC(mac)) => {
                self.mac = Some(*mac);
                writeln!(
                    debug,
                    "Queried MAC: {:02x}:{:02x}:{:02x}:{:02x}:{:02x}:{:02x}",
                    mac[0], mac[1], mac[2], mac[3], mac[4], mac[5]
                )
                .unwrap();
            }
            Response::Data(link, data) => {
                on_event(self, NetworkEvent::Data(*link, data), debug);
            }
            _ => {}
        }
        Ok(())
    }

    /** Initiate a connection */
    pub fn connect(
        &mut self,
        ctype: ConnectionType,
        addr: &[u8],
        port: u32,
    ) -> Result<u32, S::Error> {
        assert!(self.state == State::Idle);
        // pick out a free link slot automatically
        let link = self.links.iter().position(|used| !used).unwrap() as u32;
        assert!(!self.links[link as usize]);
        self.port.write_all(b"AT+CIPSTART=")?;
        write_num_u32(self.port, link)?;
        self.port.write_all(b",")?;
        write_qstr(
            self.port,
            match ctype {
                ConnectionType::TCP => b"TCP",
                ConnectionType::UDP => b"UDP",
                ConnectionType::SSL => b"SSL",
            },
        )?;
        self.port.write_all(b",")?;
        write_qstr(self.port, addr)?;
        self.port.write_all(b",")?;
        write_num_u32(self.port, port)?;
        self.port.write_all(b"\r\n")?;
        self.state = State::MakeConnection(link);
        Ok(link)
    }

    /** Send contents of send buffer to a connection */
    pub fn send(&mut self, link: u32) -> Result<(), S::Error> {
        assert!(self.state == State::Idle);
        self.port.write_all(b"AT+CIPSEND=")?;
        write_num_u32(self.port, link)?;
        self.port.write_all(b",")?;
        write_num_u32(self.port, self.txn as u32)?;
        self.port.write_all(b"\r\n")?;
        self.state = State::Sending(link);
        Ok(())
    }

    /** Listen to connections on a port */
    pub fn listen(&mut self, port: u16) -> Result<(), S::Error> {
        assert!(self.state == State::Idle);
        // TODO: stop old listeners
        self.port.write_all(b"AT+CIPSERVER=1,")?;
        write_num_u32(self.port, port.into())?;
        self.port.write_all(b"\r\n")?;
        self.state = State::RequestListen(port);
        Ok(())
    }

    // TODO missing: disconnect, unlisten
}

/** Write trait for writing to send buffer */
impl<'a, S> Write for SerialNetworkHandler<'a, S>
where
    S: Write,
{
    type Error = ();

    fn write_all(&mut self, buf: &[u8]) -> Result<(), Self::Error> {
        assert!(self.state == State::Idle);
        if (self.txn + buf.len()) <= TX_BUFFER_SIZE {
            self.txbuf[self.txn..self.txn + buf.len()].copy_from_slice(buf);
            self.txn += buf.len();
            Ok(())
        } else {
            Err(())
        }
    }
}
