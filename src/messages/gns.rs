use core::str::from_utf8_unchecked as utf8;

use crate::types::latitude::Latitude;
use crate::types::longitude::Longitude;
use crate::types::position_mode::PositionMode;
use crate::types::time::Time;
use crate::types::IntegerFloat;

#[derive(Clone, Default, Debug)]
pub struct GNS {
    pub time: Time,
    pub latitude: Latitude,
    pub longitude: Longitude,
    pub position_modes: [PositionMode; 4],
    pub num_satellites: u8,
    pub hdop: IntegerFloat<u8, u8>,
    pub altitude: IntegerFloat<i32, u8>,
}

impl From<&[u8]> for GNS {
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
        let field = fields.next().unwrap();
        let mut position_modes = [PositionMode::default(); 4];
        for i in 0..field.len() {
            position_modes[i] = PositionMode::from(&field[i..i + 1]);
        }
        let num_satellites: u8 = unsafe { utf8(fields.next().unwrap()) }.parse().unwrap_or(0);
        let hdop: IntegerFloat<u8, u8> = fields.next().unwrap().into();
        let altitude: IntegerFloat<i32, u8> = fields.next().unwrap().into();
        Self { time, latitude, longitude, position_modes, num_satellites, hdop, altitude }
    }
}

mod test {
    #[test]
    fn test_gns() {
        use super::GNS;

        let bytes = b"103600.01,5114.51176,N,00012.29380,W,ANNN,07,1.18,111.5,45.6,,,V";
        let gns = GNS::from(&bytes[..]);
        assert_eq!("10:36:00.01", format!("{:?}", gns.time));
        assert_eq!(r#"N51°14'51"176"#, format!("{:?}", gns.latitude));
        assert_eq!(r#"W000°12'29"380"#, format!("{:?}", gns.longitude));
        assert_eq!("[Autonomous, NoFix, NoFix, NoFix]", format!("{:?}", gns.position_modes));
        assert_eq!(7, gns.num_satellites);
        assert_eq!("1.18#2", format!("{:?}", gns.hdop));
        assert_eq!("111.5#1", format!("{:?}", gns.altitude));

        let bytes = b"103600.01,,,,,,,,,,,,V";
        GNS::from(&bytes[..]);
    }
}
