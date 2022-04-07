use crate::types::date::Date;
use crate::types::latitude::Latitude;
use crate::types::longitude::Longitude;
use crate::types::position_mode::PositionMode;
use crate::types::time::Time;
use crate::types::{IntegerDecimal, Status};

#[derive(Clone, Default, Debug)]
pub struct RMC {
    pub time: Time,
    pub status: Status,
    pub latitude: Latitude,
    pub longitude: Longitude,
    /// knots
    pub speed: IntegerDecimal,
    pub course: IntegerDecimal,
    pub date: Date,
    pub heading: Option<IntegerDecimal>,
    pub position_mode: PositionMode,
}

impl From<&[u8]> for RMC {
    fn from(bytes: &[u8]) -> Self {
        let mut fields = bytes.split(|&b| b == b',');
        let time = Time::from(fields.next().unwrap());
        let status = Status::from(fields.next().unwrap());
        let mut latitude = Latitude::from(fields.next().unwrap());
        if fields.next().unwrap() == b"S" {
            latitude.0 = -latitude.0;
        }
        let mut longitude = Longitude::from(fields.next().unwrap());
        if fields.next().unwrap() == b"W" {
            longitude.0 = -longitude.0;
        }
        let speed: IntegerDecimal = fields.next().unwrap_or(b"0.0").into();
        let course: IntegerDecimal = fields.next().unwrap().into();
        let date: Date = fields.next().unwrap().into();
        let mut heading: Option<IntegerDecimal> = None;
        let field = fields.next().unwrap();
        let mvew = fields.next().unwrap();
        if field.len() > 0 {
            let mut value: IntegerDecimal = field.into();
            if mvew == b"W" {
                value += 180;
            }
            heading = Some(value);
        }
        let position_mode = PositionMode::from(fields.next().unwrap());
        Self { time, status, latitude, longitude, speed, course, date, heading, position_mode }
    }
}

mod test {
    #[test]
    fn test_rmc() {
        use super::RMC;

        let bytes = b"083559.00,A,4717.11437,N,00833.91522,E,0.004,77.52,091202,,,A,V*57";
        let rmc = RMC::from(&bytes[..]);
        assert_eq!("08:35:59.00", format!("{:?}", rmc.time));
        assert_eq!(r#"N47°17'11"437"#, format!("{:?}", rmc.latitude));
        assert_eq!(r#"E008°33'91"522"#, format!("{:?}", rmc.longitude));
        assert_eq!("0.4#3", format!("{:?}", rmc.speed));
        assert_eq!("77.52#2", format!("{:?}", rmc.course));
        assert_eq!("021209", format!("{:?}", rmc.date));
        assert_eq!("Autonomous", format!("{:?}", rmc.position_mode));

        let bytes = b"083559.00,,,,,,,,,,,,";
        let _ = RMC::from(&bytes[..]);
    }
}
