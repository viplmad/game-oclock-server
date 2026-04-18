use sea_query::{BinOper, Iden, Order, SeaRc, SelectStatement, SimpleExpr};

use super::TableIden;

pub struct SearchQuery {
    pub query: SelectStatement,
    pub page: u64,
    pub size: u64,
}

pub struct AggregateQuery {
    pub query: SelectStatement,
    pub kind: AggregateType,
    pub field_kind: FieldType,
}

pub struct AggregateGroupQuery {
    pub query: SelectStatement,
    pub kind: AggregateType,
    pub field_kind: FieldType,
}

pub struct ListSearch<T: TableIden> {
    pub filter: Option<Vec<Filter<T>>>,
    pub sort: Option<Vec<Sort<T>>>,
    pub page: Option<u64>,
    pub size: Option<u64>,
}

pub struct AggregateSearch<T: TableIden> {
    pub filter: Option<Vec<Filter<T>>>,
    pub aggr: AggregateMetric<T>,
}

pub struct AggregateGroupSearch<T: TableIden> {
    pub filter: Option<Vec<Filter<T>>>,
    pub aggr: AggregateMetric<T>,
    pub group: AggregateGroup<T>,
}

/// Aggregate metric
pub enum AggregateMetric<T: TableIden> {
    Count(AggregateCountMetric<T>),
    Sum(AggregateSumMetric<T>),
}

pub struct AggregateSumMetric<T: TableIden> {
    pub kind: AggregateType,
    pub field: FieldIden<T>,
    pub default_value: Option<FieldSearchValue>,
}

pub struct AggregateCountMetric<T: TableIden> {
    pub kind: AggregateType,
    pub field: FieldIden<T>,
    pub default_value: Option<FieldSearchValue>,
    pub distinct: bool,
}

impl<T: TableIden> AggregateMetric<T> {
    pub fn kind(&self) -> AggregateType {
        match self {
            AggregateMetric::Count(c) => c.kind.clone(),
            AggregateMetric::Sum(s) => s.kind.clone(),
        }
    }

    pub fn field_kind(&self) -> FieldType {
        match self {
            AggregateMetric::Count(c) => c.field.kind(),
            AggregateMetric::Sum(s) => s.field.kind(),
        }
    }
}

impl<T: TableIden> AggregateCountMetric<T> {
    pub fn new(
        field: FieldIden<T>,
        default_value: Option<FieldSearchValue>,
        distinct: bool,
    ) -> Self {
        Self {
            kind: AggregateType::Count,
            field,
            default_value,
            distinct,
        }
    }
}

impl<T: TableIden> AggregateSumMetric<T> {
    pub fn new(field: FieldIden<T>, default_value: Option<FieldSearchValue>) -> Self {
        Self {
            kind: AggregateType::Sum,
            field,
            default_value,
        }
    }
}

/// Aggregate group
pub enum AggregateGroup<T: TableIden> {
    Field(AggregateFieldGroup<T>),
    DateHistogram(AggregateDateHistogramGroup<T>),
}

pub struct AggregateFieldGroup<T: TableIden> {
    pub field: FieldIden<T>,
    pub default_value: Option<FieldSearchValue>,
}

pub struct AggregateDateHistogramGroup<T: TableIden> {
    pub field: FieldIden<T>,
    pub default_value: Option<FieldSearchValue>,
    pub interval: GroupDateHistogramInterval,
}

pub enum GroupDateHistogramInterval {
    Year,
    Month,
    Weekday,
    Day,
    Hour,
    Minute,
}

impl<T: TableIden> AggregateFieldGroup<T> {
    pub fn new(field: FieldIden<T>, default_value: Option<FieldSearchValue>) -> Self {
        Self {
            field,
            default_value,
        }
    }
}

impl<T: TableIden> AggregateDateHistogramGroup<T> {
    pub fn new(
        field: FieldIden<T>,
        default_value: Option<FieldSearchValue>,
        interval: GroupDateHistogramInterval,
    ) -> Self {
        Self {
            field,
            default_value,
            interval,
        }
    }
}

/// Filter
pub struct Filter<T: TableIden> {
    pub field: FieldIden<T>,
    pub value: FieldValue,
    pub operator: FilterOperator,
    pub chain_operator: BinOper,
}

impl<T: TableIden> Filter<T> {
    pub fn new(
        field: FieldIden<T>,
        value: FieldValue,
        operator: FilterOperator,
        chain_operator: BinOper,
    ) -> Self {
        Self {
            field,
            value,
            operator,
            chain_operator,
        }
    }
}

pub enum FieldValue {
    Value(FieldSearchValue),
    Values(FieldSearchValues),
}

pub struct FieldSearchValue {
    pub kind: FieldType,
    pub value: String,
}

pub struct FieldSearchValues {
    pub kind: FieldType,
    pub values: Vec<String>,
}

#[derive(Clone)]
pub enum FilterOperator {
    Equal,
    NotEqual,
    GreaterThan,
    GreaterThanOrEqual,
    SmallerThan,
    SmallerThanOrEqual,
    In,
    NotIn,
    StartsWith,
    NotStartsWith,
    EndsWith,
    NotEndsWith,
    Contains,
    NotContains,
}

/// Sort
pub struct Sort<T: TableIden> {
    pub field: FieldIden<T>,
    pub order: Order,
}

impl<T: TableIden> Sort<T> {
    pub fn new(field: FieldIden<T>, order: Order) -> Self {
        Self { field, order }
    }
}

/// Field
pub enum FieldIden<T: TableIden> {
    Col(ColIden<T>),
    Expr(ExprIden<T>),
}

pub struct ColIden<T: TableIden> {
    pub table: SeaRc<dyn Iden>,
    pub iden: SeaRc<dyn Iden>,
    pub kind: FieldType,
    resource_type: std::marker::PhantomData<T>,
}

pub struct ExprIden<T: TableIden> {
    pub expr: SimpleExpr,
    pub kind: FieldType,
    resource_type: std::marker::PhantomData<T>,
}

impl<T: TableIden> FieldIden<T> {
    pub fn kind(&self) -> FieldType {
        match self {
            FieldIden::Col(c) => c.kind.clone(),
            FieldIden::Expr(e) => e.kind.clone(),
        }
    }
}

impl<T: TableIden> ColIden<T> {
    pub fn new<I: 'static + TableIden>(iden: I, kind: FieldType) -> Self {
        Self {
            table: SeaRc::new(I::TABLE),
            iden: SeaRc::new(iden),
            kind,
            resource_type: std::marker::PhantomData,
        }
    }
}

impl<T: TableIden> ExprIden<T> {
    pub fn new<I: 'static + TableIden>(expr: SimpleExpr, kind: FieldType) -> Self {
        Self {
            expr,
            kind,
            resource_type: std::marker::PhantomData,
        }
    }
}

#[derive(Clone, PartialEq)]
pub enum FieldType {
    Integer,
    String,
    Boolean,
    Date,
    DateTime,
    Interval,
    MediaStatus,
}

#[derive(Clone, PartialEq)]
pub enum AggregateType {
    Count,
    Sum,
}
