use std::fmt;
use std::str;
use std::time::Duration;

use clap::{App, AppSettings, Arg};
use serialport::prelude::*;

use esp8266at::handler::{NetworkEvent, SerialNetworkHandler};
use esp8266at::mainloop::mainloop;
use esp8266at::response::ConnectionType;
use esp8266at::traits::Write;

struct StdoutDebug {}
impl fmt::Write for StdoutDebug {
    fn write_str(&mut self, s: &str) -> Result<(), fmt::Error> {
        print!("{}", s);
        Ok(())
    }
}

fn main() {
    let matches = App::new("Serialport Example - Receive Data")
        .about("Reads data from a serial port and echoes it to stdout")
        .setting(AppSettings::DisableVersion)
        .arg(
            Arg::with_name("port")
                .help("The device path to a serial port")
                .use_delimiter(false)
                .required(true),
        )
        .arg(
            Arg::with_name("baud")
                .help("The baud rate to connect at")
                .use_delimiter(false)
                .required(true),
        )
        .arg(
            Arg::with_name("apname")
                .help("Access point name")
                .use_delimiter(false)
                .required(true),
        )
        .arg(
            Arg::with_name("appass")
                .help("Access point password")
                .use_delimiter(false)
                .required(true),
        )
        .get_matches();
    let port_name = matches.value_of("port").unwrap();
    let baud_rate = matches.value_of("baud").unwrap();
    let apname = matches.value_of("apname").unwrap();
    let appass = matches.value_of("appass").unwrap();

    let mut settings: SerialPortSettings = Default::default();
    settings.timeout = Duration::from_millis(10);
    if let Ok(rate) = baud_rate.parse::<u32>() {
        settings.baud_rate = rate.into();
    } else {
        eprintln!("Error: Invalid baud rate '{}' specified", baud_rate);
        ::std::process::exit(1);
    }

    match serialport::open_with_settings(&port_name, &settings) {
        Ok(mut tx) => {
            // Split into TX and RX halves
            let mut rx = tx.try_clone().unwrap();
            println!("Receiving data on {} at {} baud:", &port_name, &baud_rate);
            let mut sh = SerialNetworkHandler::new(&mut tx, apname.as_bytes(), appass.as_bytes());

            sh.start(true).unwrap();

            mainloop(&mut sh, &mut rx, |port, ev, debug| {
                match ev {
                    NetworkEvent::Ready => {
                        writeln!(debug, "--Ready--").unwrap();
                        port.connect(ConnectionType::TCP, b"wttr.in", 80).unwrap();
                        //port.connect(0, ConnectionType::SSL, b"wttr.in", 443);
                        true
                    }
                    NetworkEvent::Error => {
                        writeln!(debug, "--Could not connect to AP--").unwrap();
                        false
                    }
                    NetworkEvent::ConnectionEstablished(_) => {
                        port.write_all(b"GET /?0qA HTTP/1.1\r\nHost: wttr.in\r\nConnection: close\r\nUser-Agent: Weather-Spy\r\n\r\n").unwrap();
                        port.send(0).unwrap();
                        true
                    }
                    NetworkEvent::Data(_, data) => {
                        write!(debug, "{}", str::from_utf8(data).unwrap()).unwrap();
                        true
                    }
                    NetworkEvent::ConnectionClosed(_) => {
                        false
                    }
                    _ => { true }
                }
            }, &mut StdoutDebug {}).unwrap();
        }
        Err(e) => {
            eprintln!("Failed to open \"{}\". Error: {}", port_name, e);
            ::std::process::exit(1);
        }
    }
}
