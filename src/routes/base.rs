use actix_web::{HttpResponse, Responder};
use serde::Serialize;

use crate::errors::ToError;

pub(super) fn handle_get_result(
    service_result: Result<impl Serialize, impl ToError>,
) -> impl Responder {
    match service_result {
        Ok(data) => HttpResponse::Ok().json(data),
        Err(error) => error.to_error(),
    }
}

pub(super) fn handle_create_result(
    service_result: Result<impl Serialize, impl ToError>,
) -> impl Responder {
    match service_result {
        Ok(data) => HttpResponse::Created().json(data),
        Err(error) => error.to_error(),
    }
}

pub(super) fn handle_update_result(
    service_result: Result<impl Serialize, impl ToError>,
) -> impl Responder {
    match service_result {
        Ok(data) => HttpResponse::Ok().json(data),
        Err(error) => error.to_error(),
    }
}

pub(super) fn handle_delete_result(service_result: Result<(), impl ToError>) -> impl Responder {
    handle_action_result(service_result)
}

pub(super) fn handle_action_result(service_result: Result<(), impl ToError>) -> impl Responder {
    match service_result {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(error) => error.to_error(),
    }
}
