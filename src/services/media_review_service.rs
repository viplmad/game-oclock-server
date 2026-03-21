use std::collections::{HashMap, HashSet};

use chrono::{DateTime, Datelike, NaiveDate, Utc};
use uuid::Uuid;

use crate::entities::{MediaSessionWithTime, MediaWithStateWithSession};
use crate::errors::ApiErrors;
use crate::models::{
    DurationDef, MediaIdSessionDTO, MediaIdsStreakDTO, MediaReviewDTO, MediasReviewDTO, SessionDTO,
    StreakDTO,
};

use super::{MediaSessionService, MediaWithSessionService, sessions_utils};

#[derive(Clone)]
pub struct MediaReviewService {
    media_session_service: MediaSessionService,
    media_with_session_service: MediaWithSessionService,
}

impl MediaReviewService {
    pub fn with(
        media_session_service: MediaSessionService,
        media_with_session_service: MediaWithSessionService,
    ) -> Self {
        Self {
            media_session_service,
            media_with_session_service,
        }
    }
}

impl MediaReviewService {
    pub async fn get_session_medias_review(
        &self,
        user_id: &Uuid,
        start_date: NaiveDate,
        end_date: NaiveDate,
    ) -> Result<MediasReviewDTO, ApiErrors> {
        let media_with_sessions = self
            .media_with_session_service
            .find_media_with_sessions_between(user_id, start_date, end_date)
            .await?;

        let media_ids = media_with_sessions
            .iter()
            .map(|media| media.id)
            .collect::<HashSet<Uuid>>()
            .into_iter()
            .collect();
        let first_sessions = self
            .media_session_service
            .find_first_media_sessions_by_medias(user_id, media_ids)
            .await?;

        let review = build_session_review(media_with_sessions, first_sessions);
        Ok(review)
    }
}

fn build_session_review(
    media_with_sessions: Vec<MediaWithStateWithSession>,
    first_sessions: Vec<MediaSessionWithTime>,
) -> MediasReviewDTO {
    let mut map = HashMap::<Uuid, MediaReviewDTO>::new();

    let mut total_streaks: Vec<MediaIdsStreakDTO> = vec![];
    let mut longest_streak = MediaIdsStreakDTO {
        medias_ids: vec![],
        streak: StreakDTO {
            start_date: NaiveDate::default(),
            end_date: NaiveDate::default(),
            days: 0,
            devices_ids: vec![],
        },
    };

    // Fill sessions map and global streaks
    for media_with_session in media_with_sessions {
        let media_id = media_with_session.id;

        let session = SessionDTO::from(&media_with_session);

        // Fill global streaks
        sessions_utils::fill_streaks(&mut total_streaks, &media_id, session.clone());

        // Found longer global streak
        if let Some(new_longest_streak) = get_longest_streak(&total_streaks, &longest_streak) {
            longest_streak = new_longest_streak;
        }

        if !map.contains_key(&media_id) {
            let new_media = MediaReviewDTO::from(media_with_session);
            map.insert(media_id.clone(), new_media);
        }
        let media = map.get_mut(&media_id).unwrap(); // Safe unwrap: already checked the key is contained.
        fill_session_media_review(media, session);
    }

    // Fill first
    for first_session in first_sessions {
        let media_id = first_session.media_id;

        let session = SessionDTO::from(first_session);
        let first_start_datetime = session.start_datetime;

        let media = map.get_mut(&media_id).unwrap(); // Safe unwrap: already checked the key is contained.

        if let Some(last_session) = media.sessions.last() {
            let start_datetime = last_session.start_datetime;
            media.first = first_start_datetime == start_datetime;
        }
    }

    // Fill globals and grouped
    let mut total = 0;
    let mut total_first = 0;
    let mut total_sessions = 0;
    let mut total_rated = 0;
    let mut total_time = DurationDef::default();
    let mut total_time_by_month = HashMap::<u32, DurationDef>::new();
    let mut total_time_by_week = HashMap::<u32, DurationDef>::new();
    let mut total_time_by_weekday = HashMap::<u32, DurationDef>::new();
    let mut total_time_by_hour = HashMap::<u32, DurationDef>::new();
    let mut total_by_release_year = HashMap::<u32, u32>::new();
    let mut total_rated_by_rating = HashMap::<u32, u32>::new();
    let mut longest_session = MediaIdSessionDTO::default();
    let mut first_session = MediaIdSessionDTO {
        media_id: Uuid::default(),
        session: SessionDTO {
            start_datetime: DateTime::<Utc>::MAX_UTC,
            end_datetime: DateTime::default(),
            device_id: None,
            group_id: Uuid::default(),
            started: false,
            finished_status: None,
            time: DurationDef::default(),
            added_datetime: DateTime::default(),
            updated_datetime: DateTime::default(),
        },
    };
    let mut last_session = MediaIdSessionDTO {
        media_id: Uuid::default(),
        session: SessionDTO {
            start_datetime: DateTime::<Utc>::MIN_UTC,
            end_datetime: DateTime::default(),
            device_id: None,
            group_id: Uuid::default(),
            started: false,
            finished_status: None,
            time: DurationDef::default(),
            added_datetime: DateTime::default(),
            updated_datetime: DateTime::default(),
        },
    };
    // Finish
    let mut total_finished = 0;
    let mut total_first_finished = 0;
    let mut total_finished_by_release_year = HashMap::<u32, u32>::new();
    let mut first_finish = MediaIdSessionDTO {
        media_id: Uuid::default(),
        session: SessionDTO {
            start_datetime: DateTime::<Utc>::MAX_UTC,
            end_datetime: DateTime::default(),
            device_id: None,
            group_id: Uuid::default(),
            started: false,
            finished_status: None,
            time: DurationDef::default(),
            added_datetime: DateTime::default(),
            updated_datetime: DateTime::default(),
        },
    };
    let mut last_finish = MediaIdSessionDTO {
        media_id: Uuid::default(),
        session: SessionDTO {
            start_datetime: DateTime::<Utc>::MIN_UTC,
            end_datetime: DateTime::default(),
            device_id: None,
            group_id: Uuid::default(),
            started: false,
            finished_status: None,
            time: DurationDef::default(),
            added_datetime: DateTime::default(),
            updated_datetime: DateTime::default(),
        },
    };

    for media_review in map.values_mut() {
        let media = &media_review.media;
        let media_id = media.media.id;

        total += 1;
        total_first += if media_review.first { 1 } else { 0 };
        total_sessions += media_review.total_sessions;

        // Finish
        total_finished += media_review.total_finished;
        total_first_finished += if media_review.first_finished { 1 } else { 0 };

        // Fill global total time
        total_time = DurationDef::microseconds(total_time.micros + media_review.total_time.micros);
        sessions_utils::merge_total_time_grouped(
            &mut total_time_by_month,
            &media_review.total_time_by_month,
        );
        sessions_utils::merge_total_time_grouped(
            &mut total_time_by_week,
            &media_review.total_time_by_week,
        );
        sessions_utils::merge_total_time_grouped(
            &mut total_time_by_weekday,
            &media_review.total_time_by_weekday,
        );
        sessions_utils::merge_total_time_grouped(
            &mut total_time_by_hour,
            &media_review.total_time_by_hour,
        );

        // Fill global total by release year
        sessions_utils::fill_total_optional_map(
            &mut total_by_release_year,
            &media
                .media
                .release_date
                .map(|d| u32::try_from(d.year()).expect("Year is not AC")),
        );

        sessions_utils::fill_total_optional_map(
            &mut total_finished_by_release_year, // TODO
            &media
                .media
                .release_date
                .map(|d| u32::try_from(d.year()).expect("Year is not AC")),
        );

        if media.state.rating != 0 {
            // Fill global total by rating
            total_rated += 1;
            sessions_utils::fill_total_map(&mut total_rated_by_rating, media.state.rating);
        }

        // Found longer global session
        if let Some(new_longest_session) =
            get_longest_session(&media_review.longest_session, &longest_session, &media_id)
        {
            longest_session = new_longest_session;
        };

        if let Some(new_first_session) =
            get_first_session(&media_review.first_session, &first_session, &media_id)
        {
            first_session = new_first_session;
        }

        if let Some(new_last_session) =
            get_last_session(&media_review.last_session, &last_session, &media_id)
        {
            last_session = new_last_session;
        }

        if let Some(new_first_session) =
            get_first_session(&media_review.first_session, &first_finish, &media_id)
        {
            first_finish = new_first_session; // TODO
        }

        if let Some(new_last_session) =
            get_last_session(&media_review.last_session, &last_finish, &media_id)
        {
            last_finish = new_last_session;
        }
    }

    MediasReviewDTO {
        total,
        total_first,
        longest_streak,
        longest_session,
        first_session,
        last_session,
        total_sessions,
        total_time,
        total_time_by_month,
        total_time_by_week,
        total_time_by_weekday,
        total_time_by_hour,
        total_by_release_year,
        total_rated,
        total_rated_by_rating,
        // Finish
        total_finished,
        total_first_finished,
        first_finish,
        last_finish,
        total_finished_by_release_year,
        medias: map.into_values().collect(),
    }
}

fn get_longest_streak(
    streaks: &[MediaIdsStreakDTO],
    current_longest_streak: &MediaIdsStreakDTO,
) -> Option<MediaIdsStreakDTO> {
    if let Some(last_streak) = streaks.last() {
        let last_streak_days = last_streak.streak.days;
        if last_streak_days > current_longest_streak.streak.days {
            return Some(MediaIdsStreakDTO {
                medias_ids: last_streak.medias_ids.clone(),
                streak: StreakDTO {
                    start_date: last_streak.streak.start_date,
                    end_date: last_streak.streak.end_date,
                    days: last_streak_days,
                    devices_ids: last_streak.streak.devices_ids.clone(),
                },
            });
        }
    }
    None
}

fn get_longest_session(
    longest_session: &SessionDTO,
    current_longest_session: &MediaIdSessionDTO,
    media_id: &Uuid,
) -> Option<MediaIdSessionDTO> {
    let longest_session_time = longest_session.time.clone();
    if longest_session_time.micros > current_longest_session.session.time.micros {
        return Some(MediaIdSessionDTO {
            media_id: media_id.clone(),
            session: SessionDTO {
                start_datetime: longest_session.start_datetime,
                end_datetime: longest_session.end_datetime,
                device_id: longest_session.device_id,
                time: longest_session_time,
                group_id: longest_session.group_id,
                started: longest_session.started,
                finished_status: longest_session.finished_status.clone(),
                added_datetime: longest_session.added_datetime,
                updated_datetime: longest_session.updated_datetime,
            },
        });
    }
    None
}

fn get_first_session(
    first_sesion: &SessionDTO,
    current_first_session: &MediaIdSessionDTO,
    media_id: &Uuid,
) -> Option<MediaIdSessionDTO> {
    let first_session_start_datetime = first_sesion.start_datetime;
    if first_session_start_datetime < current_first_session.session.start_datetime {
        return Some(MediaIdSessionDTO {
            media_id: media_id.clone(),
            session: SessionDTO {
                start_datetime: first_session_start_datetime,
                end_datetime: first_sesion.end_datetime,
                device_id: first_sesion.device_id,
                time: first_sesion.time.clone(),
                group_id: first_sesion.group_id,
                started: first_sesion.started,
                finished_status: first_sesion.finished_status.clone(),
                added_datetime: first_sesion.added_datetime,
                updated_datetime: first_sesion.updated_datetime,
            },
        });
    }
    None
}

fn get_last_session(
    last_sesion: &SessionDTO,
    current_last_session: &MediaIdSessionDTO,
    media_id: &Uuid,
) -> Option<MediaIdSessionDTO> {
    let last_session_start_datetime = last_sesion.start_datetime;
    if last_session_start_datetime > current_last_session.session.start_datetime {
        return Some(MediaIdSessionDTO {
            media_id: media_id.clone(),
            session: SessionDTO {
                start_datetime: last_session_start_datetime,
                end_datetime: last_sesion.end_datetime,
                device_id: last_sesion.device_id,
                time: last_sesion.time.clone(),
                group_id: last_sesion.group_id,
                started: last_sesion.started,
                finished_status: last_sesion.finished_status.clone(),
                added_datetime: last_sesion.added_datetime,
                updated_datetime: last_sesion.updated_datetime,
            },
        });
    }
    None
}

fn fill_session_media_review(media: &mut MediaReviewDTO, session: SessionDTO) {
    // Fill total time
    media.total_time = DurationDef::microseconds(media.total_time.micros + session.time.micros);
    sessions_utils::fill_total_time_by_month(
        &mut media.total_time_by_month,
        session.start_datetime,
        session.time.clone(),
    );
    sessions_utils::fill_total_time_by_week(
        &mut media.total_time_by_week,
        session.start_datetime,
        session.time.clone(),
    );
    sessions_utils::fill_total_time_by_weekday(
        &mut media.total_time_by_weekday,
        session.start_datetime,
        session.time.clone(),
    );
    sessions_utils::fill_total_time_by_hour(
        &mut media.total_time_by_hour,
        session.start_datetime,
        session.end_datetime,
    );

    // Fill streaks
    sessions_utils::fill_media_streaks(&mut media.streaks, session.clone());

    // Found longer streak
    fill_longest_media_streak(media);

    // Fill sessions
    sessions_utils::fill_media_sessions(&mut media.sessions, session);
    media.total_sessions =
        u32::try_from(media.sessions.len()).expect("Count is not within valid range");

    // Found longer session
    fill_longest_first_last_media_session(media);
}

fn fill_longest_media_streak(media: &mut MediaReviewDTO) {
    if let Some(last_streak) = media.streaks.last() {
        let last_streak_days = last_streak.days;
        if last_streak_days > media.longest_streak.days {
            media.longest_streak = StreakDTO {
                start_date: last_streak.start_date,
                end_date: last_streak.end_date,
                days: last_streak_days,
                devices_ids: last_streak.devices_ids.clone(),
            }
        }
    }
}

fn fill_longest_first_last_media_session(media: &mut MediaReviewDTO) {
    if let Some(last_session) = media.sessions.last() {
        let last_session_time = last_session.time.clone();
        if last_session_time.micros > media.longest_session.time.micros {
            media.longest_session = SessionDTO {
                start_datetime: last_session.start_datetime,
                end_datetime: last_session.end_datetime,
                device_id: last_session.device_id.clone(),
                time: last_session_time.clone(),
                group_id: last_session.group_id,
                started: last_session.started,
                finished_status: last_session.finished_status.clone(),
                added_datetime: last_session.added_datetime,
                updated_datetime: last_session.updated_datetime,
            };
        }

        let last_session_start_datetime = last_session.start_datetime;
        if last_session_start_datetime <= media.first_session.start_datetime {
            media.first_session = SessionDTO {
                start_datetime: last_session_start_datetime,
                end_datetime: last_session.end_datetime,
                device_id: last_session.device_id.clone(),
                time: last_session_time.clone(),
                group_id: last_session.group_id,
                started: last_session.started,
                finished_status: last_session.finished_status.clone(),
                added_datetime: last_session.added_datetime,
                updated_datetime: last_session.updated_datetime,
            }
        }

        if last_session_start_datetime >= media.last_session.start_datetime {
            media.last_session = SessionDTO {
                start_datetime: last_session_start_datetime,
                end_datetime: last_session.end_datetime,
                device_id: last_session.device_id.clone(),
                time: last_session_time.clone(),
                group_id: last_session.group_id,
                started: last_session.started,
                finished_status: last_session.finished_status.clone(),
                added_datetime: last_session.added_datetime,
                updated_datetime: last_session.updated_datetime,
            }
        }
    }
}
