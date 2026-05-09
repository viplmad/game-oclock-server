use std::str::FromStr;

use chrono::{DateTime, NaiveDate};
use sea_query::{BinOper, Order, Value};

use crate::entities::{
    AggregateCountMetric, AggregateDateHistogramGroup, AggregateFieldGroup, AggregateGroup,
    AggregateGroupResultKey, AggregateGroupSearch, AggregateMetric, AggregateResult,
    AggregateSearch, AggregateSumMetric, FieldIden, FieldType, Filter, GroupDateHistogramInterval,
    ListSearch, MultipleValuesFilter, NoValueFilter, SingleValueFilter, Sort, TableIden,
};
use crate::errors::{MappingError, error_message_builder};
use crate::models::{
    AggregateGroupDTO, AggregateGroupResultKeyDTO, AggregateGroupSearchDTO, AggregateGroupType,
    AggregateMetricDTO, AggregateMetricType, AggregateResultDTO, AggregateSearchDTO,
    ChainOperatorType, DateHistogramInterval, DurationDef, FilterDTO, ListSearchDTO, MediaStatus,
    OperatorType, OrderType, SearchValue, SortDTO,
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
        let filter = convert_filters(search.filter)?;

        let sort = convert_sorts(search.sort)?;

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
        let filter = convert_filters(search.filter)?;

        let aggr = AggregateMetric::try_from(search.aggr)?;

        Ok(Self { filter, aggr })
    }
}

impl<I: TableIden> TryFrom<AggregateGroupSearchDTO> for AggregateGroupSearch<I>
where
    FieldIden<I>: FromStr,
{
    type Error = MappingError;

    fn try_from(search: AggregateGroupSearchDTO) -> Result<Self, Self::Error> {
        let filter = convert_filters(search.filter)?;

        let aggr = AggregateMetric::try_from(search.aggr)?;

        let group = AggregateGroup::try_from(search.group)?;

        Ok(Self {
            filter,
            aggr,
            group,
        })
    }
}

fn convert_filters<I: TableIden>(
    filter_option: Option<Vec<FilterDTO>>,
) -> Result<Option<Vec<Filter<I>>>, MappingError>
where
    FieldIden<I>: FromStr,
{
    let filter_result = filter_option.map(|filters| {
        filters
            .into_iter()
            .map(Filter::try_from)
            .collect::<Result<Vec<Filter<I>>, MappingError>>()
    });
    Ok(match filter_result {
        Some(res) => Some(res?),
        None => None,
    })
}

fn convert_sorts<I: TableIden>(
    optional_sort: Option<Vec<SortDTO>>,
) -> Result<Option<Vec<Sort<I>>>, MappingError>
where
    FieldIden<I>: FromStr,
{
    let sort_result = optional_sort.map(|sorts| {
        sorts
            .into_iter()
            .map(Sort::try_from)
            .collect::<Result<Vec<Sort<I>>, MappingError>>()
    });
    Ok(match sort_result {
        Some(res) => Some(res?),
        None => None,
    })
}

impl<I: TableIden> TryFrom<AggregateMetricDTO> for AggregateMetric<I>
where
    FieldIden<I>: FromStr,
{
    type Error = MappingError;

    fn try_from(aggr: AggregateMetricDTO) -> Result<Self, Self::Error> {
        Ok(match aggr.kind {
            AggregateMetricType::Count => {
                AggregateMetric::Count(AggregateCountMetric::try_from(aggr)?)
            }
            AggregateMetricType::Sum => AggregateMetric::Sum(AggregateSumMetric::try_from(aggr)?),
        })
    }
}

impl<I: TableIden> TryFrom<AggregateGroupDTO> for AggregateGroup<I>
where
    FieldIden<I>: FromStr,
{
    type Error = MappingError;

    fn try_from(group: AggregateGroupDTO) -> Result<Self, Self::Error> {
        Ok(match group.kind {
            AggregateGroupType::Field => {
                AggregateGroup::Field(AggregateFieldGroup::try_from(group)?)
            }
            AggregateGroupType::DateHistogram => {
                AggregateGroup::DateHistogram(AggregateDateHistogramGroup::try_from(group)?)
            }
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

impl<I: TableIden> TryFrom<AggregateMetricDTO> for AggregateCountMetric<I>
where
    FieldIden<I>: FromStr,
{
    type Error = MappingError;

    fn try_from(aggr: AggregateMetricDTO) -> Result<Self, Self::Error> {
        let field = FieldIden::<I>::from_str(&aggr.field).map_err(|_| MappingError(aggr.field))?;

        Ok(Self::new(
            field,
            aggr.default_value,
            aggr.distinct.unwrap_or(false),
        ))
    }
}

impl<I: TableIden> TryFrom<AggregateMetricDTO> for AggregateSumMetric<I>
where
    FieldIden<I>: FromStr,
{
    type Error = MappingError;

    fn try_from(aggr: AggregateMetricDTO) -> Result<Self, Self::Error> {
        let field = FieldIden::<I>::from_str(&aggr.field).map_err(|_| MappingError(aggr.field))?;

        Ok(Self::new(field, aggr.default_value))
    }
}

impl<I: TableIden> TryFrom<AggregateGroupDTO> for AggregateFieldGroup<I>
where
    FieldIden<I>: FromStr,
{
    type Error = MappingError;

    fn try_from(group: AggregateGroupDTO) -> Result<Self, Self::Error> {
        let field =
            FieldIden::<I>::from_str(&group.field).map_err(|_| MappingError(group.field))?;

        Ok(Self::new(field, group.default_value))
    }
}

impl<I: TableIden> TryFrom<AggregateGroupDTO> for AggregateDateHistogramGroup<I>
where
    FieldIden<I>: FromStr,
{
    type Error = MappingError;

    fn try_from(group: AggregateGroupDTO) -> Result<Self, Self::Error> {
        let field =
            FieldIden::<I>::from_str(&group.field).map_err(|_| MappingError(group.field))?;

        Ok(Self::new(
            field,
            group.default_value,
            match group.interval {
                Some(v) => Ok(GroupDateHistogramInterval::from(v)),
                None => Err(MappingError(error_message_builder::missing_body_field(
                    "date_histogram",
                ))),
            }?,
        ))
    }
}

impl<I: TableIden> TryFrom<FilterDTO> for Filter<I>
where
    FieldIden<I>: FromStr,
{
    type Error = MappingError;

    fn try_from(filter: FilterDTO) -> Result<Self, Self::Error> {
        Ok(match filter.operator {
            OperatorType::Eq => Filter::Equal(SingleValueFilter::try_from(filter)?),
            OperatorType::NotEq => Filter::NotEqual(SingleValueFilter::try_from(filter)?),
            OperatorType::Gt => Filter::GreaterThan(SingleValueFilter::try_from(filter)?),
            OperatorType::Gte => Filter::GreaterThanOrEqual(SingleValueFilter::try_from(filter)?),
            OperatorType::Lt => Filter::SmallerThan(SingleValueFilter::try_from(filter)?),
            OperatorType::Lte => Filter::SmallerThanOrEqual(SingleValueFilter::try_from(filter)?),
            OperatorType::In => Filter::In(MultipleValuesFilter::try_from(filter)?),
            OperatorType::NotIn => Filter::NotIn(MultipleValuesFilter::try_from(filter)?),
            OperatorType::StartsWith => Filter::StartsWith(SingleValueFilter::try_from(filter)?),
            OperatorType::NotStartsWith => {
                Filter::NotStartsWith(SingleValueFilter::try_from(filter)?)
            }
            OperatorType::EndsWith => Filter::EndsWith(SingleValueFilter::try_from(filter)?),
            OperatorType::NotEndsWith => Filter::NotEndsWith(SingleValueFilter::try_from(filter)?),
            OperatorType::Contains => Filter::Contains(SingleValueFilter::try_from(filter)?),
            OperatorType::NotContains => Filter::NotContains(SingleValueFilter::try_from(filter)?),
            OperatorType::Null => Filter::NotNull(NoValueFilter::try_from(filter)?),
            OperatorType::NotNull => Filter::NotNull(NoValueFilter::try_from(filter)?),
        })
    }
}

impl<I: TableIden> TryFrom<FilterDTO> for SingleValueFilter<I>
where
    FieldIden<I>: FromStr,
{
    type Error = MappingError;

    fn try_from(filter: FilterDTO) -> Result<Self, Self::Error> {
        let field =
            FieldIden::<I>::from_str(&filter.field).map_err(|_| MappingError(filter.field))?;

        Ok(Self::new(
            field,
            match filter.value {
                Some(sv) => match sv {
                    SearchValue::Single(v) => Ok(v),
                    _ => {
                        let operator = filter.operator;
                        Err(MappingError(format!(
                            "Field search_value must be a single value for operator \"{operator:?}\"."
                        )))
                    }
                },
                None => Err(MappingError(error_message_builder::missing_body_field(
                    "value",
                ))),
            }?,
            match filter.chain_operator {
                Some(chain_op) => BinOper::from(chain_op),
                None => BinOper::And,
            },
        ))
    }
}

impl<I: TableIden> TryFrom<FilterDTO> for MultipleValuesFilter<I>
where
    FieldIden<I>: FromStr,
{
    type Error = MappingError;

    fn try_from(filter: FilterDTO) -> Result<Self, Self::Error> {
        let field =
            FieldIden::<I>::from_str(&filter.field).map_err(|_| MappingError(filter.field))?;

        Ok(Self::new(
            field,
            match filter.value {
                Some(sv) => match sv {
                    SearchValue::Multiple(v) => Ok(v),
                    _ => {
                        let operator = filter.operator;
                        Err(MappingError(format!(
                            "Field search_value must be a list for operator \"{operator:?}\"."
                        )))
                    }
                },
                None => Err(MappingError(error_message_builder::missing_body_field(
                    "value",
                ))),
            }?,
            match filter.chain_operator {
                Some(chain_op) => BinOper::from(chain_op),
                None => BinOper::And,
            },
        ))
    }
}

impl<I: TableIden> TryFrom<FilterDTO> for NoValueFilter<I>
where
    FieldIden<I>: FromStr,
{
    type Error = MappingError;

    fn try_from(filter: FilterDTO) -> Result<Self, Self::Error> {
        let field =
            FieldIden::<I>::from_str(&filter.field).map_err(|_| MappingError(filter.field))?;

        Ok(Self::new(
            field,
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

pub fn convert_value(value: &str, kind: FieldType) -> Result<Value, MappingError> {
    match kind {
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
                log::error!("Error converting value. <{}> - {}", value, err);
                MappingError(error_message_builder::convert_to_error(value, "date"))
            })?;
            Ok(date_value.into())
        }
        FieldType::DateTime => {
            let date_time_value = DateTime::parse_from_rfc3339(value).map_err(|err| {
                log::error!("Error converting value. <{}> - {}", value, err);
                MappingError(error_message_builder::convert_to_error(value, "date time"))
            })?;
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

fn convert_with_serde<'a, T>(value: &'a str, type_string: &str) -> Result<T, MappingError>
where
    T: serde::de::Deserialize<'a>,
{
    serde_json::from_str::<T>(value).map_err(|err| {
        log::error!("Error converting value. <{}> - {}", value, err);
        MappingError(error_message_builder::convert_to_error(value, type_string))
    })
}
