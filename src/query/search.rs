use sea_query::{BinOper, Expr, Iden, LikeExpr, SelectStatement, Value};

use crate::entities::{FieldValue, Search, SearchQuery};
use crate::errors::RepositoryError;

const DEFAULT_PAGE_SIZE: u64 = 500;
const INITIAL_PAGE: u64 = 0;

pub fn apply_search<I: 'static + Iden + Clone + Copy>(
    mut select: SelectStatement,
    table_iden: I,
    search: Search<I>,
) -> Result<SearchQuery, RepositoryError> {
    if search.sort.is_some() {
        // Safe unwrap: already checked before
        for sort in search.sort.unwrap() {
            let field = sort.field;
            let order = sort.order;
            select.order_by((table_iden, field), order);
        }
    }

    if search.filter.is_some() {
        // Safe unwrap: already checked before
        for filter in search.filter.unwrap() {
            let field = filter.field;
            let col = Expr::col((table_iden, field));
            let expr = match filter.value {
                FieldValue::Value(value) => match filter.operator {
                    BinOper::Equal => col.eq(Value::try_from(value)?),
                    BinOper::NotEqual => col.ne(Value::try_from(value)?),
                    BinOper::GreaterThan => col.gt(Value::try_from(value)?),
                    BinOper::GreaterThanOrEqual => col.gte(Value::try_from(value)?),
                    BinOper::SmallerThan => col.lt(Value::try_from(value)?),
                    BinOper::SmallerThanOrEqual => col.lte(Value::try_from(value)?),
                    BinOper::Like => col.like(LikeExpr::from(value)),
                    BinOper::NotLike => col.not_like(LikeExpr::from(value)),
                    _ => unreachable!(),
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
                        BinOper::In => col.is_in(in_values),
                        BinOper::NotIn => col.is_not_in(in_values),
                        _ => unreachable!(),
                    }
                }
            };

            match filter.chain_operator {
                BinOper::And => select.and_where(expr),
                BinOper::Or => todo!(), // TODO select.cond_where(Cond::any().add(expr)),
                _ => unreachable!(),
            };
        }
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
