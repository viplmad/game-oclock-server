use sqlx::PgPool;
use uuid::Uuid;

use super::query::media_query;
use crate::entities::{
    ExternalMedia, Media, MediaSearch, MediaState, MediaStateWithExternal, MediaWithState,
    PageResult,
};
use crate::errors::{RepositoryError, SearchErrors};

use super::helpers::{
    count_all_search, execute, exists_some, fetch_all, fetch_all_search, fetch_optional,
};

#[derive(Clone)]
pub struct MediaRepository {
    pool: PgPool,
}

impl MediaRepository {
    pub fn with_connection(pool: PgPool) -> Self {
        Self { pool }
    }
}

impl MediaRepository {
    pub async fn find_by_id(
        &self,
        user_id: &Uuid,
        id: &Uuid,
    ) -> Result<Option<MediaWithState>, RepositoryError> {
        let query = media_query::select_by_id(user_id, id);
        fetch_optional(&self.pool, query).await
    }

    pub async fn find_basic_by_id(&self, id: &Uuid) -> Result<Option<Media>, RepositoryError> {
        let query = media_query::select_basic_by_id(id);
        fetch_optional(&self.pool, query).await
    }

    pub async fn find_state_by_id(
        &self,
        user_id: &Uuid,
        id: &Uuid,
    ) -> Result<Option<MediaState>, RepositoryError> {
        let query = media_query::select_state_by_id(user_id, id);
        fetch_optional(&self.pool, query).await
    }

    pub async fn find_external_by_id(
        &self,
        id: &Uuid,
    ) -> Result<Option<ExternalMedia>, RepositoryError> {
        let query = media_query::select_primary_external_by_id(id);
        fetch_optional(&self.pool, query).await
    }

    pub async fn find_by_external(
        &self,
        user_id: &Uuid,
        source: &str,
        id: &str,
    ) -> Result<Option<MediaWithState>, RepositoryError> {
        let query = media_query::select_by_external_id(user_id, source, id);
        fetch_optional(&self.pool, query).await
    }

    pub async fn find_basic_by_external(
        &self,
        source: &str,
        id: &str,
    ) -> Result<Option<Media>, RepositoryError> {
        let query = media_query::select_basic_by_external_id(source, id);
        fetch_optional(&self.pool, query).await
    }

    pub async fn find_all_by_parent_id(
        &self,
        user_id: &Uuid,
        parent_id: &Uuid,
    ) -> Result<Vec<MediaWithState>, RepositoryError> {
        let query = media_query::select_all_by_parent_id(user_id, parent_id);
        fetch_all(&self.pool, query).await
    }

    pub async fn find_all_states_by_external_ids(
        &self,
        user_id: &Uuid,
        external_ids: &Vec<(String, String)>,
    ) -> Result<Vec<MediaStateWithExternal>, RepositoryError> {
        let query = media_query::select_all_state_by_externals(user_id, external_ids);
        fetch_all(&self.pool, query).await
    }

    pub async fn search_all(
        &self,
        user_id: &Uuid,
        search: MediaSearch,
    ) -> Result<PageResult<MediaWithState>, SearchErrors> {
        let search_query = media_query::select_all_with_search(user_id, search)?;
        fetch_all_search(&self.pool, search_query).await
    }

    pub async fn count_all(
        &self,
        user_id: &Uuid,
        search: MediaSearch,
    ) -> Result<u64, SearchErrors> {
        let count_query = media_query::count_all_with_search(user_id, search)?;
        count_all_search(&self.pool, count_query).await
    }

    pub async fn create_basic(&self, media: &Media) -> Result<(), RepositoryError> {
        let query = media_query::insert_basic(media);
        execute(&self.pool, query).await
    }

    pub async fn create_state(&self, state: &MediaState) -> Result<(), RepositoryError> {
        let query = media_query::insert_state(state);
        execute(&self.pool, query).await
    }

    pub async fn create_external(&self, media: &ExternalMedia) -> Result<(), RepositoryError> {
        let query = media_query::insert_external(media);
        execute(&self.pool, query).await
    }

    pub async fn update_basic(&self, media: &Media) -> Result<(), RepositoryError> {
        let query = media_query::update_basic_by_id(media);
        execute(&self.pool, query).await
    }

    pub async fn update_state(&self, state: &MediaState) -> Result<(), RepositoryError> {
        let query = media_query::update_state_by_id(state);
        execute(&self.pool, query).await
    }

    pub async fn update_external(&self, media: &ExternalMedia) -> Result<(), RepositoryError> {
        let query = media_query::update_external_by_id(media);
        execute(&self.pool, query).await
    }

    pub async fn update_status(
        &self,
        user_id: &Uuid,
        id: &Uuid,
        status: i16,
    ) -> Result<(), RepositoryError> {
        let query = media_query::update_status_by_id(user_id, id, status);
        execute(&self.pool, query).await
    }

    pub async fn update_parent_id(
        &self,
        id: &Uuid,
        parent_id: Option<Uuid>,
    ) -> Result<(), RepositoryError> {
        let query = media_query::update_parent_id_by_id(id, parent_id);
        execute(&self.pool, query).await
    }

    pub async fn delete_basic_by_id(&self, id: &Uuid) -> Result<(), RepositoryError> {
        let query = media_query::delete_basic_by_id(id);
        execute(&self.pool, query).await
    }

    pub async fn delete_state_by_id(
        &self,
        user_id: &Uuid,
        id: &Uuid,
    ) -> Result<(), RepositoryError> {
        let query = media_query::delete_state_by_id(user_id, id);
        execute(&self.pool, query).await
    }

    pub async fn delete_external_by_id(&self, id: &Uuid) -> Result<(), RepositoryError> {
        let query = media_query::delete_external_by_id(id);
        execute(&self.pool, query).await
    }

    pub async fn exists_basic_by_id(&self, id: &Uuid) -> Result<bool, RepositoryError> {
        let query = media_query::exists_basic_by_id(id);
        exists_some(&self.pool, query).await
    }

    pub async fn exists_state_by_id(
        &self,
        user_id: &Uuid,
        id: &Uuid,
    ) -> Result<bool, RepositoryError> {
        let query = media_query::exists_state_by_id(user_id, id);
        exists_some(&self.pool, query).await
    }

    pub async fn exists_any_state_by_id(&self, id: &Uuid) -> Result<bool, RepositoryError> {
        let query = media_query::exists_any_state_by_id(id);
        exists_some(&self.pool, query).await
    }

    pub async fn exists_any_by_parent_id(&self, parent_id: &Uuid) -> Result<bool, RepositoryError> {
        let query = media_query::exists_any_by_parent_id(parent_id);
        exists_some(&self.pool, query).await
    }

    pub async fn exists_by_title_and_edition(
        &self,
        title: &str,
        edition: &str,
    ) -> Result<bool, RepositoryError> {
        let query = media_query::exists_by_title_and_edition(title, edition);
        exists_some(&self.pool, query).await
    }

    pub async fn exists_by_title_and_edition_except_id(
        &self,
        title: &str,
        edition: &str,
        excluded_id: &Uuid,
    ) -> Result<bool, RepositoryError> {
        let query =
            media_query::exists_by_title_and_edition_and_id_not(title, edition, excluded_id);
        exists_some(&self.pool, query).await
    }

    pub async fn exists_by_external_source_and_id_except_id(
        &self,
        source: &str,
        id: &str,
        excluded_id: &Uuid,
    ) -> Result<bool, RepositoryError> {
        let query = media_query::exists_by_external_source_and_external_id_and_id_not(
            source,
            id,
            excluded_id,
        );
        exists_some(&self.pool, query).await
    }
}
