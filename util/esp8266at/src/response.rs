/** Parser for ESP8266 AT responses */
use core::str;
use nom::Offset;
use nom::character::streaming::{digit1 as digit, hex_digit1 as hex_digit};

/** Connection type for CIPSTATUS etc */
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ConnectionType {
    TCP,
    UDP,
    SSL,
}

/** General command responses/statuses */
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GenResponse {
    /** Command finished with OK response */
    OK,
    /** Command finished with ERROR response */
    ERROR,
    /** Command finished with FAIL response */
    FAIL,
    /** Command could not be executed because device is busy sending */
    BUSY_S,
    /** Command could not be executed because device is busy handling previous command */
    BUSY_P,
}

/** Async status messages */
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Status {
    READY,
    WIFI_DISCONNECT,
    WIFI_CONNECTED,
    WIFI_GOT_IP,
    RECV_BYTES(u32),
    SEND_OK,
    /** TCP/UDP connection connected */
    CONNECT(u32),
    /** TCP/UDP connection closed */
    CLOSED(u32),
}

pub type IPAddress = [u8; 4];
pub type MACAddress = [u8; 6];

/** Specific command responses */
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CmdResponse<'a> {
    NO_AP,
    CWMODE(u32),
    CWJAP(u32),
    CWJAP_CUR(&'a [u8], &'a [u8], i32, i32),
    CIFSR_STAIP(IPAddress),
    CIFSR_STAMAC(MACAddress),
    STATUS(u32),
    ALREADY_CONNECTED,
    NO_CHANGE,
}

/** Parsed response */
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Response<'a> {
    Empty,
    Gen(GenResponse),
    Status(Status),
    Cmd(CmdResponse<'a>),
    Data(u32, &'a [u8]),
    Echo(&'a [u8]),
    RecvPrompt,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParseResult<'a> {
    Ok(usize, Response<'a>),
    Incomplete,
    Err,
}

/* Decimal unsigned integer */
named!(num_u32<&[u8], u32>,
    // The unwrap() here is safe because digit will never return non-UTF8
    map_res!(digit, |s| { str::from_utf8(s).unwrap().parse::<u32>() })
);

/* Decimal signed integer.
 * May start with unary minus.
 */
named!(num_i32<&[u8], i32>,
    // The unwrap() here is safe because digit will never return non-UTF8
    map_res!(
        recognize!(
            tuple!(
                opt!(one_of!("-")),
                digit
            )
        ),
        |s| { str::from_utf8(s).unwrap().parse::<i32>() })
);

/* Decimal byte */
named!(num_u8<&[u8], u8>,
    // The unwrap() here is safe because digit will never return non-UTF8
    map_res!(digit, |s| { str::from_utf8(s).unwrap().parse::<u8>() })
);

/* Hex byte */
named!(hex_u8<&[u8], u8>,
    // The unwrap() here is safe because digit will never return non-UTF8
    map_res!(hex_digit, |s| { u8::from_str_radix(str::from_utf8(s).unwrap(), 16) })
);

/* Quoted string */
named!(qstr<&[u8], &[u8]>,
    do_parse!(
        tag!(b"\"") >>
        a: escaped!(is_not!("\\\""), '\\', one_of!("\"\\")) >>
        //a: is_not!(b"\"") >>
        tag!(b"\"") >>
        ( a )
    )
);

/* Quoted IP */
named!(qip<&[u8], IPAddress>,
    do_parse!(
        tag!(b"\"") >>
        a: num_u8 >>
        tag!(b".") >>
        b: num_u8 >>
        tag!(b".") >>
        c: num_u8 >>
        tag!(b".") >>
        d: num_u8 >>
        tag!(b"\"") >>
        ( [a, b, c, d] )
    )
);

/* Quoted MAC address */
named!(qmac<&[u8], MACAddress>,
    do_parse!(
        tag!(b"\"") >>
        a: hex_u8 >>
        tag!(b":") >>
        b: hex_u8 >>
        tag!(b":") >>
        c: hex_u8 >>
        tag!(b":") >>
        d: hex_u8 >>
        tag!(b":") >>
        e: hex_u8 >>
        tag!(b":") >>
        f: hex_u8 >>
        tag!(b"\"") >>
        ( [a, b, c, d, e, f] )
    )
);

/* Parse general responses */
named!(genresponse<&[u8],GenResponse>,
    alt!(
          tag!(b"OK") => { |_| GenResponse::OK }
        | tag!(b"ERROR") => { |_| GenResponse::ERROR }
        | tag!(b"FAIL") => { |_| GenResponse::FAIL }
        | tag!(b"busy s...") => { |_| GenResponse::BUSY_S }
        | tag!(b"busy p...") => { |_| GenResponse::BUSY_P }
    )
);

/* Parse status messages */
named!(status<&[u8],Status>,
    alt!(
          //tag!(b"") => { |_| Status::EMPTY }
          tag!(b"ready") => { |_| Status::READY }
        | tag!(b"WIFI DISCONNECT") => { |_| Status::WIFI_DISCONNECT }
        | tag!(b"WIFI CONNECTED") => { |_| Status::WIFI_CONNECTED }
        | tag!(b"WIFI GOT IP") => { |_| Status::WIFI_GOT_IP }
        | tag!(b"SEND OK") => { |_| Status::SEND_OK }
        | do_parse!(
            tag!(b"Recv ") >>
            a: num_u32 >>
            tag!(b" bytes") >>
            ( Status::RECV_BYTES(a) )
        )
        | do_parse!(
            id: num_u32 >>
            tag!(b",") >>
            r: alt!(
                  tag!(b"CONNECT") => { |_| Status::CONNECT(id) }
                | tag!(b"CLOSED") => { |_| Status::CLOSED(id) }
            ) >>
            ( r )
        )
    )
);

/* Parse command-response messages */
named!(cmdresponse<&[u8],CmdResponse>,
    alt!(
        /* AT+CWJAP_CUR? */
          tag!(b"No AP") => { |_| CmdResponse::NO_AP }
        | do_parse!(
            tag!(b"+CWJAP_CUR:") >>
            a: qstr >>
            tag!(b",") >>
            b: qstr >>
            tag!(b",") >>
            c: num_i32 >>
            tag!(b",") >>
            d: num_i32 >>
            (CmdResponse::CWJAP_CUR(a,b,c,d))
        )
        /* AT+CWMODE? */
        | do_parse!(
            tag!(b"+CWMODE:") >>
            a: num_u32 >>
            (CmdResponse::CWMODE(a))
        )
        | do_parse!(
            tag!(b"+CWJAP:") >>
            a: num_u32 >>
            (CmdResponse::CWJAP(a))
        )
        | do_parse!(
            tag!(b"+CIFSR:STAIP,") >>
            a: qip >>
            (CmdResponse::CIFSR_STAIP(a))
        )
        | do_parse!(
            tag!(b"+CIFSR:STAMAC,") >>
            a: qmac >>
            (CmdResponse::CIFSR_STAMAC(a))
        )
        /* AT+CIPSTATUS */
        | do_parse!(
            tag!(b"STATUS:") >>
            a: num_u32 >>
            (CmdResponse::STATUS(a))
        )
        /* AT+CIPSTART */
        | tag!(b"ALREADY CONNECTED") => { |_| CmdResponse::ALREADY_CONNECTED }
        // DNS Fail
        /* AT+CIPSERVER */
        | tag!(b"no change") => { |_| CmdResponse::NO_CHANGE }
    )
);

/* Parse command-echo messages */
named!(cmdecho<&[u8],&[u8]>,
    recognize!(tuple!(
        tag!(b"AT"),
        take_until!("\r") // should be b"\r" but that gives a compiler error
    ))
);

/* Newline-terminated response */
named!(nl_terminated<&[u8],Response>,
    do_parse!(
        x: alt!(
              genresponse => { |x| Response::Gen(x) }
            | status => { |x| Response::Status(x) }
            | cmdresponse => { |x| Response::Cmd(x) }
            | cmdecho => { |x| Response::Echo(x) }
        ) >>
        tag!(b"\r\n") >>
        (x)
    )
);

/* Data response */
named!(ipd_data<&[u8],Response>,
    do_parse!(
        tag!(b"+IPD") >>
        tag!(b",") >>
        id: num_u32 >>
        tag!(b",") >>
        a: num_u32 >>
        tag!(b":") >>
        b: take!(a) >>
        ( Response::Data(id, b) )
    )
);

/* Parse response from line */
named!(parse_response<&[u8],Response>,
    alt!(
          nl_terminated
        | ipd_data
        | tag!(b"> ") => { |_| Response::RecvPrompt }
        | tag!(b"\r\n") => { |_| Response::Empty }
    )
);

pub fn parse(response: &[u8]) -> ParseResult {
    match parse_response(response) {
        Ok((residue, resp)) => ParseResult::Ok(response.offset(residue), resp),
        Err(nom::Err::Incomplete(_)) => ParseResult::Incomplete,
        Err(_) => ParseResult::Err,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(
            parse_response(b"AT\r\n"),
            Ok((&b""[..], Response::Echo(b"AT")))
        );
        assert_eq!(parse_response(b"\r\n"), Ok((&b""[..], Response::Empty)));
        assert_eq!(parse_response(b"> "), Ok((&b""[..], Response::RecvPrompt)));
        assert_eq!(
            parse_response(b"OK\r\n"),
            Ok((&b""[..], Response::Gen(GenResponse::OK)))
        );
    }
}
