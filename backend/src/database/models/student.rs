use super::Id;

pub struct Student {
    /// The id of the student
    id: Id,
    /// The name of the student
    name: String,
    /// The password of the student
    password: String,
    // TODO more information about the student
    /// Id's of the classes the student is in
    classes: Vec<Id>,
}
