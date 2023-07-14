use chrono::NaiveDate;
use sqlx::PgPool;

use crate::errors::ApiErrors;
use crate::models::{GameAvailableDTO, GameStatus, NewGameDTO, PlatformAvailableDTO};
use crate::repository::game_available_repository;

use super::base::{
    handle_action_result, handle_already_exists_result, handle_get_list_result,
    handle_not_found_result, handle_result,
};
use super::{games_service, platforms_service};

pub async fn get_platform_games(
    pool: &PgPool,
    user_id: &str,
    platform_id: &str,
) -> Result<Vec<GameAvailableDTO>, ApiErrors> {
    platforms_service::exists_platform(pool, user_id, platform_id).await?;

    let find_result =
        game_available_repository::find_all_games_with_platform(pool, user_id, platform_id).await;
    handle_get_list_result(find_result)
}

pub async fn get_game_platforms(
    pool: &PgPool,
    user_id: &str,
    game_id: &str,
) -> Result<Vec<PlatformAvailableDTO>, ApiErrors> {
    games_service::exists_game(pool, user_id, game_id).await?;

    let find_result =
        game_available_repository::find_all_platforms_with_game(pool, user_id, game_id).await;
    handle_get_list_result(find_result)
}

pub async fn create_game_available(
    pool: &PgPool,
    user_id: &str,
    game_id: &str,
    platform_id: &str,
    available_date: NaiveDate,
) -> Result<(), ApiErrors> {
    let game = games_service::get_game(pool, user_id, game_id).await?;
    platforms_service::exists_platform(pool, user_id, platform_id).await?;

    let exists_result =
        game_available_repository::exists_by_id(pool, user_id, game_id, platform_id).await;
    handle_already_exists_result::<GameAvailableDTO>(exists_result)?;

    if game.status == GameStatus::Wishlist {
        games_service::update_game(
            pool,
            user_id,
            game_id,
            NewGameDTO {
                status: Some(GameStatus::NextUp),
                name: None,
                edition: None,
                release_year: None,
                rating: None,
                notes: None,
                save_folder: None,
                screenshot_folder: None,
                backup: None,
            },
        )
        .await?
    }

    let create_result =
        game_available_repository::create(pool, user_id, game_id, platform_id, available_date)
            .await;
    handle_action_result::<GameAvailableDTO>(create_result)
}

pub async fn delete_game_available(
    pool: &PgPool,
    user_id: &str,
    game_id: &str,
    platform_id: &str,
) -> Result<(), ApiErrors> {
    exists_game_available(pool, user_id, game_id, platform_id).await?;

    let delete_result =
        game_available_repository::delete_by_id(pool, user_id, game_id, platform_id).await;
    handle_action_result::<GameAvailableDTO>(delete_result)
}

pub async fn exists_game_available(
    pool: &PgPool,
    user_id: &str,
    game_id: &str,
    platform_id: &str,
) -> Result<(), ApiErrors> {
    let exists_result =
        game_available_repository::exists_by_id(pool, user_id, game_id, platform_id).await;
    handle_not_found_result::<GameAvailableDTO>(exists_result)
}

pub async fn exists_no_game_available(
    pool: &PgPool,
    user_id: &str,
    game_id: &str,
) -> Result<(), ApiErrors> {
    let exists_result =
        game_available_repository::exists_platforms_with_game(pool, user_id, game_id).await;
    let exists = handle_result::<bool, GameAvailableDTO>(exists_result)?;
    match exists {
        true => Err(ApiErrors::AlreadyExists(String::from(
            "Game has platforms available",
        ))),
        false => Ok(()),
    }
}
