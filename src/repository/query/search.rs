use sea_query::{BinOper, Cond, Expr, Func, LikeExpr, SelectStatement, Value};

use crate::entities::{
    AggregateCountMetric, AggregateDateHistogramGroup, AggregateFieldGroup, AggregateGroup,
    AggregateGroupSearch, AggregateMetric, AggregateSearch, AggregateSumMetric,
    DateHistogramInterval, FieldIden, FieldSearchValue, FieldValue, Filter, FilterOperator,
    ListSearch, SearchQuery, Sort, TableIden,
};
use crate::errors::{MappingError, SearchErrors};

const DEFAULT_PAGE_SIZE: u64 = 500;
const INITIAL_PAGE: u64 = 0;
const LIKE_SYMBOL: &str = "%";

pub fn apply_search<I: 'static + TableIden + Clone + Copy>(
    mut select: SelectStatement,
    search: ListSearch<I>,
) -> Result<SearchQuery, SearchErrors> {
    apply_filter(&mut select, search.filter).map_err(SearchErrors::Mapping)?;

    apply_sort(&mut select, search.sort);

    let (page, size) = apply_pagination(&mut select, search.page, search.size);

    Ok(SearchQuery {
        query: select,
        page,
        size,
    })
}

// TODO remove
pub fn apply_search_filter<I: 'static + TableIden + Clone + Copy>(
    mut select: SelectStatement,
    search: ListSearch<I>,
) -> Result<SelectStatement, SearchErrors> {
    apply_filter(&mut select, search.filter).map_err(SearchErrors::Mapping)?;

    Ok(select)
}

pub fn apply_aggregate_search<I: 'static + TableIden + Clone + Copy>(
    mut select: SelectStatement,
    search: AggregateSearch<I>,
) -> Result<SelectStatement, SearchErrors> {
    apply_filter(&mut select, search.filter).map_err(SearchErrors::Mapping)?;

    apply_aggregate_metric(&mut select, search.aggr);

    Ok(select)
}

pub fn apply_aggregate_group_search<I: 'static + TableIden + Clone + Copy>(
    mut select: SelectStatement,
    search: AggregateGroupSearch<I>,
) -> Result<SelectStatement, SearchErrors> {
    apply_filter(&mut select, search.filter).map_err(SearchErrors::Mapping)?;

    apply_aggregate_group(&mut select, search.group);

    apply_aggregate_metric(&mut select, search.aggr);

    Ok(select)
}

fn apply_filter<I: 'static + TableIden + Clone + Copy>(
    select: &mut SelectStatement,
    filter: Option<Vec<Filter<I>>>,
) -> Result<(), MappingError> {
    Ok(if let Some(filters) = filter {
        if !filters.is_empty() {
            let mut ands = Cond::all();
            let mut ors = Cond::any();

            for filter in filters {
                let col = build_col_expr(filter.field);
                let expr = match filter.value {
                    FieldValue::Value(value) => match filter.operator {
                        FilterOperator::Equal => col.eq(Value::try_from(value)?),
                        FilterOperator::NotEqual => col.ne(Value::try_from(value)?),
                        FilterOperator::GreaterThan => col.gt(Value::try_from(value)?),
                        FilterOperator::GreaterThanOrEqual => col.gte(Value::try_from(value)?),
                        FilterOperator::SmallerThan => col.lt(Value::try_from(value)?),
                        FilterOperator::SmallerThanOrEqual => col.lte(Value::try_from(value)?),
                        FilterOperator::StartsWith => {
                            to_lower(col).like(LikeExpr::new(format_like_starts_with(value)))
                        }
                        FilterOperator::NotStartsWith => {
                            to_lower(col).not_like(LikeExpr::new(format_like_starts_with(value)))
                        }
                        FilterOperator::EndsWith => {
                            to_lower(col).like(LikeExpr::new(format_like_ends_with(value)))
                        }
                        FilterOperator::NotEndsWith => {
                            to_lower(col).not_like(LikeExpr::new(format_like_ends_with(value)))
                        }
                        FilterOperator::Contains => {
                            to_lower(col).like(LikeExpr::new(format_like_contains(value)))
                        }
                        FilterOperator::NotContains => {
                            to_lower(col).not_like(LikeExpr::new(format_like_contains(value)))
                        }
                        _ => Err(MappingError(String::from(
                            "Operator not supported with single value.",
                        )))?,
                    },
                    FieldValue::Values(value) => {
                        let kind = value.kind;
                        let in_values = value
                            .values
                            .into_iter()
                            .map(|v| {
                                let field_search_value = crate::entities::FieldSearchValue {
                                    kind: kind.clone(),
                                    value: v,
                                };
                                Value::try_from(field_search_value)
                            })
                            .collect::<Result<Vec<Value>, MappingError>>()?;

                        match filter.operator {
                            FilterOperator::In => col.is_in(in_values),
                            FilterOperator::NotIn => col.is_not_in(in_values),
                            _ => Err(MappingError(String::from(
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

            if !ands.is_empty() {
                select.cond_where(ands);
            }
            if !ors.is_empty() {
                select.cond_where(ors);
            }
        }
    })
}

fn apply_sort<I: 'static + TableIden + Clone + Copy>(
    select: &mut SelectStatement,
    sort: Option<Vec<Sort<I>>>,
) {
    if let Some(sorts) = sort {
        for sort in sorts {
            let col = build_col_expr(sort.field);
            let order = sort.order;

            select.order_by_expr(col.into(), order);
        }
    }
}

fn apply_pagination(
    select: &mut SelectStatement,
    page: Option<u64>,
    size: Option<u64>,
) -> (u64, u64) {
    let size = size.unwrap_or(DEFAULT_PAGE_SIZE);
    select.limit(size);

    let page = page.unwrap_or(INITIAL_PAGE);
    select.offset(page * size);

    (page, size)
}

fn apply_aggregate_metric<I: 'static + TableIden + Clone + Copy>(
    select: &mut SelectStatement,
    aggr: AggregateMetric<I>,
) {
    match aggr {
        AggregateMetric::Count(c) => apply_aggregate_count_metric(select, c),
        AggregateMetric::Sum(s) => apply_aggregate_sum_metric(select, s),
    }
}

fn apply_aggregate_count_metric<I: 'static + TableIden + Clone + Copy>(
    select: &mut SelectStatement,
    aggr: AggregateCountMetric<I>,
) {
    let col = build_col_expr(aggr.field);

    let expr = coalesce_default(col, aggr.default_value);

    let expr = match aggr.distinct {
        true => expr.count_distinct(),
        false => expr.count(),
    };

    select.expr(expr);
}

fn apply_aggregate_sum_metric<I: 'static + TableIden + Clone + Copy>(
    select: &mut SelectStatement,
    aggr: AggregateSumMetric<I>,
) {
    let col = build_col_expr(aggr.field);

    let expr = coalesce_default(col, aggr.default_value);

    let expr = expr.sum();

    select.expr(expr);
}

fn apply_aggregate_group<I: 'static + TableIden + Clone + Copy>(
    select: &mut SelectStatement,
    aggr: AggregateGroup<I>,
) {
    match aggr {
        AggregateGroup::Field(f) => apply_aggregate_field_group(select, f),
        AggregateGroup::DateHistogram(d) => apply_aggregate_date_histogram_group(select, d),
    }
}

fn apply_aggregate_field_group<I: 'static + TableIden + Clone + Copy>(
    select: &mut SelectStatement,
    aggr: AggregateFieldGroup<I>,
) {
    let col = build_col_expr(aggr.field);

    let expr = coalesce_default(col, aggr.default_value);

    select.expr(expr.clone());
    select.add_group_by([expr.into()]);
}

fn apply_aggregate_date_histogram_group<I: 'static + TableIden + Clone + Copy>(
    select: &mut SelectStatement,
    aggr: AggregateDateHistogramGroup<I>,
) {
    let col = build_col_expr(aggr.field);

    let expr = coalesce_default(col, aggr.default_value);

    let expr = match aggr.interval {
        DateHistogramInterval::Year => Func::cust(DatePart).arg("year").arg(expr),
        DateHistogramInterval::Month => Func::cust(DatePart).arg("month").arg(expr),
        _ => todo!(),
    };

    select.expr(expr.clone());
    select.add_group_by([expr.into()]);
}

fn to_lower(col: Expr) -> Expr {
    Expr::expr(Func::lower(col))
}

fn format_like_starts_with(search: FieldSearchValue) -> String {
    let value: &str = &search.value.to_lowercase();
    format!("{value}{LIKE_SYMBOL}")
}

fn format_like_ends_with(search: FieldSearchValue) -> String {
    let value: &str = &search.value.to_lowercase();
    format!("{LIKE_SYMBOL}{value}")
}

fn format_like_contains(search: FieldSearchValue) -> String {
    let value: &str = &search.value.to_lowercase();
    format!("{LIKE_SYMBOL}{value}{LIKE_SYMBOL}")
}

fn build_col_expr<I: 'static + TableIden + Clone + Copy>(field: FieldIden<I>) -> Expr {
    let table = field.table;
    let field = field.iden;
    Expr::col((table, field))
}

fn coalesce_default(column: Expr, default_value: Option<FieldSearchValue>) -> Expr {
    match default_value {
        Some(def) => {
            let field_search_value = crate::entities::FieldSearchValue {
                kind: def.kind,
                value: def.value,
            };
            Expr::expr(Func::coalesce([
                column.into(),
                Value::try_from(field_search_value).unwrap().into(),
            ]))
        }
        None => column,
    }
}

///
struct DatePart;

impl sea_query::Iden for DatePart {
    fn unquoted(&self, s: &mut dyn std::fmt::Write) {
        write!(s, "DATE_PART").unwrap();
    }
}
