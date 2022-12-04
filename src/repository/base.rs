use sea_query::{PostgresQueryBuilder, QueryStatementWriter};
use sqlx::{PgPool, Postgres, Transaction};

use crate::entities::{SearchQuery, SearchResult};
use crate::errors::RepositoryError;

pub(super) async fn begin_transaction(
    pool: &PgPool,
) -> Result<Transaction<Postgres>, RepositoryError> {
    pool.begin()
        .await
        .map_err(|err| RepositoryError(err.to_string()))
}

pub(super) async fn commit_transaction(
    transaction: Transaction<'_, Postgres>,
) -> Result<(), RepositoryError> {
    transaction
        .commit()
        .await
        .map_err(|err| RepositoryError(err.to_string()))
}

pub(super) async fn execute_return<'c, X, T>(
    executor: X,
    query: impl QueryStatementWriter,
) -> Result<T, RepositoryError>
where
    X: sqlx::Executor<'c, Database = Postgres>,
    T: for<'r> sqlx::FromRow<'r, sqlx::postgres::PgRow> + Send + Unpin,
{
    let sql = build_sql(query);
    log::info!("{}", sql);
    sqlx::query_as::<_, T>(&sql)
        .fetch_one(executor)
        .await
        .map_err(|err| RepositoryError(err.to_string()))
}

pub(super) async fn execute_return_single<'c, X, T>(
    executor: X,
    query: impl QueryStatementWriter,
) -> Result<T, RepositoryError>
where
    X: sqlx::Executor<'c, Database = Postgres>,
    T: for<'r> sqlx::Decode<'r, Postgres> + sqlx::Type<Postgres> + Send + Unpin,
{
    execute_return(executor, query)
        .await
        .map(|tuple: (T,)| tuple.0)
}

pub(super) async fn execute_return_id<'c, X>(
    executor: X,
    query: impl QueryStatementWriter,
) -> Result<i32, RepositoryError>
where
    X: sqlx::Executor<'c, Database = Postgres>,
{
    execute_return_single(executor, query).await
}

pub(super) async fn execute<'c, X>(
    executor: X,
    query: impl QueryStatementWriter,
) -> Result<(), RepositoryError>
where
    X: sqlx::Executor<'c, Database = Postgres>,
{
    let sql = build_sql(query);
    log::info!("{}", sql);
    sqlx::query(&sql)
        .execute(executor)
        .await
        .map(|_| ())
        .map_err(|err| RepositoryError(err.to_string()))
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
    log::info!("{}", sql);
    sqlx::query_as::<_, T>(&sql)
        .fetch_optional(executor)
        .await
        .map_err(|err| RepositoryError(err.to_string()))
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
    log::info!("{}", sql);
    sqlx::query_as::<_, T>(&sql)
        .fetch_all(executor)
        .await
        .map_err(|err| RepositoryError(err.to_string()))
}

pub(super) async fn fetch_all_search<'c, X, T>(
    executor: X,
    search_query: SearchQuery,
) -> Result<SearchResult<T>, RepositoryError>
where
    X: sqlx::Executor<'c, Database = Postgres>,
    T: for<'r> sqlx::FromRow<'r, sqlx::postgres::PgRow> + Send + Unpin,
{
    fetch_all(executor, search_query.query)
        .await
        .map(|list| SearchResult {
            data: list,
            page: search_query.page,
            size: search_query.size,
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

pub(super) async fn exists_id<'c, X>(
    executor: X,
    query: impl QueryStatementWriter,
) -> Result<bool, RepositoryError>
where
    X: sqlx::Executor<'c, Database = Postgres>,
{
    fetch_all(executor, query)
        .await
        .map(|res: Vec<(i32,)>| !res.is_empty())
}

fn build_sql(query: impl QueryStatementWriter) -> String {
    query.to_string(PostgresQueryBuilder)
}
