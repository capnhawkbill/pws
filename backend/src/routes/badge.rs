//! Routes for getting and creating badges
//! All prefixed with "/badge"

use super::Badge;
use crate::{
    auth,
    database::{
        generate_id,
        models::{self, award_badge, insert_badge},
        DbConn, Id,
    },
};

use anyhow::{anyhow, Result};
use rocket::Rocket;
use rocket_contrib::json::Json;

/// Mount all the routes
pub fn mount(rocket: Rocket) -> Rocket {
    rocket.mount(
        "/api/badge",
        routes![award, create_badge, get_badge_student, get_badge_teacher],
    )
}

/// create a badge
#[post("/create", format = "json", data = "<badge>")]
pub fn create_badge(conn: DbConn, _teacher: auth::Teacher, badge: Json<Badge>) -> Result<Id> {
    let id = generate_id(&*conn)?;
    let badge = models::Badge {
        id: id.clone(),
        name: badge.name.clone(),
        description: badge.description.clone(),
        official: false,
    };

    insert_badge(&*conn, &badge)?;

    Ok(id)
}

/// get a badge student
#[get("/get?<id>")]
pub fn get_badge_student(
    conn: DbConn,
    student: auth::Student,
    id: Id,
) -> Result<Json<models::Badge>> {
    if !(*student).badges.contains(&id) {
        return Err(anyhow!("Student {:?} doesn't own badge {}", *student, id));
    }
    let badge = models::get_badge(&*conn, id)?;
    Ok(Json(badge))
}

/// get a badge teacher
#[get("/get?<id>", rank = 2)]
pub fn get_badge_teacher(
    conn: DbConn,
    teacher: auth::Teacher,
    id: Id,
) -> Result<Json<models::Badge>> {
    if !(*teacher).badges.contains(&id) {
        return Err(anyhow!("Teacher {:?} doesn't own badge {}", *teacher, id));
    }
    let badge = models::get_badge(&*conn, id)?;
    Ok(Json(badge))
}

/// award a badge
#[get("/award?<student>&<badge>")]
pub fn award(conn: DbConn, _teacher: auth::Teacher, student: Id, badge: Id) -> Result<()> {
    // check if student is in a class of the teacher
    award_badge(&*conn, student, badge)?;
    Ok(())
}
