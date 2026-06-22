use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

/// OAuth token request
#[derive(Deserialize, ToSchema)]
pub struct TokenRequest {
    /// Grant type
    pub grant_type: GrantType,
    /// Username (used in `password` grant type)
    pub username: Option<String>,
    /// Password (used in `password` grant type)
    pub password: Option<String>,
    /// Refresh token (used in `refresh_token` grant type)
    pub refresh_token: Option<String>,
}

/// OAuth grant type
#[derive(Clone, Deserialize, ToSchema)]
pub enum GrantType {
    /// Password grant
    #[serde(rename = "password")]
    Password,
    /// Refresh token grant
    #[serde(rename = "refresh_token")]
    RefreshToken,
}

/// OAuth token response
#[derive(Serialize, ToSchema)]
pub struct TokenResponse {
    /// Access token
    pub access_token: String,
    /// Refresh token
    pub refresh_token: String,
    /// Topke type
    pub token_type: String,
    /// Expires in
    pub expires_in: i64,
}

#[derive(Serialize, Deserialize)]
pub struct UserClaims {
    /// Issuer
    pub iss: String,
    /// Subject
    pub sub: Uuid,
    /// Issued at
    pub iat: i64,
    /// Expiration time
    pub exp: i64,
    /// Key id
    pub kid: Uuid,
    /// JWT id
    pub jti: Uuid,
    /// Refresh id
    pub ati: Option<Uuid>,
}

impl UserClaims {
    pub fn is_access(&self) -> bool {
        self.ati.is_none()
    }

    pub fn is_refresh(&self) -> bool {
        !self.is_access()
    }
}

pub struct LoggedUser {
    pub id: Uuid,
}
