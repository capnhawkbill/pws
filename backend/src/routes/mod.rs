//! All the routes that form the api


pub mod teacher;
pub mod student;

use serde::{Deserialize, Serialize};

/// The credentials that are received as json
#[derive(Deserialize)]
pub struct Credentials {
    username: String,
    password: String,
}
