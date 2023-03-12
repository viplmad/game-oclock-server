use utoipa::ToSchema;

#[derive(ToSchema)]
pub struct Image {
    #[schema(value_type = String, format = Binary)]
    pub file: String,
}
