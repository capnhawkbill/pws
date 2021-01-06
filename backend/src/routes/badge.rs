use super::Badge;
use crate::{
    database::{
        Class,
        DbConn,
        Id,
        generate_id,
        models::{
            self,
            insert_badge,
        },
    },
    auth,
};

use anyhow::Result;
use rocket::Rocket;
use rocket_contrib::json::Json;

/// Mount all the routes
pub fn mount(rocket: Rocket) -> Rocket {
    rocket.mount("/api/badge", routes![create_badge, get_badge])
}

#[post("/create", format = "json", data = "<badge>")]
pub fn create_badge(conn: DbConn, teacher: auth::Teacher, badge: Json<Badge>) -> Result<()> {
    let id = generate_id(&*conn)?;
    let badge = models::Badge {
        id,
        name: badge.name,
        description: badge.description,
        official: badge.official,
    };

    insert_badge(&*conn, &badge)?;

    Ok(())
}

#[get("/get?<id>")]
pub fn get_badge(conn: DbConn, id: Id) -> Result<Json<models::Badge>> {
    let badge = models::get_badge(&*conn, id)?;
    Ok(Json(badge))
}
