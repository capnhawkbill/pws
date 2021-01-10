//! This module contains everything concerning the database
use anyhow::{anyhow, Result};
use csv::Writer;
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use rocket_contrib::databases::rusqlite;

pub mod models;
pub use models::create_tables;

use crate::auth::User;
use models::{get_student_by_name, get_teacher_by_name, insert_student, insert_teacher};

#[database("sqlite_database")]
pub struct DbConn(rusqlite::Connection);

/// Id for types in the database
/// `TEXT NOT NULL PRIMARY KEY`
pub type Id = String;
pub use models::{Badge, Class, Homework, Student, Teacher};

/// Get a user from the database using username and password
// TODO make this work with a single query
// TODO right now this doesn't handle name collisions
// TODO ugly if tree
pub fn login(conn: &rusqlite::Connection, name: &str, password: &str) -> Result<User> {
    trace!("Logging in {} with password {}", name, password);
    if let Ok(student) = get_student_by_name(&conn, name) {
        trace!("Is student");
        trace!("checking {} against {}", password, student.password);
        if student.password == password {
            trace!("Correct password");
            Ok(User::Student(student))
        } else {
            trace!("incorrect password");
            Err(anyhow!("No user with this username and password"))
        }
    } else if let Ok(teacher) = get_teacher_by_name(&conn, name) {
        trace!("Is teacher");
        if teacher.password == password {
            trace!("Correct password");
            Ok(User::Teacher(teacher))
        } else {
            trace!("incorrect password");
            Err(anyhow!("No user with this username and password"))
        }
    } else {
        trace!("Doesn't exist");
        Err(anyhow!("No user with this username and password"))
    }
}

/// Create a new user and insert it into the database
pub fn signup(conn: &rusqlite::Connection, user: &User) -> Result<()> {
    trace!("Signing up {:?}", user);
    let _r = match user {
        User::Student(student) => insert_student(&conn, &student)?,
        User::Teacher(teacher) => insert_teacher(&conn, &teacher)?,
    };

    Ok(())
}

// TODO a common trait for all the models so common functions are possible
// pub trait Model {
//     fn get(conn: &rusqlite::Connection, id: Id) -> Result<Self>
//     fn insert(&self, conn: &rusqlite::Connection) -> Result<()>
// }
// impl Model for Class {}
// impl Model for Student {}
// impl Model for Teacher {}
// impl Model for Badge {}
//
// fn get_multiple(conn: &rusqlite::Connection, ids: Vec<Id>, f: bool) -> Vec<impl Model>

/// Generates a unique id
pub fn generate_id(conn: &rusqlite::Connection) -> Result<Id> {
    trace!("generating id");
    loop {
        // generate random id
        let id: Vec<u8> = thread_rng().sample_iter(&Alphanumeric).take(50).collect();

        let id = String::from_utf8(id)?;
        trace!("trying {:?}", id);

        // check collision
        //let mut stmt = conn.prepare("SELECT * FROM badge,class,student,teacher");
        // TODO change this to above when those tables get added
        let mut stmt = conn.prepare(
            r#"
SELECT * FROM student,teacher,class,badge
WHERE student.id = ?1 OR teacher.id = ?1 OR class.id = ?1 OR badge.id = ?1"#,
        )?;
        let mut res = stmt.query(&[&id])?;
        if res.next().is_none() {
            trace!("Unique");
            return Ok(id);
        }
        trace!("Collided");
    }
}

/// Make a csv string
fn mkcsv(thing: &[String]) -> Result<String> {
    trace!("writing csv");
    let mut wtr = Writer::from_writer(vec![]);
    wtr.write_record(thing)?;
    wtr.flush()?;
    Ok(String::from_utf8(wtr.into_inner()?)?)
}

/// Parse a csv string
fn getcsv(thing: String) -> Result<Vec<String>> {
    trace!("reading csv");
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_reader(thing.as_bytes());
    if let Some(r) = rdr.records().next() {
        Ok(r?.iter().map(|x| x.to_string()).collect())
    } else {
        Err(anyhow!("No records found"))
    }
}

/// Parse a boolean to a integer
/// Booleans are stored as a integer in sqlite
fn mkbool(thing: bool) -> i32 {
    match thing {
        true => 1,
        false => 0,
    }
}

/// Parse a integer to a boolean
/// Booleans are stored as a integer in sqlite
fn getbool(thing: i32) -> Result<bool> {
    match thing {
        1 => Ok(true),
        0 => Ok(false),
        _ => Err(anyhow!("Incorrect string")),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn init() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[test]
    fn test_csv() {
        init();
        let values: Vec<String> = ["first", "second", "third"]
            .iter()
            .map(|x| x.to_string())
            .collect();

        let csv = mkcsv(&values).unwrap();
        assert_eq!("first,second,third\n", csv);

        let back = getcsv(csv).unwrap();
        assert_eq!(values, back);
    }
}
