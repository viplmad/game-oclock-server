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

pub struct AggregateGroupResult {
    pub key: AggregateGroupResultKey,
    pub value: AggregateGroupResultValue,
}

pub enum AggregateGroupResultKey {
    Integer(i64),
    String(String),
}

pub enum AggregateGroupResultValue {
    Simple(AggregateResult),
    Sub(Vec<AggregateSubgroupResult>),
}

pub struct AggregateSubgroupResult {
    pub key: AggregateGroupResultKey,
    pub value: AggregateResult,
}
