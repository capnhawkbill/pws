//! All the routes that form the api
// TODO Make a error handler for the routes returning Result<T>
pub mod badge;
pub mod class;
pub mod homework;
pub mod student;
pub mod teacher;

use chrono::NaiveDate;
use serde::Deserialize;

use crate::database::{models, Id, Student, Teacher};

/// Leaderboard for a group of students
#[derive(Deserialize, Serialize)]
pub struct LeaderBoard(Vec<LeaderStudent>);
impl From<Vec<Student>> for LeaderBoard {
    fn from(f: Vec<Student>) -> Self {
        let mut board: Vec<LeaderStudent> = f.iter().map(|x| x.into()).collect();
        board.sort_unstable_by(|a, b| a.points.partial_cmp(&b.points).unwrap());
        LeaderBoard(board)
    }
}

/// Student struct for in the leaderboard
#[derive(Deserialize, Serialize)]
pub struct LeaderStudent {
    name: String,
    points: i32,
}

impl From<&Student> for LeaderStudent {
    fn from(f: &Student) -> Self {
        // TODO Clones
        LeaderStudent {
            name: f.name.clone(),
            points: f.points.clone(),
        }
    }
}

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
    homework: Vec<Id>,
    points: i32,
}

impl From<Student> for SafeStudent {
    fn from(student: Student) -> Self {
        SafeStudent {
            name: student.name,
            classes: student.classes,
            badges: student.badges,
            homework: student.homework,
            points: student.points,
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

/// Homework to receive
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Homework {
    /// Name of the homework
    pub name: String,
    /// Description of the homework
    pub description: String,
    /// Date of the homework
    pub date: NaiveDate,
    /// Points of the homework
    pub points: i32,
}

impl From<crate::database::Homework> for Homework {
    fn from(f: crate::database::Homework) -> Homework {
        Homework {
            name: f.name,
            description: f.description,
            date: f.date,
            points: f.points,
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
