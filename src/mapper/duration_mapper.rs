use sqlx::postgres::types::PgInterval;

use crate::models::DurationDef;

impl From<DurationDef> for PgInterval {
    fn from(duration: DurationDef) -> Self {
        Self {
            months: 0,
            days: 0,
            microseconds: i64::try_from(duration.micros).expect("Time was not within valid range"),
        }
    }
}

impl From<PgInterval> for DurationDef {
    fn from(interval: PgInterval) -> Self {
        let microseconds = interval.microseconds;
        Self {
            micros: u64::try_from(microseconds).expect("Time was not within valid range"),
        }
    }
}
