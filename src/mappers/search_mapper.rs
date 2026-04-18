use std::str::FromStr;

use chrono::{DateTime, NaiveDate};
use sea_query::{BinOper, Order, Value};

use crate::entities::{
    AggregateCountMetric, AggregateDateHistogramGroup, AggregateFieldGroup,
    AggregateGroupResultKey, AggregateGroupSearch, AggregateResult, AggregateSearch,
    AggregateSumMetric, FieldIden, FieldType, Filter, GroupDateHistogramInterval, ListSearch,
    MultipleValuesFilter, NoValueFilter, SingleValueFilter, Sort, TableIden,
};
use crate::errors::{MappingError, error_message_builder};
use crate::models::{
    AggregateCountMetricDTO, AggregateDateHistogramGroupDTO, AggregateFieldGroupDTO,
    AggregateGroup, AggregateGroupResultKeyDTO, AggregateGroupSearchDTO, AggregateMetric,
    AggregateResultDTO, AggregateSearchDTO, AggregateSumMetricDTO, ChainOperatorType,
    DateHistogramInterval, DurationDef, FilterDTO, ListSearchDTO, MediaStatus,
    MultipleValuesFilterDTO, NoValueFilterDTO, OrderType, SingleValueFilterDTO, SortDTO,
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

        let aggr = convert_aggr(search.aggr)?;

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

        let aggr = convert_aggr(search.aggr)?;

        let group = convert_group(search.group)?;

        Ok(Self {
            filter,
            aggr,
            group,
        })
    }
}

fn convert_aggr<I: TableIden>(
    aggr: AggregateMetric,
) -> Result<crate::entities::AggregateMetric<I>, MappingError>
where
    FieldIden<I>: FromStr,
{
    Ok(match aggr {
        AggregateMetric::Count(c) => {
            crate::entities::AggregateMetric::Count(AggregateCountMetric::try_from(c)?)
        }
        AggregateMetric::Sum(s) => {
            crate::entities::AggregateMetric::Sum(AggregateSumMetric::try_from(s)?)
        }
    })
}

fn convert_group<I: TableIden>(
    group: AggregateGroup,
) -> Result<crate::entities::AggregateGroup<I>, MappingError>
where
    FieldIden<I>: FromStr,
{
    Ok(match group {
        AggregateGroup::Field(f) => {
            crate::entities::AggregateGroup::Field(AggregateFieldGroup::try_from(f)?)
        }
        AggregateGroup::DateHistogram(d) => crate::entities::AggregateGroup::DateHistogram(
            AggregateDateHistogramGroup::try_from(d)?,
        ),
    })
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

        Ok(Self::new(
            field,
            aggr.default_value,
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

        Ok(Self::new(field, aggr.default_value))
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

        Ok(Self::new(field, group.default_value))
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

        Ok(Self::new(
            field,
            group.default_value,
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
        Ok(match filter {
            FilterDTO::Eq(f) => Filter::Equal(SingleValueFilter::try_from(f)?),
            FilterDTO::NotEq(f) => Filter::NotEqual(SingleValueFilter::try_from(f)?),
            FilterDTO::Gt(f) => Filter::GreaterThan(SingleValueFilter::try_from(f)?),
            FilterDTO::Gte(f) => Filter::GreaterThanOrEqual(SingleValueFilter::try_from(f)?),
            FilterDTO::Lt(f) => Filter::SmallerThan(SingleValueFilter::try_from(f)?),
            FilterDTO::Lte(f) => Filter::SmallerThanOrEqual(SingleValueFilter::try_from(f)?),
            FilterDTO::In(f) => Filter::In(MultipleValuesFilter::try_from(f)?),
            FilterDTO::NotIn(f) => Filter::NotIn(MultipleValuesFilter::try_from(f)?),
            FilterDTO::StartsWith(f) => Filter::StartsWith(SingleValueFilter::try_from(f)?),
            FilterDTO::NotStartsWith(f) => Filter::NotStartsWith(SingleValueFilter::try_from(f)?),
            FilterDTO::EndsWith(f) => Filter::EndsWith(SingleValueFilter::try_from(f)?),
            FilterDTO::NotEndsWith(f) => Filter::NotEndsWith(SingleValueFilter::try_from(f)?),
            FilterDTO::Contains(f) => Filter::Contains(SingleValueFilter::try_from(f)?),
            FilterDTO::NotContains(f) => Filter::NotContains(SingleValueFilter::try_from(f)?),
            FilterDTO::Null(f) => Filter::NotNull(NoValueFilter::try_from(f)?),
            FilterDTO::NotNull(f) => Filter::NotNull(NoValueFilter::try_from(f)?),
        })
    }
}

impl<I: TableIden> TryFrom<SingleValueFilterDTO> for SingleValueFilter<I>
where
    FieldIden<I>: FromStr,
{
    type Error = MappingError;

    fn try_from(filter: SingleValueFilterDTO) -> Result<Self, Self::Error> {
        let field =
            FieldIden::<I>::from_str(&filter.field).map_err(|_| MappingError(filter.field))?;

        Ok(Self::new(
            field,
            filter.value,
            match filter.chain_operator {
                Some(chain_op) => BinOper::from(chain_op),
                None => BinOper::And,
            },
        ))
    }
}

impl<I: TableIden> TryFrom<MultipleValuesFilterDTO> for MultipleValuesFilter<I>
where
    FieldIden<I>: FromStr,
{
    type Error = MappingError;

    fn try_from(filter: MultipleValuesFilterDTO) -> Result<Self, Self::Error> {
        let field =
            FieldIden::<I>::from_str(&filter.field).map_err(|_| MappingError(filter.field))?;

        Ok(Self::new(
            field,
            filter.value,
            match filter.chain_operator {
                Some(chain_op) => BinOper::from(chain_op),
                None => BinOper::And,
            },
        ))
    }
}

impl<I: TableIden> TryFrom<NoValueFilterDTO> for NoValueFilter<I>
where
    FieldIden<I>: FromStr,
{
    type Error = MappingError;

    fn try_from(filter: NoValueFilterDTO) -> Result<Self, Self::Error> {
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
