use uuid::Uuid;

use crate::entities::{Tag, TagAggregateSearch, TagListSearch};
use crate::errors::ApiErrors;
use crate::models::{
    AggregateResultDTO, AggregateSearchDTO, ListSearchDTO, NewTagDTO, TagDTO, TagPageResult,
};
use crate::repository::TagRepository;

use super::helpers::{
    create_merged, handle_action_result, handle_aggregate_search_mapping,
    handle_already_exists_result, handle_get_aggregate_result, handle_get_list_paged_result,
    handle_get_result, handle_list_search_mapping, handle_not_found_result, handle_update_result,
    update_merged,
};

#[derive(Clone)]
pub struct TagService {
    repository: TagRepository,
}

impl TagService {
    pub fn with(repository: TagRepository) -> Self {
        Self { repository }
    }
}

impl TagService {
    pub async fn get_tag(&self, user_id: &Uuid, id: &Uuid) -> Result<TagDTO, ApiErrors> {
        let find_result = self.repository.find_by_id(user_id, id).await;
        handle_get_result(find_result)
    }

    pub async fn search_tags(
        &self,
        user_id: &Uuid,
        search: ListSearchDTO,
        quicksearch: Option<String>,
    ) -> Result<TagPageResult, ApiErrors> {
        let search = handle_list_search_mapping::<TagDTO, TagListSearch>(search, quicksearch)?;
        let find_result = self.repository.search_all(user_id, search).await;
        handle_get_list_paged_result(find_result)
    }

    pub async fn aggregate_tags(
        &self,
        user_id: &Uuid,
        search: AggregateSearchDTO,
        quicksearch: Option<String>,
    ) -> Result<AggregateResultDTO, ApiErrors> {
        let search =
            handle_aggregate_search_mapping::<TagDTO, TagAggregateSearch>(search, quicksearch)?;
        let find_result = self.repository.aggregate_all(user_id, search).await;
        handle_get_aggregate_result::<TagDTO>(find_result)
    }

    pub async fn create_tag(&self, user_id: &Uuid, tag: NewTagDTO) -> Result<Uuid, ApiErrors> {
        let new_id = crate::uuid_utils::new_model_uuid();

        create_merged::<Tag, TagDTO, NewTagDTO, _>(tag, async move |mut tag_to_create: Tag| {
            let exists_result = self
                .repository
                .exists_by_name(user_id, &tag_to_create.name)
                .await;
            handle_already_exists_result::<TagDTO>(exists_result)?;

            tag_to_create.user_id = *user_id;
            tag_to_create.id = new_id;
            tag_to_create.added_datetime = crate::date_utils::now();
            tag_to_create.updated_datetime = crate::date_utils::now();
            let create_result = self.repository.create(&tag_to_create).await;
            handle_action_result::<TagDTO>(create_result)
        })
        .await?;

        Ok(new_id)
    }

    pub async fn update_tag(
        &self,
        user_id: &Uuid,
        id: &Uuid,
        tag: NewTagDTO,
    ) -> Result<(), ApiErrors> {
        update_merged(
            tag,
            async move || self.get_tag(user_id, id).await,
            async move |mut tag_to_update: Tag| {
                let exists_result = self
                    .repository
                    .exists_by_name_except_id(user_id, &tag_to_update.name, id)
                    .await;
                handle_already_exists_result::<TagDTO>(exists_result)?;

                tag_to_update.user_id = *user_id;
                tag_to_update.id = *id;
                tag_to_update.updated_datetime = crate::date_utils::now();
                let update_result = self.repository.update(&tag_to_update).await;
                handle_update_result::<TagDTO>(update_result)
            },
        )
        .await
    }

    pub async fn delete_tag(&self, user_id: &Uuid, id: &Uuid) -> Result<(), ApiErrors> {
        self.exists_tag(user_id, id).await?;

        let delete_result = self.repository.delete_by_id(user_id, id).await;
        handle_action_result::<TagDTO>(delete_result)
    }

    pub async fn exists_tag(&self, user_id: &Uuid, id: &Uuid) -> Result<(), ApiErrors> {
        let exists_result = self.repository.exists_by_id(user_id, id).await;
        handle_not_found_result::<TagDTO>(exists_result)
    }
}
