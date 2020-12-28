use super::super::{mkbool, getbool, Id};
use anyhow::{anyhow, Result};
use rocket_contrib::databases::rusqlite::Connection;
use std::str::FromStr;

/// A badge that cab be awarded to students
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Badge {
    /// The id
    id: Id,
    /// The name
    name: String,
    /// The description
    description: String,
    /// Whether the badge is official or user created
    official: bool,
    /// The condition
    condition: Condition,
}

/// Condition for getting a badge
/// Has methodes for converting into and from strings for using
/// in the database
// TODO checking conditions
#[derive(Debug, PartialEq)]
pub enum Condition {
    /// A test condition
    Test,
}
impl std::fmt::Display for Condition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Condition::Test => "Test",
        };
        write!(f, "{}", s)
    }
}

/// Error type for parsing a condition
#[derive(Debug)]
pub enum ParseConditionError {
    /// This variant of the enum doesn't exist
    DoesntExist,
}

impl std::fmt::Display for ParseConditionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let text = match self {
            ParseConditionError::DoesntExist => "Badge doesn't exist",
        };
        write!(f, "{}", text)
    }
}

impl std::error::Error for ParseConditionError {}

impl FromStr for Condition {
    type Err = ParseConditionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Test" => Ok(Condition::Test),
            _ => Err(ParseConditionError::DoesntExist),
        }
    }
}

pub fn create_table(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE badge (
                id          TEXT NOT NULL PRIMARY KEY,
                name        TEXT NOT NULL,
                description TEXT NOT NULL,
                official    INTEGER,
                condition   TEXT NOT NULL
        )",
        &[]
    )?;

    Ok(())
}

/// Insert a badge into the database
pub fn insert_badge(conn: &Connection, badge: &Badge) -> Result<()> {
    let official = mkbool(badge.official);
    conn.execute(
        "INSERT INTO badge (id, name, description, official, condition) VALUES (?1, ?2, ?3, ?4, ?5)",
        &[&badge.id, &badge.name, &badge.description, &official, &badge.condition.to_string()]
    )?;
    Ok(())
}

/// Get a badge from the database
pub fn get_badge(conn: &Connection, id: Id) -> Result<Badge> {
    let mut stmt = conn.prepare("SELECT * FROM badge where id = ?1")?;
    let mut badges = stmt.query_map(&[&id], |row| {
        let condition = Condition::from_str(row.get::<_, String>(4).as_str())?;
        let official = getbool(row.get(3))?;
        Ok(Badge {
            id: row.get(0),
            name: row.get(1),
            description: row.get(2),
            official,
            condition,
        })
    })?;

    // TODO This just gets the first badge it sees no checks
    if let Some(badge) = badges.next() {
        badge?
    } else {
        Err(anyhow!("No badge found"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_badge_db() {
        let badge = Badge {
            id: "ID".into(),
            name: "Elias".into(),
            description: "zwart haar".into(),
            official: false,
            condition: Condition::Test,
        };
        let conn = Connection::open_in_memory().unwrap();
        create_table(&conn).unwrap();
        insert_badge(&conn, &badge).unwrap();
        let gotten = get_badge(&conn, "ID".into()).unwrap();
        assert_eq!(badge, gotten);
    }
}
