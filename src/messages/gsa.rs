use crate::types::{NavigationMode, OperationMode};

#[derive(Clone, Default, Debug)]
pub struct GSA {
    pub operation_mode: OperationMode,
    pub navigation_mode: NavigationMode,
}

impl From<&[u8]> for GSA {
    fn from(bytes: &[u8]) -> Self {
        if bytes.iter().fold(0, |sum, &b| sum + (b == b',') as usize) < 1 {
            return Self::default();
        }
        let mut fields = bytes.split(|&b| b == b',');
        let operation_mode = OperationMode::from(fields.next().unwrap());
        let navigation_mode = NavigationMode::from(fields.next().unwrap());
        Self { operation_mode, navigation_mode }
    }
}

mod test {
    #[test]
    fn test_gsa() {
        use super::GSA;

        let bytes = b"A,3,23,29,07,08,09,18,26,28,,,,,1.94,1.18,1.54,1";
        let gsa = GSA::from(&bytes[..]);
        assert_eq!("Auto", format!("{:?}", gsa.operation_mode));
        assert_eq!("_3DFix", format!("{:?}", gsa.navigation_mode));
    }
}
