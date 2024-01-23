use std::{
    fmt::{Display, Formatter},
    str::FromStr,
};

use serde::{de::Visitor, Deserialize, Serialize};

use crate::errors::error_message_builder;

#[derive(Default, Clone)]
pub struct DurationDef {
    pub micros: i64,
}

impl DurationDef {
    pub fn microseconds(microseconds: i64) -> Self {
        Self {
            micros: microseconds,
        }
    }

    fn as_secs(&self) -> i64 {
        self.micros / crate::date_utils::MICROS_PER_SECOND
    }
}

impl Serialize for DurationDef {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for DurationDef {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_str(DurationVisitor)
    }
}

struct DurationVisitor;

impl<'de> Visitor<'de> for DurationVisitor {
    type Value = DurationDef;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a string of a duration in ISO 8601 format.")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        DurationDef::from_str(v).map_err(|err| serde::de::Error::custom(err))
    }
}

impl Display for DurationDef {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let secs = self.as_secs();

        let mut hours: u32 = 0;
        let mut minutes: u32 = 0;
        let mut seconds: u32 = 0;
        if secs > 0 {
            hours = (secs / crate::date_utils::SECONDS_PER_HOUR) as u32;
            if hours > 0 {
                let remainder_seconds: u32 = (secs % crate::date_utils::SECONDS_PER_HOUR) as u32;
                minutes = remainder_seconds / (crate::date_utils::SECONDS_PER_MINUTE as u32);
                if minutes > 0 {
                    seconds = remainder_seconds % (crate::date_utils::SECONDS_PER_MINUTE as u32);
                } else {
                    seconds = remainder_seconds;
                }
            } else {
                minutes = (secs / crate::date_utils::SECONDS_PER_MINUTE) as u32;
                if minutes > 0 {
                    seconds = (secs % crate::date_utils::SECONDS_PER_MINUTE) as u32;
                } else {
                    seconds = secs as u32;
                }
            }
        }
        let duration = iso8601::Duration::YMDHMS {
            year: 0,
            month: 0,
            day: 0,
            hour: hours,
            minute: minutes,
            second: seconds,
            millisecond: 0,
        };
        duration.fmt(f)
    }
}

impl FromStr for DurationDef {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let duration = iso8601::duration(s).map_err(|err| {
            error_message_builder::inner_error("Could not format duration string.", &err)
        })?;
        match duration {
            iso8601::Duration::YMDHMS {
                year: _,
                month: _,
                day,
                hour,
                minute,
                second,
                millisecond: _,
            } => {
                let duration_secs = i64::from(
                    day * (crate::date_utils::SECONDS_PER_DAY as u32)
                        + hour * (crate::date_utils::SECONDS_PER_HOUR as u32)
                        + minute * (crate::date_utils::SECONDS_PER_MINUTE as u32)
                        + second,
                );
                Ok(DurationDef::microseconds(
                    duration_secs * crate::date_utils::MICROS_PER_SECOND,
                ))
            }
            iso8601::Duration::Weeks(_) => {
                Err(String::from("Duration format in weeks not supported"))
            }
        }
    }
}
