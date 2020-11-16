//! The request guards using the api key

use crate::database::models::User;
use crate::database::{self, UserDataBase};
use crate::login::permission::Permission;
use rocket::{
    http::Status,
    request::{self, FromRequest},
    Outcome, Request, State,
};

#[derive(Debug)]
enum ApiKeyError {
    BadCount,
    Missing,
    Invalid,
}

/// Request guard to verify someone is a student
pub struct Student(User);

impl<'a, 'r> FromRequest<'a, 'r> for Student {
    type Error = ApiKeyError;

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        let key = request.headers().get("api-key").collect();
        let conn = request.guard::<State<UserDataBase>>();
        match key.len() {
            // no key
            0 => Outcome::Failure((Status::BadRequest, ApiKeyError::Missing)),
            // valid key
            1 => {
                if let Ok(user) = database::get_user_by_key(&conn, key) {
                    if user.permission == Permission::Student {
                        Outcome::Success(Student(user))
                    } else {
                        Outcome::Failure((Status::InternalServerError, ApiKeyError::Invalid))
                    }
                } else {
                    Outcome::Failure((Status::InternalServerError, ApiKeyError::Invalid))
                }
                Outcome::Success(Student)
            }
            // invalid key
            1 => Outcome::Forward((Status::BadRequest, ApiKeyError::Invalid)),
            // too much keys
            _ => Outcome::Failure((Status::BadRequest, ApiKeyError::BadCount)),
        }
    }
}
