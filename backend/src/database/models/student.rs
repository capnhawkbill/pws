use super::Id;
use super::super::{getcsv, mkcsv};
use anyhow::{anyhow, Result};
use rocket_contrib::databases::rusqlite::Connection;

pub struct Student {
    /// The id of the student
    id: Id,
    /// The name of the student
    name: String,
    /// The password of the student TODO plaintext lol
    password: String,
    /// Id's of the classes the student is in
    /// Stored as csv
    classes: Vec<Id>,
    /// Id's of the badges the student has
    /// Stored as csv
    badges: Vec<Id>,
}

/// Insert a student into the database
pub fn insert_student(conn: Connection, student: Student) -> Result<()> {
    // Convert to csv
    let classes = mkcsv(&student.classes)?;
    let badges = mkcsv(&student.badges)?;
    // Convert to json
    conn.execute(
            "INSERT INTO student (id, name, password, classes, badges) VALUES (?1, ?2, ?3, ?4, ?5)",
            &[&student.id, &student.name, &student.password, &classes, &badges]);
    Ok(())
}

/// Get a student from the database
pub fn get_student(conn: Connection, id: Id) -> Result<Student> {
    let mut stmt = conn.prepare("SELECT * FROM student where id = ?1")?;
    let mut students = stmt.query_map(&[&id], |row| {
        // Parse from csv
        let classes = getcsv(row.get(3));
        if let Err(e) = classes {
            return Err(e)
        }
        let badges = getcsv(row.get(4));
        if let Err(e) = badges {
            return Err(e)
        }
        // Parse from json
        Ok(Student {
            id: row.get(0),
            name: row.get(1),
            password: row.get(2),
            classes: classes.unwrap(),
            badges: badges.unwrap(),
        })
    })?;

    if let Some(student) = students.next() {
        if students.next().is_some() {
            Err(anyhow!("Multiple students found with this id: {}", id))
        } else {
            Ok(student??)
        }
    } else {
        Err(anyhow!("No students found with this id: {}", id))
    }
}

pub fn get_student_by_name(conn: Connection, name: &str) -> Result<Student> {
    let mut stmt = conn.prepare("SELECT * FROM student where name = ?1")?;
    let mut students = stmt.query_map(&[&name], |row| {
        // Parse from csv
        let classes = getcsv(row.get(3));
        if let Err(e) = classes {
            return Err(e)
        }
        let badges = getcsv(row.get(4));
        if let Err(e) = badges {
            return Err(e)
        }
        // Parse from json
        Ok(Student {
            id: row.get(0),
            name: row.get(1),
            password: row.get(2),
            classes: classes.unwrap(),
            badges: badges.unwrap(),
        })
    })?;

    if let Some(student) = students.next() {
        if students.next().is_some() {
            Err(anyhow!("Multiple students found with this username: {}", name))
        } else {
            Ok(student??)
        }
    } else {
        Err(anyhow!("No students found with this username: {}", name))
    }
}

#[cfg(Test)]
mod tests {
    use super::*;
    #[test]
    fn test_student_db() -> Connection {
        let student = Student {
            id: "ID".into(),
            name: "Elias".into(),
            password: "very secure".into(),
            classes: vec!["ClassId".into(), "Second ClassId".into()],
            badges: vec!["BadgeId".into(), "Second BadgeId".into()],
        };
        let conn = Connection::open_in_memory().unwrap();
        conn.execute(
            "CREATE TABEL student (
                    id          varchar(50)
                    name        TEXT NOT NULL
                    password    TEXT NOT NULL
                    classes     TEXT
                    badges      TEXT
            )",
        )
        .unwrap();
        insert_student(conn, &badge).unwrap();
        let gotten = get_student(conn, "ID".into());
        assert_eq!(badge, gotten);
    }
}
