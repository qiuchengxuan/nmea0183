#![cfg_attr(not(test), no_std)]

extern crate micromath;

#[cfg(test)]
#[macro_use]
extern crate pretty_assertions;

pub mod message;
pub mod messages;
pub mod types;

pub use message::{Message, SentenceFormatter};

const MAX_MESSAGE_SIZE: usize = 79;

pub struct Parser {
    buffer: [u8; MAX_MESSAGE_SIZE],
    index: usize,
    enabled: u32,
}

impl Parser {
    pub fn new() -> Self {
        Self { buffer: [0u8; MAX_MESSAGE_SIZE], index: 0, enabled: u32::MAX }
    }

    pub fn with_enables(enableds: impl AsRef<[SentenceFormatter]>) -> Self {
        let mut enabled: u32 = 0;
        for &message in enableds.as_ref().iter() {
            enabled |= 1 << (message as usize);
        }
        Self { buffer: [0u8; MAX_MESSAGE_SIZE], index: 0, enabled }
    }

    fn parse_line(&mut self, line: &[u8]) -> Option<Message> {
        let mut line = line;
        if !line.starts_with(b"$") || !line.ends_with(b"\r") {
            if line.len() > self.buffer.len() - self.index {
                self.index = 0;
                return None;
            }
            self.buffer[self.index..self.index + line.len()].copy_from_slice(line);
            self.index += line.len();
            line = &self.buffer[..self.index];
        }

        if !line.ends_with(b"\r") {
            return None;
        }
        self.index = 0;

        if line.len() < 7 {
            return None;
        }

        let option = SentenceFormatter::try_from(&line[3..6]);
        if !option.map(|f| (1 << f as usize) & self.enabled > 0).unwrap_or(false) {
            return None;
        }

        Message::try_from(&line[1..line.len() - 1])
    }

    pub fn parse_bytes<'a>(&'a mut self, bytes: &'a [u8]) -> impl Iterator<Item = Message> + 'a {
        bytes
            .split(|&b| b == b'\n')
            .map(move |line| self.parse_line(line))
            .filter(|o| o.is_some())
            .map(|o| o.unwrap())
    }
}

mod test {
    #[test]
    fn test_parser() {
        use super::Parser;
        use crate::message::Message;

        let bytes = b"blablabla$GPGLL,4717.11364,N,00833.91565,E,092321.00,A,A*60\r\n\
                      $GPGGA,092725.00,4717.11399,N,00833.91590,E,1,08,1.01,499.6,M,48.0,M,,*5B\r\n\
                      $GNGNS,103600.01,5114.51176,N,00012.29380,W,ANNN,07,1.18,111.5,45.6,,,V*00\r\n\
                      $GPRMC,083559.00,A,4717.11437,N,00833.91522,E,0.004,77.52,091202,,,A,V*2D\r\n\
                      $GPGLL,4717.11364,N,00833.91565,E,092321.00,A,A*60\r\n";
        let mut parser = Parser::new();
        {
            let mut messages = parser.parse_bytes(bytes);
            match messages.next().unwrap() {
                Message::GGA(gga) => assert_eq!("499.6#1", format!("{:?}", gga.altitude)),
                _ => panic!(),
            }
            match messages.next().unwrap() {
                Message::GNS(gns) => assert_eq!("111.5#1", format!("{:?}", gns.altitude)),
                _ => panic!(),
            }
            match messages.next().unwrap() {
                Message::RMC(rmc) => assert_eq!("0.4#3", format!("{:?}", rmc.speed)),
                _ => panic!(),
            }
            assert!(messages.next().is_none());
        }

        {
            assert!(parser.parse_bytes(&bytes[0..64]).next().is_none());
        }
        {
            assert!(parser.parse_bytes(&bytes[64..128]).next().is_none());
        }
        {
            let mut results = parser.parse_bytes(&bytes[128..192]);
            assert!(results.next().is_some());
            assert!(results.next().is_none());
        }
        {
            let mut results = parser.parse_bytes(&bytes[192..256]);
            assert!(results.next().is_some());
            assert!(results.next().is_none());
        }
        {
            let mut results = parser.parse_bytes(&bytes[256..320]);
            assert!(results.next().is_some());
            assert!(results.next().is_none());
        }
    }
}
