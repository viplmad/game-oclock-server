use sea_query::{BinOper, Iden, Order, SelectStatement};

pub struct SearchQuery {
    pub query: SelectStatement,
    pub page: u64,
    pub size: u64,
}

pub struct Search<I: Iden> {
    pub filter: Option<Vec<Filter<I>>>,
    pub sort: Option<Vec<Sort<I>>>,
    pub page: Option<u64>,
    pub size: Option<u64>,
}

pub struct Filter<I: Iden> {
    pub field: I,
    pub value: FieldValue,
    pub operator: BinOper,
    pub chain_operator: BinOper,
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

pub struct Sort<I: Iden> {
    pub field: I,
    pub order: Order,
}

pub struct FieldIden<I: Iden> {
    pub iden: I,
    pub _type: FieldType,
}

#[derive(Clone)]
pub enum FieldType {
    Integer,
    String,
    Date,
    DateTime,
}
