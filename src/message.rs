use core::str::from_utf8_unchecked;

use crate::messages::gga::GGA;
use crate::messages::gns::GNS;
use crate::messages::gsa::GSA;
use crate::messages::rmc::RMC;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum SentenceFormatter {
    GGA = 0,
    GNS,
    GSA,
    RMC,
}

impl SentenceFormatter {
    pub fn try_from(bytes: &[u8]) -> Option<SentenceFormatter> {
        match bytes {
            b"GGA" => Some(Self::GGA),
            b"GNS" => Some(Self::GNS),
            b"GSA" => Some(Self::GSA),
            b"RMC" => Some(Self::RMC),
            _ => None,
        }
    }
}

#[derive(Clone, Debug)]
pub enum Message {
    GNS(GNS),
    GGA(GGA),
    GSA(GSA),
    RMC(RMC),
}

impl Message {
    pub fn try_from(line: &[u8]) -> Option<Message> {
        let mut splitted = line.rsplitn(2, |&b| b == b'*');
        let checksum = match splitted.next() {
            Some(c) => u8::from_str_radix(unsafe { from_utf8_unchecked(c) }, 16).unwrap_or(0),
            None => return None,
        };

        let payload = match splitted.next() {
            Some(v) => v,
            None => return None,
        };

        if payload.iter().fold(0, |csum, &b| csum ^ b) != checksum {
            return None;
        }

        let mut splitted = payload.splitn(2, |&b| b == b',');

        let address = splitted.next().unwrap();
        let value = match splitted.next() {
            Some(v) => v,
            None => return None,
        };

        match &address[2..] {
            b"GGA" => Some(Message::GGA(GGA::from(value))),
            b"GNS" => Some(Message::GNS(GNS::from(value))),
            b"GSA" => Some(Message::GSA(GSA::from(value))),
            b"RMC" => Some(Message::RMC(RMC::from(value))),
            _ => None,
        }
    }
}
