use chrono::{DateTime, FixedOffset};
use sea_query::{
    Alias, Expr, ExprTrait, Func, Iden, Order, Query, QueryStatementWriter, SelectStatement,
    SimpleExpr,
};
use uuid::Uuid;

use crate::entities::{
    AggregateGroup, AggregateGroupQuery, AggregateQuery, Filter, MediaIden, MediaListSearch,
    MediaSession, MediaSessionIden, MediaStateIden, QUERY_TIME_ALIAS, SESSION_ADDED_DATETIME_ALIAS,
    SESSION_DEVICE_ID_ALIAS, SESSION_END_DATE_ALIAS, SESSION_FINISHED_STATUS_ALIAS,
    SESSION_GROUP_ID_ALIAS, SESSION_MEDIA_ID_ALIAS, SESSION_START_DATE_ALIAS,
    SESSION_STARTED_ALIAS, SESSION_UPDATED_DATETIME_ALIAS, STREAK_DAYS_ALIAS,
    STREAK_DEVICE_IDS_ALIAS, STREAK_END_DATE_ALIAS, STREAK_MEDIA_IDS_ALIAS,
    STREAK_START_DATE_ALIAS, SearchQuery, SessionAggregateGroupSearch, SessionAggregateSearch,
    SessionListSearch, TableIden,
};
use crate::errors::SearchErrors;

use super::media_query;
use super::search::{
    apply_aggregate_group_search, apply_aggregate_search, apply_search, apply_search_filter,
    apply_search_pagination,
};

#[cfg(test)]
mod tests {
    use sea_query::{BinOper, PostgresQueryBuilder};
    use uuid::Uuid;

    use super::*;
    use crate::entities::{
        AggregateCountMetric, AggregateDateHistogramGroup, AggregateGroup, AggregateMetric,
        AggregateSumMetric, ColIden, ExprIden, FieldIden, FieldType, Filter,
        GroupDateHistogramInterval, ListSearch, SingleValueFilter,
    };

    #[test]
    fn count_all_sessions() {
        let user_id = Uuid::try_parse("00000000-0000-0000-0000-000000000000").unwrap();
        let query = aggregate_all_with_search(
            &user_id,
            SessionAggregateSearch {
                filter: None,
                aggr: AggregateMetric::Count(AggregateCountMetric::new(
                    FieldIden::Col(ColIden::new(MediaSessionIden::MediaId, FieldType::String)),
                    None,
                    false,
                )),
            },
        );
        assert_eq!(
            query.unwrap().query.to_string(PostgresQueryBuilder),
            [
                r#"SELECT COUNT("MediaSession"."media_id")"#,
                r#"FROM "MediaSession""#,
                r#"WHERE "MediaSession"."user_id" = '00000000-0000-0000-0000-000000000000'"#
            ]
            .join(" ")
        );
    }

    #[test]
    fn sum_time_all_sessions() {
        let user_id = Uuid::try_parse("00000000-0000-0000-0000-000000000000").unwrap();
        let query = aggregate_all_with_search(
            &user_id,
            SessionAggregateSearch {
                filter: None,
                aggr: AggregateMetric::Sum(AggregateSumMetric::new(
                    FieldIden::Expr(ExprIden::new::<MediaSessionIden>(
                        Expr::col((MediaSessionIden::Table, MediaSessionIden::EndDate)).sub(
                            Expr::col((MediaSessionIden::Table, MediaSessionIden::StartDate)),
                        ),
                        FieldType::Integer,
                    )),
                    None,
                )),
            },
        );
        assert_eq!(
            query.unwrap().query.to_string(PostgresQueryBuilder),
            [
                r#"SELECT SUM("MediaSession"."end_date" - "MediaSession"."start_date")"#,
                r#"FROM "MediaSession""#,
                r#"WHERE "MediaSession"."user_id" = '00000000-0000-0000-0000-000000000000'"#
            ]
            .join(" ")
        );
    }

    #[test]
    fn group_medias_by_month_played() {
        let user_id = Uuid::try_parse("00000000-0000-0000-0000-000000000000").unwrap();
        let query = aggregate_group_with_search(
            &user_id,
            SessionAggregateGroupSearch {
                filter: None,
                aggr: AggregateMetric::Count(AggregateCountMetric::new(
                    FieldIden::Col(ColIden::new(MediaSessionIden::MediaId, FieldType::String)),
                    None,
                    true,
                )),
                group: AggregateGroup::DateHistogram(AggregateDateHistogramGroup::new(
                    FieldIden::Col(ColIden::new(
                        MediaSessionIden::StartDate,
                        FieldType::DateTime,
                    )),
                    None,
                    GroupDateHistogramInterval::Month,
                )),
            },
        );
        assert_eq!(
            query.unwrap().query.to_string(PostgresQueryBuilder),
            [
                r#"SELECT CAST(DATE_PART('month', "MediaSession"."start_date") AS BIGINT), COUNT(DISTINCT "MediaSession"."media_id")"#,
                r#"FROM "MediaSession""#,
                r#"WHERE "MediaSession"."user_id" = '00000000-0000-0000-0000-000000000000'"#,
                r#"GROUP BY CAST(DATE_PART('month', "MediaSession"."start_date") AS BIGINT)"#
            ]
            .join(" ")
        );
    }

    #[test]
    fn streaks() {
        let user_id = Uuid::try_parse("00000000-0000-0000-0000-000000000000").unwrap();
        let query = select_streaks_with_search(
            &user_id,
            ListSearch {
                filter: Some(vec![
                    Filter::GreaterThanOrEqual(SingleValueFilter::new(
                        FieldIden::Col(ColIden::new::<MediaSessionIden>(
                            MediaSessionIden::StartDate,
                            FieldType::String,
                        )),
                        String::from("2025-01-01T00:00:00Z"),
                        BinOper::And,
                    )),
                    Filter::SmallerThan(SingleValueFilter::new(
                        FieldIden::Col(ColIden::new::<MediaSessionIden>(
                            MediaSessionIden::EndDate,
                            FieldType::String,
                        )),
                        String::from("2026-01-01T00:00:00Z"),
                        BinOper::And,
                    )),
                ]),
                sort: None,
                page: None,
                size: None,
            },
        );
        assert_eq!(
            query.unwrap().query.to_string(PostgresQueryBuilder),
            [
                r#"SELECT MIN("start_date") AS "start_date", MAX("end_date") AS "end_date", ARRAY_REMOVE(ARRAY_AGG(DISTINCT "media_id"), NULL) AS "media_ids", ARRAY_REMOVE(ARRAY_AGG(DISTINCT "device_id"), NULL) AS "device_ids", (MAX("end_date") - MIN("start_date")) + 1 AS "days""#,
                r#"FROM (SELECT "start_date", "end_date", "media_id", "device_id", COUNT(*) FILTER (WHERE "starts_streak") OVER (ORDER BY "start_date") AS "grp""#,
                r#"FROM (SELECT CAST(("MediaSession"."start_date" AT TIME ZONE "MediaSession"."start_date_tz") AS DATE) AS "start_date", CAST(("MediaSession"."end_date" AT TIME ZONE "MediaSession"."end_date_tz") AS DATE) AS "end_date", "MediaSession"."media_id" AS "media_id", "MediaSession"."device_id" AS "device_id", COALESCE(CAST(("MediaSession"."start_date" AT TIME ZONE "MediaSession"."start_date_tz") AS DATE) - (LAG(CAST(("MediaSession"."end_date" AT TIME ZONE "MediaSession"."end_date_tz") AS DATE)) OVER (ORDER BY "MediaSession"."start_date")), 99) > 1 AS "starts_streak""#,
                r#"FROM "MediaSession""#,
                r#"WHERE "MediaSession"."user_id" = '00000000-0000-0000-0000-000000000000'"#,
                r#"AND "MediaSession"."start_date" >= '2025-01-01T00:00:00Z'"#,
                r#"AND "MediaSession"."end_date" < '2026-01-01T00:00:00Z'"#,
                r#"ORDER BY "MediaSession"."start_date" ASC) AS "starts_streak_sub""#,
                r#"ORDER BY "start_date" ASC) AS "streaks_group_sub""#,
                r#"GROUP BY "grp" ORDER BY (MAX("end_date") - MIN("start_date")) + 1 DESC"#,
                r#"LIMIT 500 OFFSET 0"#,
            ]
            .join(" ")
        );
    }

    #[test]
    fn total_played_first_time() {
        let user_id = Uuid::try_parse("00000000-0000-0000-0000-000000000000").unwrap();
        let query = aggregate_all_first_with_search(
            &user_id,
            SessionAggregateSearch {
                filter: Some(vec![
                    Filter::GreaterThanOrEqual(SingleValueFilter::new(
                        FieldIden::Col(ColIden::new::<MediaSessionIden>(
                            MediaSessionIden::StartDate,
                            FieldType::String,
                        )),
                        String::from("2025-01-01T00:00:00Z"),
                        BinOper::And,
                    )),
                    Filter::SmallerThan(SingleValueFilter::new(
                        FieldIden::Col(ColIden::new::<MediaSessionIden>(
                            MediaSessionIden::StartDate,
                            FieldType::String,
                        )),
                        String::from("2026-01-01T00:00:00Z"),
                        BinOper::And,
                    )),
                ]),
                aggr: AggregateMetric::Count(AggregateCountMetric::new(
                    FieldIden::Col(ColIden::new::<MediaSessionIden>(
                        MediaSessionIden::MediaId,
                        FieldType::String,
                    )),
                    None,
                    true,
                )),
            },
        );
        assert_eq!(
            query.unwrap().query.to_string(PostgresQueryBuilder),
            [
                r#"SELECT COUNT(DISTINCT "MediaSession"."media_id")"#,
                r#"FROM "MediaSession""#,
                r#"WHERE "MediaSession"."user_id" = '00000000-0000-0000-0000-000000000000'"#,
                r#"AND "MediaSession"."start_date" IN (SELECT MIN("sub"."start_date")"#,
                r#"FROM "MediaSession" AS "sub""#,
                r#"WHERE "sub"."user_id" = '00000000-0000-0000-0000-000000000000'"#,
                r#"AND "sub"."media_id" = "MediaSession"."media_id")"#,
                r#"AND "MediaSession"."start_date" >= '2025-01-01T00:00:00Z'"#,
                r#"AND "MediaSession"."start_date" < '2026-01-01T00:00:00Z'"#,
            ]
            .join(" "),
        );
    }
}

pub fn select_by_id(
    user_id: &Uuid,
    media_id: &Uuid,
    start_datetime: DateTime<FixedOffset>,
) -> impl QueryStatementWriter {
    let mut select = Query::select();

    from_and_where_user_id_and_media_id(&mut select, user_id, media_id);
    select.and_where(
        Expr::col((MediaSessionIden::Table, MediaSessionIden::StartDate)).eq(start_datetime),
    );
    add_fields(&mut select);

    select
}

pub fn select_all_by_media_id_with_search(
    user_id: &Uuid,
    media_id: &Uuid,
    search: SessionListSearch,
) -> Result<SearchQuery, SearchErrors> {
    let mut select = Query::select();

    from_and_where_user_id_and_media_id(&mut select, user_id, media_id);
    add_fields(&mut select);

    apply_search(select, search)
}

pub fn select_all_with_search(
    user_id: &Uuid,
    search: SessionListSearch,
) -> Result<SearchQuery, SearchErrors> {
    let mut select = Query::select();

    from_and_where_user_id(&mut select, user_id);
    add_fields(&mut select);

    apply_search(select, search)
}

pub fn aggregate_all_by_media_id_with_search(
    user_id: &Uuid,
    media_id: &Uuid,
    search: SessionAggregateSearch,
) -> Result<AggregateQuery, SearchErrors> {
    let mut select = Query::select();

    from_and_where_user_id_and_media_id(&mut select, user_id, media_id);

    apply_aggregate_search(select, search)
}

pub fn aggregate_all_with_search(
    user_id: &Uuid,
    search: SessionAggregateSearch,
) -> Result<AggregateQuery, SearchErrors> {
    let mut select = Query::select();

    from_and_where_user_id(&mut select, user_id);

    apply_aggregate_search(select, search)
}

pub fn aggregate_group_with_search(
    user_id: &Uuid,
    search: SessionAggregateGroupSearch,
) -> Result<AggregateGroupQuery, SearchErrors> {
    let mut select = Query::select();

    from_and_where_user_id(&mut select, user_id);
    join_ext_tables(&mut select, user_id, extract_group_ext_table(&search.group));

    apply_aggregate_group_search(select, search)
}

fn join_ext_tables(select: &mut SelectStatement, user_id: &Uuid, tables: Vec<String>) {
    tables.into_iter().for_each(|table| {
        if table == MediaIden::Table.to_string() {
            select.left_join(
                MediaIden::Table,
                Expr::col((MediaSessionIden::Table, MediaSessionIden::MediaId))
                    .equals((MediaIden::Table, MediaIden::Id)),
            );
        } else if table == MediaStateIden::Table.to_string() {
            select.left_join(
                MediaStateIden::Table,
                Expr::col((MediaSessionIden::Table, MediaSessionIden::MediaId))
                    .equals((MediaStateIden::Table, MediaStateIden::MediaId))
                    .and(
                        Expr::col((MediaStateIden::Table, MediaStateIden::UserId))
                            .eq(crate::uuid_utils::to_string(user_id)),
                    ),
            );
        }
    });
}

fn extract_group_ext_table<T: TableIden>(group: &AggregateGroup<T>) -> Vec<String> {
    group.field_ext_table().map(|t| vec![t]).unwrap_or_default()
}

pub fn aggregate_all_first_with_search(
    user_id: &Uuid,
    search: SessionAggregateSearch,
) -> Result<AggregateQuery, SearchErrors> {
    let mut select = Query::select();

    from_and_where_user_id(&mut select, user_id);
    let min_subquery = Query::select()
        .from_as(MediaSessionIden::Table, "sub")
        .expr(Expr::col(("sub", MediaSessionIden::StartDate)).min())
        .and_where(
            Expr::col(("sub", MediaSessionIden::UserId)).eq(crate::uuid_utils::to_string(user_id)),
        )
        .and_where(
            Expr::col(("sub", MediaSessionIden::MediaId))
                .equals((MediaSessionIden::Table, MediaSessionIden::MediaId)),
        )
        .take();
    select.and_where(
        Expr::col((MediaSessionIden::Table, MediaSessionIden::StartDate)).in_subquery(min_subquery),
    );

    apply_aggregate_search(select, search)
}

fn select_all_media_with_session_by_start_datetime_gte_and_start_datetime_lte(
    user_id: &Uuid,
    start_datetime: Option<DateTime<FixedOffset>>,
    end_datetime: Option<DateTime<FixedOffset>>,
) -> SelectStatement {
    let mut select = media_query::select_all(user_id);

    join_media_session(&mut select);
    where_optional_start_datetime_gte_and_start_datetime_lte(
        &mut select,
        start_datetime,
        end_datetime,
    );

    select
}

pub fn select_all_first_media_with_session_with_search_by_start_datetime_gte_and_start_datetime_lte_order_by_start_datetime_desc(
    user_id: &Uuid,
    start_datetime: Option<DateTime<FixedOffset>>,
    end_datetime: Option<DateTime<FixedOffset>>,
    mut search: MediaListSearch,
) -> Result<SearchQuery, SearchErrors> {
    let mut select = select_all_media_with_session_by_start_datetime_gte_and_start_datetime_lte(
        user_id,
        start_datetime,
        end_datetime,
    );

    add_join_fields(&mut select);
    select.order_by_expr(
        Expr::col((MediaSessionIden::Table, MediaSessionIden::StartDate)).min(),
        Order::Asc,
    );

    // Ignore sort, might conflict with date ordering
    search.sort = None;
    apply_search(select, search)
}

pub fn select_all_last_media_with_session_with_search_by_start_datetime_gte_and_start_datetime_lte_order_by_start_datetime_desc(
    user_id: &Uuid,
    start_datetime: Option<DateTime<FixedOffset>>,
    end_datetime: Option<DateTime<FixedOffset>>,
    mut search: MediaListSearch,
) -> Result<SearchQuery, SearchErrors> {
    let mut select = select_all_media_with_session_by_start_datetime_gte_and_start_datetime_lte(
        user_id,
        start_datetime,
        end_datetime,
    );

    add_join_fields(&mut select);
    select.order_by_expr(
        Expr::col((MediaSessionIden::Table, MediaSessionIden::StartDate)).max(),
        Order::Desc,
    );

    // Ignore sort, might conflict with date ordering
    search.sort = None;
    apply_search(select, search)
}

fn add_join_fields(select: &mut SelectStatement) {
    select
        .expr_as(
            Expr::col((MediaSessionIden::Table, MediaSessionIden::MediaId)),
            Alias::new(SESSION_MEDIA_ID_ALIAS),
        )
        .expr_as(
            Expr::col((MediaSessionIden::Table, MediaSessionIden::StartDate)),
            Alias::new(SESSION_START_DATE_ALIAS),
        )
        .expr_as(
            Expr::col((MediaSessionIden::Table, MediaSessionIden::EndDate)),
            Alias::new(SESSION_END_DATE_ALIAS),
        )
        .expr_as(
            Expr::col((MediaSessionIden::Table, MediaSessionIden::DeviceId)),
            Alias::new(SESSION_DEVICE_ID_ALIAS),
        )
        .expr_as(
            Expr::col((MediaSessionIden::Table, MediaSessionIden::GroupId)),
            Alias::new(SESSION_GROUP_ID_ALIAS),
        )
        .expr_as(
            Expr::col((MediaSessionIden::Table, MediaSessionIden::Started)),
            Alias::new(SESSION_STARTED_ALIAS),
        )
        .expr_as(
            Expr::col((MediaSessionIden::Table, MediaSessionIden::FinishedStatus)),
            Alias::new(SESSION_FINISHED_STATUS_ALIAS),
        )
        .expr_as(
            Expr::col((MediaSessionIden::Table, MediaSessionIden::AddedDatetime)),
            Alias::new(SESSION_ADDED_DATETIME_ALIAS),
        )
        .expr_as(
            Expr::col((MediaSessionIden::Table, MediaSessionIden::UpdatedDatetime)),
            Alias::new(SESSION_UPDATED_DATETIME_ALIAS),
        )
        .expr_as(derived_time_expr(), Alias::new(QUERY_TIME_ALIAS));
}

pub fn insert(media_session: &MediaSession) -> impl QueryStatementWriter {
    let mut insert = Query::insert();

    insert
        .into_table(MediaSessionIden::Table)
        .columns([
            MediaSessionIden::UserId,
            MediaSessionIden::MediaId,
            MediaSessionIden::StartDate,
            MediaSessionIden::EndDate,
            MediaSessionIden::DeviceId,
            MediaSessionIden::GroupId,
            MediaSessionIden::Started,
            MediaSessionIden::FinishedStatus,
            MediaSessionIden::AddedDatetime,
            MediaSessionIden::UpdatedDatetime,
        ])
        .values_panic([
            crate::uuid_utils::to_string(&media_session.user_id).into(),
            crate::uuid_utils::to_string(&media_session.media_id).into(),
            media_session.start_date.into(),
            media_session.end_date.into(),
            media_session
                .device_id
                .map(|id| crate::uuid_utils::to_string(&id))
                .into(),
            crate::uuid_utils::to_string(&media_session.group_id).into(),
            media_session.started.into(),
            media_session.finished_status.into(),
            media_session.added_datetime.into(),
            media_session.updated_datetime.into(),
        ]);

    insert
}

pub fn delete_by_id(
    user_id: &Uuid,
    media_id: &Uuid,
    start_datetime: DateTime<FixedOffset>,
) -> impl QueryStatementWriter {
    let mut delete = Query::delete();

    delete
        .from_table(MediaSessionIden::Table)
        .and_where(Expr::col(MediaSessionIden::UserId).eq(crate::uuid_utils::to_string(user_id)))
        .and_where(Expr::col(MediaSessionIden::MediaId).eq(crate::uuid_utils::to_string(media_id)))
        .and_where(Expr::col(MediaSessionIden::StartDate).eq(start_datetime));

    delete
}

pub fn exists_by_id(
    user_id: &Uuid,
    media_id: &Uuid,
    start_datetime: DateTime<FixedOffset>,
) -> impl QueryStatementWriter {
    let mut select = Query::select();

    from_and_where_user_id_and_media_id(&mut select, user_id, media_id);
    select
        .column((MediaSessionIden::Table, MediaSessionIden::MediaId))
        .and_where(Expr::col(MediaSessionIden::StartDate).eq(start_datetime));

    select
}

pub fn exists_by_start_datetime_lt_or_end_datetime_gt(
    user_id: &Uuid,
    end_datetime: DateTime<FixedOffset>,
    start_datetime: DateTime<FixedOffset>,
) -> impl QueryStatementWriter {
    let mut select = Query::select();

    from_and_where_user_id(&mut select, user_id);
    select
        .column((MediaSessionIden::Table, MediaSessionIden::MediaId))
        .and_where(Expr::col(MediaSessionIden::StartDate).lt(end_datetime))
        .and_where(Expr::col(MediaSessionIden::EndDate).gt(start_datetime));

    select
}

fn join_media_session(select: &mut SelectStatement) {
    select.left_join(
        MediaSessionIden::Table,
        Expr::col((MediaIden::Table, MediaIden::Id))
            .equals((MediaSessionIden::Table, MediaSessionIden::MediaId)),
    );
}

fn from_and_where_user_id_and_media_id(
    select: &mut SelectStatement,
    user_id: &Uuid,
    media_id: &Uuid,
) {
    from_and_where_user_id(select, user_id);
    select.and_where(
        Expr::col((MediaSessionIden::Table, MediaSessionIden::MediaId))
            .eq(crate::uuid_utils::to_string(media_id)),
    );
}

fn from_and_where_user_id(select: &mut SelectStatement, user_id: &Uuid) {
    select.from(MediaSessionIden::Table).and_where(
        Expr::col((MediaSessionIden::Table, MediaSessionIden::UserId))
            .eq(crate::uuid_utils::to_string(user_id)),
    );
}

fn where_optional_start_datetime_gte_and_start_datetime_lte(
    select: &mut SelectStatement,
    start_datetime: Option<DateTime<FixedOffset>>,
    end_datetime: Option<DateTime<FixedOffset>>,
) {
    if let Some(start) = start_datetime {
        select.and_where(
            Expr::col((MediaSessionIden::Table, MediaSessionIden::StartDate)).gte(start),
        );
    }

    if let Some(end) = end_datetime {
        select.and_where(Expr::col((MediaSessionIden::Table, MediaSessionIden::StartDate)).lt(end));
    }
}

fn add_fields(select: &mut SelectStatement) {
    select
        .column((MediaSessionIden::Table, MediaSessionIden::UserId))
        .column((MediaSessionIden::Table, MediaSessionIden::MediaId))
        .column((MediaSessionIden::Table, MediaSessionIden::StartDate))
        .column((MediaSessionIden::Table, MediaSessionIden::EndDate))
        .column((MediaSessionIden::Table, MediaSessionIden::DeviceId))
        .column((MediaSessionIden::Table, MediaSessionIden::GroupId))
        .column((MediaSessionIden::Table, MediaSessionIden::Started))
        .column((MediaSessionIden::Table, MediaSessionIden::FinishedStatus))
        .column((MediaSessionIden::Table, MediaSessionIden::AddedDatetime))
        .column((MediaSessionIden::Table, MediaSessionIden::UpdatedDatetime))
        .expr_as(derived_time_expr(), Alias::new(QUERY_TIME_ALIAS));
}

fn derived_time_expr() -> SimpleExpr {
    Expr::col((MediaSessionIden::Table, MediaSessionIden::EndDate)).sub(Expr::col((
        MediaSessionIden::Table,
        MediaSessionIden::StartDate,
    )))
}

/// Streaks
const STREAK_START_DATE_SUB_ALIAS: &str = "start_date";
const STREAK_END_DATE_SUB_ALIAS: &str = "end_date";
const STREAK_MEDIA_ID_SUB_ALIAS: &str = "media_id";
const STREAK_DEVICE_ID_SUB_ALIAS: &str = "device_id";
const STREAK_STARTS_STREAK_SUB_ALIAS: &str = "starts_streak";
const STREAK_GROUP_SUB_ALIAS: &str = "grp";

pub fn select_streaks_with_search(
    user_id: &Uuid,
    search: SessionListSearch,
) -> Result<SearchQuery, SearchErrors> {
    let mut select = Query::select();

    let min_start_date = Expr::col(STREAK_START_DATE_SUB_ALIAS).min();
    let max_end_date = Expr::col(STREAK_END_DATE_SUB_ALIAS).max();
    let streak_days = max_end_date
        .clone()
        .sub(min_start_date.clone())
        .add(Expr::val(1));
    select
        .expr_as(min_start_date.clone(), STREAK_START_DATE_ALIAS)
        .expr_as(max_end_date.clone(), STREAK_END_DATE_ALIAS)
        .expr_as(
            array_agg_distinct(Expr::col(STREAK_MEDIA_ID_SUB_ALIAS).into()),
            STREAK_MEDIA_IDS_ALIAS,
        )
        .expr_as(
            array_agg_distinct(Expr::col(STREAK_DEVICE_ID_SUB_ALIAS).into()),
            STREAK_DEVICE_IDS_ALIAS,
        )
        .expr_as(streak_days.clone(), STREAK_DAYS_ALIAS);
    select.from_subquery(streaks_group(user_id, search.filter)?, "streaks_group_sub");
    select.add_group_by([Expr::col(STREAK_GROUP_SUB_ALIAS).into()]);
    select.order_by_expr(streak_days.clone(), Order::Desc);

    Ok(apply_search_pagination(select, search.page, search.size))
}

fn streaks_group(
    user_id: &Uuid,
    filter: Option<Vec<Filter<MediaSessionIden>>>,
) -> Result<SelectStatement, SearchErrors> {
    let mut select = Query::select();

    select
        .column(STREAK_START_DATE_SUB_ALIAS)
        .column(STREAK_END_DATE_SUB_ALIAS)
        .column(STREAK_MEDIA_ID_SUB_ALIAS)
        .column(STREAK_DEVICE_ID_SUB_ALIAS)
        .expr_as(
            filter_over_order_by(
                Expr::col(sea_query::Asterisk).count(),
                Expr::col(STREAK_STARTS_STREAK_SUB_ALIAS).into(),
                Expr::col(STREAK_START_DATE_SUB_ALIAS).into(),
            ),
            STREAK_GROUP_SUB_ALIAS,
        );
    select.from_subquery(starts_streak(user_id, filter)?, "starts_streak_sub");
    select.order_by(STREAK_START_DATE_SUB_ALIAS, Order::Asc);

    Ok(select)
}

fn starts_streak(
    user_id: &Uuid,
    filter: Option<Vec<Filter<MediaSessionIden>>>,
) -> Result<SelectStatement, SearchErrors> {
    let mut select = Query::select();

    let start_date_as_date = at_time_zone(
        Expr::col((MediaSessionIden::Table, MediaSessionIden::StartDate)).into(),
        Expr::col((MediaSessionIden::Table, MediaSessionIden::StartDateTz)).into(),
    )
    .cast_as("DATE");
    let end_date_as_date = at_time_zone(
        Expr::col((MediaSessionIden::Table, MediaSessionIden::EndDate)).into(),
        Expr::col((MediaSessionIden::Table, MediaSessionIden::EndDateTz)).into(),
    )
    .cast_as("DATE");
    select
        .expr_as(start_date_as_date.clone(), STREAK_START_DATE_SUB_ALIAS)
        .expr_as(end_date_as_date.clone(), STREAK_END_DATE_SUB_ALIAS)
        .expr_as(
            Expr::col((MediaSessionIden::Table, MediaSessionIden::MediaId)),
            STREAK_MEDIA_ID_SUB_ALIAS,
        )
        .expr_as(
            Expr::col((MediaSessionIden::Table, MediaSessionIden::DeviceId)),
            STREAK_DEVICE_ID_SUB_ALIAS,
        )
        .expr_as(
            Func::coalesce([
                start_date_as_date.clone().sub(over_order_by(
                    Func::cust(Lag).arg(end_date_as_date.clone()).into(),
                    Expr::col((MediaSessionIden::Table, MediaSessionIden::StartDate)).into(),
                )),
                // 99 > 1 for first lag so it always starts streak
                Expr::val(99).into(),
            ])
            .gt(Expr::val(1)),
            STREAK_STARTS_STREAK_SUB_ALIAS,
        );
    from_and_where_user_id(&mut select, user_id);
    select.order_by(
        (MediaSessionIden::Table, MediaSessionIden::StartDate),
        Order::Asc,
    );

    apply_search_filter(select, filter)
}

fn over_order_by(from: SimpleExpr, order_by: SimpleExpr) -> SimpleExpr {
    Expr::cust_with_exprs("$1 OVER (ORDER BY $2)", vec![from, order_by])
}

fn filter_over_order_by(from: SimpleExpr, filter: SimpleExpr, order_by: SimpleExpr) -> SimpleExpr {
    over_order_by(
        Expr::cust_with_exprs("$1 FILTER (WHERE $2)", vec![from, filter]),
        order_by,
    )
}

fn at_time_zone(from: SimpleExpr, zone: SimpleExpr) -> SimpleExpr {
    Expr::cust_with_exprs("$1 AT TIME ZONE $2", vec![from, zone])
}

fn array_agg_distinct(from: SimpleExpr) -> SimpleExpr {
    Expr::cust_with_exprs("ARRAY_REMOVE(ARRAY_AGG(DISTINCT $1), NULL)", vec![from])
}

//
struct Lag;

impl sea_query::Iden for Lag {
    fn unquoted(&self, s: &mut dyn std::fmt::Write) {
        #[allow(clippy::unwrap_used)]
        write!(s, "LAG").unwrap(); // Safe unwrap: just a function name
    }
}
