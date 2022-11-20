use std::{future::Future, pin::Pin};

use actix_web::{Error, FromRequest};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Deserialize, ToSchema)]
pub struct TokenRequest {
    pub grant_type: GrantType,
    pub username: Option<String>,
    pub password: Option<String>,
    pub refresh_token: Option<String>,
}

#[derive(Clone, Deserialize, ToSchema)]
pub enum GrantType {
    #[serde(rename = "password")]
    Password,
    #[serde(rename = "refresh_token")]
    RefreshToken,
}

#[derive(Serialize, ToSchema)]
pub struct TokenResponse {
    pub access_token: String,
    pub refresh_token: String,
    pub token_type: String,
    pub expires_in: i64,
}

#[derive(Serialize, Deserialize)]
pub struct UserClaims {
    // Issuer
    pub iss: String,
    // Subject
    pub sub: String,
    // Issued at
    pub iat: i64,
    // Expiration time
    pub exp: i64,
    // Key id
    pub kid: String,
    // JWT id
    pub jti: String,
    // Refresh id
    pub ati: Option<String>,
}

impl UserClaims {
    pub fn is_access(&self) -> bool {
        self.ati.is_none()
    }

    pub fn is_refresh(&self) -> bool {
        !self.is_access()
    }

    pub fn sub_as_user_id(&self) -> i32 {
        self.sub.parse().unwrap()
    }
}

pub struct LoggedUser {
    pub id: i32,
}

/// Takes the result of a rsplit and ensure we only get 2 parts
/// Errors if we don't
macro_rules! expect_two {
    ($iter:expr) => {{
        let mut i = $iter;
        (i.next().unwrap(), i.next().unwrap())
    }};
}

fn b64_decode<T: AsRef<[u8]>>(input: T) -> Result<Vec<u8>, base64::DecodeError> {
    base64::decode_config(input, base64::URL_SAFE_NO_PAD)
}

impl FromRequest for LoggedUser {
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self, Error>>>>;

    fn from_request(
        req: &actix_web::HttpRequest,
        _payload: &mut actix_web::dev::Payload,
    ) -> Self::Future {
        let request = req.clone();
        Box::pin(async move {
            let bearer_auth = BearerAuth::extract(&request).await.unwrap();
            let token = bearer_auth.token();

            let (_, message) = expect_two!(token.rsplitn(2, '.'));
            let (payload, _) = expect_two!(message.rsplitn(2, '.'));
            let decoded = b64_decode(payload).unwrap();
            let claims: UserClaims = serde_json::from_slice(&decoded).unwrap();

            Ok(LoggedUser {
                id: claims.sub_as_user_id(),
            })
        })
    }
}
