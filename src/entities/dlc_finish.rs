use sea_query::Iden;

use super::TableIden;

#[derive(Iden)]
#[iden = "DLCFinish"]
pub enum DLCFinishIden {
    Table,
    #[iden = "user_id"]
    UserId,
    #[iden = "dlc_id"]
    DLCId,
    #[iden = "date"]
    Date,
}

impl TableIden for DLCFinishIden {
    const TABLE: Self = Self::Table;
}
