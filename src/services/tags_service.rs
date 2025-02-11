use crate::entities::TagSearch;
use crate::errors::ApiErrors;
use crate::models::{NewTagDTO, SearchDTO, TagDTO, TagPageResult};
use crate::repository::TagRepository;

use super::base::{
    create_merged, handle_action_result, handle_already_exists_result, handle_create_result,
    handle_get_list_paged_result, handle_get_result, handle_not_found_result, handle_query_mapping,
    handle_update_result, update_merged,
};

pub async fn get_tag(
    repository: &TagRepository,
    user_id: &str,
    tag_id: &str,
) -> Result<TagDTO, ApiErrors> {
    let find_result = repository.find_by_id(user_id, tag_id).await;
    handle_get_result(find_result)
}

pub async fn search_tags(
    repository: &TagRepository,
    user_id: &str,
    search: SearchDTO,
    quicksearch: Option<String>,
) -> Result<TagPageResult, ApiErrors> {
    let search = handle_query_mapping::<TagDTO, TagSearch>(search, quicksearch)?;
    let find_result = repository.search_all(user_id, search).await;
    handle_get_list_paged_result(find_result)
}

pub async fn create_tag(
    repository: &TagRepository,
    user_id: &str,
    tag: NewTagDTO,
) -> Result<TagDTO, ApiErrors> {
    create_merged(
        tag,
        async move |created_tag_id| get_tag(repository, user_id, &created_tag_id).await,
        async move |tag_to_create| {
            let exists_result = repository.exists_with_unique(user_id, &tag_to_create).await;
            handle_already_exists_result::<TagDTO>(exists_result)?;

            let create_result = repository.create(user_id, &tag_to_create).await;
            handle_create_result::<String, TagDTO>(create_result)
        },
    )
    .await
}

pub async fn update_tag(
    repository: &TagRepository,
    user_id: &str,
    tag_id: &str,
    tag: NewTagDTO,
) -> Result<(), ApiErrors> {
    update_merged(
        tag,
        async move || get_tag(repository, user_id, tag_id).await,
        async move |tag_to_update| {
            let exists_result = repository
                .exists_with_unique_except_id(user_id, &tag_to_update, tag_id)
                .await;
            handle_already_exists_result::<TagDTO>(exists_result)?;

            let update_result = repository
                .update_by_id(user_id, tag_id, &tag_to_update)
                .await;
            handle_update_result::<TagDTO>(update_result)
        },
    )
    .await
}

pub async fn delete_tag(
    repository: &TagRepository,
    user_id: &str,
    tag_id: &str,
) -> Result<(), ApiErrors> {
    exists_tag(repository, user_id, tag_id).await?;

    let delete_result = repository.delete_by_id(user_id, tag_id).await;
    handle_action_result::<TagDTO>(delete_result)
}

pub async fn exists_tag(
    repository: &TagRepository,
    user_id: &str,
    tag_id: &str,
) -> Result<(), ApiErrors> {
    let exists_result = repository.exists_by_id(user_id, tag_id).await;
    handle_not_found_result::<TagDTO>(exists_result)
}
