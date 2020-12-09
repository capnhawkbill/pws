use super::Id;
use crate::params;
use anyhow::{anyhow, Result};
use rocket_contrib::databases::rusqlite::Connection;
use std::str::FromStr;

/// A badge that cab be awarded to students
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
pub enum Condition {
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

enum ParseConditionError {
    DoesntExist,
}

impl FromStr for Condition {
    type Err = ParseConditionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Test" => Ok(Condition::Test),
            _ => Err(ParseConditionError::DoesntExist),
        }
    }
}

pub fn insert_badge(conn: Connection, badge: &Badge) -> Result<()> {
    conn.execute(
        "INSERT INTO badge (id, name, description, official, condition) VALUES (?1, ?2, ?3, ?4, ?5)",
        params![badge.id, badge.name, badge.description, badge.official, badge.condition]
    )?;
    Ok(())
}

pub fn get_badge(conn: Connection, id: Id) -> Result<Badge> {
    let stmt = conn.prepare("SELECT * FROM badge where id = ?1")?;
    let badges = stmt.query_map(params![id], |row| {
        let condition = Condition::from_str(row.get::<_, String>(4).as_str())?;
        Ok(Badge {
            id: row.get(0),
            name: row.get(1),
            description: row.get(2),
            official: row.get(3),
            condition,
        })
    })?;

    if let Some(badge) = badges.next() {
        Ok(badge?)
    } else {
        Err(anyhow!("No badge found"))
    }
}

#[cfg(Test)]
mod tests {
    use super::*;
    #[test]
    fn test_badge_db() -> Connection {
        let badge = Badge {
            id: "ID".into(),
            name: "Elias".into(),
            description: "zwart haar".into(),
            official: false,
            condition: Condition::Test,
        };
        let conn = Connection::open_in_memory().unwrap();
        conn.execute(
            "CREATE TABEL badge (
                    id          TEXT NOT NULL PRIMARY KEY
                    name        TEXT NOT NULL
                    description TEXT NOT NULL
                    official    TEXT NOT NULL
                    condition   TEXT NOT NULL
            )",
        );
        insert_badge(conn, &badge).unwrap();
        let gotten = get_badge(conn, "ID".into());
        assert_eq!(badge, gotten);
    }
}
