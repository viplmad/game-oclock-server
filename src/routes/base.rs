use actix_web::HttpResponse;
use serde::Serialize;
use sqlx::PgPool;

use crate::errors::{forbidden_error, ToError};
use crate::models::{ModelInfo, PageResultDTO};
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

pub(super) fn handle_multipart_result(
    multipart_result: Result<String, impl ToError>,
) -> Result<String, HttpResponse> {
    multipart_result.map_err(|err| err.to_error())
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

pub(super) fn populate_get_result<T>(
    service_result: &mut Result<T, impl ToError>,
    mut populate_function: impl FnMut(&mut T),
) {
    if let Ok(item) = service_result {
        populate_function(item);
    }
}

pub(super) fn populate_get_page_result<T: ModelInfo>(
    service_result: &mut Result<PageResultDTO<T>, impl ToError>,
    mut populate_function: impl FnMut(&mut Vec<T>),
) {
    if let Ok(page) = service_result {
        populate_function(&mut page.data);
    }
}
