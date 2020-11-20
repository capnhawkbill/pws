use super::Id;

pub struct Student {
    /// The id of the student
    id: Id,
    /// The name of the student
    name: String,
    /// The password of the student
    password: String,
    /// Other information that isn't strictly necessary
    info: Option<>StudentInfo>,
    /// Id's of the classes the student is in
    classes: Vec<Id>,
}

/// Non necessary information about a student
pub struct StudentInfo {
    /// Gender
    gender: Option<String>,
    // TODO more unnecessary information
}
