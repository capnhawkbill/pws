//! All the permission stuff

use anyhow::Result;
use std::str::FromStr;
use std::string::ToString;
use strum_macros::{Display, EnumString};

/// The different permissions a user can have
/// Convertable to and from strings
#[derive(Display, Debug, Clone, Copy, EnumString)]
pub enum Permission {
    Student,
    Teacher,
    Admin,
}

/// Changes the permission of a user
pub fn set_permission(username: String, permission: Permission) -> Result<()> {
    // Query database
    // Set permission
    todo![]
}
