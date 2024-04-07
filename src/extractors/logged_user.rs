use std::{future::Future, pin::Pin};

use actix_web::{Error, FromRequest};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use base64::Engine;

use crate::models::{LoggedUser, UserClaims};

const URL_SAFE_NO_PAD: &base64::engine::GeneralPurpose = &base64::engine::GeneralPurpose::new(
    &base64::alphabet::URL_SAFE,
    base64::engine::general_purpose::NO_PAD,
);

/// Takes the result of a rsplit and ensure we only get 2 parts
/// Errors if we don't
macro_rules! expect_two {
    ($iter:expr) => {{
        let mut i = $iter;
        (i.next().unwrap(), i.next().unwrap())
    }};
}

impl FromRequest for LoggedUser {
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;

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

            let user_id = claims.sub_as_user_id();

            Ok(LoggedUser { id: user_id })
        })
    }
}

fn b64_decode<T: AsRef<[u8]>>(input: T) -> Result<Vec<u8>, base64::DecodeError> {
    URL_SAFE_NO_PAD.decode(input)
}
