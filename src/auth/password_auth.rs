use bcrypt::{DEFAULT_COST, hash, verify};

use crate::errors::{PasswordError, ValidationError};

pub fn verify_password(password: &str, password_hash: &str) -> Result<bool, ValidationError> {
    verify(password, password_hash).map_err(|err| {
        log::error!("Error verifying password. - {}", err.to_string());
        ValidationError()
    })
}

pub fn hash_password(password: &str) -> Result<String, PasswordError> {
    hash(password, DEFAULT_COST).map_err(|err| {
        log::error!("Error hashing password. - {}", err.to_string());
        PasswordError()
    })
}
