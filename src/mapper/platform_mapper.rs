use uuid::Uuid;

use crate::entities::{Platform, PlatformWithDate};
use crate::models::{PlatformAvailableDTO, PlatformDTO, PlatformType};

impl From<Platform> for PlatformDTO {
    fn from(platform: Platform) -> Self {
        Self {
            id: platform.id.to_string(),
            name: platform.name,
            ptype: platform.ptype.map(|ptype| {
                PlatformType::try_from(ptype).expect("Type was not within valid range")
            }),
            icon_filename: platform.icon_filename,
            icon_url: None,
            added_datetime: platform.added_datetime,
            updated_datetime: platform.updated_datetime,
        }
    }
}

impl From<PlatformDTO> for Platform {
    fn from(platform: PlatformDTO) -> Self {
        Self {
            id: Uuid::default(),
            user_id: Uuid::default(),
            name: platform.name,
            ptype: platform.ptype.map(i16::from),
            icon_filename: platform.icon_filename,
            added_datetime: platform.added_datetime,
            updated_datetime: platform.updated_datetime,
        }
    }
}

impl From<PlatformWithDate> for PlatformAvailableDTO {
    fn from(platform: PlatformWithDate) -> Self {
        Self {
            id: platform.id.to_string(),
            available_date: platform.query_date,
            name: platform.name,
            ptype: platform.ptype.map(|ptype| {
                PlatformType::try_from(ptype).expect("Type was not within valid range")
            }),
            icon_filename: platform.icon_filename,
            icon_url: None,
            added_datetime: platform.added_datetime,
            updated_datetime: platform.updated_datetime,
        }
    }
}
