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
        let days_in_micros = i64::from(interval.days)
            * crate::date_utils::SECONDS_PER_DAY
            * crate::date_utils::MICROS_PER_SECOND;
        Self::microseconds(interval.microseconds + days_in_micros)
    }
}
