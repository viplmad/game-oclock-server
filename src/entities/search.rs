use sea_query::{BinOper, Iden, Order, SeaRc, SelectStatement};

use super::TableIden;

pub struct SearchQuery {
    pub query: SelectStatement,
    pub page: u64,
    pub size: u64,
}

pub struct ListSearch<I: TableIden> {
    pub filter: Option<Vec<Filter<I>>>,
    pub sort: Option<Vec<Sort<I>>>,
    pub page: Option<u64>,
    pub size: Option<u64>,
}

pub struct AggregateSearch<I: TableIden> {
    pub filter: Option<Vec<Filter<I>>>,
    pub aggr: AggregateMetric<I>,
}

pub struct AggregateGroupSearch<I: TableIden> {
    pub filter: Option<Vec<Filter<I>>>,
    pub aggr: AggregateMetric<I>,
    pub group: AggregateGroup<I>,
}

/// Aggregate metric
pub enum AggregateMetric<I: TableIden> {
    Count(AggregateCountMetric<I>),
    Sum(AggregateSumMetric<I>),
}

pub struct AggregateSumMetric<I: TableIden> {
    pub field: FieldIden<I>,
    pub default_value: Option<FieldSearchValue>,
}

pub struct AggregateCountMetric<I: TableIden> {
    pub field: FieldIden<I>,
    pub default_value: Option<FieldSearchValue>,
    pub distinct: bool,
}

impl<I: TableIden> AggregateCountMetric<I> {
    pub fn new(
        field: FieldIden<I>,
        default_value: Option<FieldSearchValue>,
        distinct: bool,
    ) -> Self {
        Self {
            field,
            default_value,
            distinct,
        }
    }
}

impl<I: TableIden> AggregateSumMetric<I> {
    pub fn new(field: FieldIden<I>, default_value: Option<FieldSearchValue>) -> Self {
        Self {
            field,
            default_value,
        }
    }
}

/// Aggregate group
pub enum AggregateGroup<I: TableIden> {
    Field(AggregateFieldGroup<I>),
    DateHistogram(AggregateDateHistogramGroup<I>),
}

pub struct AggregateFieldGroup<I: TableIden> {
    pub field: FieldIden<I>,
    pub default_value: Option<FieldSearchValue>,
}

pub struct AggregateDateHistogramGroup<I: TableIden> {
    pub field: FieldIden<I>,
    pub default_value: Option<FieldSearchValue>,
    pub interval: DateHistogramInterval,
}

pub enum DateHistogramInterval {
    Year,
    Month,
    Week,
    Day,
    Hour,
    Minute,
}

impl<I: TableIden> AggregateFieldGroup<I> {
    pub fn new(field: FieldIden<I>, default_value: Option<FieldSearchValue>) -> Self {
        Self {
            field,
            default_value,
        }
    }
}

impl<I: TableIden> AggregateDateHistogramGroup<I> {
    pub fn new(
        field: FieldIden<I>,
        default_value: Option<FieldSearchValue>,
        interval: DateHistogramInterval,
    ) -> Self {
        Self {
            field,
            default_value,
            interval,
        }
    }
}

/// Filter
pub struct Filter<I: TableIden> {
    pub field: FieldIden<I>,
    pub value: FieldValue,
    pub operator: FilterOperator,
    pub chain_operator: BinOper,
}

impl<I: TableIden> Filter<I> {
    pub fn new(
        field: FieldIden<I>,
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
pub struct Sort<I: TableIden> {
    pub field: FieldIden<I>,
    pub order: Order,
}

impl<I: TableIden> Sort<I> {
    pub fn new(field: FieldIden<I>, order: Order) -> Self {
        Self { field, order }
    }
}

/// Field
pub struct FieldIden<I: TableIden> {
    pub table: SeaRc<dyn Iden>,
    pub iden: SeaRc<dyn Iden>,
    pub kind: FieldType,
    resource_type: std::marker::PhantomData<I>,
}

impl<T: TableIden> FieldIden<T> {
    pub fn new<I: 'static + TableIden>(iden: I, kind: FieldType) -> Self {
        Self {
            table: SeaRc::new(I::TABLE),
            iden: SeaRc::new(iden),
            kind,
            resource_type: std::marker::PhantomData,
        }
    }
}

#[derive(Clone)]
pub enum FieldType {
    Integer,
    String,
    Boolean,
    Date,
    DateTime,
    MediaStatus,
}
