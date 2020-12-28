use super::super::{getcsv, mkcsv, Id};
use anyhow::{anyhow, Result};
use rocket_contrib::databases::rusqlite::Connection;

/// A Class
pub struct Class {
    /// The id of the class
    pub id: Id,
    /// The name of the class
    pub name: String,
    /// The id's of the teachers in the class
    pub teachers: Vec<Id>,
    /// The id's of the students in the class
    pub students: Vec<Id>,
}

pub fn create_table(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE class (
                id          TEXT NOT NULL PRIMARY KEY,
                name        TEXT NOT NULL,
                teachers    TEXT NOT NULL,
                students    TEXT NOT NULL
        )",
        &[]
    )?;

    Ok(())
}

/// Insert a class into the database
pub fn insert_class(conn: &Connection, class: &Class) -> Result<()> {
    trace!("Inserting class {}", class.name);
    let teachers = mkcsv(&class.teachers)?;
    let students = mkcsv(&class.students)?;
    conn.execute("INSERT INTO class (id, name, teachers, students) VALUES (?1, ?2, ?3, ?4)",
                 &[&class.id, &class.name, &teachers, &students])?;
    Ok(())
}

/// Get a class from the database
pub fn get_class(conn: &Connection, id: Id) -> Result<Class> {
    trace!("Getting class with id {}", id);
    let mut stmt = conn.prepare("SELECT * FROM student where id = ?1")?;
    let mut classes = stmt.query_map(&[&id], |row| {
        // Parse from csv
        let teachers = getcsv(row.get(2));
        if let Err(e) = teachers {
            return Err(e)
        }
        let students = getcsv(row.get(3));
        if let Err(e) = students {
            return Err(e)
        }

        Ok(Class {
            id: row.get(0),
            name: row.get(1),
            teachers: teachers.unwrap(),
            students: students.unwrap(),
        })
    })?;

    // Check if exactly one class is found
    if let Some(class) = classes.next() {
        if classes.next().is_some() {
            Err(anyhow!("Multiple classes found with this id: {}", id))
        } else {
            Ok(class??)
        }
    } else {
        Err(anyhow!("No classes found with this id: {}", id))
    }

}
