use super::super::Id;
use anyhow::Result;
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
    pub student: Vec<Id>,
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
