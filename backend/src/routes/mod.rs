//! All the routes that form the api
// TODO Make a error handler for the routes returning Result<T>
pub mod badge;
pub mod class;
pub mod homework;
pub mod student;
pub mod teacher;
pub mod frontend;

use serde::Deserialize;

use crate::database::{models, Id, Student, Teacher};

/// The credentials that are received as json
#[derive(Deserialize)]
pub struct Credentials {
    username: String,
    password: String,
}

/// A struct from a student to send as a response
/// It doesn't contain the password
#[derive(Serialize)]
pub struct SafeStudent {
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

/// A struct from a teacher to send as a response
/// It doesn't contain the password
#[derive(Serialize)]
pub struct SafeTeacher {
    name: String,
    classes: Vec<Id>,
    badges: Vec<Id>,
}

impl From<Teacher> for SafeTeacher {
    fn from(teacher: Teacher) -> Self {
        SafeTeacher {
            name: teacher.name,
            classes: teacher.classes,
            badges: teacher.badges,
        }
    }
}

/// A badge to get send and receive
#[derive(Deserialize, Serialize)]
pub struct Badge {
    /// name of the badge
    pub name: String,
    /// description of the badge
    pub description: String,
}

impl From<models::Badge> for Badge {
    fn from(badge: models::Badge) -> Self {
        Badge {
            name: badge.name,
            description: badge.description,
        }
    }
}
