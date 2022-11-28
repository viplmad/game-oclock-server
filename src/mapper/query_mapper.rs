use std::fmt::Debug;
use std::str::FromStr;

use sea_query::{BinOper, Iden, LikeExpr, Order, Value};

use crate::entities::{
    FieldIden, FieldSearchValue, FieldSearchValues, FieldType, FieldValue, Query, SearchFilter,
    Sort,
};
use crate::models::{OperatorType, OrderType, QueryDTO, SearchFilterDTO, SearchValue, SortDTO};

impl From<OrderType> for Order {
    fn from(order: OrderType) -> Self {
        match order {
            OrderType::Asc => Order::Asc,
            OrderType::Desc => Order::Desc,
        }
    }
}

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

impl<I: Iden> From<QueryDTO> for Query<I>
where
    FieldIden<I>: FromStr,
    <FieldIden<I> as FromStr>::Err: Debug,
{
    fn from(query: QueryDTO) -> Self {
        Self {
            filter: query
                .filter
                .map(|filter| filter.into_iter().map(SearchFilter::from).collect()),
            sort: query
                .sort
                .map(|sorts| sorts.into_iter().map(Sort::from).collect()),
            page: query.page,
            size: query.size,
        }
    }
}

impl<I: Iden> From<SortDTO> for Sort<I>
where
    FieldIden<I>: FromStr,
    <FieldIden<I> as FromStr>::Err: Debug,
{
    fn from(sort: SortDTO) -> Self {
        let field_iden = FieldIden::<I>::from_str(&sort.field).expect("");
        Self {
            field: field_iden.iden,
            order: Order::from(sort.order),
        }
    }
}

impl<I: Iden> From<SearchFilterDTO> for SearchFilter<I>
where
    FieldIden<I>: FromStr,
    <FieldIden<I> as FromStr>::Err: Debug,
{
    fn from(filter: SearchFilterDTO) -> Self {
        let field_iden = FieldIden::<I>::from_str(&filter.field).unwrap();
        Self {
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
        }
    }
}

impl From<FieldSearchValue> for Value {
    fn from(search: FieldSearchValue) -> Self {
        let value: &str = &search.value;
        match search._type {
            FieldType::Integer => i32::from_str(value).expect("").into(),
            FieldType::String => value.into(),
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
