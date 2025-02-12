use uuid::Uuid;

use crate::entities::Genre;
use crate::models::GenreDTO;

impl From<Genre> for GenreDTO {
    fn from(genre: Genre) -> Self {
        Self {
            id: genre.id.to_string(),
            name: genre.name,
            added_datetime: genre.added_datetime,
            updated_datetime: genre.updated_datetime,
        }
    }
}

impl From<GenreDTO> for Genre {
    fn from(genre: GenreDTO) -> Self {
        Self {
            id: Uuid::default(),
            user_id: Uuid::default(),
            name: genre.name,
            added_datetime: genre.added_datetime,
            updated_datetime: genre.updated_datetime,
        }
    }
}
