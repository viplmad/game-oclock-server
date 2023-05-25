use sqlx::postgres::types::PgInterval;

use crate::models::DurationDef;

impl From<DurationDef> for PgInterval {
    fn from(duration: DurationDef) -> Self {
        Self {
            months: 0,
            days: 0,
            microseconds: duration.micros,
        }
    }
}

impl From<PgInterval> for DurationDef {
    fn from(interval: PgInterval) -> Self {
        Self::microseconds(interval.microseconds)
    }
}
