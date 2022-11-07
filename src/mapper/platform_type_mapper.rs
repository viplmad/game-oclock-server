use crate::models::PlatformType;

impl TryFrom<i16> for PlatformType {
    type Error = ();

    fn try_from(ptype: i16) -> Result<Self, Self::Error> {
        match ptype {
            0 => Ok(PlatformType::Physical),
            1 => Ok(PlatformType::Digital),
            _ => Err(()),
        }
    }
}

impl From<PlatformType> for i16 {
    fn from(ptype: PlatformType) -> Self {
        match ptype {
            PlatformType::Physical => 0,
            PlatformType::Digital => 1,
        }
    }
}
