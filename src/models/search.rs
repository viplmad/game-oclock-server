use serde::Deserialize;
use utoipa::{IntoParams, ToSchema};

#[derive(Deserialize, IntoParams)]
pub struct QuicksearchQuery {
    pub q: Option<String>,
}

#[derive(Deserialize, IntoParams)]
pub struct ExternalQuicksearchQuery {
    pub source: String,
    pub q: String,
}

#[derive(Deserialize, ToSchema)]
pub struct ListSearchDTO {
    pub filter: Option<Vec<FilterDTO>>,
    pub sort: Option<Vec<SortDTO>>,
    pub page: Option<u64>,
    pub size: Option<u64>,
}

#[derive(Deserialize, ToSchema)]
pub struct AggregateSearchDTO {
    pub filter: Option<Vec<FilterDTO>>,
    pub aggr: AggregateMetricDTO,
}

#[derive(Deserialize, ToSchema)]
pub struct AggregateGroupSearchDTO {
    pub filter: Option<Vec<FilterDTO>>,
    pub sort: Option<AggregateGroupSortDTO>,
    pub aggr: AggregateMetricDTO,
    pub group: AggregateGroupDTO,
    pub size: Option<u64>,
}

/// Aggregate metric
#[derive(Clone, Deserialize, ToSchema)]
pub enum AggregateMetricType {
    Count,
    Sum,
}

#[derive(Deserialize, ToSchema)]
pub struct AggregateMetricDTO {
    pub kind: AggregateMetricType,
    pub field: String,
    pub default_value: Option<String>,
    // Count
    pub distinct: Option<bool>,
}

/// Aggregate group
#[derive(Clone, Deserialize, ToSchema)]
pub enum AggregateGroupType {
    Field,
    DateHistogram,
}

#[derive(Deserialize, ToSchema)]
pub struct AggregateGroupDTO {
    pub kind: AggregateGroupType,
    pub field: String,
    pub default_value: Option<String>,
    // DateHistogram
    pub interval: Option<DateHistogramInterval>,
}

#[derive(Clone, Deserialize, ToSchema)]
pub enum DateHistogramInterval {
    Year,
    Month,
    Weekday,
    Day,
    Hour,
    Minute,
}
//
#[derive(Deserialize, ToSchema)]
pub struct AggregateGroupSortDTO {
    pub field: AggregateGroupSortType,
    pub order: OrderType,
}

#[derive(Clone, Deserialize, ToSchema)]
pub enum AggregateGroupSortType {
    Group,
    Metric,
}

/// Filter
#[derive(Debug, Clone, Deserialize, ToSchema)]
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
    Null,
    NotNull,
}

#[derive(Deserialize, ToSchema)]
pub struct FilterDTO {
    pub operator: OperatorType,
    pub field: String,
    pub chain_operator: Option<ChainOperatorType>,
    // Single and Multiple
    pub value: Option<SearchValue>,
}

#[derive(Clone, Deserialize, ToSchema)]
#[serde(untagged)]
pub enum SearchValue {
    Single(String),
    Multiple(Vec<String>),
}

#[derive(Clone, Deserialize, ToSchema)]
pub enum ChainOperatorType {
    And,
    Or,
}

/// Sort
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
