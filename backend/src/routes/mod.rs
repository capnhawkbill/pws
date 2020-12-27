//! All the routes that form the api

pub mod student;
pub mod teacher;

use serde::Deserialize;

/// The credentials that are received as json
#[derive(Deserialize)]
pub struct Credentials {
    username: String,
    password: String,
}
