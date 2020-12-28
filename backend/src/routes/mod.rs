//! All the routes that form the api

pub mod student;
pub mod teacher;

use serde::Deserialize;

/// The credentials that are received as json
#[derive(Deserialize)]
pub struct Credentials {
    username: String,
    password: String,
}

/// A struct from a student to send as a response
/// Checks should make sure that classes can only be accessed
/// when the user is in it
#[derive(Serialize)]
struct SafeStudent {
    name: String,
    classes: Vec<Id>,
    badges: Vec<Id>,
}

impl From<Student> for SafeStudent {
    fn from(student: Student) -> Self {
        SafeStudent {
            name: student.name,
            classes: student.classes,
            badges: student.badges,
        }
    }
}
