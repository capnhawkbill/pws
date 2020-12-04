use super::Id;
use rusqlite::Connection;

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
pub enum Condition {}
// TODO display condition
impl std::fmt::Display for Condition {}
// TODO fromstr condition
impl FromStr for Condition {}

fn insert_badge(conn: Connection, badge: Badge) -> Result<()> {
    conn.execute(
        "INSERT INTO badge (id, name, description, official, condition)",
        params![badge.id, badge.name, badge.description, badge.official, badge.condition]
    )?;
    Ok(())
}

fn get_badge(conn: Connection, id: Id) -> Result<Badge> {
    let stmt = conn.prepare("SELECT * FROM badge where id = :id");
    let badges = stmt.query_map(params!["id", id], |row| {
        Ok(Badge {
            id: row.get(0),
            name: row.get(1),
            description: row.get(2),
            official: row.get(3),
            condition: row.get(4),
        })
    })?;

    if let Some(badge) = badges.next() {
        Ok(badge)
    } else {
        Err(anyhow!("No badge found"))
    }
}
