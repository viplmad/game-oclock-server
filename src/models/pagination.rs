use serde::Serialize;
use utoipa::ToSchema;

use super::{
    DeviceDTO, GameDTO, GameWithFinishDTO, GameWithLogDTO, GenreDTO, LocationDTO, ModelInfo,
    TagDTO, UserDTO,
};

pub type GamePageResult = PageResultDTO<GameDTO>;
pub type GameWithFinishPageResult = PageResultDTO<GameWithFinishDTO>;
pub type GameWithLogPageResult = PageResultDTO<GameWithLogDTO>;
pub type LocationPageResult = PageResultDTO<LocationDTO>;
pub type GenrePageResult = PageResultDTO<GenreDTO>;
pub type DevicePageResult = PageResultDTO<DeviceDTO>;
pub type TagPageResult = PageResultDTO<TagDTO>;
pub type UserPageResult = PageResultDTO<UserDTO>;

#[derive(Serialize, ToSchema)]
pub struct PageResultDTO<T>
where
    T: ModelInfo,
{
    pub data: Vec<T>,
    pub page: u64,
    pub size: u64,
}
