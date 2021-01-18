use super::{super::Id, DatabaseModel};
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

impl DatabaseModel for Homework {
    fn table() -> &'static str {
        "CREATE TABLE homework (
                id          TEXT NOT NULL PRIMARY KEY,
                name        TEXT NOT NULL,
                date        TEXT NOT NULL,
                description TEXT NOT NULL,
                points      INTEGER
        )"
    }

    fn get_query() -> &'static str {
        "SELECT * FROM homework where id = ?1"
    }

    fn insert_query() -> &'static str {
        "INSERT INTO homework (id, name, date, description, points) VALUES (?1, ?2, ?3, ?4, ?5)"
    }

    fn remove_query() -> &'static str {
        "DELETE FROM homework WHERE id = ?1"
    }

    fn construct<T>(row: &rocket_contrib::databases::rusqlite::Row<'_, '_>) -> Result<T> {
        let date = NaiveDate::parse_from_str(&row.get::<_, String>(2), "%Y-%m-%d")?;

        Ok(Homework {
            id: row.get(0),
            name: row.get(1),
            date,
            description: row.get(3),
            points: row.get(4),
        })
    }
}

/// Insert new homework and add it to a class
pub fn add_homework(conn: &Connection, homework: &Homework, class: Id) -> Result<()> {
    homework.insert(&conn)?;
    super::class::add_homework(&conn, homework.id.clone(), class)?;

    Ok(())
}

/// Remove homework and remove it from a class
pub fn remove_homework(conn: &Connection, homework: Id, class: Id) -> Result<()> {
    homework.remove(&conn)?;
    super::class::remove_homework(&conn, homework, class)?;

    Ok(())
}
