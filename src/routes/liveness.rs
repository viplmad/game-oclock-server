use actix_web::{HttpResponse, Responder, get};

#[utoipa::path(
    get,
    path = "/health",
    tag = "Health check",
    responses(
        (status = 200, description = "Alive"),
    ),
)]
#[get("/health")]
pub async fn health() -> impl Responder {
    HttpResponse::Ok().finish()
}
