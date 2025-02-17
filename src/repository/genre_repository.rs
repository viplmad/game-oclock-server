use sqlx::PgPool;
use uuid::Uuid;

use super::query::genre_query;
use crate::entities::{Genre, GenreSearch, PageResult};
use crate::errors::{RepositoryError, SearchErrors};

use super::helpers::{execute, exists_id, fetch_all_search, fetch_optional};

#[derive(Clone)]
pub struct GenreRepository {
    pool: PgPool,
}

impl GenreRepository {
    pub fn with_connection(pool: PgPool) -> Self {
        Self { pool }
    }
}

impl GenreRepository {
    pub async fn find_by_id(
        &self,
        user_id: &Uuid,
        id: &Uuid,
    ) -> Result<Option<Genre>, RepositoryError> {
        let query = genre_query::select_by_id(user_id, id);
        fetch_optional(&self.pool, query).await
    }

    pub async fn search_all(
        &self,
        user_id: &Uuid,
        search: GenreSearch,
    ) -> Result<PageResult<Genre>, SearchErrors> {
        let search_query = genre_query::select_all_with_search(user_id, search)?;
        fetch_all_search(&self.pool, search_query).await
    }

    pub async fn create(&self, genre: &Genre) -> Result<(), RepositoryError> {
        let query = genre_query::insert(genre);
        execute(&self.pool, query).await
    }

    pub async fn update(&self, genre: &Genre) -> Result<(), RepositoryError> {
        let query = genre_query::update_by_id(genre);
        execute(&self.pool, query).await
    }

    pub async fn delete_by_id(&self, user_id: &Uuid, id: &Uuid) -> Result<(), RepositoryError> {
        let query = genre_query::delete_by_id(user_id, id);
        execute(&self.pool, query).await
    }

    pub async fn exists_by_id(&self, user_id: &Uuid, id: &Uuid) -> Result<bool, RepositoryError> {
        let query = genre_query::exists_by_id(user_id, id);
        exists_id(&self.pool, query).await
    }

    pub async fn exists_by_name(
        &self,
        user_id: &Uuid,
        name: &str,
    ) -> Result<bool, RepositoryError> {
        let query = genre_query::exists_by_name(user_id, name);
        exists_id(&self.pool, query).await
    }

    pub async fn exists_by_name_except_id(
        &self,
        user_id: &Uuid,
        name: &str,
        excluded_id: &Uuid,
    ) -> Result<bool, RepositoryError> {
        let query = genre_query::exists_by_name_and_id_not(user_id, name, excluded_id);
        exists_id(&self.pool, query).await
    }
}
