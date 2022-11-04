use jsonwebtoken::{DecodingKey, EncodingKey};
use sqlx::PgPool;

use crate::entities::User;
use crate::errors::{error_message_builder, TokenErrors, ApiErrors};
use crate::models::{TokenRequest, TokenResponse};
use crate::repository::user_repository;

use super::users_service;

const GRANT_TYPE_PASSWORD: &str = "password";
const GRANT_TYPE_REFRESH: &str = "refresh_token";

pub async fn get_token(
    pool: &PgPool,
    encoding_key: &EncodingKey,
    decoding_key: &DecodingKey,
    token_request: TokenRequest,
) -> Result<TokenResponse, TokenErrors> {
    match token_request.grant_type.as_str() {
        GRANT_TYPE_PASSWORD => {
            if crate::utils::optional_string_is_none_or_blank(&token_request.username) {
                return Err(TokenErrors::InvalidRequest(String::from(
                    "Request was missing the 'username' parameter.",
                )));
            }

            if crate::utils::optional_string_is_none_or_blank(&token_request.password) {
                return Err(TokenErrors::InvalidRequest(String::from(
                    "Request was missing the 'password' parameter.",
                )));
            }

            get_token_from_password(
                pool,
                encoding_key,
                &token_request.username.unwrap(), // Safe unwrap: already checked before
                &token_request.password.unwrap(), // Safe unwrap: already checked before
            )
            .await
        }
        GRANT_TYPE_REFRESH => {
            if crate::utils::optional_string_is_none_or_blank(&token_request.refresh_token) {
                return Err(TokenErrors::InvalidRequest(String::from(
                    "Request was missing the 'refresh_token' parameter.",
                )));
            }

            get_token_from_refresh(
                pool,
                encoding_key,
                decoding_key,
                &token_request.refresh_token.unwrap(), // Safe unwrap: already checked before
            )
            .await
        }
        _ => Err(TokenErrors::UnsupportedGrantType(String::from(
            "Only 'password' and 'refresh_token' grant types are supported.",
        ))),
    }
}

async fn get_token_from_password(
    pool: &PgPool,
    encoding_key: &EncodingKey,
    username: &str,
    password: &str,
) -> Result<TokenResponse, TokenErrors> {
    let user: User = user_repository::find_first_by_username(pool, username)
        .await
        .map_err(|err| {
            TokenErrors::UnknownError(error_message_builder::inner_error(
                "User could not be retrieved",
                &err.0,
            ))
        })?
        .ok_or_else(|| TokenErrors::InvalidRequest(String::from("User does not exist.")))?;

    let verify_pass: bool =
        crate::auth::verify_password(password, &user.password).map_err(|err| {
            TokenErrors::UnknownError(error_message_builder::inner_error(
                "Password verification failed",
                &err.0,
            ))
        })?;

    if verify_pass {
        crate::auth::generate_token_response(user.id, encoding_key)
    } else {
        Err(TokenErrors::InvalidGrant(String::from("Wrong password.")))
    }
}

async fn get_token_from_refresh(
    pool: &PgPool,
    encoding_key: &EncodingKey,
    decoding_key: &DecodingKey,
    refresh_token: &str,
) -> Result<TokenResponse, TokenErrors> {
    let token_data = crate::auth::validate_token(refresh_token, decoding_key).map_err(|err| {
        TokenErrors::InvalidRequest(error_message_builder::inner_error(
            "Invalid refresh token",
            &err.0,
        ))
    })?;

    // Ensure only refresh token is validated
    if token_data.claims.is_access() {
        return Err(TokenErrors::InvalidRequest(String::from(
            "Invalid refresh token: Was provided access token",
        )));
    }

    let user_id = token_data.claims.sub_as_user_id();
    let user = users_service::get_user(pool, user_id)
        .await
        .map_err(|err| match err {
            ApiErrors::NotFound(msg) => TokenErrors::InvalidRequest(msg),
            ApiErrors::UnknownError(msg) => TokenErrors::UnknownError(msg),
            _ => TokenErrors::UnknownError(String::default()), // Other errors will never happen with a get call
        })?;

    crate::auth::generate_token_response(user.id, encoding_key)
}
