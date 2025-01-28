use serde::Serialize;
use utoipa::ToSchema;

use super::{
    DeviceDTO, GameDTO, GameWithFinishDTO, GameWithLogDTO, GenreDTO, LocationDTO, ModelInfo,
    TagDTO, UserDTO,
};

#[derive(Serialize, ToSchema)]
#[aliases(GamePageResult = PageResultDTO<GameDTO>, GameWithFinishPageResult = PageResultDTO<GameWithFinishDTO>,
    GameWithLogPageResult = PageResultDTO<GameWithLogDTO>, LocationPageResult = PageResultDTO<LocationDTO>,
    GenrePageResult = PageResultDTO<GenreDTO>, DevicePageResult = PageResultDTO<DeviceDTO>,
    TagPageResult = PageResultDTO<TagDTO>, UserPageResult = PageResultDTO<UserDTO>)]
pub struct PageResultDTO<T>
where
    T: ModelInfo,
{
    pub data: Vec<T>,
    pub page: u64,
    pub size: u64,
}
