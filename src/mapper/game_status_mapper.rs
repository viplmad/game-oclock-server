use crate::models::GameStatus;

impl TryFrom<i16> for GameStatus {
    type Error = ();

    fn try_from(status: i16) -> Result<Self, Self::Error> {
        match status {
            0 => Ok(GameStatus::LowPriority),
            1 => Ok(GameStatus::NextUp),
            2 => Ok(GameStatus::Playing),
            3 => Ok(GameStatus::Played),
            _ => Err(()),
        }
    }
}

impl From<GameStatus> for i16 {
    fn from(status: GameStatus) -> Self {
        match status {
            GameStatus::LowPriority => 0,
            GameStatus::NextUp => 1,
            GameStatus::Playing => 2,
            GameStatus::Played => 3,
        }
    }
}
