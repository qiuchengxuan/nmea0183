use core::str::from_utf8_unchecked as utf8;

#[derive(Copy, Clone, Default, PartialEq)]
pub struct Time {
    pub hour: u8,
    pub minute: u8,
    pub seconds: u8,
    pub sub_seconds: u8,
}

impl From<&[u8]> for Time {
    fn from(bytes: &[u8]) -> Self {
        if bytes.len() == 0 {
            return Self::default();
        }
        let mut splitted = bytes.split(|&b| b == b'.');
        let hhmmss = match splitted.next() {
            Some(field) => unsafe { utf8(field) },
            None => return Time::default(),
        };
        let sub_seconds = match splitted.next() {
            Some(field) => unsafe { utf8(field) },
            None => return Time::default(),
        };
        let hhmmss: u32 = hhmmss.parse().unwrap_or(0);
        let sub_seconds: u8 = sub_seconds.parse().unwrap_or(0);
        Time {
            hour: (hhmmss / 10000) as u8,
            minute: ((hhmmss / 100) % 100) as u8,
            seconds: (hhmmss % 100) as u8,
            sub_seconds,
        }
    }
}

impl core::fmt::Display for Time {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{:02}:{:02}:{:02}.{:02}", self.hour, self.minute, self.seconds, self.sub_seconds)
    }
}

impl core::fmt::Debug for Time {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self)
    }
}
