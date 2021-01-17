//! Routes for getting and editing homework

use anyhow::{anyhow, Result};
use rocket::Rocket;
use rocket_contrib::json::Json;

use super::Homework;
use crate::auth;
use crate::database::{generate_id, models, DbConn, Id};

/// Mount all the routes
pub fn mount(rocket: Rocket) -> Rocket {
    rocket.mount(
        "/api/homework",
        routes![
            add_homework,
            finished_homework,
            unfinish_homework,
            get_homework,
            get_homework_class,
            get_homework_id,
            remove_homework
        ],
    )
}

/// Add homework to a class as a teacher
#[post("/add?<class>", format = "json", data = "<homework>")]
pub fn add_homework(
    conn: DbConn,
    homework: Json<Homework>,
    teacher: auth::Teacher,
    class: Id,
) -> Result<()> {
    // Check if the theacher is theacher from that class
    if !(*teacher).classes.contains(&class) {
        return Err(anyhow!("{:?} is not a teacher of this class", teacher));
    }

    let id = generate_id(&*conn)?;

    let new_homework = models::Homework {
        id,
        name: homework.name.clone(),
        date: homework.date,
        description: homework.description.clone(),
        points: homework.points,
    };

    // Add the homework to a class
    models::add_homework(&*conn, &new_homework.into(), class)?;

    Ok(())
}

/// Remove homework from a class as a teacher
#[get("/remove?<class>&<homework>")]
pub fn remove_homework(
    conn: DbConn,
    homework: Id,
    teacher: auth::Teacher,
    class: Id,
) -> Result<()> {
    // Check if the theacher is theacher from that class
    if !(*teacher).classes.contains(&class) {
        return Err(anyhow!("{:?} is not a teacher of this class", teacher));
    }
    // TODO check homework ownership

    // Remove the homework from a class
    models::remove_homework(&*conn, homework, class)?;

    Ok(())
}

/// Get all the homework from a student unsorted
#[get("/get", rank = 3)]
pub fn get_homework(conn: DbConn, student: auth::Student) -> Result<Json<Vec<Id>>> {
    // Check if the student is student in that class
    let mut homework = Vec::new();
    for class in student.classes.iter() {
        // FIXME HACK the first item of classes and badges is empty
        // this is probably because of the parsing this is a fast hack
        if class.is_empty() {
            continue;
        }
        homework.append(&mut models::get_class(&*conn, class.to_string())?.homework)
    }
    Ok(Json(homework))
}

/// Get all the homework from a class
#[get("/get?<class>")]
pub fn get_homework_class(
    conn: DbConn,
    student: auth::Student,
    class: Id,
) -> Result<Json<Vec<Id>>> {
    // Check if the student is student in that class
    if !(*student).classes.contains(&class) {
        return Err(anyhow!("{:?} is not a student in this class", student));
    }

    Ok(Json(models::get_class(&*conn, class)?.homework))
}

/// Get the homework with this id
#[get("/get?<id>", rank = 2)]
pub fn get_homework_id(
    conn: DbConn,
    student: auth::Student,
    id: Id,
) -> Result<Json<models::Homework>> {
    let hw = models::get_homework(&*conn, id)?;
    Ok(Json(hw))
}

/// Mark homework as finished
#[get("/done?<class>&<homework>")]
pub fn finished_homework(
    conn: DbConn,
    student: auth::Student,
    class: Id,
    homework: Id,
) -> Result<()> {
    if !(*student).classes.contains(&class) {
        return Err(anyhow!("{:?} is not a student in this class", student));
    }
    let hw = models::get_homework(&*conn, homework)?;

    models::finish_homework(&*conn, (*student).id.clone(), &hw)?;

    Ok(())
}

/// Unmark homework as finished
#[get("/undone?<class>&<homework>")]
pub fn unfinish_homework(
    conn: DbConn,
    student: auth::Student,
    class: Id,
    homework: Id,
) -> Result<()> {
    if !(*student).classes.contains(&class) {
        return Err(anyhow!("{:?} is not a student in this class", student));
    }
    let hw = models::get_homework(&*conn, homework)?;

    models::unfinish_homework(&*conn, (*student).id.clone(), &hw)?;

    Ok(())
}
