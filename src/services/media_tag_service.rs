use uuid::Uuid;

use crate::entities::{MediaSearch, MediaTag, TagSearch};
use crate::errors::ApiErrors;
use crate::models::{
    MediaDTO, MediaTagDTO, MediaTagPageResult, SearchDTO, TagDTO, TagMediaPageResult,
};
use crate::repository::MediaTagRepository;

use super::helpers::{
    handle_action_result, handle_already_exists_result, handle_get_count_result,
    handle_get_list_paged_result, handle_not_found_result, handle_query_mapping,
};
use super::{MediaService, TagService};

#[derive(Clone)]
pub struct MediaTagService {
    repository: MediaTagRepository,
    media_service: MediaService,
    tag_service: TagService,
}

impl MediaTagService {
    pub fn with(
        repository: MediaTagRepository,
        media_service: MediaService,
        tag_service: TagService,
    ) -> Self {
        Self {
            repository,
            media_service,
            tag_service,
        }
    }
}

impl MediaTagService {
    pub async fn search_tag_medias(
        &self,
        user_id: &Uuid,
        tag_id: &Uuid,
        search: SearchDTO,
        quicksearch: Option<String>,
    ) -> Result<MediaTagPageResult, ApiErrors> {
        self.tag_service.exists_tag(user_id, tag_id).await?;

        let search = handle_query_mapping::<MediaDTO, MediaSearch>(search, quicksearch)?;
        let find_result = self
            .repository
            .search_all_medias_with_tag(user_id, tag_id, search)
            .await;
        handle_get_list_paged_result(find_result)
    }

    pub async fn count_tag_medias(
        &self,
        user_id: &Uuid,
        tag_id: &Uuid,
        search: SearchDTO,
        quicksearch: Option<String>,
    ) -> Result<u64, ApiErrors> {
        self.tag_service.exists_tag(user_id, tag_id).await?;

        let search = handle_query_mapping::<MediaDTO, MediaSearch>(search, quicksearch)?;
        let count_result = self
            .repository
            .count_all_medias_with_tag(user_id, tag_id, search)
            .await;
        handle_get_count_result::<MediaDTO>(count_result)
    }

    pub async fn search_media_tags(
        &self,
        user_id: &Uuid,
        media_id: &Uuid,
        search: SearchDTO,
        quicksearch: Option<String>,
    ) -> Result<TagMediaPageResult, ApiErrors> {
        self.media_service.exists_media(media_id).await?;

        let search = handle_query_mapping::<TagDTO, TagSearch>(search, quicksearch)?;
        let find_result = self
            .repository
            .search_all_tags_with_media(user_id, media_id, search)
            .await;
        handle_get_list_paged_result(find_result)
    }

    pub async fn count_media_tags(
        &self,
        user_id: &Uuid,
        media_id: &Uuid,
        search: SearchDTO,
        quicksearch: Option<String>,
    ) -> Result<u64, ApiErrors> {
        self.media_service.exists_media(media_id).await?;

        let search = handle_query_mapping::<TagDTO, TagSearch>(search, quicksearch)?;
        let find_result = self
            .repository
            .count_all_tags_with_media(user_id, media_id, search)
            .await;
        handle_get_count_result::<TagDTO>(find_result)
    }

    pub async fn create_media_tag(
        &self,
        user_id: &Uuid,
        media_id: &Uuid,
        tag_id: &Uuid,
        order: Option<u32>,
    ) -> Result<(), ApiErrors> {
        self.media_service.exists_media(media_id).await?;
        self.tag_service.exists_tag(user_id, tag_id).await?;

        let exists_result = self
            .repository
            .exists_by_id(user_id, media_id, tag_id)
            .await;
        handle_already_exists_result::<MediaTagDTO>(exists_result)?;

        let create_result = self
            .repository
            .create(&MediaTag {
                user_id: user_id.clone(),
                media_id: media_id.clone(),
                tag_id: tag_id.clone(),
                order: order
                    .map(|o| i32::try_from(o).expect("Order is not within valid range"))
                    .unwrap_or(0),
                added_datetime: crate::date_utils::now(),
                updated_datetime: crate::date_utils::now(),
            })
            .await;
        handle_action_result::<MediaTagDTO>(create_result)
    }

    pub async fn delete_media_tag(
        &self,
        user_id: &Uuid,
        media_id: &Uuid,
        tag_id: &Uuid,
    ) -> Result<(), ApiErrors> {
        self.exists_media_tag(user_id, media_id, tag_id).await?;

        let delete_result = self
            .repository
            .delete_by_id(user_id, media_id, tag_id)
            .await;
        handle_action_result::<MediaTagDTO>(delete_result)
    }

    pub async fn exists_media_tag(
        &self,
        user_id: &Uuid,
        media_id: &Uuid,
        tag_id: &Uuid,
    ) -> Result<(), ApiErrors> {
        let exists_result = self
            .repository
            .exists_by_id(user_id, media_id, tag_id)
            .await;
        handle_not_found_result::<MediaTagDTO>(exists_result)
    }
}
