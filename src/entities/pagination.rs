use crate::models::DurationDef;

pub struct PageResult<E> {
    pub data: Vec<E>,
    pub page: u64,
    pub size: u64,
}

pub enum AggregateResult {
    Integer(i64),
    Duration(DurationDef),
}
