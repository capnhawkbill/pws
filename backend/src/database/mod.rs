//! This module contains everything concerning the database
use anyhow::{anyhow, Result};
use csv::{Reader, Writer};
use rocket_contrib::databases::rusqlite;

use crate::auth::User;
pub mod models;
use models::{get_teacher_by_name, get_student_by_name};

#[database("sqlite_database")]
pub struct DbConn(rusqlite::Connection);

/// Get a user from the database using username and password
// TODO make this work with a single query
// TODO right now this doesn't handle name collisions
pub fn login(conn: &rusqlite::Connection, name: &str, password: &str) -> Result<User> {
    if let Ok(student) = get_student_by_name(&conn, name) {
        if student.password == password {
            Err(anyhow!("No user with this username and password"))
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
