use sea_query::Iden;

pub trait TableIden
where
    Self: Iden,
{
    /// Obtain table Iden
    const TABLE: Self;
}
