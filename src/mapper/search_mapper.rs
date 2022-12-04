use std::str::FromStr;

use chrono::{NaiveDate, NaiveDateTime};
use sea_query::{BinOper, Order, Value};

use crate::entities::{
    FieldIden, FieldSearchValue, FieldSearchValues, FieldType, FieldValue, Filter, FilterOperator,
    Search, Sort, TableIden,
};
use crate::errors::{error_message_builder, FieldMappingError, RepositoryError};
use crate::models::{
    ChainOperatorType, FilterDTO, GameStatus, OperatorType, OrderType, PlatformType, SearchDTO,
    SearchValue, SortDTO,
};

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

impl<I: TableIden> TryFrom<SearchDTO> for Search<I>
where
    FieldIden<I>: FromStr,
{
    type Error = FieldMappingError;

    fn try_from(search: SearchDTO) -> Result<Self, Self::Error> {
        let filter_result = search.filter.map(|filters| {
            filters
                .into_iter()
                .map(Filter::try_from)
                .collect::<Result<Vec<Filter<I>>, FieldMappingError>>()
        });
        let filter = match filter_result {
            Some(res) => Some(res?),
            None => None,
        };

        let sort_result = search.sort.map(|sorts| {
            sorts
                .into_iter()
                .map(Sort::try_from)
                .collect::<Result<Vec<Sort<I>>, FieldMappingError>>()
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

impl<I: TableIden> TryFrom<FilterDTO> for Filter<I>
where
    FieldIden<I>: FromStr,
{
    type Error = FieldMappingError;

    fn try_from(filter: FilterDTO) -> Result<Self, Self::Error> {
        let field_iden =
            FieldIden::<I>::from_str(&filter.field).map_err(|_| FieldMappingError(filter.field))?;

        Ok(Self::new::<I>(
            field_iden.table,
            field_iden.iden,
            match filter.value {
                SearchValue::Value(value) => FieldValue::Value(FieldSearchValue {
                    _type: field_iden._type,
                    value,
                }),
                SearchValue::Values(values) => FieldValue::Values(FieldSearchValues {
                    _type: field_iden._type,
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
    type Error = FieldMappingError;

    fn try_from(sort: SortDTO) -> Result<Self, Self::Error> {
        let field_iden =
            FieldIden::<I>::from_str(&sort.field).map_err(|_| FieldMappingError(sort.field))?;

        Ok(Self::new::<I>(
            field_iden.table,
            field_iden.iden,
            Order::from(sort.order),
        ))
    }
}

impl TryFrom<FieldSearchValue> for Value {
    type Error = RepositoryError;

    fn try_from(search: FieldSearchValue) -> Result<Self, Self::Error> {
        let value: &str = &search.value;
        match search._type {
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
                let date_value = convert_with_serde::<NaiveDate>(value, "date")?;
                Ok(date_value.into())
            }
            FieldType::DateTime => {
                let date_time_value = convert_with_serde::<NaiveDateTime>(value, "date time")?;
                Ok(date_time_value.into())
            }
            FieldType::GameStatus => {
                let status =
                    convert_with_serde::<GameStatus>(&format!("\"{value}\""), "game status")?;
                let status_value = i16::from(status);
                Ok(status_value.into())
            }
            FieldType::PlatformType => {
                let ptype =
                    convert_with_serde::<PlatformType>(&format!("\"{value}\""), "platform type")?;
                let ptype_value = i16::from(ptype);
                Ok(ptype_value.into())
            }
        }
    }
}

fn convert_with_serde<'a, T>(value: &'a str, type_string: &str) -> Result<T, RepositoryError>
where
    T: serde::de::Deserialize<'a>,
{
    serde_json::from_str::<T>(value)
        .map_err(|_| RepositoryError(error_message_builder::convert_to_error(value, type_string)))
}
