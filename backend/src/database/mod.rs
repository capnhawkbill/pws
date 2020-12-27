//! This module contains everything concerning the database
use anyhow::{anyhow, Result};
use csv::{Reader, Writer};
use rocket_contrib::databases::rusqlite;
use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;

use crate::auth::User;
pub mod models;
pub use models::create_tables;
use models::{get_student_by_name, get_teacher_by_name, insert_student, insert_teacher};

#[database("sqlite_database")]
pub struct DbConn(rusqlite::Connection);

/// Id for types in the database
/// `TEXT NOT NULL PRIMARY KEY`
pub type Id = String;

/// Get a user from the database using username and password
// TODO make this work with a single query
// TODO right now this doesn't handle name collisions
pub fn login(conn: &rusqlite::Connection, name: &str, password: &str) -> Result<User> {
    if let Ok(student) = get_student_by_name(&conn, name) {
        if student.password == password {
            Ok(User::Student(student))
        } else {
            Err(anyhow!("No user with this username and password"))
        }
    } else if let Ok(teacher) = get_teacher_by_name(&conn, name) {
        if teacher.password == password {
            Ok(User::Teacher(teacher))
        } else {
            Err(anyhow!("No user with this username and password"))
        }
    } else {
        Err(anyhow!("No user with this username and password"))
    }
}

/// Create a new user and insert it into the database
pub fn signup(conn: &rusqlite::Connection, user: &User) -> Result<()> {
    let r = match user {
        User::Student(student) => insert_student(&conn, &student)?,
        User::Teacher(teacher) => insert_teacher(&conn, &teacher)?,
    };

    Ok(r)
}

/// Generates a unique id
pub fn generate_id(conn: &rusqlite::Connection) -> Result<Id> {
    loop {
    // generate random id
        let id: Vec<u8> = thread_rng()
            .sample_iter(&Alphanumeric)
            .take(50)
            .collect();

        let id = String::from_utf8(id)?;

        // check collision
        //let mut stmt = conn.prepare("SELECT * FROM badge,class,student,teacher");
        // TODO change this to above when those tables get added
        let mut stmt = conn.prepare("SELECT * FROM student,teacher WHERE id = ?1")?;
        let mut res = stmt.query(&[&id])?;
        if !res.next().is_some() {
            return Ok(id)
        }
    }
}

/// Make a csv string
fn mkcsv(thing: &[String]) -> Result<String> {
    let mut wtr = Writer::from_writer(vec![]);
    wtr.write_record(thing)?;
    Ok(String::from_utf8(wtr.into_inner()?)?)
}

/// Parse a csv string
fn getcsv(thing: String) -> Result<Vec<String>> {
    let mut rdr = Reader::from_reader(thing.as_bytes());
    let mut r = Vec::new();
    for record in rdr.records() {
        let record = record?;
        r.push(record.as_slice().to_string())
    }
    Ok(r)
}
