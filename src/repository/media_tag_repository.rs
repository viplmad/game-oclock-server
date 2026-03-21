use sqlx::PgPool;
use uuid::Uuid;

use super::query::media_tag_query;
use crate::entities::{
    MediaSearch, MediaTag, MediaWithStateWithTag, PageResult, TagSearch, TagWithTag,
};
use crate::errors::{RepositoryError, SearchErrors};

use super::helpers::{count_all_search, execute, exists_some, fetch_all_search};

#[derive(Clone)]
pub struct MediaTagRepository {
    pool: PgPool,
}

impl MediaTagRepository {
    pub fn with_connection(pool: PgPool) -> Self {
        Self { pool }
    }
}

impl MediaTagRepository {
    pub async fn search_all_medias_with_tag(
        &self,
        user_id: &Uuid,
        tag_id: &Uuid,
        search: MediaSearch,
    ) -> Result<PageResult<MediaWithStateWithTag>, SearchErrors> {
        let search_query = media_tag_query::select_all_medias_by_tag_id(user_id, tag_id, search)?;
        fetch_all_search(&self.pool, search_query).await
    }

    pub async fn count_all_medias_with_tag(
        &self,
        user_id: &Uuid,
        tag_id: &Uuid,
        search: MediaSearch,
    ) -> Result<u64, SearchErrors> {
        let count_query = media_tag_query::count_all_medias_by_tag_id(user_id, tag_id, search)?;
        count_all_search(&self.pool, count_query).await
    }

    pub async fn search_all_tags_with_media(
        &self,
        user_id: &Uuid,
        media_id: &Uuid,
        search: TagSearch,
    ) -> Result<PageResult<TagWithTag>, SearchErrors> {
        let search_query = media_tag_query::select_all_tags_by_media_id(user_id, media_id, search)?;
        fetch_all_search(&self.pool, search_query).await
    }

    pub async fn count_all_tags_with_media(
        &self,
        user_id: &Uuid,
        media_id: &Uuid,
        search: TagSearch,
    ) -> Result<u64, SearchErrors> {
        let search_query = media_tag_query::count_all_tags_by_media_id(user_id, media_id, search)?;
        count_all_search(&self.pool, search_query).await
    }

    pub async fn create(&self, media_tag: &MediaTag) -> Result<(), RepositoryError> {
        let query = media_tag_query::insert(media_tag);
        execute(&self.pool, query).await
    }

    pub async fn delete_by_id(
        &self,
        user_id: &Uuid,
        media_id: &Uuid,
        tag_id: &Uuid,
    ) -> Result<(), RepositoryError> {
        let query = media_tag_query::delete_by_id(user_id, media_id, tag_id);
        execute(&self.pool, query).await
    }

    pub async fn exists_by_id(
        &self,
        user_id: &Uuid,
        media_id: &Uuid,
        tag_id: &Uuid,
    ) -> Result<bool, RepositoryError> {
        let query = media_tag_query::exists_by_id(user_id, media_id, tag_id);
        exists_some(&self.pool, query).await
    }
}
