use actix_web::{get, HttpResponse, Responder};

#[utoipa::path(
    get,
    path = "/is-alive",
    tag = "Health check",
    responses(
        (status = 200, description = "Alive"),
    ),
)]
#[get("/is-alive")]
async fn is_alive() -> impl Responder {
    HttpResponse::Ok().finish()
}
