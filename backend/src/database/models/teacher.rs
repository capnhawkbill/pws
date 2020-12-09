use super::Id;

/// The teacher
pub struct Teacher {
    /// The id of the student
    id: Id,
    /// The name of the student
    name: String,
    /// The password of the teacher
    password: String,
    /// Other information that isn't strictly necessary
    info: Option<TeacherInfo>,
    /// Id's of the classes the student is in
    classes: Vec<Id>,
}

/// Non necessary information about a student
pub struct TeacherInfo {
    gender: Option<String>, // TODO more unnecessary information
}
