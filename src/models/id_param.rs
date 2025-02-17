use chrono::NaiveDate;
use serde::Deserialize;
use utoipa::IntoParams;
use uuid::Uuid;

#[derive(Deserialize, IntoParams)]
#[into_params(names("id"))]
pub struct ItemId(#[param(value_type = String)] pub Uuid);

#[derive(Deserialize, IntoParams)]
#[into_params(names("id", "other_id"))]
pub struct ItemIdAndRelatedId(
    #[param(value_type = String)] pub Uuid,
    #[param(value_type = String)] pub Uuid,
);

#[derive(Deserialize, IntoParams)]
pub struct StartEndDateQuery {
    #[param(value_type = String, format = Date)]
    pub start_date: NaiveDate,
    #[param(value_type = String, format = Date)]
    pub end_date: NaiveDate,
}

#[derive(Deserialize, IntoParams)]
pub struct OptionalStartEndDateQuery {
    #[param(value_type = Option<String>, format = Date)]
    pub start_date: Option<NaiveDate>,
    #[param(value_type = Option<String>, format = Date)]
    pub end_date: Option<NaiveDate>,
}
