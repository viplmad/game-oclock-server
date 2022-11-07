use sea_query::Iden;

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
