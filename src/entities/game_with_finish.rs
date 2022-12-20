use std::str::FromStr;

use chrono::{NaiveDate, NaiveDateTime};
use sea_query::Iden;
use sqlx::FromRow;

use super::{convert_game_field, FieldIden, FieldType, GameFinishIden, Search, TableIden};

pub type GameWithFinishSearch = Search<GameWithFinishIden>;

#[derive(Clone, Copy, Iden)]
pub enum GameWithFinishIden {
    Table,
}

impl TableIden for GameWithFinishIden {
    const TABLE: Self = Self::Table;
}

#[derive(FromRow, Clone)]
pub struct GameWithFinish {
    pub id: i32,
    pub user_id: i32,
    pub name: String,
    pub edition: String,
    pub release_year: Option<i32>,
    pub cover_filename: Option<String>,
    pub added_datetime: NaiveDateTime,
    pub updated_datetime: NaiveDateTime,
    pub status: i16,
    pub rating: i32,
    pub notes: String,
    pub save_folder: String,
    pub screenshot_folder: String,
    pub backup: bool,
    pub finish_date: NaiveDate,
}

impl FromStr for FieldIden<GameWithFinishIden> {
    type Err = ();

    fn from_str(field: &str) -> Result<Self, Self::Err> {
        convert_game_field::<GameWithFinishIden>(field).map_or_else(
            |_| match field {
                "date" => Ok(FieldIden::new(GameFinishIden::Date, FieldType::Date)),
                _ => Err(()),
            },
            Ok,
        )
    }
}
