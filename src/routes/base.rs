use actix_web::{HttpResponse, Responder};
use serde::Serialize;

use crate::errors::ToError;
use crate::models::{ModelInfo, PageResultDTO};

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
        Ok(data) => HttpResponse::Ok().json(data), // TODO no devolver con update
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
