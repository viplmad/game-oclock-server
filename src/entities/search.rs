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
    pub table: SeaRc<dyn Iden>,
    pub field: SeaRc<dyn Iden>,
    pub default_value: Option<FieldSearchValue>,
    resource_type: std::marker::PhantomData<I>,
}

pub struct AggregateCountMetric<I: TableIden> {
    pub table: SeaRc<dyn Iden>,
    pub field: SeaRc<dyn Iden>,
    pub default_value: Option<FieldSearchValue>,
    pub distinct: bool,
    resource_type: std::marker::PhantomData<I>,
}

impl<T: TableIden> AggregateCountMetric<T> {
    pub fn new<I: TableIden>(
        table: SeaRc<dyn Iden>,
        field: SeaRc<dyn Iden>,
        default_value: Option<FieldSearchValue>,
        distinct: bool,
    ) -> Self {
        Self {
            table,
            field,
            default_value,
            distinct,
            resource_type: std::marker::PhantomData,
        }
    }
}

impl<T: TableIden> AggregateSumMetric<T> {
    pub fn new<I: TableIden>(
        table: SeaRc<dyn Iden>,
        field: SeaRc<dyn Iden>,
        default_value: Option<FieldSearchValue>,
    ) -> Self {
        Self {
            table,
            field,
            default_value,
            resource_type: std::marker::PhantomData,
        }
    }
}

/// Aggregate group
pub enum AggregateGroup<I: TableIden> {
    Field(AggregateFieldGroup<I>),
    DateHistogram(AggregateDateHistogramGroup<I>),
}

pub struct AggregateFieldGroup<I: TableIden> {
    pub table: SeaRc<dyn Iden>,
    pub field: SeaRc<dyn Iden>,
    pub default_value: Option<FieldSearchValue>,
    resource_type: std::marker::PhantomData<I>,
}

pub struct AggregateDateHistogramGroup<I: TableIden> {
    pub table: SeaRc<dyn Iden>,
    pub field: SeaRc<dyn Iden>,
    pub default_value: Option<FieldSearchValue>,
    pub interval: DateHistogramInterval,
    resource_type: std::marker::PhantomData<I>,
}

pub enum DateHistogramInterval {
    Year,
    Month,
    Week,
    Day,
    Hour,
    Minute,
}

impl<T: TableIden> AggregateFieldGroup<T> {
    pub fn new<I: TableIden>(
        table: SeaRc<dyn Iden>,
        field: SeaRc<dyn Iden>,
        default_value: Option<FieldSearchValue>,
    ) -> Self {
        Self {
            table,
            field,
            default_value,
            resource_type: std::marker::PhantomData,
        }
    }
}

impl<T: TableIden> AggregateDateHistogramGroup<T> {
    pub fn new<I: TableIden>(
        table: SeaRc<dyn Iden>,
        field: SeaRc<dyn Iden>,
        default_value: Option<FieldSearchValue>,
        interval: DateHistogramInterval,
    ) -> Self {
        Self {
            table,
            field,
            default_value,
            interval,
            resource_type: std::marker::PhantomData,
        }
    }
}

/// Filter
pub struct Filter<I: TableIden> {
    pub table: SeaRc<dyn Iden>,
    pub field: SeaRc<dyn Iden>,
    pub value: FieldValue,
    pub operator: FilterOperator,
    pub chain_operator: BinOper,
    resource_type: std::marker::PhantomData<I>,
}

impl<T: TableIden> Filter<T> {
    pub fn new<I: TableIden>(
        table: SeaRc<dyn Iden>,
        field: SeaRc<dyn Iden>,
        value: FieldValue,
        operator: FilterOperator,
        chain_operator: BinOper,
    ) -> Self {
        Self {
            table,
            field,
            value,
            operator,
            chain_operator,
            resource_type: std::marker::PhantomData,
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
    pub table: SeaRc<dyn Iden>,
    pub field: SeaRc<dyn Iden>,
    pub order: Order,
    resource_type: std::marker::PhantomData<I>,
}

impl<T: TableIden> Sort<T> {
    pub fn new<I: TableIden>(table: SeaRc<dyn Iden>, field: SeaRc<dyn Iden>, order: Order) -> Self {
        Self {
            table,
            field,
            order,
            resource_type: std::marker::PhantomData,
        }
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
