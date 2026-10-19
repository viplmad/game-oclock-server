use sqlx::PgPool;
use uuid::Uuid;

use super::query::media_tag_query;
use crate::entities::{
    AggregateResult, MediaAggregateSearch, MediaListSearch, MediaTag, MediaWithStateWithTag,
    PageResult, TagAggregateSearch, TagListSearch, TagWithTag,
};
use crate::errors::{RepositoryError, SearchErrors};

use super::helpers::{aggregate_all_search, execute, exists_some, fetch_all_search};

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
        search: MediaListSearch,
    ) -> Result<PageResult<MediaWithStateWithTag>, SearchErrors> {
        let query = media_tag_query::select_all_medias_by_tag_id(user_id, tag_id, search)?;
        fetch_all_search(&self.pool, query).await
    }

    pub async fn aggregate_all_medias_with_tag(
        &self,
        user_id: &Uuid,
        tag_id: &Uuid,
        search: MediaAggregateSearch,
    ) -> Result<AggregateResult, SearchErrors> {
        let query = media_tag_query::aggregate_all_medias_by_tag_id(user_id, tag_id, search)?;
        aggregate_all_search(&self.pool, query).await
    }

    pub async fn search_all_tags_with_media(
        &self,
        user_id: &Uuid,
        media_id: &Uuid,
        search: TagListSearch,
    ) -> Result<PageResult<TagWithTag>, SearchErrors> {
        let query = media_tag_query::select_all_tags_by_media_id(user_id, media_id, search)?;
        fetch_all_search(&self.pool, query).await
    }

    pub async fn aggregate_all_tags_with_media(
        &self,
        user_id: &Uuid,
        media_id: &Uuid,
        search: TagAggregateSearch,
    ) -> Result<AggregateResult, SearchErrors> {
        let query = media_tag_query::aggregate_all_tags_by_media_id(user_id, media_id, search)?;
        aggregate_all_search(&self.pool, query).await
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
