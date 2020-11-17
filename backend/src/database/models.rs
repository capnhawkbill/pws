use super::schema::*;
use super::auth;
use super::config::EXP;

#[derive(Debug, Queryable)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
    pub apikey: String,
    pub permission: String,
}

#[derive(Debug, Insertable)]
#[table_name = "users"]
pub struct NewUser<'a> {
    pub username: &'a str,
    pub password: &'a str,
    pub apikey: &'a str,
    pub permission: String,
}

<<<<<<< HEAD
#[derive(Debug, Queryable)]
pub struct Badge {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub condition: String,
}

#[derive(Debug, Insertable)]
#[table_name = "badges"]
pub struct NewBadge<'a> {
    pub id: i32,
    pub name: &'a str,
    pub description: &'a str,
    pub condition: &'a str,
}

/// The class
#[derive(Debug, Queryable)]
pub struct Class {
    pub id: i32,
    pub teachers: Vec<Teacher>,
    pub homework: Vec<Assignment>,
    pub badges: Vec<Badge>,
}

#[derive(Debug, Insertable)]
#[table_name = "classes"]
pub struct NewClass<'a> {
    pub id: i32,
    pub teachers: &'a [Teacher],
    pub homework: &'a [Assignment],
    pub badges: &'a [Badge],
}

/// The teacher
#[derive(Debug, Queryable)]
pub struct Teacher {
    name: String,
    id: i64,
}

#[derive(Debug, Insertable)]
#[table_name = "teachers"]
pub struct NewTeacher<'a> {
    name: &'a str,
    id: i64,
}
/// The student
#[derive(Debug, Queryable)]
pub struct Student {
    name: String,
    classes: Vec<Class>,
    id: i64,
    progress: Progress,
}

#[derive(Debug, Insertable)]
#[table_name = "students"]
pub struct NewStudent<'a> {
    name: &'a str,
    classes: &'a [Class],
    id: i64,
    progress: Progress,
}

/// The progress a student made
pub struct Progress {
    badges: Vec<Badge>,
    assignments: Vec<Assignment>
}

/// An assignment for in a class
pub struct Assignment {
    name: String,
    id: i64,
    description: String,
}

/// Conditions for badges
pub enum Condition {
    /// If n amount of assignments is done
    AssignmentsDone(i32),
=======
impl User {
    fn to_auth(&self) {

    }
>>>>>>> 3f190f961927985159b262e44c1ad6ed0b25df13
}
