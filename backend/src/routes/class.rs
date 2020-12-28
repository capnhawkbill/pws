//! The routes to interact with classes

use rocket::Rocket;
use anyhow::Result;

use crate::{
    database::{
        Class,
        DbConn,
        Id,
        generate_id,
        models::{
            insert_class,
            add_to_class,
        },
    },
    auth,
};

/// Mount all the routes
pub fn mount(rocket: Rocket) -> Rocket {
    rocket.mount("/api/student", routes![])
}

/// Create a class
#[get("/class?<name>")]
pub fn create_class(name: String, teacher: auth::Teacher, conn: DbConn) -> Result<Id> {
    trace!("Creating class {:?}", name);
    let id = generate_id(&*conn)?;
    let class = Class {
        id: id.clone(),
        name,
        teachers: Vec::new(),
        students: Vec::new(),
    };

    insert_class(&*conn, &class)?;
    add_to_class(&*conn, teacher.id.clone(), id.clone())?;
    Ok(id)
}

// add to class, request a student or teacher to come into the class
// creator for below link
// Link to get put into a class
