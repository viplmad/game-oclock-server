use utoipa::ToSchema;

#[derive(ToSchema)]
#[schema(format = Binary)]
pub struct Image(String);
