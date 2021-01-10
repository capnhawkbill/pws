use super::super::{getcsv, mkcsv, Id};
use anyhow::{anyhow, Result};
use rocket_contrib::databases::rusqlite::Connection;

/// The teacher
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Teacher {
    /// The id of the teacher
    pub id: Id,
    /// The name of the teacher
    pub name: String,
    /// The password of the teacher
    pub password: String,
    /// Id's of the classes the teacher is in
    pub classes: Vec<Id>,
    /// Id's of the badges the teacher has access to
    pub badges: Vec<Id>,
}

/// Create a table for the teachers
pub fn create_table(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE teacher (
            id          varchar(50),
            name        TEXT NOT NULL,
            password    TEXT NOT NULL,
            classes     TEXT,
            badges      TEXT
        )",
        &[],
    )?;
    Ok(())
}

/// Insert a teacher into the database
pub fn insert_teacher(conn: &Connection, teacher: &Teacher) -> Result<()> {
    trace!("Inserting teacher {:?}", teacher);
    // Convert to csv
    let classes = mkcsv(&teacher.classes)?;
    let badges = mkcsv(&teacher.badges)?;
    // Convert to json
    conn.execute(
        "INSERT INTO teacher (id, name, password, classes, badges) VALUES (?1, ?2, ?3, ?4, ?5)",
        &[
            &teacher.id,
            &teacher.name,
            &teacher.password,
            &classes,
            &badges,
        ],
    )?;
    Ok(())
}

/// Update a teacher in the database
pub fn update_teacher(conn: &Connection, teacher: &Teacher) -> Result<()> {
    trace!("Inserting teacher {:?}", teacher);
    // Convert to csv
    let classes = mkcsv(&teacher.classes)?;
    let badges = mkcsv(&teacher.badges)?;
    // Convert to json
    conn.execute(
        "UPDATE teacher SET name = ?2, password = ?3, classes = ?4, badges = ?5 WHERE id = ?1",
        &[
            &teacher.id,
            &teacher.name,
            &teacher.password,
            &classes,
            &badges,
        ],
    )?;

    Ok(())
}
/// Gets a teacher from the database
pub fn get_teacher(conn: &Connection, id: Id) -> Result<Teacher> {
    trace!("Getting teacher with id {}", id);
    let mut stmt = conn.prepare("SELECT * FROM teacher where id = ?1")?;
    let mut teachers = stmt.query_map(&[&id], |row| {
        // Parse from csv
        let classes = getcsv(row.get(3));
        if let Err(e) = classes {
            return Err(e);
        }
        let badges = getcsv(row.get(4));
        if let Err(e) = badges {
            return Err(e);
        }
        // Parse from json
        Ok(Teacher {
            id: row.get(0),
            name: row.get(1),
            password: row.get(2),
            classes: classes.unwrap(),
            badges: badges.unwrap(),
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

/// Get a teacher from the database using the username
pub fn get_teacher_by_name(conn: &Connection, name: &str) -> Result<Teacher> {
    trace!("Getting teacher {}", name);
    let mut stmt = conn.prepare("SELECT * FROM teacher where name = ?1")?;
    let mut teachers = stmt.query_map(&[&name], |row| {
        // Parse from csv
        let classes = getcsv(row.get(3));
        if let Err(e) = classes {
            return Err(e);
        }
        let badges = getcsv(row.get(4));
        if let Err(e) = badges {
            return Err(e);
        }
        // Parse from json
        Ok(Teacher {
            id: row.get(0),
            name: row.get(1),
            password: row.get(2),
            classes: classes.unwrap(),
            badges: badges.unwrap(),
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

/// Remove a teacher from the database
pub fn remove_teacher(conn: &Connection, id: Id) -> Result<()> {
    trace!("Removing teacher {:?}", id);
    let teacher = get_teacher(&conn, id.clone())?;

    // Remove teacher from all classes
    for class in teacher.classes {
        remove_from_class(&conn, id.clone(), class)?;
    }

    conn.execute("DELETE FROM teacher WHERE id = ?1", &[&id])?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn init() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[test]
    fn test_teacher_db() {
        init();

        let teacher = Teacher {
            id: "ID".into(),
            name: "Elias".into(),
            password: "very secure".into(),
            classes: vec!["ClassId".into(), "Second ClassId".into()],
            badges: vec!["BadgeId".into(), "Second BadgeId".into()],
        };
        let conn = &Connection::open_in_memory().unwrap();
        create_table(&conn).unwrap();

        insert_teacher(conn, &teacher).unwrap();
        let gotten = get_teacher(conn, "ID".into()).unwrap();
        assert_eq!(teacher, gotten);
    }
}
