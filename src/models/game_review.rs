use std::collections::HashMap;

use chrono::NaiveDate;
use serde::Serialize;
use utoipa::ToSchema;
use uuid::Uuid;

use super::{DurationDef, FinishDTO, GameDTO, LogDTO};

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
    pub game: GameDTO,
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
    pub game: GameDTO,
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
    #[schema(value_type = String, format = Date)]
    pub start_date: NaiveDate,
    #[schema(value_type = String, format = Date)]
    pub end_date: NaiveDate,
    pub days: i64,
    // TODO add device
}

#[derive(Default, Serialize, ToSchema)]
pub struct GameLogDTO {
    #[schema(value_type = String)]
    pub user_id: Uuid,
    #[schema(value_type = String)]
    pub game_id: Uuid,
    pub log: LogDTO,
}

#[derive(Default, Serialize, ToSchema)]
pub struct GameFinishDTO {
    #[schema(value_type = String)]
    pub user_id: Uuid,
    #[schema(value_type = String)]
    pub game_id: Uuid,
    pub finish: FinishDTO,
}

#[derive(Serialize, ToSchema)]
pub struct GamesStreakDTO {
    #[schema(value_type = String)]
    pub games_ids: Vec<Uuid>,
    pub streak: StreakDTO,
}
