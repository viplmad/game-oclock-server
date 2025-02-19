use uuid::Uuid;

use crate::entities::{Location, LocationWithDate};
use crate::models::{LocationAvailableDTO, LocationDTO};

impl From<Location> for LocationDTO {
    fn from(location: Location) -> Self {
        Self {
            id: location.id,
            user_id: location.user_id,
            name: location.name,
            icon_url: location.icon_url,
            added_datetime: location.added_datetime,
            updated_datetime: location.updated_datetime,
        }
    }
}

impl From<LocationDTO> for Location {
    fn from(location: LocationDTO) -> Self {
        Self {
            id: Uuid::default(),
            user_id: Uuid::default(),
            name: location.name,
            icon_url: location.icon_url,
            added_datetime: location.added_datetime,
            updated_datetime: location.updated_datetime,
        }
    }
}

impl From<LocationWithDate> for LocationAvailableDTO {
    fn from(location: LocationWithDate) -> Self {
        Self {
            id: location.id,
            user_id: location.user_id,
            date: location.query_date,
            name: location.name,
            icon_url: location.icon_url,
            added_datetime: location.added_datetime,
            updated_datetime: location.updated_datetime,
        }
    }
}
