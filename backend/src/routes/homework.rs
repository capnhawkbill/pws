//! Routes for getting and editing homework

use anyhow::{anyhow, Result};
use rocket::Rocket;
use rocket_contrib::json::Json;

use crate::database::{DbConn, Id, Homework, models};
use crate::auth;

/// Mount all the routes
pub fn mount(rocket: Rocket) -> Rocket {
    rocket.mount("/api/homework", routes![add_homework])
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
