use serde::Serialize;
use utoipa::ToSchema;

use super::{
    DLCWithFinishDTO, GameDTO, GameWithFinishDTO, GameWithLogDTO, ModelInfo, PlatformDTO, TagDTO,
    DLCDTO,
};

#[derive(Serialize, ToSchema)]
#[aliases(DLCWithFinishSearchResult = SearchResultDTO<DLCWithFinishDTO>, GameSearchResult = SearchResultDTO<GameDTO>,
    GameWithFinishSearchResult = SearchResultDTO<GameWithFinishDTO>, GameWithLogSearchResult = SearchResultDTO<GameWithLogDTO>,
    PlatformSearchResult = SearchResultDTO<PlatformDTO>, TagSearchResult = SearchResultDTO<TagDTO>, DLCSearchResult = SearchResultDTO<DLCDTO>)]
pub struct SearchResultDTO<T>
where
    T: ModelInfo,
{
    pub data: Vec<T>,
    pub page: u64,
    pub size: u64,
}
