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
    rocket.mount("/api", routes![create_class])
}

/// Create a class
#[get("/class/create?<name>")]
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

/// Make a request to here to get added to a class as a student
#[get("/class/join?<class>")]
pub fn join_class_student(class: Id, student: auth::Student, conn: DbConn) -> Result<()> {
    add_to_class(&*conn, student.id.clone(), class)?;
    Ok(())
}

/// Make a request to here to get added to a class as a teacher
#[get("/class/join?<class>", rank = 2)]
pub fn join_class_teacher(class: Id, teacher: auth::Teacher, conn: DbConn) -> Result<()> {
    add_to_class(&*conn, teacher.id.clone(), class)?;
    Ok(())
}

// add to class, request a student or teacher to come into the class
// creator for below link
// Link to get put into a class
