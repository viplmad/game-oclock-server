use actix_web::{HttpResponse, Responder, get};

/// Liveness check
#[utoipa::path(
    get,
    path = "/health",
    tag = "Health",
    responses(
        (status = 200, description = "Alive"),
    ),
)]
#[get("/health")]
pub async fn health() -> impl Responder {
    HttpResponse::Ok().finish()
}
