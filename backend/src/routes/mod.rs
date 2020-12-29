//! All the routes that form the api
// TODO Make a error handler for the routes returning Result<T>
pub mod student;
pub mod teacher;
pub mod class;
pub mod homework;

use serde::Deserialize;

use crate::database::{Id, Student, Teacher};

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
}

impl From<Teacher> for SafeTeacher {
    fn from(teacher: Teacher) -> Self {
        SafeTeacher {
            name: teacher.name,
            classes: teacher.classes,
        }
    }
}
