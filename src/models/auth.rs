use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

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
    pub sub: Uuid,
    // Issued at
    pub iat: i64,
    // Expiration time
    pub exp: i64,
    // Key id
    pub kid: Uuid,
    // JWT id
    pub jti: Uuid,
    // Refresh id
    pub ati: Option<Uuid>,
}

impl UserClaims {
    pub fn is_access(&self) -> bool {
        self.ati.is_none()
    }

    pub fn is_refresh(&self) -> bool {
        !self.is_access()
    }

    pub fn sub_as_user_id(&self) -> Uuid {
        self.sub
    }
}

pub struct LoggedUser {
    pub id: Uuid,
}
