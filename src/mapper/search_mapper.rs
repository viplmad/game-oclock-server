use std::str::FromStr;

use sea_query::{BinOper, Iden, LikeExpr, Order, Value};

use crate::entities::{
    FieldIden, FieldSearchValue, FieldSearchValues, FieldType, FieldValue, Filter, Search, Sort,
};
use crate::errors::{error_message_builder, FieldMappingError, RepositoryError};
use crate::models::{
    ChainOperatorType, FilterDTO, OperatorType, OrderType, SearchDTO, SearchValue, SortDTO,
};

impl From<OperatorType> for BinOper {
    fn from(operator: OperatorType) -> Self {
        match operator {
            OperatorType::Eq => BinOper::Equal,
            OperatorType::NotEq => BinOper::NotEqual,
            OperatorType::Gt => BinOper::GreaterThan,
            OperatorType::Gte => BinOper::GreaterThanOrEqual,
            OperatorType::Lt => BinOper::SmallerThan,
            OperatorType::Lte => BinOper::SmallerThanOrEqual,
            OperatorType::In => BinOper::In,
            OperatorType::NotIn => BinOper::NotIn,
            OperatorType::StartsWith => BinOper::Like,
            OperatorType::NotStartsWith => BinOper::NotLike,
            OperatorType::EndsWith => BinOper::Like,
            OperatorType::NotEndsWith => BinOper::NotLike,
            OperatorType::Contains => BinOper::Like,
            OperatorType::NotContains => BinOper::NotLike,
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

impl<I: Iden> TryFrom<SearchDTO> for Search<I>
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

impl<I: Iden> TryFrom<FilterDTO> for Filter<I>
where
    FieldIden<I>: FromStr,
{
    type Error = FieldMappingError;

    fn try_from(filter: FilterDTO) -> Result<Self, Self::Error> {
        let field_iden =
            FieldIden::<I>::from_str(&filter.field).map_err(|_| FieldMappingError(filter.field))?;

        Ok(Self {
            field: field_iden.iden,
            value: match filter.value {
                SearchValue::Value(value) => FieldValue::Value(FieldSearchValue {
                    _type: field_iden._type,
                    value,
                }),
                SearchValue::Values(values) => FieldValue::Values(FieldSearchValues {
                    _type: field_iden._type,
                    values,
                }),
            },
            operator: BinOper::from(filter.operator),
            chain_operator: match filter.chain_operator {
                Some(chain_op) => BinOper::from(chain_op),
                None => BinOper::And,
            },
        })
    }
}

impl<I: Iden> TryFrom<SortDTO> for Sort<I>
where
    FieldIden<I>: FromStr,
{
    type Error = FieldMappingError;

    fn try_from(sort: SortDTO) -> Result<Self, Self::Error> {
        let field_iden =
            FieldIden::<I>::from_str(&sort.field).map_err(|_| FieldMappingError(sort.field))?;

        Ok(Self {
            field: field_iden.iden,
            order: Order::from(sort.order),
        })
    }
}

impl TryFrom<FieldSearchValue> for Value {
    type Error = RepositoryError;

    fn try_from(search: FieldSearchValue) -> Result<Self, Self::Error> {
        let value: &str = &search.value;
        match search._type {
            FieldType::Integer => {
                let int_value = i32::from_str(value).map_err(|_| {
                    RepositoryError(error_message_builder::convert_to_error(value, "integer"))
                })?;
                Ok(int_value.into())
            }
            FieldType::String => Ok(value.into()),
            FieldType::Date => todo!(),
            FieldType::DateTime => todo!(),
        }
    }
}

impl From<FieldSearchValue> for LikeExpr {
    fn from(search: FieldSearchValue) -> Self {
        let value: &str = &search.value;
        match search._type {
            FieldType::Integer => todo!(),
            FieldType::String => LikeExpr::str(value),
            FieldType::Date => todo!(),
            FieldType::DateTime => todo!(),
        }
    }
}
