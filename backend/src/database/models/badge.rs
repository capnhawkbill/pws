use super::super::{getbool, mkbool, Id};
use anyhow::{anyhow, Result};
use rocket_contrib::databases::rusqlite::Connection;

/// A badge that cab be awarded to students
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Badge {
    /// The id
    pub id: Id,
    /// The name
    pub name: String,
    /// The description
    pub description: String,
    /// Whether the badge is official or user created
    pub official: bool,
}

pub fn create_table(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE badge (
                id          TEXT NOT NULL PRIMARY KEY,
                name        TEXT NOT NULL,
                description TEXT NOT NULL,
                official    INTEGER
        )",
        &[],
    )?;

    Ok(())
}

/// Insert a badge into the database
pub fn insert_badge(conn: &Connection, badge: &Badge) -> Result<()> {
    trace!("Inserting Badge {:?}", badge.name);
    let official = mkbool(badge.official);
    conn.execute(
        "INSERT INTO badge (id, name, description, official) VALUES (?1, ?2, ?3, ?4)",
        &[&badge.id, &badge.name, &badge.description, &official],
    )?;
    Ok(())
}

/// Get a badge from the database
pub fn get_badge(conn: &Connection, id: Id) -> Result<Badge> {
    trace!("Getting badge with id {:?}", id);
    let mut stmt = conn.prepare("SELECT * FROM badge where id = ?1")?;
    let mut badges = stmt.query_map(&[&id], |row| {
        let official = getbool(row.get(3))?;
        Ok(Badge {
            id: row.get(0),
            name: row.get(1),
            description: row.get(2),
            official,
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
        };
        let conn = Connection::open_in_memory().unwrap();
        create_table(&conn).unwrap();
        insert_badge(&conn, &badge).unwrap();
        let gotten = get_badge(&conn, "ID".into()).unwrap();
        assert_eq!(badge, gotten);
    }
}
