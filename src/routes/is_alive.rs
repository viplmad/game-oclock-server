use actix_web::{get, HttpResponse, Responder};

#[utoipa::path(
    get,
    path = "/health",
    tag = "Health check",
    responses(
        (status = 200, description = "Alive"),
    ),
)]
#[get("/health")]
async fn health() -> impl Responder {
    HttpResponse::Ok().finish()
}
