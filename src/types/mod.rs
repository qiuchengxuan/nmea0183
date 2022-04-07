use core::fmt::Debug;
use core::str::from_utf8_unchecked as utf8;

pub mod date;
pub mod latitude;
pub mod longitude;
pub mod position_mode;
pub mod time;

pub type Quality = position_mode::PositionMode;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum OperationMode {
    Auto,
    Manual,
}

impl Default for OperationMode {
    fn default() -> Self {
        Self::Manual
    }
}

impl From<&[u8]> for OperationMode {
    fn from(bytes: &[u8]) -> Self {
        match bytes.first().map(|&b| b).unwrap_or(b'M') {
            b'A' => Self::Auto,
            b'M' => Self::Manual,
            _ => Self::Manual,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum NavigationMode {
    NoFix,
    _3DFix,
    _2DFix,
}

impl Default for NavigationMode {
    fn default() -> Self {
        Self::NoFix
    }
}

impl From<&[u8]> for NavigationMode {
    fn from(bytes: &[u8]) -> Self {
        match bytes.first().map(|&b| b).unwrap_or(b'1') {
            b'1' => Self::NoFix,
            b'2' => Self::_2DFix,
            b'3' => Self::_3DFix,
            _ => Self::NoFix,
        }
    }
}

#[derive(Copy, Clone, Default, Debug, PartialEq)]
pub struct Status(pub bool);

impl From<&[u8]> for Status {
    fn from(bytes: &[u8]) -> Self {
        Self(bytes.first().map(|&b| b).unwrap_or(b'V') == b'A')
    }
}

#[derive(Copy, Clone, Default, PartialEq)]
pub struct IntegerDecimal(pub i32);

impl IntegerDecimal {
    pub fn new(value: i32, decimal_length: u8) -> Self {
        Self(value << 8 | decimal_length as i32)
    }

    pub fn real(self) -> i32 {
        self.0 >> 8
    }

    pub fn decimal_length(self) -> u8 {
        self.0 as u8
    }

    pub fn exp(self) -> u32 {
        let decimal_length = self.0 as u8;
        10_u32.pow(decimal_length as u32)
    }

    pub fn integer(self) -> i32 {
        let number = self.0 >> 8;
        number / self.exp() as i32
    }

    pub fn decimal(self) -> i32 {
        let number = self.0 >> 8;
        number % self.exp() as i32
    }
}

impl core::ops::AddAssign<i32> for IntegerDecimal {
    fn add_assign(&mut self, value: i32) {
        self.0 += value * self.exp() as i32
    }
}

impl Into<f32> for IntegerDecimal {
    fn into(self) -> f32 {
        let number = self.0 >> 8;
        number as f32 / self.exp() as f32
    }
}

impl Debug for IntegerDecimal {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}.{}#{}", self.integer(), self.decimal(), self.decimal_length())
    }
}

impl From<&[u8]> for IntegerDecimal {
    fn from(bytes: &[u8]) -> Self {
        if bytes.len() == 0 {
            return Self::default();
        }
        let mut splitted = bytes.split(|&b| b == b'.');
        let mut integer = 0;
        if let Some(field) = splitted.next() {
            integer = unsafe { utf8(field) }.parse().unwrap_or_default();
        }
        let mut decimal_length = 0;
        let mut decimal = 0;
        if let Some(field) = splitted.next() {
            decimal_length = core::cmp::min(field.len(), 255);
            decimal = unsafe { utf8(&field[..decimal_length]) }.parse().unwrap_or_default();
            if integer < 0 {
                decimal = -decimal
            }
        }
        let exp = 10_i32.pow(decimal_length as u32);
        Self::new(integer * exp + decimal, decimal_length as u8)
    }
}
