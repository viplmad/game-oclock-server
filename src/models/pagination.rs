use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use super::{
    DeviceDTO, DurationDef, LocationAvailableDTO, LocationDTO, MediaAvailableDTO, MediaDTO,
    MediaSessionDTO, MediaTagDTO, ModelInfo, SessionDTO, SessionStreakDTO, TagDTO, TagMediaDTO,
    UserDTO,
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
pub type SessionStreakPageResult = PageResultDTO<SessionStreakDTO>;

#[derive(Deserialize, Serialize, ToSchema)]
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

#[derive(ToSchema, Deserialize, Serialize)]
pub enum AggregateResultDTO {
    #[serde(untagged)]
    Integer(i64),
    #[serde(untagged)]
    #[schema(value_type = String)]
    Duration(DurationDef),
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct AggregateGroupResultDTO {
    pub key: AggregateGroupResultKeyDTO,
    pub value: AggregateGroupResultValueDTO,
}

#[derive(ToSchema, Deserialize, Serialize)]
pub enum AggregateGroupResultKeyDTO {
    #[serde(untagged)]
    Integer(i64),
    #[serde(untagged)]
    String(String),
}

#[derive(ToSchema, Deserialize, Serialize)]
pub enum AggregateGroupResultValueDTO {
    #[serde(untagged)]
    Simple(AggregateResultDTO),
    #[serde(untagged)]
    Sub(Vec<AggregateSubgroupResultDTO>),
}

#[derive(Deserialize, Serialize, ToSchema)]
pub struct AggregateSubgroupResultDTO {
    pub key: AggregateGroupResultKeyDTO,
    pub value: AggregateResultDTO,
}

#[cfg(test)]
mod tests {
    use crate::models::{
        AggregateGroupResultKeyDTO, AggregateGroupResultValueDTO, AggregateResultDTO,
        AggregateSubgroupResultDTO, DurationDef,
    };

    #[test]
    fn convert_integer() {
        let value = AggregateResultDTO::Integer(267);
        let a = serde_json::to_string::<AggregateResultDTO>(&value).unwrap();
        println!("{}", a);
        serde_json::from_str::<AggregateResultDTO>(&a).unwrap();
    }

    #[test]
    fn convert_duration() {
        let value = AggregateResultDTO::Duration(DurationDef::microseconds(10000));
        let a = serde_json::to_string::<AggregateResultDTO>(&value).unwrap();
        println!("{}", a);
        serde_json::from_str::<AggregateResultDTO>(&a).unwrap();
    }

    #[test]
    fn convert_group_key_integer() {
        let value = AggregateGroupResultKeyDTO::Integer(267);
        let a = serde_json::to_string::<AggregateGroupResultKeyDTO>(&value).unwrap();
        println!("{}", a);
        serde_json::from_str::<AggregateGroupResultKeyDTO>(&a).unwrap();
    }

    #[test]
    fn convert_group_key_string() {
        let value = AggregateGroupResultKeyDTO::String(String::from("test"));
        let a = serde_json::to_string::<AggregateGroupResultKeyDTO>(&value).unwrap();
        println!("{}", a);
        serde_json::from_str::<AggregateGroupResultKeyDTO>(&a).unwrap();
    }

    #[test]
    fn convert_group_simple() {
        let value = AggregateGroupResultValueDTO::Simple(AggregateResultDTO::Integer(234));
        let a = serde_json::to_string::<AggregateGroupResultValueDTO>(&value).unwrap();
        println!("{}", a);
        serde_json::from_str::<AggregateGroupResultValueDTO>(&a).unwrap();
    }

    #[test]
    fn convert_group_sub() {
        let value = AggregateGroupResultValueDTO::Sub(vec![AggregateSubgroupResultDTO {
            key: AggregateGroupResultKeyDTO::Integer(543),
            value: AggregateResultDTO::Duration(DurationDef::microseconds(123409)),
        }]);
        let a = serde_json::to_string::<AggregateGroupResultValueDTO>(&value).unwrap();
        println!("{}", a);
        serde_json::from_str::<AggregateGroupResultValueDTO>(&a).unwrap();
    }
}
