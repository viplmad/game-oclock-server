use sqlx::PgPool;
use uuid::Uuid;

use super::query::stored_response_query;
use crate::entities::StoredResponse;
use crate::errors::RepositoryError;

use super::helpers::{execute, fetch_optional};

#[derive(Clone)]
pub struct StoredResponseRepository {
    pool: PgPool,
}

impl StoredResponseRepository {
    pub fn with_connection(pool: PgPool) -> Self {
        Self { pool }
    }
}

impl StoredResponseRepository {
    pub async fn find_by_id(
        &self,
        user_id: &Uuid,
        scope: &str,
        request_hash: &str,
    ) -> Result<Option<StoredResponse>, RepositoryError> {
        let query =
            stored_response_query::select_by_scope_and_request_hash(user_id, scope, request_hash);
        fetch_optional(&self.pool, query).await
    }

    pub async fn create_or_update(&self, res: &StoredResponse) -> Result<(), RepositoryError> {
        let query = stored_response_query::upsert(res);
        execute(&self.pool, query).await
    }

    pub async fn update_last_used_date(
        &self,
        user_id: &Uuid,
        scope: &str,
        request_hash: &str,
    ) -> Result<(), RepositoryError> {
        let query = stored_response_query::update_last_used_date_by_scope_and_request_hash(
            user_id,
            scope,
            request_hash,
        );
        execute(&self.pool, query).await
    }

    pub async fn delete_by_id(
        &self,
        user_id: &Uuid,
        scope: &str,
        request_hash: &str,
    ) -> Result<(), RepositoryError> {
        let query =
            stored_response_query::delete_by_scope_and_request_hash(user_id, scope, request_hash);
        execute(&self.pool, query).await
    }
}
