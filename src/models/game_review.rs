use std::collections::HashMap;

use chrono::{NaiveDate, NaiveDateTime};
use serde::Serialize;
use utoipa::ToSchema;

use super::{DurationDef, GameLogDTO, GameStatus};

#[derive(Serialize, ToSchema)]
pub struct GamesPlayedReviewDTO {
    pub total_played: i32,
    pub total_first_played: i32,
    pub longest_streak: GamesStreakDTO,
    pub longest_session: GamesLogDTO,
    pub total_sessions: i32,
    #[schema(value_type = String)]
    pub total_time: DurationDef,
    pub total_time_grouped: HashMap<u32, DurationDef>,
    pub total_played_by_release_year: HashMap<i32, i32>,
    pub total_rated: i32,
    pub total_rated_by_rating: HashMap<i32, i32>,
    pub games: Vec<GamePlayedReviewDTO>,
}

#[derive(Serialize, ToSchema)]
pub struct GamePlayedReviewDTO {
    pub id: String,
    pub name: String,
    pub edition: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub release_year: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cover_filename: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cover_url: Option<String>,
    #[schema(value_type = String, format = DateTime)]
    pub added_datetime: NaiveDateTime,
    #[schema(value_type = String, format = DateTime)]
    pub updated_datetime: NaiveDateTime,
    pub status: GameStatus,
    pub rating: i32,
    pub notes: String,
    pub save_folder: String,
    pub screenshot_folder: String,
    pub backup: bool,
    pub longest_streak: GameStreakDTO,
    pub longest_session: GameLogDTO,
    pub total_sessions: i32,
    #[schema(value_type = String)]
    pub total_time: DurationDef,
    pub total_time_grouped: HashMap<u32, DurationDef>,
    pub first_played: bool,
    #[schema(value_type = String, format = DateTime)]
    pub first_play_start_datetime: NaiveDateTime,
    #[schema(value_type = String, format = DateTime)]
    pub last_play_start_datetime: NaiveDateTime,
    #[serde(skip)]
    pub streaks: Vec<GameStreakDTO>,
    #[serde(skip)]
    pub sessions: Vec<GameLogDTO>,
}

#[derive(Serialize, ToSchema)]
pub struct GamesFinishedReviewDTO {
    pub total_finished: i32,
    pub total_first_finished: i32,
    pub total_finished_grouped: HashMap<u32, i32>,
    pub total_finished_by_release_year: HashMap<i32, i32>,
    pub games: Vec<GameFinishedReviewDTO>,
}

#[derive(Serialize, ToSchema)]
pub struct GameFinishedReviewDTO {
    pub id: String,
    pub name: String,
    pub edition: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub release_year: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cover_filename: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cover_url: Option<String>,
    #[schema(value_type = String, format = DateTime)]
    pub added_datetime: NaiveDateTime,
    #[schema(value_type = String, format = DateTime)]
    pub updated_datetime: NaiveDateTime,
    pub status: GameStatus,
    pub rating: i32,
    pub notes: String,
    pub save_folder: String,
    pub screenshot_folder: String,
    pub backup: bool,
    pub total_finished: i32,
    pub total_finished_grouped: HashMap<u32, i32>,
    pub first_finished: bool,
    #[schema(value_type = String, format = Date)]
    pub first_finish: NaiveDate,
    #[schema(value_type = String, format = Date)]
    pub last_finish: NaiveDate,
    #[serde(skip)]
    pub finishes: Vec<NaiveDate>,
}

#[derive(Serialize, ToSchema)]
pub struct GameStreakDTO {
    pub days: i64,
    #[schema(value_type = String, format = Date)]
    pub start_date: NaiveDate,
    #[schema(value_type = String, format = Date)]
    pub end_date: NaiveDate,
}

#[derive(Serialize, ToSchema)]
pub struct GamesLogDTO {
    pub game_id: String,
    #[schema(value_type = String, format = DateTime)]
    pub start_datetime: NaiveDateTime,
    #[schema(value_type = String, format = DateTime)]
    pub end_datetime: NaiveDateTime,
    #[schema(value_type = String)]
    pub time: DurationDef,
}

#[derive(Serialize, ToSchema)]
pub struct GamesStreakDTO {
    pub games_ids: Vec<String>,
    pub days: i64,
    #[schema(value_type = String, format = Date)]
    pub start_date: NaiveDate,
    #[schema(value_type = String, format = Date)]
    pub end_date: NaiveDate,
}
