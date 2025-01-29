use actix_web::HttpResponse;
use serde::Serialize;
use sqlx::PgPool;

use crate::errors::{forbidden_error, ToError};
use crate::services::users_service;

pub(super) fn handle_get_result(
    service_result: Result<impl Serialize, impl ToError>,
) -> HttpResponse {
    match service_result {
        Ok(data) => HttpResponse::Ok().json(data),
        Err(error) => error.to_error(),
    }
}

pub(super) fn handle_create_result(
    service_result: Result<impl Serialize, impl ToError>,
) -> HttpResponse {
    match service_result {
        Ok(data) => HttpResponse::Created().json(data),
        Err(error) => error.to_error(),
    }
}

pub(super) fn handle_update_result(service_result: Result<(), impl ToError>) -> HttpResponse {
    match service_result {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(error) => error.to_error(),
    }
}

pub(super) fn handle_delete_result(service_result: Result<(), impl ToError>) -> HttpResponse {
    handle_action_result(service_result)
}

pub(super) fn handle_action_result(service_result: Result<(), impl ToError>) -> HttpResponse {
    match service_result {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(error) => error.to_error(),
    }
}

pub(super) async fn require_admin(pool: &PgPool, user_id: &str) -> Result<(), HttpResponse> {
    let admin_result = users_service::is_user_admin(pool, user_id).await;
    match admin_result {
        Ok(admin) => {
            if !admin {
                return Err(forbidden_error());
            }
            Ok(())
        }
        Err(_) => Err(forbidden_error()),
    }
}

pub(super) async fn require_admin_or_current_user(
    pool: &PgPool,
    user_id: &str,
    id: &str,
) -> Result<(), HttpResponse> {
    match user_id == id {
        true => Ok(()),
        false => require_admin(pool, user_id).await,
    }
}
