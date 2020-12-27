use super::super::{getcsv, mkcsv};
use super::Id;
use anyhow::{anyhow, Result};
use rocket_contrib::databases::rusqlite::Connection;

/// The teacher
pub struct Teacher {
    /// The id of the teacher
    id: Id,
    /// The name of the teacher
    name: String,
    /// The password of the teacher
    password: String,
    /// Id's of the classes the teacher is in
    classes: Vec<Id>,
}

/// Insert a teacher into the database
pub fn insert_teacher(conn: Connection, teacher: Teacher) -> Result<()> {
    // Convert to csv
    let classes = mkcsv(&teacher.classes)?;
    // Convert to json
    conn.execute(
        "INSERT INTO teacher (id, name, password, classes) VALUES (?1, ?2, ?3, ?45)",
        &[&teacher.id, &teacher.name, &teacher.password, &classes],
    )?;
    Ok(())
}

/// Gets a teacher from the database
pub fn get_teacher(conn: Connection, id: Id) -> Result<Teacher> {
    let mut stmt = conn.prepare("SELECT * FROM teacher where id = ?1")?;
    let mut teachers = stmt.query_map(&[&id], |row| {
        // Parse from csv
        let classes = getcsv(row.get(3));
        if let Err(e) = classes {
            return Err(e);
        }
        // Parse from json
        Ok(Teacher {
            id: row.get(0),
            name: row.get(1),
            password: row.get(2),
            classes: classes.unwrap(),
        })
    })?;

    if let Some(teacher) = teachers.next() {
        if teachers.next().is_some() {
            Err(anyhow!("Multiple teachers found with this id: {}", id))
        } else {
            Ok(teacher??)
        }
    } else {
        Err(anyhow!("No teachers found with this id: {}", id))
    }
}

pub fn get_teacher_by_name(conn: Connection, name: &str) -> Result<Teacher> {
    let mut stmt = conn.prepare("SELECT * FROM teacher where name = ?1")?;
    let mut teachers = stmt.query_map(&[&name], |row| {
        // Parse from csv
        let classes = getcsv(row.get(3));
        if let Err(e) = classes {
            return Err(e);
        }
        // Parse from json
        Ok(Teacher {
            id: row.get(0),
            name: row.get(1),
            password: row.get(2),
            classes: classes.unwrap(),
        })
    })?;

    if let Some(teacher) = teachers.next() {
        if teachers.next().is_some() {
            Err(anyhow!(
                "Multiple teachers found with this username: {}",
                name
            ))
        } else {
            Ok(teacher??)
        }
    } else {
        Err(anyhow!("No teachers found with this username: {}", name))
    }
}
