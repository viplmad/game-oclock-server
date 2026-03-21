use std::collections::HashMap;

use chrono::NaiveDate;
use serde::Serialize;
use utoipa::ToSchema;
use uuid::Uuid;

use super::{DurationDef, MediaDTO, SessionDTO};

// TODO generate token for anonymous users to see review of a specific year
#[derive(Serialize, ToSchema)]
pub struct MediasReviewDTO {
    pub total: u32,
    pub total_first: u32,
    pub longest_streak: MediaIdsStreakDTO,
    pub longest_session: MediaIdSessionDTO,
    pub first_session: MediaIdSessionDTO,
    pub last_session: MediaIdSessionDTO,
    pub total_sessions: u32,
    #[schema(value_type = String)]
    pub total_time: DurationDef,
    #[schema(additional_properties)]
    pub total_time_by_month: HashMap<u32, DurationDef>,
    #[schema(additional_properties)]
    pub total_time_by_week: HashMap<u32, DurationDef>,
    #[schema(additional_properties)]
    pub total_time_by_weekday: HashMap<u32, DurationDef>,
    #[schema(additional_properties)]
    pub total_time_by_hour: HashMap<u32, DurationDef>,
    pub total_by_release_year: HashMap<u32, u32>,
    pub total_rated: u32,
    pub total_rated_by_rating: HashMap<u32, u32>,
    // Finished
    pub total_finished: u32,
    pub total_first_finished: u32,
    pub first_finish: MediaIdSessionDTO,
    pub last_finish: MediaIdSessionDTO,
    // TODO pub total_finished_grouped: HashMap<u32, u32>,
    pub total_finished_by_release_year: HashMap<u32, u32>,
    //
    pub medias: Vec<MediaReviewDTO>,
}

#[derive(Serialize, ToSchema)]
pub struct MediaReviewDTO {
    pub media: MediaDTO,
    pub first: bool,
    pub longest_streak: StreakDTO,
    pub longest_session: SessionDTO,
    pub first_session: SessionDTO,
    pub last_session: SessionDTO,
    pub total_sessions: u32,
    #[schema(value_type = String)]
    pub total_time: DurationDef,
    #[schema(additional_properties)]
    pub total_time_by_month: HashMap<u32, DurationDef>,
    #[schema(additional_properties)]
    pub total_time_by_week: HashMap<u32, DurationDef>,
    #[schema(additional_properties)]
    pub total_time_by_weekday: HashMap<u32, DurationDef>,
    #[schema(additional_properties)]
    pub total_time_by_hour: HashMap<u32, DurationDef>,
    // Finished
    pub total_finished: u32,
    // TODO pub total_finished_grouped: HashMap<u32, u32>,
    pub first_finished: bool,
    pub first_finish: SessionDTO,
    pub last_finish: SessionDTO,
    //
    #[serde(skip)]
    pub streaks: Vec<StreakDTO>,
    #[serde(skip)]
    pub sessions: Vec<SessionDTO>,
}

#[derive(Serialize, ToSchema)]
pub struct StreakDTO {
    #[schema(value_type = String, format = Date)]
    pub start_date: NaiveDate,
    #[schema(value_type = String, format = Date)]
    pub end_date: NaiveDate,
    pub days: i64,
    #[schema(value_type = String)]
    pub devices_ids: Vec<Uuid>,
    // TODO add sessions
}

#[derive(Default, Serialize, ToSchema)]
pub struct MediaIdSessionDTO {
    #[schema(value_type = String)]
    pub media_id: Uuid,
    pub session: SessionDTO,
}

#[derive(Serialize, ToSchema)]
pub struct MediaIdsStreakDTO {
    #[schema(value_type = String)]
    pub medias_ids: Vec<Uuid>,
    pub streak: StreakDTO,
    // TODO add sessions
}
