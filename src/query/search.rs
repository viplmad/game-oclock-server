use sea_query::{BinOper, Cond, Expr, LikeExpr, SelectStatement, Value};

use crate::entities::{
    FieldSearchValue, FieldValue, FilterOperator, Search, SearchQuery, TableIden,
};
use crate::errors::RepositoryError;

const DEFAULT_PAGE_SIZE: u64 = 500;
const INITIAL_PAGE: u64 = 0;
const LIKE_SYMBOL: &str = "%";

pub fn apply_search<I: 'static + TableIden + Clone + Copy>(
    mut select: SelectStatement,
    search: Search<I>,
) -> Result<SearchQuery, RepositoryError> {
    if let Some(sorts) = search.sort {
        for sort in sorts {
            let table = sort.table;
            let field = sort.field;
            let order = sort.order;
            select.order_by((table, field), order);
        }
    }

    if let Some(filters) = search.filter {
        let mut ands = Cond::all();
        let mut ors = Cond::any();

        for filter in filters {
            let table = filter.table;
            let field = filter.field;
            let col = Expr::col((table, field));
            let expr = match filter.value {
                FieldValue::Value(value) => match filter.operator {
                    FilterOperator::Equal => col.eq(Value::try_from(value)?),
                    FilterOperator::NotEqual => col.ne(Value::try_from(value)?),
                    FilterOperator::GreaterThan => col.gt(Value::try_from(value)?),
                    FilterOperator::GreaterThanOrEqual => col.gte(Value::try_from(value)?),
                    FilterOperator::SmallerThan => col.lt(Value::try_from(value)?),
                    FilterOperator::SmallerThanOrEqual => col.lte(Value::try_from(value)?),
                    FilterOperator::StartsWith => {
                        col.like(LikeExpr::str(&format_like_starts_with(value)))
                    }
                    FilterOperator::NotStartsWith => {
                        col.not_like(LikeExpr::str(&format_like_starts_with(value)))
                    }
                    FilterOperator::EndsWith => {
                        col.like(LikeExpr::str(&format_like_ends_with(value)))
                    }
                    FilterOperator::NotEndsWith => {
                        col.not_like(LikeExpr::str(&format_like_ends_with(value)))
                    }
                    FilterOperator::Contains => {
                        col.like(LikeExpr::str(&format_like_contains(value)))
                    }
                    FilterOperator::NotContains => {
                        col.not_like(LikeExpr::str(&format_like_contains(value)))
                    }
                    _ => Err(RepositoryError(String::from(
                        "Operator not supported with single value.",
                    )))?,
                },
                FieldValue::Values(value) => {
                    let _type = value._type;
                    let in_values = value
                        .values
                        .into_iter()
                        .map(|v| {
                            let field_search_value = crate::entities::FieldSearchValue {
                                _type: _type.clone(),
                                value: v,
                            };
                            Value::try_from(field_search_value)
                        })
                        .collect::<Result<Vec<Value>, RepositoryError>>()?;

                    match filter.operator {
                        FilterOperator::In => col.is_in(in_values),
                        FilterOperator::NotIn => col.is_not_in(in_values),
                        _ => Err(RepositoryError(String::from(
                            "Operator not supported with multiple values.",
                        )))?,
                    }
                }
            };

            match filter.chain_operator {
                BinOper::And => ands = ands.add(expr),
                BinOper::Or => ors = ors.add(expr),
                _ => unreachable!(),
            };
        }

        select.cond_where(ands);
        select.cond_where(ors);
    }

    let size = search.size.unwrap_or(DEFAULT_PAGE_SIZE);
    let page = search.page.unwrap_or(INITIAL_PAGE);
    if size > 0 {
        select.limit(size);
        if page > 0 {
            select.offset(page * size);
        }
    }

    Ok(SearchQuery {
        query: select,
        page,
        size,
    })
}

fn format_like_starts_with(search: FieldSearchValue) -> String {
    let value: &str = &search.value;
    format!("{value}{LIKE_SYMBOL}")
}

fn format_like_ends_with(search: FieldSearchValue) -> String {
    let value: &str = &search.value;
    format!("{LIKE_SYMBOL}{value}")
}

fn format_like_contains(search: FieldSearchValue) -> String {
    let value: &str = &search.value;
    format!("{LIKE_SYMBOL}{value}{LIKE_SYMBOL}")
}
