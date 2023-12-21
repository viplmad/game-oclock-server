use chrono::{Duration, NaiveDate};
use sqlx::PgPool;

use crate::entities::GameLogWithTime;
use crate::errors::ApiErrors;
use crate::models::{GameLogDTO, GameStreakDTO};
use crate::repository::game_log_repository;

use super::base::{check_start_end, handle_result, start_end_to_datetime};
use super::games_service;

pub async fn get_game_streaks(
    pool: &PgPool,
    user_id: &str,
    game_id: &str,
    start_date: NaiveDate,
    end_date: NaiveDate,
) -> Result<Vec<GameStreakDTO>, ApiErrors> {
    check_start_end(start_date, end_date)?;
    games_service::exists_game(pool, user_id, game_id).await?;

    let (start_datetime, end_datetime) = start_end_to_datetime(start_date, end_date);
    let find_result = game_log_repository::find_all_by_game_id_between(
        pool,
        user_id,
        game_id,
        start_datetime,
        end_datetime,
    )
    .await;
    let logs = handle_result::<Vec<GameLogWithTime>, GameLogDTO>(find_result)?;

    let mut streaks: Vec<GameStreakDTO> = vec![];
    for log in logs {
        match streaks.last_mut() {
            Some(last_streak) => {
                let previous_date = last_streak.start_date - Duration::days(1);
                if log.datetime.date() == previous_date {
                    // Continued the streak
                    last_streak.start_date = log.datetime.date();
                    last_streak.days += 1;
                } else if log.datetime.date() < previous_date {
                    // Lost the streak, add new
                    streaks.push(GameStreakDTO {
                        start_date: log.datetime.date(),
                        end_date: log.end_datetime.date(),
                        days: 1,
                    });
                }
            }
            None => streaks.push(GameStreakDTO {
                start_date: log.datetime.date(),
                end_date: log.end_datetime.date(),
                days: 1,
            }),
        }
    }

    Ok(streaks)
}
