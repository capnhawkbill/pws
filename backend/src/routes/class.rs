//! The routes to interact with classes

use anyhow::Result;
use rocket::Rocket;

use crate::{
    auth,
    database::{
        generate_id,
        models::{add_to_class, insert_class},
        Class, DbConn, Id,
    },
};

/// Mount all the routes
pub fn mount(rocket: Rocket) -> Rocket {
    rocket.mount(
        "/api/class",
        routes![create_class, join_class_student, join_class_teacher],
    )
}

/// Create a class
#[get("/create?<name>")]
pub fn create_class(name: String, teacher: auth::Teacher, conn: DbConn) -> Result<Id> {
    trace!("Creating class {:?}", name);
    let id = generate_id(&*conn)?;
    let class = Class {
        id: id.clone(),
        name,
        teachers: Vec::new(),
        students: Vec::new(),
        homework: Vec::new(),
    };

    insert_class(&*conn, &class)?;
    add_to_class(&*conn, teacher.id.clone(), id.clone())?;
    Ok(id)
}

/// Make a request to here to get added to a class as a student
#[get("/join?<id>")]
pub fn join_class_student(id: Id, student: auth::Student, conn: DbConn) -> Result<()> {
    add_to_class(&*conn, student.id.clone(), id)?;
    Ok(())
}

/// Make a request to here to get added to a class as a teacher
#[get("/join?<id>", rank = 2)]
pub fn join_class_teacher(id: Id, teacher: auth::Teacher, conn: DbConn) -> Result<()> {
    add_to_class(&*conn, teacher.id.clone(), id)?;
    Ok(())
}

// add to class, request a student or teacher to come into the class
// creator for below link
// Link to get put into a class
