use serde::Serialize;
use utoipa::ToSchema;

use super::{
    DeviceDTO, DurationDef, LocationAvailableDTO, LocationDTO, MediaAvailableDTO, MediaDTO,
    MediaSessionDTO, MediaTagDTO, ModelInfo, SessionDTO, TagDTO, TagMediaDTO, UserDTO,
};

pub type DevicePageResult = PageResultDTO<DeviceDTO>;
pub type LocationPageResult = PageResultDTO<LocationDTO>;
pub type LocationAvailablePageResult = PageResultDTO<LocationAvailableDTO>;
pub type TagPageResult = PageResultDTO<TagDTO>;
pub type TagMediaPageResult = PageResultDTO<TagMediaDTO>;
pub type UserPageResult = PageResultDTO<UserDTO>;
pub type MediaPageResult = PageResultDTO<MediaDTO>;
pub type MediaSessionPageResult = PageResultDTO<MediaSessionDTO>;
pub type MediaAvailablePageResult = PageResultDTO<MediaAvailableDTO>;
pub type MediaTagPageResult = PageResultDTO<MediaTagDTO>;
pub type SessionPageResult = PageResultDTO<SessionDTO>;

#[derive(Serialize, ToSchema)]
pub struct PageResultDTO<T>
where
    T: ModelInfo,
{
    /// List of elements in the current page
    pub data: Vec<T>,
    /// Current page number (starting from 1)
    pub page: u64,
    /// Number of items per page
    pub size: u64,
}

#[derive(ToSchema)]
pub enum AggregateResultDTO {
    Integer(i64),
    #[schema(value_type = String)]
    Duration(DurationDef),
}

impl Serialize for AggregateResultDTO {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match &self {
            AggregateResultDTO::Integer(i) => i.serialize(serializer),
            AggregateResultDTO::Duration(d) => d.serialize(serializer),
        }
    }
}
