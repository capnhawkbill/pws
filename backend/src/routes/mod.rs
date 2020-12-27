//! All the routes that form the api

use anyhow::Result;
use rocket_contrib::json::Json;
use serde::{Deserialize, Serialize};

use crate::database::DbConn;

mod teacher;
pub use teacher::*;
mod student;
pub use student::*;

/// The credentials that are received as json
#[derive(Deserialize)]
pub struct Credentials {
    username: String,
    password: String,
}

/// Signup
#[post("/signup", format = "json", data = "<credentials>")]
pub fn signup(conn: DbConn, credentials: Json<Credentials>) -> Result<Json<ApiKey>> {
    todo![]
}

#[get("/student")]
pub fn student(auth: Student) -> String {
    format!("Hello {}", student.name)
}
