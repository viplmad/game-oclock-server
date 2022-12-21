use actix_web::HttpResponse;

use crate::models::ErrorMessage;

pub trait ToError {
    /// Converts to error response
    #[must_use]
    fn to_error(&self) -> HttpResponse;
}

pub mod error_message_builder {
    pub fn missing_param(param_name: &str) -> String {
        format!("Request parameter \"{param_name}\" cannot be empty.")
    }

    pub fn missing_body_field(field_name: &str) -> String {
        format!("Request body field \"{field_name}\" cannot be empty.")
    }

    pub fn empty_param(param_name: &str) -> String {
        format!("{param_name} is empty.")
    }

    pub fn param_not_match(param_name: &str) -> String {
        format!("{param_name} does not match the one provided.")
    }

    pub fn database_error(model_name: &str) -> String {
        format!("Database error accessing {model_name}.")
    }

    pub fn created_but_error_get(model_name: &str) -> String {
        format!("{model_name} was created but cannot be retrieved.")
    }

    pub fn updated_but_error_get(model_name: &str) -> String {
        format!("{model_name} was updated but cannot be retrieved.")
    }

    pub fn not_found(model_name: &str, parameters: &[&str]) -> String {
        let parameters_join = parameters.join("-");
        format!("{model_name} with specified {parameters_join} does not exist.")
    }

    pub fn already_exists(model_name: &str, parameters: &[&str]) -> String {
        let parameters_join = parameters.join("-");
        format!("{model_name} with specified {parameters_join} already exists.")
    }

    pub fn inner_error(msg: &str, inner_msg: &str) -> String {
        format!("{msg} - {inner_msg}")
    }

    pub fn field_not_found(model_name: &str, field_name: &str) -> String {
        format!("Field \"{field_name}\" from {model_name} does not exist")
    }

    pub fn convert_to_error(value: &str, type_string: &str) -> String {
        format!("Could not convert \"{value}\" to {type_string}")
    }
}

pub struct ValidationError(pub String);

pub struct PasswordError(pub String);

pub struct RepositoryError(pub String);

pub struct FieldMappingError(pub String);

pub enum ApiErrors {
    InvalidParameter(String),
    AlreadyExists(String),
    NotFound(String),
    UnknownError(String),
}

impl ToError for ApiErrors {
    fn to_error(&self) -> HttpResponse {
        match self {
            ApiErrors::InvalidParameter(msg) => {
                HttpResponse::BadRequest().json(ErrorMessage::new("invalid_parameter", msg))
            }
            ApiErrors::AlreadyExists(msg) => {
                HttpResponse::BadRequest().json(ErrorMessage::new("already_exists", msg))
            }
            ApiErrors::NotFound(msg) => {
                HttpResponse::NotFound().json(ErrorMessage::new("not_found", msg))
            }
            ApiErrors::UnknownError(msg) => {
                HttpResponse::InternalServerError().json(ErrorMessage::new("unknown_error", msg))
            }
        }
    }
}

pub enum TokenErrors {
    InvalidRequest(String),
    InvalidGrant(String),
    UnsupportedGrantType(String),
    UnknownError(String),
}

impl ToError for TokenErrors {
    fn to_error(&self) -> HttpResponse {
        match self {
            TokenErrors::InvalidRequest(msg) => {
                HttpResponse::BadRequest().json(ErrorMessage::new("invalid_request", msg))
            }
            TokenErrors::InvalidGrant(msg) => {
                HttpResponse::Unauthorized().json(ErrorMessage::new("invalid_grant", msg))
            }
            TokenErrors::UnsupportedGrantType(msg) => {
                HttpResponse::BadRequest().json(ErrorMessage::new("unsupported_grant_type", msg))
            }
            TokenErrors::UnknownError(msg) => {
                HttpResponse::InternalServerError().json(ErrorMessage::new("unknown_error", msg))
            }
        }
    }
}
