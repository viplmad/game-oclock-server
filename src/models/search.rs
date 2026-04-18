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
    pub aggr: AggregateMetric,
}

#[derive(Deserialize, ToSchema)]
pub struct AggregateGroupSearchDTO {
    pub filter: Option<Vec<FilterDTO>>,
    pub aggr: AggregateMetric,
    pub group: AggregateGroup,
}

/// Aggregate metric
#[derive(Deserialize, ToSchema)]
#[serde(tag = "kind")]
pub enum AggregateMetric {
    Count(AggregateCountMetricDTO),
    Sum(AggregateSumMetricDTO),
}

#[derive(Deserialize, ToSchema)]
pub struct AggregateSumMetricDTO {
    pub field: String,
    pub default_value: Option<String>,
}

#[derive(Deserialize, ToSchema)]
pub struct AggregateCountMetricDTO {
    pub field: String,
    pub default_value: Option<String>,
    pub distinct: Option<bool>,
}

/// Aggregate group
#[derive(Deserialize, ToSchema)]
#[serde(tag = "kind")]
pub enum AggregateGroup {
    Field(AggregateFieldGroupDTO),
    DateHistogram(AggregateDateHistogramGroupDTO),
}

#[derive(Deserialize, ToSchema)]
pub struct AggregateFieldGroupDTO {
    pub field: String,
    pub default_value: Option<String>,
}

#[derive(Deserialize, ToSchema)]
pub struct AggregateDateHistogramGroupDTO {
    pub field: String,
    pub default_value: Option<String>,
    pub interval: DateHistogramInterval,
}

#[derive(Clone, Deserialize, ToSchema)]
pub enum DateHistogramInterval {
    Year,
    Month,
    Week,
    Day,
    Hour,
    Minute,
}

/// Filter
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
