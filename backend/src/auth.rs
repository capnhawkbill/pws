use anyhow::Result;
use rocket::request::{self, Request, FromRequest, Outcome};
use rocket::http::Status;

use super::models;

// TODO Nonsense
type Database = bool;

/// This is a request guard for logging in as any user
pub enum User {
    Student(models::Student),
    Teacher(models::Teacher),
    Admin(models::Admin),
}


/// This is a request guard for logging in as a user
pub struct Student {}

/// This is a request guard for logging in as a teacher
pub struct Teacher {}

/// This is a request guard for logging in as a admin
pub struct Admin {}

/// The error type for logging in
#[derive(Debug)]
pub enum LoginError {
    /// Invalid password or username
    Invalid,
    /// Database error
    Database,
    /// Invalid format
    Format,
    /// Password or username missing
    Missing,
    /// Invalid permissions
    Permission,
}

/// Verify that the user is an admin
impl<'a, 'r> FromRequest<'a, 'r> for Admin {
    type Error = LoginError;
    fn from_request(req: &'a Request<'r>) -> Outcome<Self, Self::Error> {
        // TODO get the database
        //let db = req.guard::
        let db = true;

        // Retrieve header
        let header: Vec<_> = req.headers().get("Authorization").collect();
        // Check for the correct amount
        if header < 1 {
            return Outcome::Failure((Status::BadRequest, LoginError::Missing))
        } else if header > 1 {
            return Outcome::Failure((Status::BadRequest, LoginError::Format))
        }

        let user = check_value(header, db).map_err(|err|{
            Outcome::Failure((Status::BadRequest, err))
        })?;

        match user {
            User::Admin(a) => Outcome::Success(a),
            _ => Outcome::Failure((Status::BadRequest, LoginError::Permission))
        }
    }
}

/// Verify that the user is a teacher
impl<'a, 'r> FromRequest<'a, 'r> for Teacher {
    type Error = LoginError;
    fn from_request(req: &'a Request<'r>) -> Outcome<Self, Self::Error> {
        // TODO get the database
        //let db = req.guard::
        let db = true;

        // Retrieve header
        let header: Vec<_> = req.headers().get("Authorization").collect();
        // Check for the correct amount
        if header < 1 {
            return Outcome::Failure((Status::BadRequest, LoginError::Missing))
        } else if header > 1 {
            return Outcome::Failure((Status::BadRequest, LoginError::Format))
        }

        let user = check_value(header, db).map_err(|err|{
            Outcome::Failure((Status::BadRequest, err))
        })?;

        match user {
            User::Teacher(t) => Outcome::Success(t),
            _ => Outcome::Failure((Status::BadRequest, LoginError::Permission))
        }
    }
}

/// Verify that the user is a student
impl<'a, 'r> FromRequest<'a, 'r> for Student {
    type Error = LoginError;
    fn from_request(req: &'a Request<'r>) -> Outcome<Self, Self::Error> {
        // TODO get the database
        //let db = req.guard::
        let db = true;

        // Retrieve header
        let header: Vec<_> = req.headers().get("Authorization").collect();
        // Check for the correct amount
        if header < 1 {
            return Outcome::Failure((Status::BadRequest, LoginError::Missing))
        } else if header > 1 {
            return Outcome::Failure((Status::BadRequest, LoginError::Format))
        }

        let user = check_value(header, db).map_err(|err|{
            Outcome::Failure((Status::BadRequest, err))
        })?;

        match user {
            User::Student(s) => Outcome::Success(s),
            _ => Outcome::Failure((Status::BadRequest, LoginError::Permission))
        }
    }
}

/// Verify that the user is authenticated
impl<'a, 'r> FromRequest<'a, 'r> for User {
    type Error = LoginError;
    fn from_request(req: &'a Request<'r>) -> Outcome<Self, Self::Error> {
        // TODO get the database
        //let db = req.guard::
        let db = true;

        // Retrieve header
        let header: Vec<_> = req.headers().get("Authorization").collect();
        // Check for the correct amount
        if header < 1 {
            return Outcome::Failure((Status::BadRequest, LoginError::Missing))
        } else if header > 1 {
            return Outcome::Failure((Status::BadRequest, LoginError::Format))
        }

        let user = check_value(header, db).map_err(|err|{
            Outcome::Failure((Status::BadRequest, err))
        })?;

        Outcome::Success(user)

    }
}

/// Verify a base64 encoded username and password pair
/// It should be in the format "username:password"
// TODO it currently gets the whole header i don't know if this is a problem
pub fn check_value(value: &[u8], db: Database) -> std::result::Result<User, LoginError> {
    let value = base64::decode(value).split(|x| x == ':');

    if value.clone().count() != 2 {
        return Err(LoginError::Format)
    }

    // Unwraps should be save because of the check above
    let (username, password) = (value.next().unwrap(), value.next().unwrap());

    // TODO Get the stuff from the database

    // TODO Check them against eachother
    // TODO Return the correct variant

    todo![];
}
