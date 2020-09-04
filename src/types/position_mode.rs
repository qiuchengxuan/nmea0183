#[derive(Copy, Clone, Debug)]
pub enum PositionMode {
    NoFix,

    Estimated,

    Autonomous,
    Differential,

    RealTimeKinematicFloat,
    RealTimeKinematicFixed,
}

impl Default for PositionMode {
    fn default() -> Self {
        Self::NoFix
    }
}

impl From<&[u8]> for PositionMode {
    fn from(bytes: &[u8]) -> Self {
        match bytes.first().map(|&b| b).unwrap_or(b'0') {
            b'0' => Self::NoFix,
            b'1' => Self::Autonomous,
            b'2' => Self::Differential,
            b'4' => Self::RealTimeKinematicFixed,
            b'5' => Self::RealTimeKinematicFloat,
            b'6' => Self::Estimated,

            b'N' => Self::NoFix,
            b'E' => Self::Estimated,
            b'F' => Self::RealTimeKinematicFloat,
            b'R' => Self::RealTimeKinematicFixed,
            b'A' => Self::Autonomous,
            b'D' => Self::Differential,
            _ => Self::NoFix,
        }
    }
}
