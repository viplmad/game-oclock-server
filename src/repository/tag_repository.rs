use sqlx::PgPool;
use uuid::Uuid;

use super::query::tag_query;
use crate::entities::{PageResult, Tag, TagListSearch};
use crate::errors::{RepositoryError, SearchErrors};

use super::helpers::{count_all_search, execute, exists_some, fetch_all_search, fetch_optional};

#[derive(Clone)]
pub struct TagRepository {
    pool: PgPool,
}

impl TagRepository {
    pub fn with_connection(pool: PgPool) -> Self {
        Self { pool }
    }
}

impl TagRepository {
    pub async fn find_by_id(
        &self,
        user_id: &Uuid,
        id: &Uuid,
    ) -> Result<Option<Tag>, RepositoryError> {
        let query = tag_query::select_by_id(user_id, id);
        fetch_optional(&self.pool, query).await
    }

    pub async fn search_all(
        &self,
        user_id: &Uuid,
        search: TagListSearch,
    ) -> Result<PageResult<Tag>, SearchErrors> {
        let search_query = tag_query::select_all_with_query(user_id, search)?;
        fetch_all_search(&self.pool, search_query).await
    }

    pub async fn count_all(
        &self,
        user_id: &Uuid,
        search: TagListSearch,
    ) -> Result<u64, SearchErrors> {
        let count_query = tag_query::count_all_with_query(user_id, search)?;
        count_all_search(&self.pool, count_query).await
    }

    pub async fn create(&self, tag: &Tag) -> Result<(), RepositoryError> {
        let query = tag_query::insert(tag);
        execute(&self.pool, query).await
    }

    pub async fn update(&self, tag: &Tag) -> Result<(), RepositoryError> {
        let query = tag_query::update_by_id(tag);
        execute(&self.pool, query).await
    }

    pub async fn delete_by_id(&self, user_id: &Uuid, id: &Uuid) -> Result<(), RepositoryError> {
        let query = tag_query::delete_by_id(user_id, id);
        execute(&self.pool, query).await
    }

    pub async fn exists_by_id(&self, user_id: &Uuid, id: &Uuid) -> Result<bool, RepositoryError> {
        let query = tag_query::exists_by_id(user_id, id);
        exists_some(&self.pool, query).await
    }

    pub async fn exists_by_name(
        &self,
        user_id: &Uuid,
        name: &str,
    ) -> Result<bool, RepositoryError> {
        let query = tag_query::exists_by_name(user_id, name);
        exists_some(&self.pool, query).await
    }

    pub async fn exists_by_name_except_id(
        &self,
        user_id: &Uuid,
        name: &str,
        excluded_id: &Uuid,
    ) -> Result<bool, RepositoryError> {
        let query = tag_query::exists_by_name_and_id_not(user_id, name, excluded_id);
        exists_some(&self.pool, query).await
    }
}
