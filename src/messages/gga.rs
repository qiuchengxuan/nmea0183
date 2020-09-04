use core::str::from_utf8_unchecked as utf8;

use crate::types::latitude::Latitude;
use crate::types::longitude::Longitude;
use crate::types::time::Time;
use crate::types::{IntegerFloat, Quality};

#[derive(Clone, Default, Debug)]
pub struct GGA {
    pub time: Time,
    pub latitude: Latitude,
    pub longitude: Longitude,
    pub quality: Quality,
    pub num_satellites: u8,
    pub hdop: IntegerFloat<u8, u8>,
    pub altitude: IntegerFloat<i32, u8>,
}

impl From<&[u8]> for GGA {
    fn from(bytes: &[u8]) -> Self {
        if bytes.iter().fold(0, |sum, &b| sum + (b == b',') as usize) < 9 {
            return Self::default();
        }
        let mut fields = bytes.split(|&b| b == b',');
        let time = Time::from(fields.next().unwrap());
        let mut latitude = Latitude::from(fields.next().unwrap());
        if fields.next().unwrap() == b"S" {
            latitude.0 = -latitude.0;
        }
        let mut longitude = Longitude::from(fields.next().unwrap());
        if fields.next().unwrap() == b"W" {
            longitude.0 = -longitude.0;
        }
        let quality = Quality::from(fields.next().unwrap());
        let num_satellites: u8 = unsafe { utf8(fields.next().unwrap()) }.parse().unwrap_or(0);
        let hdop: IntegerFloat<u8, u8> = fields.next().unwrap().into();
        let altitude: IntegerFloat<i32, u8> = fields.next().unwrap().into();
        Self { time, latitude, longitude, quality, num_satellites, hdop, altitude }
    }
}

mod test {
    #[test]
    fn test_gga() {
        use super::GGA;

        let bytes = b"092725.00,4717.11399,N,00833.91590,E,1,08,1.01,499.6,M,48.0,M,,";
        let gga = GGA::from(&bytes[..]);
        assert_eq!("09:27:25.00", format!("{:?}", gga.time));
        assert_eq!(r#"N47°17'11"399"#, format!("{:?}", gga.latitude));
        assert_eq!(r#"E008°33'91"590"#, format!("{:?}", gga.longitude));
        assert_eq!("Autonomous", format!("{:?}", gga.quality));
        assert_eq!(8, gga.num_satellites);
        assert_eq!("1.1#2", format!("{:?}", gga.hdop));
        assert_eq!("499.6#1", format!("{:?}", gga.altitude));

        let bytes = b"092725.00,,,,,,,,,,,,,";
        GGA::from(&bytes[..]);
    }
}
