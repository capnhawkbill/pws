//! Routes for teachers
//! All these require you to be authenticated as a teacher
//! They are all prefixed with "/teacher"

use anyhow::Result;
use rocket_contrib::json::Json;
use rocket::Rocket;

use super::Credentials;
use crate::auth::{self, User};
use crate::database::{self, models::Teacher, Id};
use crate::database::DbConn;

/// Mount all the routes
pub fn mount(rocket: Rocket) -> Rocket {
    rocket.mount("/api/teacher", routes![signup, teacher])
}

/// Signup
#[post("/signup", format = "json", data = "<credentials>")]
fn signup(conn: DbConn, credentials: Json<Credentials>) -> Result<Id> {
    let id = database::generate_id()?;

    // type declarations
    let classes: Vec<Id> = Vec::new();

    let teacher = Teacher {
        id: id.clone(),
        name: credentials.username.clone(),
        password: credentials.password.clone(),
        classes,
    };

    database::signup(&*conn, &User::Teacher(teacher))?;

    Ok(id)
}

/// A test route
#[get("/teacher")]
fn teacher(teacher: auth::Teacher) -> String {
    let name = &(*teacher).name;
    format!("Hello teacher {}", name)
}
