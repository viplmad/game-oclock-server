use sea_query::{BinOper, Iden, Order, SelectStatement};

use super::TableIden;

pub struct SearchQuery {
    pub query: SelectStatement,
    pub page: u64,
    pub size: u64,
}

pub struct Search<I: TableIden> {
    pub filter: Option<Vec<Filter<I>>>,
    pub sort: Option<Vec<Sort<I>>>,
    pub page: Option<u64>,
    pub size: Option<u64>,
}

pub struct Filter<I: TableIden> {
    pub table: std::rc::Rc<dyn Iden>,
    pub field: std::rc::Rc<dyn Iden>,
    pub value: FieldValue,
    pub operator: FilterOperator,
    pub chain_operator: BinOper,
    resource_type: std::marker::PhantomData<I>,
}

impl<T: TableIden> Filter<T> {
    pub fn new<I: TableIden>(
        table: std::rc::Rc<dyn Iden>,
        field: std::rc::Rc<dyn Iden>,
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
    pub _type: FieldType,
    pub value: String,
}

pub struct FieldSearchValues {
    pub _type: FieldType,
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

pub struct Sort<I: TableIden> {
    pub table: std::rc::Rc<dyn Iden>,
    pub field: std::rc::Rc<dyn Iden>,
    pub order: Order,
    resource_type: std::marker::PhantomData<I>,
}

impl<T: TableIden> Sort<T> {
    pub fn new<I: TableIden>(
        table: std::rc::Rc<dyn Iden>,
        field: std::rc::Rc<dyn Iden>,
        order: Order,
    ) -> Self {
        Self {
            table,
            field,
            order,
            resource_type: std::marker::PhantomData,
        }
    }
}

pub struct FieldIden<I: TableIden> {
    pub table: std::rc::Rc<dyn Iden>,
    pub iden: std::rc::Rc<dyn Iden>,
    pub _type: FieldType,
    resource_type: std::marker::PhantomData<I>,
}

impl<T: TableIden> FieldIden<T> {
    pub fn new<I: 'static + TableIden>(iden: I, _type: FieldType) -> Self {
        Self {
            table: std::rc::Rc::new(I::TABLE),
            iden: std::rc::Rc::new(iden),
            _type,
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
    GameStatus,
    PlatformType,
}
