use super::super::Id;
use anyhow::{anyhow, Result};
use chrono::naive::NaiveDate;
use rocket_contrib::databases::rusqlite::Connection;

/// A homework assignment
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Homework {
    /// Id of the homework
    pub id: Id,
    /// Name/Title of the homework
    pub name: String,
    /// Due date of the homework
    pub date: NaiveDate,
    /// Description of the homework
    pub description: String,
    /// Points awarded when homework is finished
    pub points: i32,
}

/// Create the table for the homework
pub fn create_table(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE homework (
                id          TEXT NOT NULL PRIMARY KEY,
                name        TEXT NOT NULL,
                date        TEXT NOT NULL,
                description TEXT NOT NULL,
                points      INTEGER
        )",
        &[],
    )?;

    Ok(())
}

/// Insert new homework and add it to a class
pub fn add_homework(conn: &Connection, homework: &Homework, class: Id) -> Result<()> {
    insert_homework(&conn, &homework)?;
    super::class::add_homework(&conn, homework.id.clone(), class)?;

    Ok(())
}

/// Remove homework and remove it from a class
pub fn remove_homework(conn: &Connection, homework: Id, class: Id) -> Result<()> {
    delete_homework(&conn, homework.clone())?;
    super::class::remove_homework(&conn, homework, class)?;

    Ok(())
}
/// Insert homework into the database
pub fn insert_homework(conn: &Connection, homework: &Homework) -> Result<()> {
    trace!("Inserting homework {}", homework.name);
    conn.execute(
        "INSERT INTO homework (id, name, date, description, points) VALUES (?1, ?2, ?3, ?4, ?5)",
        &[
            &homework.id,
            &homework.name,
            &homework.date.to_string(),
            &homework.description,
            &homework.points,
        ],
    )?;
    Ok(())
}

/// Get homework from the database
pub fn get_homework<'a>(conn: &'a Connection, id: Id) -> Result<Homework> {
    trace!("Getting homework with id {}", id);
    let mut stmt = conn.prepare("SELECT * FROM homework where id = ?1")?;
    let mut homeworks = stmt.query_map(&[&id], |row| {
        let date = NaiveDate::parse_from_str(&row.get::<_, String>(2), "%Y-%m-%d");
        if let Err(e) = date {
            return Err(e);
        }
        Ok(Homework {
            id: row.get(0),
            name: row.get(1),
            date: date.unwrap(),
            description: row.get(3),
            points: row.get(4),
        })
    })?;

    // Check if exactly one homework is found
    if let Some(homework) = homeworks.next() {
        if homeworks.next().is_some() {
            Err(anyhow!("Multiple homework found with this id: {}", id))
        } else {
            Ok(homework??)
        }
    } else {
        Err(anyhow!("No homework found with this id: {}", id))
    }
}

/// Delete homework from the database
pub fn delete_homework(conn: &Connection, id: Id) -> Result<()> {
    conn.execute("DELETE FROM homework WHERE id = ?1", &[&id])?;

    Ok(())
}
