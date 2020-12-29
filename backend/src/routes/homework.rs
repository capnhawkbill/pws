//! Routes for getting and editing homework

use anyhow::{anyhow, Result};
use rocket::Rocket;
use rocket_contrib::json::Json;

use crate::database::{DbConn, Id, Homework, models};
use crate::auth;

/// Mount all the routes
pub fn mount(rocket: Rocket) -> Rocket {
    rocket.mount("/api/homework", routes![add_homework, get_homework, remove_homework])
}

/// Add homework to a class as a teacher
#[post("/add?<class>", format = "json", data = "<homework>")]
pub fn add_homework(conn: DbConn, homework: Json<Homework>, teacher: auth::Teacher, class: Id) -> Result<()> {
    // Check if the theacher is theacher from that class
    if !(*teacher).classes.contains(&class) {
        return Err(anyhow!("{:?} is not a teacher of this class", teacher))
    }

    // Add the homework to a class
    models::add_homework(&*conn, &*homework, class)?;

    Ok(())
}

/// Remove homework from a class as a teacher
#[post("/remove?<class>", format = "json", data = "<homework>")]
pub fn remove_homework(conn: DbConn, homework: Json<Homework>, teacher: auth::Teacher, class: Id) -> Result<()> {
    // Check if the theacher is theacher from that class
    if !(*teacher).classes.contains(&class) {
        return Err(anyhow!("{:?} is not a teacher of this class", teacher))
    }

    // Remove the homework from a class
    models::remove_homework(&*conn, &*homework, class)?;

    Ok(())
}

/// Get all the homework from a class
#[get("/get?<class>")]
pub fn get_homework(conn: DbConn, student: auth::Student, class: Id) -> Result<Json<Vec<Homework>>> {
    // Check if the student is student in that class
    if !(*student).classes.contains(&class) {
        return Err(anyhow!("{:?} is not a student in this class", student))
    }

    Ok(Json(models::get_class(&*conn, class)?.homework))
}
