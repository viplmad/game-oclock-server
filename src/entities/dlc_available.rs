use sea_query::Iden;

#[derive(Iden)]
#[iden = "DLCAvailable"]
pub enum DLCAvailableIden {
    Table,
    #[iden = "user_id"]
    UserId,
    #[iden = "dlc_id"]
    DLCId,
    #[iden = "platform_id"]
    PlatformId,
    #[iden = "added_date"]
    AddedDate,
}
