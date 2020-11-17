use rocket::http::Status;
use rocket::request::{self, FromRequest, Request};
use rocket::{Outcome, State};
use serde::{Deserialize, Serialize};
use crate::config::AppState;

use jsonwebtoken as jwt;

<<<<<<< HEAD
/// A request guard for JWT authentication
=======
use crate::config;

>>>>>>> 3f190f961927985159b262e44c1ad6ed0b25df13
#[derive(Debug, Deserialize, Serialize)]
pub struct Auth {
    /// timestamp
    pub exp: i64,
    /// user id
    pub id: i32,
    pub username: String,
}

impl Auth {
<<<<<<< HEAD
    /// Generate a token for the Auth
    pub fn token(&self, secret: &jwt::EncodingKey) -> String {
        jwt::encode(&jwt::Header::default(), self, &secret).expect("jwt")
=======
    pub fn token(&self, secret: &[u8]) -> String {
        jwt::encode(&jwt::Header::default(), self, secret).expect("jwt")
>>>>>>> 3f190f961927985159b262e44c1ad6ed0b25df13
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for Auth {
    type Error = ();

    /// Extract Auth token from the "Authorization" header.
    ///
    /// Handlers with Auth guard will fail with 503 error.
    /// Handlers with Option<Auth> will be called with None.
    fn from_request(request: &'a Request<'r>) -> request::Outcome<Auth, Self::Error> {
        let state: State<AppState> = request.guard()?;
        if let Some(auth) = extract_auth_from_request(request, &state.secret) {
            Outcome::Success(auth)
        } else {
            Outcome::Failure((Status::Forbidden, ()))
        }
    }
}

fn extract_auth_from_request(request: &Request, secret: &[u8]) -> Option<Auth> {
    request
        .headers()
        .get_one("authorization")
        .and_then(extract_token_from_header)
        .and_then(|token| decode_token(token, secret))
}

fn extract_token_from_header(header: &str) -> Option<&str> {
    if header.starts_with(config::TOKEN_PREFIX) {
        Some(&header[config::TOKEN_PREFIX.len()..])
    } else {
        None
    }
}

/// Decode token into `Auth` struct. If any error is encountered, log it
/// an return None.
fn decode_token(token: &str, secret: &[u8]) -> Option<Auth> {
    use jwt::{Algorithm, Validation};

    jwt::decode(token, secret, &Validation::new(Algorithm::HS256))
        .map_err(|err| {
            eprintln!("Auth decode error: {:?}", err);
        })
        .ok()
        .map(|token_data| token_data.claims)
}