use core::fmt::{Debug, Display};
use core::num::ParseIntError;
use core::str::from_utf8_unchecked as utf8;
use core::str::FromStr;

#[allow(unused_imports)] // false warning
use micromath::F32Ext;

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
pub struct IntegerFloat<I, D> {
    pub integer: I,
    pub decimal: D,
    pub decimal_length: u8,
}

macro_rules! impl_into_f32 {
    () => {};
    (, ($integer_type:tt, $decimal_type:tt) $(, ($integer_types:tt, $decimal_types:tt))*) => {
        impl_into_f32!{ ($integer_type, $decimal_type) $(, ($integer_types, $decimal_types))* }
    };
    (($integer_type:tt, $decimal_type:tt) $(, ($integer_types:tt, $decimal_types:tt))*) => {
        impl Into<f32> for IntegerFloat<$integer_type, $decimal_type> {
            fn into(self) -> f32 {
                let integer = self.integer as f32;
                let decimal = (self.decimal as f32).copysign(integer);
                integer + decimal * 0.1f32.powf(self.decimal_length as f32)
            }
        }

        impl_into_f32!{ $(, ($integer_types, $decimal_types))* }
    };
}

impl_into_f32! { (u8, u8), (u16, u8), (i16, u8), (i32, u8), (i32, u16) }

impl<I: Display, D: Display> Debug for IntegerFloat<I, D> {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}.{}#{}", self.integer, self.decimal, self.decimal_length)
    }
}

macro_rules! impl_from_str {
    () => {};
    (, ($type:tt -> $length:expr) $(, ($types:tt -> $lengths:expr))*) => {
        impl_from_str!{ ($type -> $length) $(, ($types -> $lengths))* }
    };
    (($type:tt -> $length:expr) $(, ($types:tt -> $lengths:expr))*) => {
        impl<I: FromStr<Err = ParseIntError> + Default> From<&[u8]> for IntegerFloat<I, $type> {
            fn from(bytes: &[u8]) -> Self {
                if bytes.len() == 0 {
                    return Self::default();
                }
                let mut splitted = bytes.split(|&b| b == b'.');
                let mut integer = I::default();
                if let Some(field) = splitted.next() {
                    integer = unsafe { utf8(field) }.parse().unwrap_or_default();
                }
                let mut decimal_length = 0;
                let mut decimal = $type::default();
                if let Some(field) = splitted.next() {
                    decimal_length = core::cmp::min(field.len(), $length);
                    decimal = unsafe { utf8(&field[..decimal_length]) }.parse().unwrap_or_default();
                }
                Self {
                    integer,
                    decimal,
                    decimal_length: decimal_length as u8,
                }
            }
        }

        impl_from_str!{ $(, ($types -> $lengths))* }
    };
}

impl_from_str! { (u8 -> 2), (u16 -> 4) }
