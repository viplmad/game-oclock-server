use serde::Deserialize;
use utoipa::ToSchema;

#[derive(Deserialize, ToSchema)]
pub struct QueryDTO {
    pub filter: Option<Vec<SearchFilterDTO>>,
    pub sort: Option<Vec<SortDTO>>,
    pub page: Option<u64>,
    pub size: Option<u64>,
    pub limit: Option<u64>,
}

/*#[derive(Deserialize, ToSchema)]
#[serde(tag = "type")]
pub enum FilterDTO {}*/

#[derive(Deserialize, ToSchema)]
pub struct SearchFilterDTO {
    pub field: String,
    pub value: SearchValue,
    pub operator: OperatorType,
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
