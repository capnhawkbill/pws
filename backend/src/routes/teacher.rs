//! Routes for teachers
//! All these require you to be authenticated as a teacher except the signup route
//! They are all prefixed with "/teacher"

use anyhow::Result;
use rocket::Rocket;
use rocket_contrib::json::Json;

use super::{Credentials, SafeStudent, SafeTeacher};
use crate::auth::{self, User};
use crate::database::DbConn;
use crate::database::{
    generate_id,
    models::{get_student, remove_teacher},
    signup, Id, Teacher,
};

/// Mount all the routes
pub fn mount(rocket: Rocket) -> Rocket {
    rocket.mount(
        "/api/teacher",
        routes![signup_route, teacher, info, id_teacher],
    )
}

/// Signup
#[post("/signup", format = "json", data = "<credentials>")]
fn signup_route(conn: DbConn, credentials: Json<Credentials>) -> Result<Id> {
    let id = generate_id(&*conn)?;

    // type declarations
    let classes: Vec<Id> = Vec::new();
    // TODO default badges
    let badges: Vec<Id> = Vec::new();

    let teacher = Teacher {
        id: id.clone(),
        name: credentials.username.clone(),
        password: credentials.password.clone(),
        classes,
        badges,
    };

    signup(&*conn, &User::Teacher(teacher))?;

    Ok(id)
}

#[get("/remove")]
fn remove(conn: DbConn, teacher: auth::Teacher) -> Result<()> {
    remove_teacher(&*conn, (*teacher).id.clone())?;
    Ok(())
}

#[get("/info")]
fn info(teacher: auth::Teacher) -> Json<SafeTeacher> {
    Json((*teacher).clone().into())
}

// TODO Teacher lookup
/// Look up a student with the id
#[get("/id?<id>", rank = 2)]
fn id_teacher(id: Id, conn: DbConn, _teacher: auth::Teacher) -> Result<Json<SafeStudent>> {
    Ok(Json(get_student(&*conn, id)?.into()))
}
