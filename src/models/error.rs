use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema)]
pub struct ErrorMessage {
    pub error: String,
    pub error_description: String,
}

impl ErrorMessage {
    pub fn new(code: &str, description: &str) -> Self {
        ErrorMessage {
            error: String::from(code),
            error_description: String::from(description),
        }
    }
}
