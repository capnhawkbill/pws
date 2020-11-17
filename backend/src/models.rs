#![allow(missing_docs)]
use derive_builder::Builder;

/// The class
#[derive(Builder, Clone, Debug)]
pub struct Class {
    teachers: Vec<Teacher>,
    homework: Vec<Assignment>,
    badges: Vec<Badge>,
}

/// The teacher
#[derive(Builder, Clone, Debug)]
pub struct Teacher {
    name: String,
    id: i64,
}

/// The student
#[derive(Builder, Clone, Debug)]
pub struct Student {
    name: String,
    classes: Vec<Class>,
    id: i64,
    progress: Progress,
}

/// The progress a student made
#[derive(Builder, Clone, Debug)]
pub struct Progress {
    badges: Vec<Badge>,
    assignments: Vec<Assignment>
}

/// An assignment for in a class
#[derive(Builder, Clone, Debug)]
pub struct Assignment {
    name: String,
    id: i64,
    description: String,
}

/// A badge that a student can earn
#[derive(Builder, Clone, Debug)]
pub struct Badge {
    /// Name of the badge
    name: String,
    /// Id of the badge
    id: i64,
    /// Description of the badge
    description: String,
    /// Base64 encoded image
    image: String,
    /// Condition to get the badge
    condition: Condition,
}

/// Conditions for badges
#[derive(Clone, Debug)]
pub enum Condition {
    /// If n amount of assignments is done
    AssignmentsDone(i32),
}

impl Condition {
    /// Check if a Student has the condition
    fn check(&self, student: Student) -> bool {
        match self {
            Condition::AssignmentsDone(n) => student.progress.assignments.len() <= n.clone() as usize,
        }
    }
}
