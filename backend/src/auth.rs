use crate::database::DbConn;
use anyhow::Result;
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome, Request, State};

use crate::database::models;

/// This is a request guard for logging in as any user
pub enum User {
    /// A student
    Student(models::Student),
    /// A teacher
    Teacher(models::Teacher),
    // A admin
    //Admin(models::Admin),
}

/// This is a request guard for logging in as a user
/// It is a wrapper for the struct from the database
pub struct Student(models::Student);

/// This is a request guard for logging in as a teacher
/// It is a wrapper for the struct from the database
pub struct Teacher(models::Teacher);

/// This is a request guard for logging in as a admin
/// It is a wrapper for the struct from the database
//pub struct Admin(crate::database::models::Admin);

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

impl std::fmt::Display for LoginError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let text = match self {
            LoginError::Invalid => "Invalid username or password",
            LoginError::Database => "Database error",
            LoginError::Format => "Incorrect format",
            LoginError::Missing => "Missing username or password",
            LoginError::Permission => "Insufficient permission",
        };
        write!(f, "{}", text)
    }
}
impl std::error::Error for LoginError {}

// /// Verify that the user is an admin
// impl<'a, 'r> FromRequest<'a, 'r> for Admin {
//     type Error = LoginError;
//     fn from_request(req: &'a Request<'r>) -> Outcome<Self, Self::Error> {
//         match get_user(req) {
//             Ok(User::Admin(a)) => Outcome::Success(a),
//             Ok(_) => Outcome::Failure((Status::BadRequest, LoginError::Permission)),
//             Err(e) => Outcome::Failure((Status::BadRequest, e)),
//         }
//     }
// }

/// Verify that the user is a teacher
impl<'a, 'r> FromRequest<'a, 'r> for Teacher {
    type Error = anyhow::Error;
    fn from_request(req: &'a Request<'r>) -> Outcome<Self, Self::Error> {
        match get_user(req) {
            Ok(User::Teacher(t)) => Outcome::Success(Teacher(t)),
            Ok(_) => Outcome::Failure((Status::BadRequest, LoginError::Permission.into())),
            Err(e) => Outcome::Failure((Status::BadRequest, e)),
        }
    }
}

/// Verify that the user is a student
impl<'a, 'r> FromRequest<'a, 'r> for Student {
    type Error = anyhow::Error;
    fn from_request(req: &'a Request<'r>) -> Outcome<Self, Self::Error> {
        match get_user(req) {
            Ok(User::Student(s)) => Outcome::Success(Student(s)),
            Ok(_) => Outcome::Failure((Status::BadRequest, LoginError::Permission.into())),
            Err(e) => Outcome::Failure((Status::BadRequest, e)),
        }
    }
}

/// Verify that the user is authenticated
impl<'a, 'r> FromRequest<'a, 'r> for User {
    type Error = anyhow::Error;
    fn from_request(req: &'a Request<'r>) -> Outcome<Self, Self::Error> {
        match get_user(req) {
            Ok(r) => Outcome::Success(r),
            Err(err) => Outcome::Failure((Status::BadRequest, err)),
        }
    }
}

pub fn get_user<'a, 'r>(req: &'a Request<'r>) -> Result<User> {
    // Retrieve header
    let header: Vec<_> = req.headers().get("Authorization").collect();

    // Retrieve database
    let db = match req.guard::<DbConn>() {
        Outcome::Success(v) => Ok(v),
        _ => Err(LoginError::Database),
    }?;

    // Check for the correct amount
    if header.len() < 1 {
        return Err(LoginError::Missing.into());
    } else if header.len() > 1 {
        return Err(LoginError::Format.into());
    }

    Ok(check_value(header[0].as_bytes(), db)?)
}

/// Verify a base64 encoded username and password pair
/// It should be in the format "username:password"
// TODO it currently gets the whole header i don't know if this is a problem
pub fn check_value(value: &[u8], db: DbConn) -> Result<User> {
    let decoded =  base64::decode(value)?;
    let mut value = decoded.split(|x| *x == ":".as_bytes()[0]);

    if value.clone().count() != 2 {
        return Err(LoginError::Format.into());
    }

    // Unwraps should be save because of the check above
    let (username, password) = (value.next().unwrap(), value.next().unwrap());

    // TODO Get the stuff from the database

    // TODO Check them against eachother
    // TODO Return the correct variant

    todo![];
}

#[cfg(test)]
mod tests {
    use super::*;
    use rusqlite::Connection;

    fn custom_database() -> DbConn {
        let db = Connection::open_in_memory().unwrap();
        // TODO put stuff in the database
        DbConn(db)
    }

    #[test]
    fn test_check_value() {
        let db = custom_database();
        check_value(&[], db).unwrap();
    }
}
