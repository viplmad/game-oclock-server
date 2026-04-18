use sqlx::postgres::types::PgInterval;

pub struct PageResult<E> {
    pub data: Vec<E>,
    pub page: u64,
    pub size: u64,
}

pub enum AggregateResult {
    Integer(i64),
    Duration(PgInterval),
}

#[derive(Eq, PartialEq, Hash)]
pub enum AggregateGroupResultKey {
    Integer(i64),
}
