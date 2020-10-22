//! Functions handling authentication stuff

use crate::database::UserDataBase;
use anyhow::Result;

pub mod permission;

/// Representation of the apikey
pub type ApiKey = String;

/// Uses login credentials to get apikey
pub fn login(_conn: &UserDataBase, _username: String, _password: String) -> Result<ApiKey> {
    // Query database
    // Check shit
    // Send Apikey
    todo![]
}

/// Signs the username up with the password
pub fn signup(_conn: &UserDataBase, _username: String, _password: String) -> Result<ApiKey> {
    // Query database
    // Check duplicates
    // Check if allowed teacher
    // Create user
    // Send Apikey
    todo![]
}
