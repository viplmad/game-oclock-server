use sea_query::{BinOper, Cond, Expr, ExprTrait, Func, LikeExpr, SelectStatement, Value};

use crate::entities::{
    AggregateCountMetric, AggregateDateHistogramGroup, AggregateFieldGroup, AggregateGroup,
    AggregateGroupQuery, AggregateGroupSearch, AggregateMetric, AggregateQuery, AggregateSearch,
    AggregateSumMetric, AggregateType, ColIden, FieldIden, FieldType, Filter,
    GroupDateHistogramInterval, ListSearch, MultipleValuesFilter, SearchQuery, SingleValueFilter,
    Sort, TableIden,
};
use crate::errors::{MappingError, SearchErrors};
use crate::mappers::convert_value;

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
) -> Result<AggregateQuery, SearchErrors> {
    apply_filter(&mut select, search.filter).map_err(SearchErrors::Mapping)?;

    let (kind, field_kind) = apply_aggregate_metric(&mut select, search.aggr);

    Ok(AggregateQuery {
        query: select,
        kind,
        field_kind,
    })
}

pub fn apply_aggregate_group_search<I: 'static + TableIden + Clone + Copy>(
    mut select: SelectStatement,
    search: AggregateGroupSearch<I>,
) -> Result<AggregateGroupQuery, SearchErrors> {
    apply_filter(&mut select, search.filter).map_err(SearchErrors::Mapping)?;

    apply_aggregate_group(&mut select, search.group);

    let (kind, field_kind) = apply_aggregate_metric(&mut select, search.aggr);

    Ok(AggregateGroupQuery {
        query: select,
        kind,
        field_kind,
    })
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
                let chain_operator = filter.chain_operator();
                let expr = match filter {
                    Filter::Equal(f) => {
                        let (col, value) = build_single_value(f)?;

                        col.eq(value)
                    }
                    Filter::NotEqual(f) => {
                        let (col, value) = build_single_value(f)?;

                        col.ne(value)
                    }
                    Filter::GreaterThan(f) => {
                        let (col, value) = build_single_value(f)?;

                        col.gt(value)
                    }
                    Filter::GreaterThanOrEqual(f) => {
                        let (col, value) = build_single_value(f)?;

                        col.gte(value)
                    }
                    Filter::SmallerThan(f) => {
                        let (col, value) = build_single_value(f)?;

                        col.lt(value)
                    }
                    Filter::SmallerThanOrEqual(f) => {
                        let (col, value) = build_single_value(f)?;

                        col.lte(value)
                    }
                    Filter::In(f) => {
                        let (col, values) = build_multiple_values(f)?;

                        col.is_in(values)
                    }
                    Filter::NotIn(f) => {
                        let (col, values) = build_multiple_values(f)?;

                        col.is_not_in(values)
                    }
                    Filter::StartsWith(f) => {
                        let value = &f.value;
                        let col = build_field_expr(f.field);

                        to_lower(col).like(LikeExpr::new(format_like_starts_with(value)))
                    }
                    Filter::NotStartsWith(f) => {
                        let value = &f.value;
                        let col = build_field_expr(f.field);

                        to_lower(col).not_like(LikeExpr::new(format_like_starts_with(value)))
                    }
                    Filter::EndsWith(f) => {
                        let value = &f.value;
                        let col = build_field_expr(f.field);

                        to_lower(col).like(LikeExpr::new(format_like_ends_with(value)))
                    }
                    Filter::NotEndsWith(f) => {
                        let value = &f.value;
                        let col = build_field_expr(f.field);

                        to_lower(col).not_like(LikeExpr::new(format_like_ends_with(value)))
                    }
                    Filter::Contains(f) => {
                        let value = &f.value;
                        let col = build_field_expr(f.field);

                        to_lower(col).like(LikeExpr::new(format_like_contains(value)))
                    }
                    Filter::NotContains(f) => {
                        let value = &f.value;
                        let col = build_field_expr(f.field);

                        to_lower(col).not_like(LikeExpr::new(format_like_contains(value)))
                    }
                    Filter::Null(f) => {
                        let col = build_field_expr(f.field);

                        col.is_null()
                    }
                    Filter::NotNull(f) => {
                        let col = build_field_expr(f.field);

                        col.is_not_null()
                    }
                };

                match chain_operator {
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

fn build_single_value<I: 'static + TableIden + Clone + Copy>(
    filter: SingleValueFilter<I>,
) -> Result<(Expr, Value), MappingError> {
    let field_kind = filter.field.kind();
    let col = build_field_expr(filter.field);

    let value = convert_value(&filter.value, field_kind)?;

    Ok((col, value))
}

fn build_multiple_values<I: 'static + TableIden + Clone + Copy>(
    filter: MultipleValuesFilter<I>,
) -> Result<(Expr, Vec<Value>), MappingError> {
    let field_kind = filter.field.kind();
    let col = build_field_expr(filter.field);

    let values = filter
        .value
        .iter()
        .map(|v| convert_value(&v, field_kind.clone()))
        .collect::<Result<Vec<Value>, MappingError>>()?;

    Ok((col, values))
}

fn apply_sort<I: 'static + TableIden + Clone + Copy>(
    select: &mut SelectStatement,
    sort: Option<Vec<Sort<I>>>,
) {
    if let Some(sorts) = sort {
        for sort in sorts {
            let col = build_field_expr(sort.field);
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
) -> (AggregateType, FieldType) {
    let kind = aggr.kind();
    let field_kind = aggr.field_kind();
    match aggr {
        AggregateMetric::Count(c) => apply_aggregate_count_metric(select, c),
        AggregateMetric::Sum(s) => apply_aggregate_sum_metric(select, s),
    }

    (kind, field_kind)
}

fn apply_aggregate_count_metric<I: 'static + TableIden + Clone + Copy>(
    select: &mut SelectStatement,
    aggr: AggregateCountMetric<I>,
) {
    let field_kind = aggr.field.kind();
    let col = build_field_expr(aggr.field);

    let expr = coalesce_default(col, aggr.default_value, field_kind);

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
    let field_kind = aggr.field.kind();
    let col = build_field_expr(aggr.field);

    let expr = coalesce_default(col, aggr.default_value, field_kind);

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
    let field_kind = aggr.field.kind();
    let col = build_field_expr(aggr.field);

    let expr = coalesce_default(col, aggr.default_value, field_kind);

    select.expr(expr.clone());
    select.add_group_by([expr.into()]);
}

fn apply_aggregate_date_histogram_group<I: 'static + TableIden + Clone + Copy>(
    select: &mut SelectStatement,
    aggr: AggregateDateHistogramGroup<I>,
) {
    let field_kind = aggr.field.kind();
    let col = build_field_expr(aggr.field);

    let expr = coalesce_default(col, aggr.default_value, field_kind);

    let expr = match aggr.interval {
        GroupDateHistogramInterval::Year => Func::cust(DatePart).arg("year").arg(expr),
        GroupDateHistogramInterval::Month => Func::cust(DatePart).arg("month").arg(expr),
        GroupDateHistogramInterval::Day => Func::cust(DatePart).arg("day").arg(expr),
        GroupDateHistogramInterval::Weekday => Func::cust(DatePart).arg("isodow").arg(expr),
        GroupDateHistogramInterval::Hour => Func::cust(DatePart).arg("hour").arg(expr),
        GroupDateHistogramInterval::Minute => Func::cust(DatePart).arg("minute").arg(expr),
    }
    .cast_as("BIGINT"); // Cast as int as no float date parts are used

    select.expr(expr.clone());
    select.add_group_by([expr.into()]);
}

fn to_lower(col: Expr) -> Expr {
    Expr::expr(Func::lower(col))
}

fn format_like_starts_with(search: &str) -> String {
    let value: &str = &search.to_lowercase();
    format!("{value}{LIKE_SYMBOL}")
}

fn format_like_ends_with(search: &str) -> String {
    let value: &str = &search.to_lowercase();
    format!("{LIKE_SYMBOL}{value}")
}

fn format_like_contains(search: &str) -> String {
    let value: &str = &search.to_lowercase();
    format!("{LIKE_SYMBOL}{value}{LIKE_SYMBOL}")
}

fn build_field_expr<I: 'static + TableIden + Clone + Copy>(field: FieldIden<I>) -> Expr {
    match field {
        FieldIden::Col(c) => build_col_expr(c),
        FieldIden::Expr(e) => Expr::expr(e.expr),
    }
}

fn build_col_expr<I: 'static + TableIden + Clone + Copy>(field: ColIden<I>) -> Expr {
    let table = field.table;
    let field = field.iden;
    Expr::col((table, field))
}

fn coalesce_default(column: Expr, default_value: Option<String>, field_kind: FieldType) -> Expr {
    match default_value {
        Some(def) => Expr::expr(Func::coalesce([
            column.into(),
            convert_value(&def, field_kind).unwrap().into(),
        ])),
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
