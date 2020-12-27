use super::Id;

/// A Class
pub struct Class {
    /// The id of the class
    id: Id,
    /// The name of the class
    name: String,
    /// The id's of the teachers in the class
    teachers: Vec<Id>,
    /// The id's of the students in the class
    student: Vec<Id>,
}
