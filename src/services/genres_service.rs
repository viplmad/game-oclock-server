use sqlx::PgPool;

use crate::entities::GenreSearch;
use crate::errors::ApiErrors;
use crate::models::{GenreDTO, GenrePageResult, NewGenreDTO, SearchDTO};
use crate::repository::genre_repository;

use super::base::{
    create_merged, handle_action_result, handle_already_exists_result, handle_create_result,
    handle_get_list_paged_result, handle_get_result, handle_not_found_result, handle_query_mapping,
    handle_update_result, update_merged,
};

pub async fn get_genre(
    pool: &PgPool,
    user_id: &str,
    genre_id: &str,
) -> Result<GenreDTO, ApiErrors> {
    let find_result = genre_repository::find_by_id(pool, user_id, genre_id).await;
    handle_get_result(find_result)
}

pub async fn search_genres(
    pool: &PgPool,
    user_id: &str,
    search: SearchDTO,
    quicksearch: Option<String>,
) -> Result<GenrePageResult, ApiErrors> {
    let search = handle_query_mapping::<GenreDTO, GenreSearch>(search, quicksearch)?;
    let find_result = genre_repository::search_all(pool, user_id, search).await;
    handle_get_list_paged_result(find_result)
}

pub async fn create_genre(
    pool: &PgPool,
    user_id: &str,
    genre: NewGenreDTO,
) -> Result<GenreDTO, ApiErrors> {
    create_merged(
        genre,
        async move |created_genre_id| get_genre(pool, user_id, &created_genre_id).await,
        async move |genre_to_create| {
            let exists_result =
                genre_repository::exists_with_unique(pool, user_id, &genre_to_create).await;
            handle_already_exists_result::<GenreDTO>(exists_result)?;

            let create_result = genre_repository::create(pool, user_id, &genre_to_create).await;
            handle_create_result::<String, GenreDTO>(create_result)
        },
    )
    .await
}

pub async fn update_genre(
    pool: &PgPool,
    user_id: &str,
    genre_id: &str,
    genre: NewGenreDTO,
) -> Result<(), ApiErrors> {
    update_merged(
        genre,
        async move || get_genre(pool, user_id, genre_id).await,
        async move |genre_to_update| {
            let exists_result = genre_repository::exists_with_unique_except_id(
                pool,
                user_id,
                &genre_to_update,
                genre_id,
            )
            .await;
            handle_already_exists_result::<GenreDTO>(exists_result)?;

            let update_result =
                genre_repository::update_by_id(pool, user_id, genre_id, &genre_to_update).await;
            handle_update_result::<GenreDTO>(update_result)
        },
    )
    .await
}

pub async fn delete_genre(pool: &PgPool, user_id: &str, genre_id: &str) -> Result<(), ApiErrors> {
    let delete_result = genre_repository::delete_by_id(pool, user_id, genre_id).await;
    handle_action_result::<GenreDTO>(delete_result)
}

pub async fn exists_genre(pool: &PgPool, user_id: &str, genre_id: &str) -> Result<(), ApiErrors> {
    let exists_result = genre_repository::exists_by_id(pool, user_id, genre_id).await;
    handle_not_found_result::<GenreDTO>(exists_result)
}
