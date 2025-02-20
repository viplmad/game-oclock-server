use actix_web::HttpResponse;
use serde::Serialize;
use uuid::Uuid;

use crate::errors::{ToError, forbidden_error};
use crate::services::UserService;

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

pub(super) async fn require_admin(
    user_service: &UserService,
    user_id: &Uuid,
) -> Result<(), HttpResponse> {
    let admin_result = user_service.is_user_admin(user_id).await;
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
    user_service: &UserService,
    user_id: &Uuid,
    id: &Uuid,
) -> Result<(), HttpResponse> {
    match user_id == id {
        true => Ok(()),
        false => require_admin(user_service, user_id).await,
    }
}
