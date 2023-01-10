use sqlx::PgPool;

use crate::entities::TagSearch;
use crate::errors::ApiErrors;
use crate::models::{NewTagDTO, SearchDTO, TagDTO, TagPageResult};
use crate::repository::tag_repository;

use super::base::{
    create_merged, handle_action_result, handle_already_exists_result, handle_create_result,
    handle_get_list_paged_result, handle_get_result, handle_not_found_result, handle_query_mapping,
    handle_update_result, update_merged,
};

pub async fn get_tag(pool: &PgPool, user_id: i32, tag_id: i32) -> Result<TagDTO, ApiErrors> {
    let find_result = tag_repository::find_by_id(pool, user_id, tag_id).await;
    handle_get_result(find_result)
}

pub async fn search_tags(
    pool: &PgPool,
    user_id: i32,
    search: SearchDTO,
    quicksearch: Option<String>,
) -> Result<TagPageResult, ApiErrors> {
    let search = handle_query_mapping::<TagDTO, TagSearch>(search, quicksearch)?;
    let find_result = tag_repository::search_all(pool, user_id, search).await;
    handle_get_list_paged_result(find_result)
}

pub async fn create_tag(pool: &PgPool, user_id: i32, tag: NewTagDTO) -> Result<TagDTO, ApiErrors> {
    create_merged(
        tag,
        async move |created_tag_id| get_tag(pool, user_id, created_tag_id).await,
        async move |tag_to_create| {
            let exists_result =
                tag_repository::exists_with_unique(pool, user_id, &tag_to_create).await;
            handle_already_exists_result::<TagDTO>(exists_result)?;

            let create_result = tag_repository::create(pool, user_id, &tag_to_create).await;
            handle_create_result::<i32, TagDTO>(create_result)
        },
    )
    .await
}

pub async fn update_tag(
    pool: &PgPool,
    user_id: i32,
    tag_id: i32,
    tag: NewTagDTO,
) -> Result<TagDTO, ApiErrors> {
    update_merged(
        tag,
        async move || get_tag(pool, user_id, tag_id).await,
        async move |tag_to_update| {
            let exists_result =
                tag_repository::exists_with_unique_except_id(pool, user_id, &tag_to_update, tag_id)
                    .await;
            handle_already_exists_result::<TagDTO>(exists_result)?;

            let update_result =
                tag_repository::update_by_id(pool, user_id, tag_id, &tag_to_update).await;
            handle_update_result::<i32, TagDTO>(update_result)
        },
    )
    .await
}

pub async fn delete_tag(pool: &PgPool, user_id: i32, tag_id: i32) -> Result<(), ApiErrors> {
    exists_tag(pool, user_id, tag_id).await?;

    let delete_result = tag_repository::delete_by_id(pool, user_id, tag_id).await;
    handle_action_result::<TagDTO>(delete_result)
}

pub async fn exists_tag(pool: &PgPool, user_id: i32, tag_id: i32) -> Result<(), ApiErrors> {
    let exists_result = tag_repository::exists_by_id(pool, user_id, tag_id).await;
    handle_not_found_result::<TagDTO>(exists_result)
}
