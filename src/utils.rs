pub fn optional_string_is_none_or_blank(optional_value: &Option<String>) -> bool {
    match optional_value {
        Some(v) => string_is_blank(v),
        None => false,
    }
}

pub fn string_is_blank(value: &str) -> bool {
    value.trim().is_empty()
}

pub fn now() -> chrono::NaiveDateTime {
    chrono::Utc::now().naive_utc()
}
