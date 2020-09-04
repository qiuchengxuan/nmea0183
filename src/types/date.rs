use core::str::from_utf8_unchecked as utf8;

#[derive(Copy, Clone, Default, PartialEq)]
pub struct Date {
    pub year: u8,
    pub month: u8,
    pub day: u8,
}

impl From<&[u8]> for Date {
    fn from(bytes: &[u8]) -> Self {
        let ddmmyy: u32 = unsafe { utf8(bytes) }.parse().unwrap_or(0);
        Date {
            year: (ddmmyy % 100) as u8,
            month: ((ddmmyy / 100) % 100) as u8,
            day: (ddmmyy / 10000) as u8,
        }
    }
}

impl core::fmt::Display for Date {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{:02}{:02}{:02}", self.year, self.month, self.day)
    }
}

impl core::fmt::Debug for Date {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self)
    }
}
