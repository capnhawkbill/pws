use super::Id;
use super::super::{getcsv, mkcsv};
use anyhow::Result;
use rocket_contrib::databases::rusqlite::Connection;
use serde::{Deserialize, Serialize};

pub struct Student {
    /// The id of the student
    id: Id,
    /// The name of the student
    name: String,
    /// The password of the student TODO plaintext lol
    password: String,
    /// Other information that isn't strictly necessary
    /// Stored in json inside of the database
    info: Option<StudentInfo>,
    /// Id's of the classes the student is in
    /// Stored as csv
    classes: Vec<Id>,
    /// Id's of the badges the student has
    /// Stored as csv
    badges: Vec<Id>,
}

/// Non necessary information about a student
/// Stored in json
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StudentInfo {
    /// Gender
    gender: Option<String>,
    /// Status
    status: Option<String>,
    /// Disorder
    disorders: Option<Vec<String>>, // TODO more unnecessary information
}

/// Insert a student into the database
pub fn insert_student(conn: Connection, student: Student) -> Result<()> {
    // Convert to csv
    let classes = mkcsv(&student.classes)?;
    let badges = mkcsv(&student.badges)?;
    // Convert to json
    if let Some(info) = student.info {
        conn.execute(
                "INSERT INTO student (id, name, password, info, classes, badges) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
                &[&student.id, &student.name, &student.password, &serde_json::to_string(&info)?, &classes, &badges]
            )?;
    } else {
        conn.execute(
                "INSERT INTO student (id, name, password, info, classes, badges) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
                &[&student.id, &student.name, &student.password, "None".into(), &classes, &badges]
            )?;
    }
    Ok(())
}

/// Gets a student from the database
pub fn get_student(conn: Connection, id: Id) -> Result<()> {
    let stmt = conn.prepare("SELECT * FROM student where id = ?1")?;
    let students = stmt.query_map(&[&id], |row| {
        // Parse from csv
        let classes = getcsv(row.get(4))?;
        let badges = getcsv(row.get(5))?;
        // Parse from json
        let info: Option<StudentInfo> = match row.get::<_, String>(3).as_str() {
            "None" => None,
            s => serde_json::from_str(s)?,
        };
        Ok(Student {
            id: row.get(0),
            name: row.get(1),
            password: row.get(2),
            info,
            classes,
            badges,
        })
    })?;
    Ok(())
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
            info: None,
            classes: vec!["ClassId".into(), "Second ClassId".into()],
            badges: vec!["BadgeId".into(), "Second BadgeId".into()],
        };
        let conn = Connection::open_in_memory().unwrap();
        conn.execute(
            "CREATE TABEL student (
                    id          varchar(50)
                    name        TEXT NOT NULL
                    password    TEXT NOT NULL
                    info        TEXT
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
