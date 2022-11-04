use bcrypt::{hash, verify, DEFAULT_COST};

use crate::errors::{PasswordError, ValidationError};

pub fn verify_password(password: &str, password_hash: &str) -> Result<bool, ValidationError> {
    verify(password, password_hash).map_err(|err| ValidationError(err.to_string()))
}

pub fn hash_password(password: &str) -> Result<String, PasswordError> {
    hash(password, DEFAULT_COST).map_err(|err| PasswordError(err.to_string()))
}
