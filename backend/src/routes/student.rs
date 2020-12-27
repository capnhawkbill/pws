//! Routes for students
//! All these require you to be authenticated as a student
//! They are all prefixed with "/student"
use anyhow::Result;
use rocket_contrib::json::Json;
use rocket::Rocket;

use super::Credentials;
use crate::auth::{self, User};
use crate::database::{self, models::Student, Id};
use crate::database::DbConn;

/// Mount all the routes
pub fn mount(rocket: Rocket) -> Rocket {
    rocket.mount("/api/student", routes![signup, student])
}

/// Signup
#[post("/signup", format = "json", data = "<credentials>")]
fn signup(conn: DbConn, credentials: Json<Credentials>) -> Result<Id> {
    let id = database::generate_id()?;

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

    database::signup(&*conn, &User::Student(student))?;

    Ok(id)
}

/// A test route
#[get("/student")]
fn student(student: auth::Student) -> String {
    let name = &(*student).name;
    format!("Hello student {}", name)
}
