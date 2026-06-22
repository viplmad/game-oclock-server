use sea_query::{PostgresQueryBuilder, QueryStatementWriter};
use sqlx::{AssertSqlSafe, Postgres, postgres::types::PgInterval};
use uuid::Uuid;

use crate::entities::{
    AggregateGroupQuery, AggregateGroupResult, AggregateGroupResultKey, AggregateGroupResultValue,
    AggregateGroupType, AggregateQuery, AggregateResult, AggregateSubgroupResult, AggregateType,
    FieldType, PageResult, SearchQuery,
};
use crate::errors::{RepositoryError, SearchErrors};

pub(super) async fn fetch_one<'c, X, T>(
    executor: X,
    query: impl QueryStatementWriter,
) -> Result<T, RepositoryError>
where
    X: sqlx::Executor<'c, Database = Postgres>,
    T: for<'r> sqlx::FromRow<'r, sqlx::postgres::PgRow> + Send + Unpin,
{
    let sql = build_sql(query);
    sqlx::query_as::<_, T>(sql)
        .fetch_one(executor)
        .await
        .map_err(|err| {
            log::error!("Error executing query. - {}", err);
            RepositoryError()
        })
}

pub(super) async fn execute<'c, X>(
    executor: X,
    query: impl QueryStatementWriter,
) -> Result<(), RepositoryError>
where
    X: sqlx::Executor<'c, Database = Postgres>,
{
    let sql = build_sql(query);
    sqlx::query(sql)
        .execute(executor)
        .await
        .map(|_| ())
        .map_err(|err| {
            log::error!("Error executing query. - {}", err);
            RepositoryError()
        })
}

pub(super) async fn fetch_optional<'c, X, T>(
    executor: X,
    query: impl QueryStatementWriter,
) -> Result<Option<T>, RepositoryError>
where
    X: sqlx::Executor<'c, Database = Postgres>,
    T: for<'r> sqlx::FromRow<'r, sqlx::postgres::PgRow> + Send + Unpin,
{
    let sql = build_sql(query);
    sqlx::query_as::<_, T>(sql)
        .fetch_optional(executor)
        .await
        .map_err(|err| {
            log::error!("Error executing query. - {}", err);
            RepositoryError()
        })
}

pub(super) async fn fetch_optional_single<'c, X, T>(
    executor: X,
    query: impl QueryStatementWriter,
) -> Result<Option<T>, RepositoryError>
where
    X: sqlx::Executor<'c, Database = Postgres>,
    T: for<'r> sqlx::Decode<'r, Postgres> + sqlx::Type<Postgres> + Send + Unpin,
{
    fetch_optional(executor, query)
        .await
        .map(|optional_tuple: Option<(T,)>| optional_tuple.map(|tuple| tuple.0))
}

pub(super) async fn fetch_all<'c, X, T>(
    executor: X,
    query: impl QueryStatementWriter,
) -> Result<Vec<T>, RepositoryError>
where
    X: sqlx::Executor<'c, Database = Postgres>,
    T: for<'r> sqlx::FromRow<'r, sqlx::postgres::PgRow> + Send + Unpin,
{
    let sql = build_sql(query);
    sqlx::query_as::<_, T>(sql)
        .fetch_all(executor)
        .await
        .map_err(|err| {
            log::error!("Error executing query. - {}", err);
            RepositoryError()
        })
}

pub(super) async fn fetch_all_single<'c, X, T>(
    executor: X,
    query: impl QueryStatementWriter,
) -> Result<Vec<T>, RepositoryError>
where
    X: sqlx::Executor<'c, Database = Postgres>,
    T: for<'r> sqlx::Decode<'r, Postgres> + sqlx::Type<Postgres> + Send + Unpin,
{
    fetch_all(executor, query)
        .await
        .map(|list: Vec<(T,)>| list.into_iter().map(|tuple| tuple.0).collect())
}

pub(super) async fn fetch_all_search<'c, X, T>(
    executor: X,
    query: SearchQuery,
) -> Result<PageResult<T>, SearchErrors>
where
    X: sqlx::Executor<'c, Database = Postgres>,
    T: for<'r> sqlx::FromRow<'r, sqlx::postgres::PgRow> + Send + Unpin,
{
    fetch_all(executor, query.query)
        .await
        .map(|list| PageResult {
            data: list,
            page: query.page,
            size: query.size,
        })
        .map_err(SearchErrors::Repository)
}

pub(super) async fn aggregate_all_search<'c, X>(
    executor: X,
    query: AggregateQuery,
) -> Result<AggregateResult, SearchErrors>
where
    X: sqlx::Executor<'c, Database = Postgres>,
{
    if query.kind == AggregateType::Sum && query.field_kind == FieldType::Interval {
        return fetch_one(executor, query.query)
            .await
            .map(|tuple: (PgInterval,)| AggregateResult::Duration(tuple.0))
            .map_err(SearchErrors::Repository);
    }

    return fetch_one(executor, query.query)
        .await
        .map(|tuple: (i64,)| AggregateResult::Integer(tuple.0))
        .map_err(SearchErrors::Repository);
}

pub(super) async fn aggregate_group_search<'c, X>(
    executor: X,
    query: AggregateGroupQuery,
) -> Result<Vec<AggregateGroupResult>, SearchErrors>
where
    X: sqlx::Executor<'c, Database = Postgres>,
{
    // Integer -> String -> Duration => (Year/Month) -> Id -> Time
    if query.subgroup_kind.is_some() && query.subgroup_field_kind.is_some() {
        return fetch_all(executor, query.query)
            .await
            .map(|list: Vec<(i64, String, PgInterval)>| {
                let mut m: Vec<(i64, Vec<(String, PgInterval)>)> = vec![];
                for tuple in list {
                    let existing = m.iter().position(|x| x.0 == tuple.0);
                    if let Some(ex) = existing {
                        // Safe unwrap, index from an existing element
                        #[allow(clippy::unwrap_used)]
                        m.get_mut(ex).unwrap().1.push((tuple.1, tuple.2));
                    } else {
                        m.push((tuple.0, vec![(tuple.1, tuple.2)]));
                    }
                }

                m.into_iter()
                    .map(|e| AggregateGroupResult {
                        key: AggregateGroupResultKey::Integer(e.0),
                        value: AggregateGroupResultValue::Sub(
                            e.1.into_iter()
                                .map(|se| AggregateSubgroupResult {
                                    key: AggregateGroupResultKey::String(se.0),
                                    value: AggregateResult::Duration(se.1),
                                })
                                .collect(),
                        ),
                    })
                    .collect()
            })
            .map_err(SearchErrors::Repository);
    }

    if query.group_kind == AggregateGroupType::Field
        && (query.group_field_kind == FieldType::String
            || query.group_field_kind == FieldType::Array)
    {
        // String -> Duration => Id -> Time
        if query.kind == AggregateType::Sum && query.field_kind == FieldType::Interval {
            return fetch_all(executor, query.query)
                .await
                .map(|list: Vec<(String, PgInterval)>| {
                    list.into_iter()
                        .map(|tuple| AggregateGroupResult {
                            key: AggregateGroupResultKey::String(tuple.0),
                            value: AggregateGroupResultValue::Simple(AggregateResult::Duration(
                                tuple.1,
                            )),
                        })
                        .collect()
                })
                .map_err(SearchErrors::Repository);
        }

        // String -> Integer => Genre -> Total
        return fetch_all(executor, query.query)
            .await
            .map(|list: Vec<(String, i64)>| {
                list.into_iter()
                    .map(|tuple| AggregateGroupResult {
                        key: AggregateGroupResultKey::String(tuple.0),
                        value: AggregateGroupResultValue::Simple(AggregateResult::Integer(tuple.1)),
                    })
                    .collect()
            })
            .map_err(SearchErrors::Repository);
    }

    // Integer -> Duration => (Year/Month) -> Time
    if query.kind == AggregateType::Sum && query.field_kind == FieldType::Interval {
        return fetch_all(executor, query.query)
            .await
            .map(|list: Vec<(i64, PgInterval)>| {
                list.into_iter()
                    .map(|tuple| AggregateGroupResult {
                        key: AggregateGroupResultKey::Integer(tuple.0),
                        value: AggregateGroupResultValue::Simple(AggregateResult::Duration(
                            tuple.1,
                        )),
                    })
                    .collect()
            })
            .map_err(SearchErrors::Repository);
    }

    // Integer -> Integer => (Year/Month) -> Total
    return fetch_all(executor, query.query)
        .await
        .map(|list: Vec<(i64, i64)>| {
            list.into_iter()
                .map(|tuple| AggregateGroupResult {
                    key: AggregateGroupResultKey::Integer(tuple.0),
                    value: AggregateGroupResultValue::Simple(AggregateResult::Integer(tuple.1)),
                })
                .collect()
        })
        .map_err(SearchErrors::Repository);
}

pub(super) async fn exists_some<'c, X>(
    executor: X,
    query: impl QueryStatementWriter,
) -> Result<bool, RepositoryError>
where
    X: sqlx::Executor<'c, Database = Postgres>,
{
    fetch_all(executor, query)
        .await
        .map(|res: Vec<(Uuid,)>| !res.is_empty())
}

fn build_sql(query: impl QueryStatementWriter) -> AssertSqlSafe<String> {
    // Only Postgres allowed
    let sql = query.to_string(PostgresQueryBuilder);
    log::info!("{}", sql);
    AssertSqlSafe(sql)
}
