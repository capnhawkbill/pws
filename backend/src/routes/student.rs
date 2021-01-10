//! Routes for students
//! All these require you to be authenticated as a student
//! They are all prefixed with "/student"
use anyhow::Result;
use rocket::Rocket;
use rocket_contrib::json::Json;

use super::{Credentials, SafeStudent};
use crate::auth::{self, User};
use crate::database::DbConn;
use crate::database::{
    generate_id,
    models::{get_student, remove_student},
    signup, Id, Student,
};

/// Mount all the routes
pub fn mount(rocket: Rocket) -> Rocket {
    rocket.mount(
        "/api/student",
        routes![signup_route, student, id_student, info],
    )
}

/// Signup
#[post("/signup", format = "json", data = "<credentials>")]
fn signup_route(conn: DbConn, credentials: Json<Credentials>) -> Result<Id> {
    let id = generate_id(&*conn)?;

    // type declarations
    let classes: Vec<Id> = Vec::new();
    let badges: Vec<Id> = Vec::new();
    let homework: Vec<Id> = Vec::new();

    let student = Student {
        id: id.clone(),
        name: credentials.username.clone(),
        password: credentials.password.clone(),
        classes,
        badges,
        homework,
    };

    signup(&*conn, &User::Student(student))?;

    Ok(id)
}

#[get("/remove")]
fn remove(conn: DbConn, student: auth::Student) -> Result<()> {
    remove_student(&*conn, (*student).id.clone())?;
    Ok(())
}

#[get("/info")]
fn info(student: auth::Student) -> Json<SafeStudent> {
    Json((*student).clone().into())
}

/// Look up a student with the id
#[get("/id?<id>")]
fn id_student(id: Id, conn: DbConn, _student: auth::Student) -> Result<Json<SafeStudent>> {
    Ok(Json(get_student(&*conn, id)?.into()))
}

/// A test route
#[get("/student")]
fn student(student: auth::Student) -> String {
    let name = &(*student).name;
    format!("Hello student {}", name)
}
