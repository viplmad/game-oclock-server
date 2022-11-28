use sea_query::{
    BinOper, Expr, Iden, LikeExpr, QueryStatementWriter, SelectStatement, SimpleExpr, Value,
};

use crate::entities::{FieldValue, Query};

const DEFAULT_PAGE_SIZE: u64 = 500;

pub fn apply_query<I: 'static + Iden + Clone + Copy>(
    mut select: SelectStatement,
    table_iden: I,
    query: Query<I>,
) -> impl QueryStatementWriter {
    if query.sort.is_some() {
        // Safe unwrap: already checked before
        for sort in query.sort.unwrap() {
            let field = sort.field;
            let order = sort.order;
            select.order_by((table_iden, field), order);
        }
    }

    if query.filter.is_some() {
        // Safe unwrap: already checked before
        for filter in query.filter.unwrap() {
            let field = filter.field;
            let col = Expr::col((table_iden, field));
            let expr = match filter.value {
                FieldValue::Value(value) => match filter.operator {
                    BinOper::Equal => col.eq(SimpleExpr::from(value)),
                    BinOper::NotEqual => col.ne(SimpleExpr::from(value)),
                    BinOper::GreaterThan => col.gt(Value::from(value)),
                    BinOper::GreaterThanOrEqual => col.gte(Value::from(value)),
                    BinOper::SmallerThan => col.lt(Value::from(value)),
                    BinOper::SmallerThanOrEqual => col.lte(Value::from(value)),
                    BinOper::Like => col.like(LikeExpr::from(value)),
                    BinOper::NotLike => col.not_like(LikeExpr::from(value)),
                    _ => panic!(), // TODO
                },
                FieldValue::Values(values) => {
                    let _type = values._type;
                    let smth = values.values.into_iter().map(|v| {
                        let field_search_value = crate::entities::FieldSearchValue {
                            _type: _type.clone(),
                            value: v,
                        };
                        Value::from(field_search_value)
                    });

                    match filter.operator {
                        BinOper::In => col.is_in(smth),
                        BinOper::NotIn => col.is_not_in(smth),
                        _ => panic!(), // TODO
                    }
                }
            };

            select.and_where(expr);
        }
    }

    let size = query.size.unwrap_or(DEFAULT_PAGE_SIZE);
    if size > 0 {
        select.limit(size);

        let page = query.page.unwrap_or(0);
        if page > 0 {
            select.offset(page * size);
        }
    }

    select
}
