use sqlx::PgPool;

use crate::entities::DLCSearch;
use crate::errors::{error_message_builder, ApiErrors};
use crate::models::{DLCPageResult, GameDTO, NewDLCDTO, SearchDTO, DLCDTO};
use crate::providers::ImageClientProvider;
use crate::repository::dlc_repository;

use super::base::{
    create_merged, handle_action_result, handle_already_exists_result, handle_create_result,
    handle_get_list_paged_result, handle_get_list_result, handle_get_result,
    handle_not_found_result, handle_query_mapping, handle_update_result, update_merged,
};
use super::{dlc_image_service, games_service};

pub async fn get_dlc(pool: &PgPool, user_id: &str, dlc_id: &str) -> Result<DLCDTO, ApiErrors> {
    let find_result = dlc_repository::find_by_id(pool, user_id, dlc_id).await;
    handle_get_result(find_result)
}

pub async fn get_dlc_base_game(
    pool: &PgPool,
    user_id: &str,
    dlc_id: &str,
) -> Result<GameDTO, ApiErrors> {
    let dlc = get_dlc(pool, user_id, dlc_id).await?;
    let base_game_id = dlc.base_game_id.ok_or_else(|| {
        ApiErrors::InvalidParameter(error_message_builder::empty_param("DLC base game"))
    })?;
    games_service::get_game(pool, user_id, &base_game_id).await
}

pub async fn get_game_dlcs(
    pool: &PgPool,
    user_id: &str,
    game_id: &str,
) -> Result<Vec<DLCDTO>, ApiErrors> {
    games_service::exists_game(pool, user_id, game_id).await?;

    let find_result = dlc_repository::find_all_by_base_game_id(pool, user_id, game_id).await;
    handle_get_list_result(find_result)
}

pub async fn search_dlcs(
    pool: &PgPool,
    user_id: &str,
    search: SearchDTO,
    quicksearch: Option<String>,
) -> Result<DLCPageResult, ApiErrors> {
    let search = handle_query_mapping::<DLCDTO, DLCSearch>(search, quicksearch)?;
    let find_result = dlc_repository::search_all(pool, user_id, search).await;
    handle_get_list_paged_result(find_result)
}

pub async fn create_dlc(pool: &PgPool, user_id: &str, dlc: NewDLCDTO) -> Result<DLCDTO, ApiErrors> {
    create_merged(
        dlc,
        async move |created_dlc_id| get_dlc(pool, user_id, &created_dlc_id).await,
        async move |dlc_to_create| {
            let exists_result =
                dlc_repository::exists_with_unique(pool, user_id, &dlc_to_create).await;
            handle_already_exists_result::<DLCDTO>(exists_result)?;

            exists_base_game(
                pool,
                user_id,
                &dlc_to_create.base_game_id.map(|id| id.to_string()),
            )
            .await?;

            let create_result = dlc_repository::create(pool, user_id, &dlc_to_create).await;
            handle_create_result::<String, DLCDTO>(create_result)
        },
    )
    .await
}

pub async fn update_dlc(
    pool: &PgPool,
    user_id: &str,
    dlc_id: &str,
    dlc: NewDLCDTO,
) -> Result<(), ApiErrors> {
    update_merged(
        dlc,
        async move || get_dlc(pool, user_id, dlc_id).await,
        async move |dlc_to_update| {
            let exists_result =
                dlc_repository::exists_with_unique_except_id(pool, user_id, &dlc_to_update, dlc_id)
                    .await;
            handle_already_exists_result::<DLCDTO>(exists_result)?;

            exists_base_game(
                pool,
                user_id,
                &dlc_to_update.base_game_id.map(|id| id.to_string()),
            )
            .await?;

            let update_result =
                dlc_repository::update_by_id(pool, user_id, dlc_id, &dlc_to_update).await;
            handle_update_result::<DLCDTO>(update_result)
        },
    )
    .await
}

pub async fn set_dlc_base_game(
    pool: &PgPool,
    user_id: &str,
    dlc_id: &str,
    base_game_id: Option<String>,
) -> Result<(), ApiErrors> {
    exists_dlc(pool, user_id, dlc_id).await?;

    if let Some(game_id) = &base_game_id {
        games_service::exists_game(pool, user_id, game_id).await?;
    }

    let update_result =
        dlc_repository::update_base_game_id(pool, user_id, dlc_id, base_game_id).await;
    handle_action_result::<DLCDTO>(update_result)
}

pub async fn delete_dlc(
    pool: &PgPool,
    image_client_provider: &ImageClientProvider,
    user_id: &str,
    dlc_id: &str,
) -> Result<(), ApiErrors> {
    let dlc = get_dlc(pool, user_id, dlc_id).await?;

    if let Some(cover_filename) = &dlc.cover_filename {
        let delete_cover_result =
            dlc_image_service::delete_dlc_cover(image_client_provider, cover_filename).await;
        if delete_cover_result.is_err() {
            log::warn!("DLC deletion - Image client could not delete DLC with image.")
        }
    }

    let delete_result = dlc_repository::delete_by_id(pool, user_id, dlc_id).await;
    handle_action_result::<DLCDTO>(delete_result)
}

pub async fn set_dlc_cover(
    pool: &PgPool,
    image_client_provider: &ImageClientProvider,
    user_id: &str,
    dlc_id: &str,
    file_path: &str,
) -> Result<(), ApiErrors> {
    exists_dlc(pool, user_id, dlc_id).await?;
    let filename =
        dlc_image_service::set_dlc_cover(image_client_provider, user_id, dlc_id, file_path).await?;
    set_dlc_cover_filename(pool, user_id, dlc_id, Some(filename)).await
}

pub async fn rename_dlc_cover(
    pool: &PgPool,
    image_client_provider: &ImageClientProvider,
    user_id: &str,
    dlc_id: &str,
    new_name: &str,
) -> Result<(), ApiErrors> {
    let old_filename = get_dlc_cover_filename(pool, user_id, dlc_id).await?;
    let new_filename = dlc_image_service::rename_dlc_cover(
        image_client_provider,
        user_id,
        dlc_id,
        &old_filename,
        new_name,
    )
    .await?;
    set_dlc_cover_filename(pool, user_id, dlc_id, Some(new_filename)).await
}

pub async fn delete_dlc_cover(
    pool: &PgPool,
    image_client_provider: &ImageClientProvider,
    user_id: &str,
    dlc_id: &str,
) -> Result<(), ApiErrors> {
    let filename = get_dlc_cover_filename(pool, user_id, dlc_id).await?;
    dlc_image_service::delete_dlc_cover(image_client_provider, &filename).await?;
    set_dlc_cover_filename(pool, user_id, dlc_id, Option::<String>::None).await
}

pub async fn exists_dlc(pool: &PgPool, user_id: &str, dlc_id: &str) -> Result<(), ApiErrors> {
    let exists_result = dlc_repository::exists_by_id(pool, user_id, dlc_id).await;
    handle_not_found_result::<DLCDTO>(exists_result)
}

async fn exists_base_game(
    pool: &PgPool,
    user_id: &str,
    base_game: &Option<String>,
) -> Result<(), ApiErrors> {
    match base_game {
        Some(base_game_id) => games_service::exists_game(pool, user_id, base_game_id).await,
        None => Ok(()),
    }
}

async fn get_dlc_cover_filename(
    pool: &PgPool,
    user_id: &str,
    dlc_id: &str,
) -> Result<String, ApiErrors> {
    let dlc = get_dlc(pool, user_id, dlc_id).await?;
    dlc.cover_filename
        .ok_or_else(|| ApiErrors::InvalidParameter(error_message_builder::empty_param("DLC cover")))
}

async fn set_dlc_cover_filename(
    pool: &PgPool,
    user_id: &str,
    dlc_id: &str,
    filename: Option<String>,
) -> Result<(), ApiErrors> {
    let update_result =
        dlc_repository::update_cover_filename_by_id(pool, user_id, dlc_id, filename).await;
    handle_action_result::<DLCDTO>(update_result)
}
