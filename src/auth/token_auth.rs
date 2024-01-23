use actix_web::{dev::ServiceRequest, Error};
use actix_web_httpauth::extractors::bearer::{BearerAuth, Config};
use actix_web_httpauth::extractors::AuthenticationError;
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, TokenData, Validation};

use crate::errors::{TokenErrors, ValidationError};
use crate::models::{TokenResponse, UserClaims};

const KID: &str = "075d91f0-a35b-455a-9d78-8598846805e8"; // Random UUID
const ISSUER: &str = "game_oclock";

const TOKEN_TYPE_BEARER: &str = "bearer";

const DAYS_PER_WEEK: i64 = 7;
const SECONDS_PER_ONE_WEEK: i64 = crate::date_utils::SECONDS_PER_DAY * DAYS_PER_WEEK;

pub async fn token_validator(
    req: ServiceRequest,
    credentials: BearerAuth,
) -> Result<ServiceRequest, (Error, ServiceRequest)> {
    let config = req.app_data::<Config>().cloned().unwrap_or_default();
    let decoding_key = req
        .app_data::<actix_web::web::Data<DecodingKey>>()
        .cloned()
        .expect("Decoding key not found");

    match validate_token(credentials.token(), &decoding_key) {
        Ok(token_data) => {
            // Ensure only access token is validated
            if token_data.claims.is_access() {
                Ok(req)
            } else {
                Err((AuthenticationError::from(config).into(), req))
            }
        }
        Err(_) => Err((AuthenticationError::from(config).into(), req)),
    }
}

pub fn validate_token(
    token: &str,
    decoding_key: &DecodingKey,
) -> Result<TokenData<UserClaims>, ValidationError> {
    let mut validation: Validation = Validation::new(Algorithm::HS256);
    validation.set_issuer(&[String::from(ISSUER)]);
    validation.set_required_spec_claims(&["iss", "sub", "iat", "exp", "jti"]);

    jsonwebtoken::decode::<UserClaims>(token, decoding_key, &validation).map_err(|err| {
        log::error!("Error decoding JWT. - {}", err.to_string());
        ValidationError()
    })
}

pub fn generate_token_response(
    user_id: &str,
    encoding_key: &EncodingKey,
) -> Result<TokenResponse, TokenErrors> {
    let access_token_claims = create_access_token_claims(user_id);
    let refresh_token_claims = create_refresh_token_claims(user_id, &access_token_claims.jti);

    let access_token = generate_token(&access_token_claims, encoding_key)
        .map_err(|_| TokenErrors::UnknownError(String::from("Access token generation error.")))?;
    let refresh_token = generate_token(&refresh_token_claims, encoding_key)
        .map_err(|_| TokenErrors::UnknownError(String::from("Refresh token generation error.")))?;
    Ok(TokenResponse {
        access_token,
        refresh_token,
        token_type: String::from(TOKEN_TYPE_BEARER),
        expires_in: access_token_claims.exp,
    })
}

fn create_access_token_claims(user_id: &str) -> UserClaims {
    create_token_claims(user_id, crate::date_utils::SECONDS_PER_DAY, None)
}

fn create_refresh_token_claims(user_id: &str, access_token_id: &str) -> UserClaims {
    create_token_claims(
        user_id,
        SECONDS_PER_ONE_WEEK,
        Some(String::from(access_token_id)),
    )
}

fn create_token_claims(
    user_id: &str,
    expiry_seconds: i64,
    access_token_id: Option<String>,
) -> UserClaims {
    let now = crate::date_utils::now().timestamp();
    UserClaims {
        iss: String::from(ISSUER),
        sub: user_id.to_string(),
        iat: now,
        exp: now + expiry_seconds,
        kid: String::from(KID),
        jti: crate::uuid_utils::new_random_uuid(),
        ati: access_token_id,
    }
}

fn generate_token(
    claims: &UserClaims,
    encoding_key: &EncodingKey,
) -> Result<String, ValidationError> {
    jsonwebtoken::encode(&Header::default(), &claims, encoding_key).map_err(|err| {
        log::error!("Error encodign JWT. - {}", err.to_string());
        ValidationError()
    })
}
