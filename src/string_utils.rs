use sha2::{Digest, Sha256};

pub fn optional_string_is_none_or_blank(optional_value: &Option<String>) -> bool {
    match optional_value {
        Some(v) => string_is_blank(v),
        None => false,
    }
}

pub fn string_is_blank(value: &str) -> bool {
    value.trim().is_empty()
}

pub fn sha256_hash(value: &str) -> String {
    format!("{:x}", Sha256::digest(value))
}
