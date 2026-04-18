use std::str::FromStr;

use chrono::{DateTime, NaiveDate};
use sea_query::{BinOper, Order, Value};

use crate::entities::{
    AggregateCountMetric, AggregateDateHistogramGroup, AggregateFieldGroup,
    AggregateGroupResultKey, AggregateGroupSearch, AggregateResult, AggregateSearch,
    AggregateSumMetric, FieldIden, FieldSearchValue, FieldSearchValues, FieldType, FieldValue,
    Filter, FilterOperator, GroupDateHistogramInterval, ListSearch, Sort, TableIden,
};
use crate::errors::{MappingError, error_message_builder};
use crate::models::{
    AggregateCountMetricDTO, AggregateDateHistogramGroupDTO, AggregateFieldGroupDTO,
    AggregateGroup, AggregateGroupResultKeyDTO, AggregateGroupSearchDTO, AggregateMetric,
    AggregateResultDTO, AggregateSearchDTO, AggregateSumMetricDTO, ChainOperatorType,
    DateHistogramInterval, DurationDef, FilterDTO, ListSearchDTO, MediaStatus, OperatorType,
    OrderType, SearchValue, SortDTO,
};

impl From<DateHistogramInterval> for GroupDateHistogramInterval {
    fn from(operator: DateHistogramInterval) -> Self {
        match operator {
            DateHistogramInterval::Year => GroupDateHistogramInterval::Year,
            DateHistogramInterval::Month => GroupDateHistogramInterval::Month,
            DateHistogramInterval::Weekday => GroupDateHistogramInterval::Weekday,
            DateHistogramInterval::Day => GroupDateHistogramInterval::Day,
            DateHistogramInterval::Hour => GroupDateHistogramInterval::Hour,
            DateHistogramInterval::Minute => GroupDateHistogramInterval::Minute,
        }
    }
}

impl From<OperatorType> for FilterOperator {
    fn from(operator: OperatorType) -> Self {
        match operator {
            OperatorType::Eq => FilterOperator::Equal,
            OperatorType::NotEq => FilterOperator::NotEqual,
            OperatorType::Gt => FilterOperator::GreaterThan,
            OperatorType::Gte => FilterOperator::GreaterThanOrEqual,
            OperatorType::Lt => FilterOperator::SmallerThan,
            OperatorType::Lte => FilterOperator::SmallerThanOrEqual,
            OperatorType::In => FilterOperator::In,
            OperatorType::NotIn => FilterOperator::NotIn,
            OperatorType::StartsWith => FilterOperator::StartsWith,
            OperatorType::NotStartsWith => FilterOperator::NotStartsWith,
            OperatorType::EndsWith => FilterOperator::EndsWith,
            OperatorType::NotEndsWith => FilterOperator::NotEndsWith,
            OperatorType::Contains => FilterOperator::Contains,
            OperatorType::NotContains => FilterOperator::NotContains,
        }
    }
}

impl From<OrderType> for Order {
    fn from(order: OrderType) -> Self {
        match order {
            OrderType::Asc => Order::Asc,
            OrderType::Desc => Order::Desc,
        }
    }
}

impl From<ChainOperatorType> for BinOper {
    fn from(operator: ChainOperatorType) -> Self {
        match operator {
            ChainOperatorType::And => BinOper::And,
            ChainOperatorType::Or => BinOper::Or,
        }
    }
}

impl<I: TableIden> TryFrom<ListSearchDTO> for ListSearch<I>
where
    FieldIden<I>: FromStr,
{
    type Error = MappingError;

    fn try_from(search: ListSearchDTO) -> Result<Self, Self::Error> {
        let filter_result = search.filter.map(|filters| {
            filters
                .into_iter()
                .map(Filter::try_from)
                .collect::<Result<Vec<Filter<I>>, MappingError>>()
        });
        let filter = match filter_result {
            Some(res) => Some(res?),
            None => None,
        };

        let sort_result = search.sort.map(|sorts| {
            sorts
                .into_iter()
                .map(Sort::try_from)
                .collect::<Result<Vec<Sort<I>>, MappingError>>()
        });
        let sort = match sort_result {
            Some(res) => Some(res?),
            None => None,
        };

        Ok(Self {
            filter,
            sort,
            page: search.page,
            size: search.size,
        })
    }
}

impl<I: TableIden> TryFrom<AggregateSearchDTO> for AggregateSearch<I>
where
    FieldIden<I>: FromStr,
{
    type Error = MappingError;

    fn try_from(search: AggregateSearchDTO) -> Result<Self, Self::Error> {
        let filter_result = search.filter.map(|filters| {
            filters
                .into_iter()
                .map(Filter::try_from)
                .collect::<Result<Vec<Filter<I>>, MappingError>>()
        });
        let filter = match filter_result {
            Some(res) => Some(res?),
            None => None,
        };

        let aggr = match search.aggr {
            AggregateMetric::Count(c) => {
                crate::entities::AggregateMetric::Count(AggregateCountMetric::try_from(c)?)
            }
            AggregateMetric::Sum(s) => {
                crate::entities::AggregateMetric::Sum(AggregateSumMetric::try_from(s)?)
            }
        };

        Ok(Self { filter, aggr })
    }
}
impl<I: TableIden> TryFrom<AggregateGroupSearchDTO> for AggregateGroupSearch<I>
where
    FieldIden<I>: FromStr,
{
    type Error = MappingError;

    fn try_from(search: AggregateGroupSearchDTO) -> Result<Self, Self::Error> {
        let filter_result = search.filter.map(|filters| {
            filters
                .into_iter()
                .map(Filter::try_from)
                .collect::<Result<Vec<Filter<I>>, MappingError>>()
        });
        let filter = match filter_result {
            Some(res) => Some(res?),
            None => None,
        };

        let aggr = match search.aggr {
            AggregateMetric::Count(c) => {
                crate::entities::AggregateMetric::Count(AggregateCountMetric::try_from(c)?)
            }
            AggregateMetric::Sum(s) => {
                crate::entities::AggregateMetric::Sum(AggregateSumMetric::try_from(s)?)
            }
        };

        let group = match search.group {
            AggregateGroup::Field(f) => {
                crate::entities::AggregateGroup::Field(AggregateFieldGroup::try_from(f)?)
            }
            AggregateGroup::DateHistogram(d) => crate::entities::AggregateGroup::DateHistogram(
                AggregateDateHistogramGroup::try_from(d)?,
            ),
        };

        Ok(Self {
            filter,
            aggr,
            group,
        })
    }
}

impl From<AggregateResult> for AggregateResultDTO {
    fn from(res: AggregateResult) -> Self {
        match res {
            AggregateResult::Integer(i) => AggregateResultDTO::Integer(i),
            AggregateResult::Duration(d) => AggregateResultDTO::Duration(DurationDef::from(d)),
        }
    }
}

impl From<AggregateGroupResultKey> for AggregateGroupResultKeyDTO {
    fn from(res: AggregateGroupResultKey) -> Self {
        match res {
            AggregateGroupResultKey::Integer(i) => AggregateGroupResultKeyDTO::Integer(i),
        }
    }
}

impl<I: TableIden> TryFrom<AggregateCountMetricDTO> for AggregateCountMetric<I>
where
    FieldIden<I>: FromStr,
{
    type Error = MappingError;

    fn try_from(aggr: AggregateCountMetricDTO) -> Result<Self, Self::Error> {
        let field = FieldIden::<I>::from_str(&aggr.field).map_err(|_| MappingError(aggr.field))?;
        let field_kind = field.kind();

        Ok(Self::new(
            field,
            aggr.default_value.map(|value| FieldSearchValue {
                kind: field_kind,
                value,
            }),
            aggr.distinct.unwrap_or(false),
        ))
    }
}

impl<I: TableIden> TryFrom<AggregateSumMetricDTO> for AggregateSumMetric<I>
where
    FieldIden<I>: FromStr,
{
    type Error = MappingError;

    fn try_from(aggr: AggregateSumMetricDTO) -> Result<Self, Self::Error> {
        let field = FieldIden::<I>::from_str(&aggr.field).map_err(|_| MappingError(aggr.field))?;
        let field_kind = field.kind();

        Ok(Self::new(
            field,
            aggr.default_value.map(|value| FieldSearchValue {
                kind: field_kind,
                value,
            }),
        ))
    }
}

impl<I: TableIden> TryFrom<AggregateFieldGroupDTO> for AggregateFieldGroup<I>
where
    FieldIden<I>: FromStr,
{
    type Error = MappingError;

    fn try_from(group: AggregateFieldGroupDTO) -> Result<Self, Self::Error> {
        let field =
            FieldIden::<I>::from_str(&group.field).map_err(|_| MappingError(group.field))?;
        let field_kind = field.kind();

        Ok(Self::new(
            field,
            group.default_value.map(|value| FieldSearchValue {
                kind: field_kind,
                value,
            }),
        ))
    }
}

impl<I: TableIden> TryFrom<AggregateDateHistogramGroupDTO> for AggregateDateHistogramGroup<I>
where
    FieldIden<I>: FromStr,
{
    type Error = MappingError;

    fn try_from(group: AggregateDateHistogramGroupDTO) -> Result<Self, Self::Error> {
        let field =
            FieldIden::<I>::from_str(&group.field).map_err(|_| MappingError(group.field))?;
        let field_kind = field.kind();

        Ok(Self::new(
            field,
            group.default_value.map(|value| FieldSearchValue {
                kind: field_kind,
                value,
            }),
            GroupDateHistogramInterval::from(group.interval),
        ))
    }
}

impl<I: TableIden> TryFrom<FilterDTO> for Filter<I>
where
    FieldIden<I>: FromStr,
{
    type Error = MappingError;

    fn try_from(filter: FilterDTO) -> Result<Self, Self::Error> {
        let field =
            FieldIden::<I>::from_str(&filter.field).map_err(|_| MappingError(filter.field))?;
        let field_kind = field.kind();

        Ok(Self::new(
            field,
            match filter.value {
                SearchValue::Value(value) => FieldValue::Value(FieldSearchValue {
                    kind: field_kind,
                    value,
                }),
                SearchValue::Values(values) => FieldValue::Values(FieldSearchValues {
                    kind: field_kind,
                    values,
                }),
            },
            FilterOperator::from(filter.operator),
            match filter.chain_operator {
                Some(chain_op) => BinOper::from(chain_op),
                None => BinOper::And,
            },
        ))
    }
}

impl<I: TableIden> TryFrom<SortDTO> for Sort<I>
where
    FieldIden<I>: FromStr,
{
    type Error = MappingError;

    fn try_from(sort: SortDTO) -> Result<Self, Self::Error> {
        let field = FieldIden::<I>::from_str(&sort.field).map_err(|_| MappingError(sort.field))?;

        Ok(Self::new(field, Order::from(sort.order)))
    }
}

impl TryFrom<FieldSearchValue> for Value {
    type Error = MappingError;

    fn try_from(search: FieldSearchValue) -> Result<Self, Self::Error> {
        let value: &str = &search.value;
        match search.kind {
            FieldType::Integer => {
                let int_value = convert_with_serde::<i32>(value, "integer")?;
                Ok(int_value.into())
            }
            FieldType::String => Ok(value.into()),
            FieldType::Boolean => {
                let bool_value = convert_with_serde::<bool>(value, "boolean")?;
                Ok(bool_value.into())
            }
            FieldType::Date => {
                let date_value = NaiveDate::parse_from_str(value, "%Y-%m-%d").map_err(|err| {
                    log::error!(
                        "Error converting value. <{}> - {}",
                        value.to_string(),
                        err.to_string()
                    );
                    MappingError(error_message_builder::convert_to_error(value, "date"))
                })?;
                Ok(date_value.into())
            }
            FieldType::DateTime => {
                let date_time_value = DateTime::parse_from_rfc3339(value)
                    .map_err(|err| {
                        log::error!(
                            "Error converting value. <{}> - {}",
                            value.to_string(),
                            err.to_string()
                        );
                        MappingError(error_message_builder::convert_to_error(value, "date time"))
                    })?
                    .to_utc();
                Ok(date_time_value.into())
            }
            FieldType::Interval => {
                let int_value = convert_with_serde::<i32>(value, "integer")?;
                Ok(int_value.into())
            }
            FieldType::MediaStatus => {
                let status =
                    convert_with_serde::<MediaStatus>(&format!("\"{value}\""), "media status")?;
                let status_value = i16::from(status);
                Ok(status_value.into())
            }
        }
    }
}

fn convert_with_serde<'a, T>(value: &'a str, type_string: &str) -> Result<T, MappingError>
where
    T: serde::de::Deserialize<'a>,
{
    serde_json::from_str::<T>(value).map_err(|err| {
        log::error!(
            "Error converting value. <{}> - {}",
            value.to_string(),
            err.to_string()
        );
        MappingError(error_message_builder::convert_to_error(value, type_string))
    })
}
