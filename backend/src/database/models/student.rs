use super::ClassId;

/// The type of the id of the student
/// `varchar(50)`
pub type StudentId = String;
pub struct Student {
    /// The id of the student
    id: StudentId,
    /// The name of the student
    name: String,
    /// The password of the student
    password: String,
    // TODO more information about the student
    /// Id's of the classes the student is in
    classes: Vec<ClassId>,
}
