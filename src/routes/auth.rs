use actix_web::{post, web, Responder};
use jsonwebtoken::{DecodingKey, EncodingKey};
use sqlx::PgPool;

use crate::models::TokenRequest;
use crate::services::auth_service;

use super::base::handle_get_result;

#[utoipa::path(
    post,
    path = "/auth/token",
    tag = "Auth",
    request_body(content = TokenRequest, description = "Token request (supported grant_type 'password' or 'refresh_token')", content_type = "application/x-www-form-urlencoded"),
    responses(
        (status = 200, description = "Pair of access and refresh token in JWT format", body = TokenResponse, content_type = "application/json"),
        (status = 400, description = "Bad request", body = ErrorMessage, content_type = "application/json"),
        (status = 401, description = "Unauthorized", body = ErrorMessage, content_type = "application/json"),
        (status = 500, description = "Internal server error", body = ErrorMessage, content_type = "application/json"),
    ),
)]
#[post("/token")]
pub async fn token(
    pool: web::Data<PgPool>,
    encoding_key: web::Data<EncodingKey>,
    decoding_key: web::Data<DecodingKey>,
    body: web::Form<TokenRequest>,
) -> impl Responder {
    let get_result = auth_service::get_token(&pool, &encoding_key, &decoding_key, body.0).await;
    handle_get_result(get_result)
}
