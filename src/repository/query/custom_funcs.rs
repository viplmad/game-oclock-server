use sea_query::Iden;

#[derive(Iden)]
#[iden = "LAG"]
pub(super) struct Lag;

#[derive(Iden)]
#[iden = "DATE_PART"]
pub(super) struct DatePart;

#[derive(Iden)]
#[iden = "UNNEST"]
pub(super) struct Unnest;

#[derive(Iden)]
#[iden = "ARRAY_POSITIONS"]
pub(super) struct ArrayPositions;

#[derive(Iden)]
#[iden = "ARRAY_FILL"]
pub(super) struct ArrayFill;
