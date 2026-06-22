use crate::errors::{MappingError, error_message_builder};

pub fn from_json_string<'a, T>(value: &'a str, type_string: &str) -> Result<T, MappingError>
where
    T: serde::de::Deserialize<'a>,
{
    serde_json::from_str::<T>(value).map_err(|err| {
        log::error!(
            "Error converting value from JSON. <{}> - {} - {}",
            value,
            type_string,
            err
        );
        MappingError(error_message_builder::convert_to_error(value, type_string))
    })
}

pub fn to_json_string<T>(value: &T, type_string: &str) -> Result<String, MappingError>
where
    T: ?Sized + serde::Serialize,
{
    serde_json::to_string::<T>(value).map_err(|err| {
        log::error!(
            "Error converting value to JSON. <{}> - {}",
            type_string,
            err
        );
        MappingError(error_message_builder::convert_to_error(type_string, "JSON"))
    })
}
