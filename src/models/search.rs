use serde::Deserialize;
use utoipa::{IntoParams, ToSchema};

#[derive(Deserialize, IntoParams)]
pub struct QuicksearchQuery {
    pub q: Option<String>,
}

#[derive(Deserialize, ToSchema)]
pub struct SearchDTO {
    pub filter: Option<Vec<FilterDTO>>,
    pub sort: Option<Vec<SortDTO>>,
    pub page: Option<u64>,
    pub size: Option<u64>,
}

#[derive(Deserialize, ToSchema)]
pub struct FilterDTO {
    pub field: String,
    pub value: SearchValue,
    pub operator: OperatorType,
    pub chain_operator: Option<ChainOperatorType>,
}

#[derive(Clone, Deserialize, ToSchema)]
#[serde(untagged)]
pub enum SearchValue {
    Value(String),
    Values(Vec<String>),
}

#[derive(Clone, Deserialize, ToSchema)]
pub enum OperatorType {
    Eq,
    NotEq,
    Gt,
    Gte,
    Lt,
    Lte,
    In,
    NotIn,
    StartsWith,
    NotStartsWith,
    EndsWith,
    NotEndsWith,
    Contains,
    NotContains,
}

#[derive(Clone, Deserialize, ToSchema)]
pub enum ChainOperatorType {
    And,
    Or,
}

#[derive(Deserialize, ToSchema)]
pub struct SortDTO {
    pub field: String,
    pub order: OrderType,
}

#[derive(Clone, Deserialize, ToSchema)]
pub enum OrderType {
    Asc,
    Desc,
}
