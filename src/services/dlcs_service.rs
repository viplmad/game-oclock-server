use sqlx::PgPool;

use crate::entities::DLC;
use crate::errors::{error_message_builder, ApiErrors};
use crate::models::{GameDTO, NewDLCDTO, QueryRequest, DLCDTO, UserDTO};
use crate::repository::dlc_repository;

use super::base::{
    create_merged, handle_already_exists_result, handle_create_result, handle_action_result,
    handle_get_list_result, handle_get_result, handle_not_found_result, handle_update_result,
    update_merged,
};
use super::games_service;

pub async fn get_dlc(pool: &PgPool, user_id: i32, dlc_id: i32) -> Result<DLCDTO, ApiErrors> {
    let find_result = dlc_repository::find_by_id(pool, user_id, dlc_id).await;
    handle_get_result(find_result)
}

pub async fn get_dlc_base_game(
    pool: &PgPool,
    user_id: i32,
    dlc_id: i32,
) -> Result<GameDTO, ApiErrors> {
    let dlc = get_dlc(pool, user_id, dlc_id).await?;
    let base_game_id = dlc.base_game_id.ok_or_else(|| {
        ApiErrors::InvalidParameter(error_message_builder::empty_param("Base game"))
    })?;
    games_service::get_game(pool, user_id, base_game_id).await
}

pub async fn get_game_dlcs(
    pool: &PgPool,
    user_id: i32,
    game_id: i32,
) -> Result<Vec<DLCDTO>, ApiErrors> {
    games_service::exists_game(pool, user_id, game_id).await?;

    let find_result = dlc_repository::find_all_by_base_game_id(pool, user_id, game_id).await;
    handle_get_list_result(find_result)
}

pub async fn get_dlcs(
    pool: &PgPool,
    user_id: i32,
    query: QueryRequest,
) -> Result<Vec<DLCDTO>, ApiErrors> {
    let find_result = dlc_repository::find_all(pool, user_id, query.limit.unwrap_or(10)).await;
    handle_get_list_result(find_result)
}

pub async fn create_dlc(pool: &PgPool, user_id: i32, dlc: NewDLCDTO) -> Result<DLCDTO, ApiErrors> {
    create_merged(
        dlc,
        async move |created_dlc_id: i32| get_dlc(pool, user_id, created_dlc_id).await,
        async move |dlc_to_create: DLC| {
            let exists_result =
                dlc_repository::exists_with_unique(pool, user_id, &dlc_to_create).await;
            handle_already_exists_result::<DLCDTO>(exists_result)?;

            exists_base_game(pool, user_id, dlc_to_create.base_game_id).await?;

            let create_result = dlc_repository::create(pool, user_id, &dlc_to_create).await;
            handle_create_result::<i32, DLCDTO>(create_result)
        },
    )
    .await
}

pub async fn update_dlc(
    pool: &PgPool,
    user_id: i32,
    dlc_id: i32,
    dlc: NewDLCDTO,
) -> Result<DLCDTO, ApiErrors> {
    update_merged(
        dlc,
        async move || get_dlc(pool, user_id, dlc_id).await,
        async move |dlc_to_update: DLC| {
            let exists_result =
                dlc_repository::exists_with_unique_except_id(pool, user_id, &dlc_to_update, dlc_id)
                    .await;
            handle_already_exists_result::<DLCDTO>(exists_result)?;

            exists_base_game(pool, user_id, dlc_to_update.base_game_id).await?;

            let update_result = dlc_repository::update(pool, user_id, dlc_id, &dlc_to_update).await;
            handle_update_result::<i32, DLCDTO>(update_result)
        },
    )
    .await
}

pub async fn update_dlc_base_game(
    pool: &PgPool,
    user_id: i32,
    dlc_id: i32,
    base_game_id: i32,
) -> Result<(), ApiErrors> {
    exists_dlc(pool, user_id, dlc_id).await?;
    games_service::exists_game(pool, user_id, base_game_id).await?;

    let update_result = dlc_repository::update_base_game_id(pool, user_id, dlc_id, base_game_id).await;
    handle_action_result::<UserDTO>(update_result)
}

pub async fn delete_dlc(pool: &PgPool, user_id: i32, dlc_id: i32) -> Result<(), ApiErrors> {
    exists_dlc(pool, user_id, dlc_id).await?;

    let delete_result = dlc_repository::delete_by_id(pool, user_id, dlc_id).await;
    handle_action_result::<DLCDTO>(delete_result)
}

pub async fn exists_dlc(
    pool: &PgPool,
    user_id: i32,
    dlc_id: i32,
) -> Result<(), ApiErrors> {
    let exists_result = dlc_repository::exists_by_id(pool, user_id, dlc_id).await;
    handle_not_found_result::<DLCDTO>(exists_result)
}

async fn exists_base_game(
    pool: &PgPool,
    user_id: i32,
    base_game: Option<i32>,
) -> Result<(), ApiErrors> {
    match base_game {
        Some(base_game_id) => games_service::exists_game(pool, user_id, base_game_id).await,
        None => Ok(()),
    }
}
