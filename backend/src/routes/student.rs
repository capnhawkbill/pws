//! Routes for students
//! All these require you to be authenticated as a student
//! They are all prefixed with "/student" and the function names with "student_"
use crate::auth::Student;

#[get("/student/homework")]
pub fn student_homework(student: Student) -> Result<Json<Vec<Assignment>>> {}
