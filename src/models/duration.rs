use std::{
    fmt::{Display, Formatter},
    str::FromStr,
};

use serde::{de::Visitor, Deserialize, Serialize};

const MICROS_PER_SEC: u64 = 1_000_000;
const SECONDS_PER_HOUR: u64 = 3600;
const SECONDS_PER_MINUTE: u64 = 60;

pub struct DurationDef {
    pub micros: u64,
}

impl DurationDef {
    fn as_secs(&self) -> u64 {
        self.micros / MICROS_PER_SEC
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
        formatter.write_str("a string of a duration in iso8601 format")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        let duration = DurationDef::from_str(v);
        Ok(duration.unwrap()) // TODO
    }
}

impl Display for DurationDef {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        // TODO
        let secs = self.as_secs();

        let mut hours: u32 = 0;
        let mut minutes: u32 = 0;
        let mut seconds: u32 = secs as u32;
        if secs > 0 {
            hours = (secs / SECONDS_PER_HOUR) as u32;
            if hours > 0 {
                let remainder_seconds: u32 = (secs % SECONDS_PER_HOUR) as u32;
                minutes = remainder_seconds / (SECONDS_PER_MINUTE as u32);
                if minutes > 0 {
                    seconds = remainder_seconds % (SECONDS_PER_MINUTE as u32);
                }
            } else {
                minutes = (secs / SECONDS_PER_MINUTE) as u32;
                if minutes > 0 {
                    seconds = (secs % SECONDS_PER_MINUTE) as u32;
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
    type Err = (); // TODO
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let duration = iso8601::duration(s).map_err(|_| ())?;
        match duration {
            iso8601::Duration::YMDHMS {
                year: _,
                month: _,
                day: _,
                hour,
                minute,
                second,
                millisecond: _,
            } => {
                let duration_secs: u64 = u64::from(
                    hour * (SECONDS_PER_HOUR as u32)
                        + minute * (SECONDS_PER_MINUTE as u32)
                        + second,
                );
                Ok(DurationDef {
                    micros: duration_secs * MICROS_PER_SEC,
                })
            }
            iso8601::Duration::Weeks(_) => todo!(),
        }
    }
}
