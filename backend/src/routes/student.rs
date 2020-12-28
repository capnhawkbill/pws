//! Routes for students
//! All these require you to be authenticated as a student
//! They are all prefixed with "/student"
use anyhow::Result;
use rocket::Rocket;
use rocket_contrib::json::Json;

use super::Credentials;
use crate::auth::{self, User};
use crate::database::DbConn;
use crate::database::{self, Student, Id, generate_id, signup};

/// Mount all the routes
pub fn mount(rocket: Rocket) -> Rocket {
    rocket.mount("/api/student", routes![signup_route, student])
}

/// Signup
#[post("/signup", format = "json", data = "<credentials>")]
fn signup_route(conn: DbConn, credentials: Json<Credentials>) -> Result<Id> {
    let id = generate_id(&*conn)?;

    // type declarations
    let classes: Vec<Id> = Vec::new();
    let badges: Vec<Id> = Vec::new();

    let student = Student {
        id: id.clone(),
        name: credentials.username.clone(),
        password: credentials.password.clone(),
        classes,
        badges,
    };

    signup(&*conn, &User::Student(student))?;

    Ok(id)
}

#[get("/info")]
fn info(student: auth::Student) -> Json<Student> {
    Json(*student.clone())
}

/// A test route
#[get("/student")]
fn student(student: auth::Student) -> String {
    let name = &(*student).name;
    format!("Hello student {}", name)
}
