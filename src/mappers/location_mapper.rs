use uuid::Uuid;

use crate::entities::Location;
use crate::models::LocationDTO;

impl From<Location> for LocationDTO {
    fn from(location: Location) -> Self {
        Self {
            id: location.id,
            name: location.name,
            image_url: location.image_url,
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
            image_url: location.image_url,
            added_datetime: location.added_datetime,
            updated_datetime: location.updated_datetime,
        }
    }
}
