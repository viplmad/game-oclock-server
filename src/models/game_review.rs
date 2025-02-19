use std::collections::HashMap;

use chrono::{NaiveDate, NaiveDateTime};
use serde::Serialize;
use utoipa::ToSchema;
use uuid::Uuid;

use super::{DurationDef, FinishDTO, GameStatus, LogDTO};

#[derive(Serialize, ToSchema)]
pub struct GamesPlayedReviewDTO {
    pub total_played: u32,
    pub total_first_played: u32,
    pub longest_streak: GamesStreakDTO,
    pub longest_session: GameLogDTO,
    pub first_session: GameLogDTO,
    pub last_session: GameLogDTO,
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
    pub total_played_by_release_year: HashMap<u32, u32>,
    pub total_rated: u32,
    pub total_rated_by_rating: HashMap<u32, u32>,
    pub games: Vec<GamePlayedReviewDTO>,
}

#[derive(Serialize, ToSchema)]
pub struct GamePlayedReviewDTO {
    #[schema(value_type = String)]
    pub id: Uuid,
    #[schema(value_type = String)]
    pub user_id: Uuid,
    pub title: String,
    pub edition: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub release_date: Option<NaiveDate>,
    #[schema(value_type = String)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_game_id: Option<Uuid>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cover_url: Option<String>,
    #[schema(value_type = String, format = DateTime)]
    pub added_datetime: NaiveDateTime,
    #[schema(value_type = String, format = DateTime)]
    pub updated_datetime: NaiveDateTime,
    pub status: GameStatus,
    pub rating: u32,
    pub notes: String,
    pub first_played: bool,
    pub longest_streak: StreakDTO,
    pub longest_session: LogDTO,
    pub first_session: LogDTO,
    pub last_session: LogDTO,
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
    #[serde(skip)]
    pub streaks: Vec<StreakDTO>,
    #[serde(skip)]
    pub sessions: Vec<LogDTO>,
}

#[derive(Serialize, ToSchema)]
pub struct GamesFinishedReviewDTO {
    pub total_finished: u32,
    pub total_first_finished: u32,
    pub first_finish: GameFinishDTO,
    pub last_finish: GameFinishDTO,
    pub total_finished_grouped: HashMap<u32, u32>,
    pub total_finished_by_release_year: HashMap<u32, u32>,
    pub games: Vec<GameFinishedReviewDTO>,
}

#[derive(Serialize, ToSchema)]
pub struct GameFinishedReviewDTO {
    #[schema(value_type = String)]
    pub id: Uuid,
    #[schema(value_type = String)]
    pub user_id: Uuid,
    pub title: String,
    pub edition: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub release_date: Option<NaiveDate>,
    #[schema(value_type = String)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_game_id: Option<Uuid>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cover_url: Option<String>,
    #[schema(value_type = String, format = DateTime)]
    pub added_datetime: NaiveDateTime,
    #[schema(value_type = String, format = DateTime)]
    pub updated_datetime: NaiveDateTime,
    pub status: GameStatus,
    pub rating: u32,
    pub notes: String,
    pub total_finished: u32,
    pub total_finished_grouped: HashMap<u32, u32>,
    pub first_finished: bool,
    pub first_finish: FinishDTO,
    pub last_finish: FinishDTO,
    #[serde(skip)]
    pub finishes: Vec<FinishDTO>,
}

#[derive(Serialize, ToSchema)]
pub struct StreakDTO {
    pub days: i64,
    #[schema(value_type = String, format = Date)]
    pub start_date: NaiveDate,
    #[schema(value_type = String, format = Date)]
    pub end_date: NaiveDate,
    // TODO add device
}

#[derive(Default, Serialize, ToSchema)]
pub struct GameLogDTO {
    #[schema(value_type = String)]
    pub user_id: Uuid,
    #[schema(value_type = String)]
    pub game_id: Uuid,
    #[schema(value_type = String, format = DateTime)]
    pub start_datetime: NaiveDateTime,
    #[schema(value_type = String, format = DateTime)]
    pub end_datetime: NaiveDateTime,
    #[schema(value_type = String)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_id: Option<Uuid>,
    #[schema(value_type = String)]
    pub time: DurationDef,
}

#[derive(Default, Serialize, ToSchema)]
pub struct GameFinishDTO {
    #[schema(value_type = String)]
    pub user_id: Uuid,
    #[schema(value_type = String)]
    pub game_id: Uuid,
    #[schema(value_type = String, format = Date)]
    pub date: NaiveDate,
    pub status: GameStatus,
    #[schema(value_type = String)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_id: Option<Uuid>,
}

#[derive(Serialize, ToSchema)]
pub struct GamesStreakDTO {
    #[schema(value_type = String)]
    pub games_ids: Vec<Uuid>,
    pub days: i64,
    #[schema(value_type = String, format = Date)]
    pub start_date: NaiveDate,
    #[schema(value_type = String, format = Date)]
    pub end_date: NaiveDate,
    // TODO Add device
}
