use serde::Serialize;
use utoipa::ToSchema;

use super::{
    DLCWithFinishDTO, GameDTO, GameWithFinishDTO, GameWithLogDTO, ModelInfo, PlatformDTO, TagDTO,
    UserDTO, DLCDTO,
};

#[derive(Serialize, ToSchema)]
#[aliases(DLCWithFinishPageResult = PageResultDTO<DLCWithFinishDTO>, GamePageResult = PageResultDTO<GameDTO>,
    GameWithFinishPageResult = PageResultDTO<GameWithFinishDTO>, GameWithLogPageResult = PageResultDTO<GameWithLogDTO>,
    PlatformPageResult = PageResultDTO<PlatformDTO>, TagPageResult = PageResultDTO<TagDTO>, UserPageResult = PageResultDTO<UserDTO>,
    DLCPageResult = PageResultDTO<DLCDTO>)]
pub struct PageResultDTO<T>
where
    T: ModelInfo,
{
    pub data: Vec<T>,
    pub page: u64,
    pub size: u64,
}
