use core::str::from_utf8_unchecked as utf8;

#[derive(Copy, Clone, Default, PartialEq)]
pub struct Longitude(pub i32);

impl Longitude {
    pub fn degrees(self) -> u8 {
        (self.0.abs() / 100_00000) as u8
    }

    pub fn minutes(self) -> u8 {
        ((self.0.abs() / 100000) % 100) as u8
    }

    pub fn seconds(self) -> u8 {
        ((self.0.abs() / 1000) % 100) as u8
    }

    pub fn sub_seconds(self) -> u16 {
        (self.0.abs() % 1000) as u16
    }

    pub fn is_east(self) -> bool {
        self.0 >= 0
    }

    pub fn is_west(self) -> bool {
        self.0 < 0
    }
}

impl From<&[u8]> for Longitude {
    fn from(bytes: &[u8]) -> Self {
        if bytes.len() == 0 {
            return Self::default();
        }
        let mut s = bytes.split(|&b| b == b'.');
        let mut integer = 0i32;
        if let Some(field) = s.next() {
            integer = unsafe { utf8(field) }.parse().unwrap_or(0);
        }
        let mut decimal = 0i32;
        if let Some(field) = s.next() {
            decimal = unsafe { utf8(field) }.parse().unwrap_or(0);
        }
        Self(integer * 100000 + decimal)
    }
}

impl core::fmt::Display for Longitude {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        let direction = if self.0 >= 0 { "E" } else { "W" };
        let degrees = self.degrees();
        let minutes = self.minutes();
        let seconds = self.seconds();
        let sub_seconds = self.sub_seconds();
        write!(f, "{}{:03}°{:02}'{:02}\"{:03}", direction, degrees, minutes, seconds, sub_seconds)
    }
}

impl core::fmt::Debug for Longitude {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self)
    }
}
