use jsonwebtoken::{DecodingKey, EncodingKey};

use crate::errors::{ApiErrors, TokenErrors};
use crate::models::{GrantType, TokenRequest, TokenResponse};

use super::UserService;

#[derive(Clone)]
pub struct AuthService {
    user_service: UserService,
}

impl AuthService {
    pub fn with(user_service: UserService) -> Self {
        Self { user_service }
    }
}

impl AuthService {
    pub async fn get_token(
        &self,
        encoding_key: &EncodingKey,
        decoding_key: &DecodingKey,
        token_request: TokenRequest,
    ) -> Result<TokenResponse, TokenErrors> {
        match token_request.grant_type {
            GrantType::Password => {
                if crate::string_utils::optional_string_is_none_or_blank(&token_request.username) {
                    return Err(TokenErrors::InvalidRequest(String::from(
                        "Request was missing the 'username' parameter.",
                    )));
                }

                if crate::string_utils::optional_string_is_none_or_blank(&token_request.password) {
                    return Err(TokenErrors::InvalidRequest(String::from(
                        "Request was missing the 'password' parameter.",
                    )));
                }

                self.get_token_from_password(
                    encoding_key,
                    &token_request.username.unwrap(), // Safe unwrap: already checked before
                    &token_request.password.unwrap(), // Safe unwrap: already checked before
                )
                .await
            }
            GrantType::RefreshToken => {
                if crate::string_utils::optional_string_is_none_or_blank(
                    &token_request.refresh_token,
                ) {
                    return Err(TokenErrors::InvalidRequest(String::from(
                        "Request was missing the 'refresh_token' parameter.",
                    )));
                }

                self.get_token_from_refresh(
                    encoding_key,
                    decoding_key,
                    &token_request.refresh_token.unwrap(), // Safe unwrap: already checked before
                )
                .await
            }
        }
    }

    async fn get_token_from_password(
        &self,
        encoding_key: &EncodingKey,
        username: &str,
        password: &str,
    ) -> Result<TokenResponse, TokenErrors> {
        let user = self
            .user_service
            .get_user_by_username(username)
            .await
            .map_err(|err| match err {
                ApiErrors::NotFound(msg) => TokenErrors::InvalidRequest(msg),
                ApiErrors::UnknownError(msg) => TokenErrors::UnknownError(msg),
                _ => unreachable!("Other errors will never happen with a get call"),
            })?;

        let verify_pass: bool =
            crate::auth::verify_password(password, &user.password).map_err(|_| {
                TokenErrors::UnknownError(String::from("Password verification failed."))
            })?;

        if verify_pass {
            crate::auth::generate_token_response(&user.id.to_string(), encoding_key)
        } else {
            Err(TokenErrors::InvalidGrant(String::from("Wrong password.")))
        }
    }

    async fn get_token_from_refresh(
        &self,
        encoding_key: &EncodingKey,
        decoding_key: &DecodingKey,
        refresh_token: &str,
    ) -> Result<TokenResponse, TokenErrors> {
        let token_data = crate::auth::validate_token(refresh_token, decoding_key)
            .map_err(|_| TokenErrors::InvalidRequest(String::from("Invalid refresh token.")))?;

        // Ensure only refresh token is validated
        if token_data.claims.is_access() {
            return Err(TokenErrors::InvalidRequest(String::from(
                "Invalid refresh token: Was provided access token",
            )));
        }

        let user_id = token_data.claims.sub_as_user_id();
        let user = self
            .user_service
            .get_user(&user_id)
            .await
            .map_err(|err| match err {
                ApiErrors::NotFound(msg) => TokenErrors::InvalidRequest(msg),
                ApiErrors::UnknownError(msg) => TokenErrors::UnknownError(msg),
                _ => unreachable!("Other errors will never happen with a get call"),
            })?;

        crate::auth::generate_token_response(&user.id, encoding_key)
    }
}
