use std::str::FromStr;

use sea_query::Iden;

use super::{convert_game_field, FieldIden, FieldType, GameFinishIden, Search, TableIden};

pub type GameWithFinishSearch = Search<GameWithFinishIden>;

// Fake iden to allow query with finish date
#[derive(Clone, Copy, Iden)]
pub enum GameWithFinishIden {
    Table,
}

impl TableIden for GameWithFinishIden {
    const TABLE: Self = Self::Table;
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
