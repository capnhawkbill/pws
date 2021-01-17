//! The routes to interact with classes

use anyhow::{anyhow, Result};
use rocket::Rocket;
use rocket_contrib::json::Json;

use super::LeaderBoard;
use crate::{
    auth,
    database::{
        generate_id,
        models::{self, add_to_class, insert_class},
        Class, DbConn, Id,
    },
};

/// Mount all the routes
pub fn mount(rocket: Rocket) -> Rocket {
    rocket.mount(
        "/api/class",
        routes![
            create_class,
            get_leaderboard,
            join_class_student,
            join_class_teacher
        ],
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

/// Get the leaderboard from a class
#[get("/leaderboard?<class>")]
pub fn get_leaderboard(
    conn: DbConn,
    student: Option<auth::Student>,
    teacher: Option<auth::Teacher>,
    class: Id,
) -> Result<Json<LeaderBoard>> {
    // Checking
    if let Some(student) = student {
        if !(*student).classes.contains(&class) {
            return Err(anyhow!("{:?} is not a student in this class", student));
        }
    } else if let Some(teacher) = teacher {
        if !(*teacher).classes.contains(&class) {
            return Err(anyhow!("{:?} is not a teacher of this class", teacher));
        }
    } else {
        return Err(anyhow!("No login provided"));
    }

    let class = models::get_class(&*conn, class)?;

    // get all the students
    let mut students = Vec::new();
    for student in class.students {
        let student = models::get_student(&*conn, student)?;
        students.push(student);
    }

    // Convert to leaderboard
    Ok(Json(students.into()))
}
