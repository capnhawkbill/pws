use super::Id;

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
