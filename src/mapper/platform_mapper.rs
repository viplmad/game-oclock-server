use crate::entities::{Platform, PlatformAvailable};
use crate::models::{PlatformAvailableDTO, PlatformDTO, PlatformType};

impl From<Platform> for PlatformDTO {
    fn from(platform: Platform) -> Self {
        Self {
            id: platform.id,
            user_id: platform.user_id,
            name: platform.name,
            ptype: platform._type.map(|ptype| {
                PlatformType::try_from(ptype).expect("Type was not within valid range")
            }),
            icon_filename: platform.icon_filename,
            added_datetime: platform.added_datetime,
            updated_datetime: platform.updated_datetime,
        }
    }
}

impl From<PlatformDTO> for Platform {
    fn from(platform: PlatformDTO) -> Self {
        Self {
            id: platform.id,
            user_id: platform.user_id,
            name: platform.name,
            _type: platform.ptype.map(i16::from),
            icon_filename: platform.icon_filename,
            added_datetime: platform.added_datetime,
            updated_datetime: platform.updated_datetime,
        }
    }
}

impl From<PlatformAvailable> for PlatformAvailableDTO {
    fn from(platform: PlatformAvailable) -> Self {
        Self {
            id: platform.id,
            user_id: platform.user_id,
            available_date: platform.available_date,
            name: platform.name,
            ptype: platform._type.map(|ptype| {
                PlatformType::try_from(ptype).expect("Type was not within valid range")
            }),
            icon_filename: platform.icon_filename,
            added_datetime: platform.added_datetime,
            updated_datetime: platform.updated_datetime,
        }
    }
}
