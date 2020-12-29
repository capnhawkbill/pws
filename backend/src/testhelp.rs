//! Functions to help with integration tests
use crate::database;
use rocket_contrib::databases::rusqlite::Connection;
use std::collections::HashMap;
use rocket::config::{Config, Environment, Value};
use rocket::http::{ContentType, Header};
use rocket::local::Client;

use database::{Id, Student, Teacher};

/// Create an empty test database and return a config for it
pub fn init_test_db() -> Config {
    // Create the database
    let path = "test.sqlite";
    let _ = std::fs::remove_file(&path);
    let db = Connection::open(&path).unwrap();
    database::create_tables(&db).unwrap();

    // Create the config
    let mut database_config = HashMap::new();
    let mut databases = HashMap::new();

    database_config.insert("url", Value::from(path));
    databases.insert("sqlite_database", Value::from(database_config));
    Config::build(Environment::Development)
        .extra("databases", databases)
        .finalize()
        .unwrap()
}

/// Initialize logger for tests
pub fn init_logger() {
    let _ = env_logger::builder().is_test(true).try_init();
}

/// Student without password for parsing the return
#[derive(Debug, Deserialize, PartialEq)]
pub struct SafeStudent {
    /// Name of the student
    pub name: String,
    /// Classes of the student
    pub classes: Vec<Id>,
    /// Badges of the student
    pub badges: Vec<Id>,
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

/// Teacher without password for parsing the return
#[derive(Debug, Deserialize, PartialEq)]
pub struct SafeTeacher {
    /// Name of the teacher
    pub name: String,
    /// Classes of the teacher
    pub classes: Vec<Id>,
}

impl From<Teacher> for SafeTeacher {
    fn from(teacher: Teacher) -> Self {
        SafeTeacher {
            name: teacher.name,
            classes: teacher.classes,
        }
    }
}

/// Get your own info student
pub fn get_self_info_student(client: &Client, auth: &'static str) -> SafeStudent {
    let auth = Header::new("Authorization", auth);
    let req = client
        .get("/api/student/info")
        .header(auth);

    let body = &req.dispatch().body_string().unwrap();
    trace!("{}", body);
    serde_json::from_str(&body).unwrap()
}

/// Get your own info teacher
pub fn get_self_info_teacher(client: &Client, auth: &'static str) -> SafeTeacher {
    let auth = Header::new("Authorization", auth);
    let req = client
        .get("/api/teacher/info")
        .header(auth);

    let body = &req.dispatch().body_string().unwrap();
    trace!("{}", body);
    serde_json::from_str(&body).unwrap()
}

/// Get info of id
pub fn get_info(client: &Client, auth: &'static str, lookup: Id) -> SafeStudent {
    let auth = Header::new("Authorization", auth);
    let req = client
        .get(format!("/api/student/id?id={}", lookup))
        .header(auth);

    let body = &req.dispatch().body_string().unwrap();
    trace!("{}", body);
    serde_json::from_str(&body).unwrap()
}

/// Sign up student
pub fn get_id_student(client: &Client, body: &'static str) -> Id {
    // Signup
    let signup = client
        .post("/api/student/signup")
        .body(body)
        .header(ContentType::JSON);

    signup.dispatch().body_string().unwrap()
}

/// Sign up teacher
pub fn get_id_teacher(client: &Client, body: &'static str) -> Id {
    // Signup
    let signup = client
        .post("/api/teacher/signup")
        .body(body)
        .header(ContentType::JSON);

    signup.dispatch().body_string().unwrap()
}
