use chrono::{Duration, NaiveDateTime};
use sqlx::postgres::types::PgInterval;
use uuid::Uuid; // TODO remove reference to sqlx in service

use crate::entities::{GameLog, GameLogWithTime};
use crate::errors::ApiErrors;
use crate::models::{DurationDef, LogDTO, Merge, NewLogDTO};
use crate::repository::GameLogRepository;

use super::helpers::{
    handle_action_result, handle_already_exists_result, handle_get_list_result,
    handle_not_found_result, handle_result,
};
use super::GameService;

#[derive(Clone)]
pub struct GameLogService {
    repository: GameLogRepository,
    game_service: GameService,
}

impl GameLogService {
    pub fn with(repository: GameLogRepository, game_service: GameService) -> Self {
        Self {
            repository,
            game_service,
        }
    }
}

impl GameLogService {
    pub async fn get_sum_game_logs(
        &self,
        user_id: &Uuid,
        game_id: &Uuid,
    ) -> Result<DurationDef, ApiErrors> {
        self.game_service.exists_game(user_id, game_id).await?;

        let find_result = self
            .repository
            .find_sum_time_by_game_id(user_id, game_id)
            .await;
        let duration = handle_result::<PgInterval, LogDTO>(find_result)?;
        Ok(DurationDef::from(duration))
    }

    pub async fn get_game_logs(
        &self,
        user_id: &Uuid,
        game_id: &Uuid,
    ) -> Result<Vec<LogDTO>, ApiErrors> {
        self.game_service.exists_game(user_id, game_id).await?;

        let find_result = self.repository.find_all_by_game_id(user_id, game_id).await;
        handle_get_list_result::<GameLogWithTime, LogDTO>(find_result)
    }

    // For review
    pub(super) async fn find_first_game_logs_by_games(
        &self,
        user_id: &Uuid,
        game_ids: Vec<Uuid>,
    ) -> Result<Vec<GameLogWithTime>, ApiErrors> {
        let find_result = self
            .repository
            .find_all_first_by_user_id_and_game_id_in(user_id, game_ids)
            .await;
        handle_result::<Vec<GameLogWithTime>, LogDTO>(find_result)
    }

    pub async fn create_game_log(
        &self,
        user_id: &Uuid,
        game_id: &Uuid,
        log: NewLogDTO,
    ) -> Result<(), ApiErrors> {
        self.game_service.exists_game(user_id, game_id).await?;

        let start_datetime = log.start_datetime;
        let end_datetime = log.end_datetime;

        if start_datetime > end_datetime {
            return Err(ApiErrors::InvalidParameter(String::from(
                "Session start datetime must be previous than end datetime",
            )));
        }

        let logs: Vec<NewLogDTO> =
            split_session_into_logs(start_datetime, end_datetime, log.device_id);
        if logs.is_empty() {
            return Err(ApiErrors::InvalidParameter(String::from(
                "Session to add must not have an empty span of time",
            )));
        }

        let exists_result = self
            .repository
            .exists_gap(user_id, start_datetime, end_datetime)
            .await;
        handle_already_exists_result::<LogDTO>(exists_result)?;

        let logs_to_create: Vec<GameLog> = logs
            .into_iter()
            .map(LogDTO::merge_with_default)
            .map(GameLog::from)
            .collect();
        let create_result = self.repository.create_multiple(logs_to_create).await;
        handle_action_result::<LogDTO>(create_result)
    }

    pub async fn delete_game_log(
        &self,
        user_id: &Uuid,
        game_id: &Uuid,
        start_datetime: NaiveDateTime,
    ) -> Result<(), ApiErrors> {
        self.game_service.exists_game(user_id, game_id).await?;
        self.exists_game_log(user_id, game_id, start_datetime)
            .await?;

        let delete_result = self
            .repository
            .delete_by_id(user_id, game_id, start_datetime)
            .await;
        handle_action_result::<LogDTO>(delete_result)
    }

    pub async fn exists_game_log(
        &self,
        user_id: &Uuid,
        game_id: &Uuid,
        start_datetime: NaiveDateTime,
    ) -> Result<(), ApiErrors> {
        let exists_result = self
            .repository
            .exists_by_id(user_id, game_id, start_datetime)
            .await;
        handle_not_found_result::<LogDTO>(exists_result)
    }
}

fn split_session_into_logs(
    start_datetime: NaiveDateTime,
    end_datetime: NaiveDateTime,
    device_id: Option<Uuid>,
) -> Vec<NewLogDTO> {
    let mut sessions: Vec<NewLogDTO> = vec![];
    if start_datetime.date() == end_datetime.date() {
        // If session happens on the same day
        if start_datetime.time() != end_datetime.time() {
            // Avoid empty log -> return single session if span of time is valid
            sessions.push(NewLogDTO {
                start_datetime,
                end_datetime,
                device_id: device_id.clone(),
            });
        }
    } else {
        // If session spans differents day
        let mut temp_date = start_datetime;
        while temp_date.date() < end_datetime.date() {
            let next_day_at_start_of_day =
                crate::date_utils::date_at_start_of_day(temp_date.date() + Duration::days(1));
            sessions.push(NewLogDTO {
                start_datetime: temp_date,
                end_datetime: next_day_at_start_of_day,
                device_id: device_id.clone(),
            });
            temp_date = next_day_at_start_of_day;
        }
        if end_datetime.time() != temp_date.time() {
            // Avoid empty log -> store last log if span of time until end date is valid
            sessions.push(NewLogDTO {
                start_datetime: temp_date,
                end_datetime,
                device_id: device_id.clone(),
            });
        }
    }
    sessions
}
