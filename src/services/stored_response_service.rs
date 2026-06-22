use uuid::Uuid;

use crate::entities::StoredResponse;
use crate::errors::ApiErrors;
use crate::models::FetchMode;
use crate::models::ModelInfo;
use crate::repository::StoredResponseRepository;

use super::helpers::{handle_action_result, handle_get_result_raw, handle_update_result};

struct StoredResponseDTO;
impl ModelInfo for StoredResponseDTO {
    const MODEL_NAME: &'static str = "Stored Response";
    const ID_FIELDS: &'static [&'static str] = &["scope", "request"];
    const UNIQUE_FIELDS: &'static [&'static str] = Self::ID_FIELDS;
}

#[derive(Clone)]
pub struct StoredResponseService {
    repository: StoredResponseRepository,
}

impl StoredResponseService {
    pub fn with(repository: StoredResponseRepository) -> Self {
        Self { repository }
    }
}

impl StoredResponseService {
    pub async fn get_based_on_mode<T, GF>(
        &self,
        mode: Option<FetchMode>,
        user_id: &Uuid,
        scope: &str,
        request: &str,
        get_function: impl FnOnce() -> GF,
    ) -> Result<T, ApiErrors>
    where
        T: serde::de::DeserializeOwned + Sized + serde::Serialize,
        GF: Future<Output = Result<T, ApiErrors>>,
    {
        match mode.unwrap_or_default() {
            FetchMode::OnlyCalculate => get_function().await,
            FetchMode::OnlyStored => {
                self.restore_aggregate_sessions(user_id, scope, request)
                    .await
            }
            FetchMode::ForceCalculateAndStore => {
                self.calculate_store_aggregate_sessions(user_id, scope, request, get_function)
                    .await
            }
            FetchMode::StoredOrCalculate => {
                let stored_res = self
                    .restore_aggregate_sessions(user_id, scope, request)
                    .await;
                match stored_res {
                    Ok(stored) => Ok(stored),
                    Err(err) => match err {
                        ApiErrors::NotFound(_) => {
                            self.calculate_store_aggregate_sessions(
                                user_id,
                                scope,
                                request,
                                get_function,
                            )
                            .await
                        }
                        _ => Err(err),
                    },
                }
            }
        }
    }

    async fn calculate_store_aggregate_sessions<T, GF>(
        &self,
        user_id: &Uuid,
        scope: &str,
        request: &str,
        get_function: impl FnOnce() -> GF,
    ) -> Result<T, ApiErrors>
    where
        T: Sized + serde::Serialize,
        GF: Future<Output = Result<T, ApiErrors>>,
    {
        let res = get_function().await?;

        let response = crate::convert_utils::to_json_string(&res, scope)
            .map_err(|err| ApiErrors::UnknownError(err.0))?;
        self.save_response(user_id, scope, request, &response)
            .await?;

        Ok(res)
    }

    async fn restore_aggregate_sessions<T>(
        &self,
        user_id: &Uuid,
        scope: &str,
        request: &str,
    ) -> Result<T, ApiErrors>
    where
        T: serde::de::DeserializeOwned,
    {
        let stored_res = self.get_response(user_id, scope, request).await?;
        crate::convert_utils::from_json_string::<T>(&stored_res, scope)
            .map_err(|err| ApiErrors::UnknownError(err.0))
    }

    async fn get_response(
        &self,
        user_id: &Uuid,
        scope: &str,
        request: &str,
    ) -> Result<String, ApiErrors> {
        let request_hash = crate::string_utils::sha256_hash(request);
        let find_result = self
            .repository
            .find_by_id(user_id, scope, &request_hash)
            .await;

        if find_result.as_ref().is_ok_and(|r| r.is_some()) {
            // Update last used date if hit
            let update_result = self
                .repository
                .update_last_used_date(user_id, scope, &request_hash)
                .await;
            handle_action_result::<StoredResponseDTO>(update_result)?;
        }

        handle_get_result_raw::<StoredResponse, StoredResponseDTO>(find_result)
            .map(|res| res.response)
    }

    async fn save_response(
        &self,
        user_id: &Uuid,
        scope: &str,
        request: &str,
        response: &str,
    ) -> Result<(), ApiErrors> {
        let request_hash = crate::string_utils::sha256_hash(request);
        let entity = StoredResponse {
            user_id: *user_id,
            scope: String::from(scope),
            request_hash,
            response: String::from(response),
            last_used_date: crate::date_utils::now(),
            added_datetime: crate::date_utils::now(),
            updated_datetime: crate::date_utils::now(),
        };
        let upsert_result = self.repository.create_or_update(&entity).await;
        handle_update_result::<StoredResponseDTO>(upsert_result)
    }
}
