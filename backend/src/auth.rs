//! This module contains the request guards for logging in.
//!
//! It has four guards:
//! + User if you don't care about permission
//! + Student, Teacher or Admin if you do

use anyhow::Result;
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome, Request};
use std::ops::{Deref, DerefMut};

use crate::database::{login, models, DbConn};

/// This is a request guard for logging in as any user
#[derive(Clone, Debug)]
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
#[derive(Clone, Debug)]
pub struct Student(models::Student);

impl Deref for Student {
    type Target = models::Student;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Student {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

/// This is a request guard for logging in as a teacher
/// It is a wrapper for the struct from the database
#[derive(Clone, Debug)]
pub struct Teacher(models::Teacher);

impl Deref for Teacher {
    type Target = models::Teacher;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Teacher {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
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

fn get_user(req: &'_ Request<'_>) -> Result<User> {
    trace!("getting user");
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
// NOTE it currently gets the whole header i don't know if this is a problem
fn check_value(value: &[u8], conn: DbConn) -> Result<User> {
    trace!("Checking value");
    // Decode base64
    let decoded = base64::decode(value)?;
    let mut value = decoded.split(|x| *x == ":".as_bytes()[0]);

    if value.clone().count() != 2 {
        return Err(LoginError::Format.into());
    }

    // Unwraps should be save because of the check above
    let (username, password) = (value.next().unwrap(), value.next().unwrap());

    // parse the byte arrays
    let username = String::from_utf8(username.to_vec())?;
    let password = String::from_utf8(password.to_vec())?;

    // login
    Ok(login(&*conn, &username, &password)?)
}
