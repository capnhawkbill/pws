//! All the permission stuff

use anyhow::Result;

use strum_macros::{Display, EnumString};

/// The different permissions a user can have
/// Convertable to and from strings
#[derive(Display, Debug, Clone, Copy, EnumString)]
pub enum Permission {
    /// A student permission
    Student,
    /// A teacher permission
    Teacher,
    /// A admin permission
    Admin,
}

/// Changes the permission of a user
pub fn set_permission(_username: String, _permission: Permission) -> Result<()> {
    // Query database
    // Set permission
    todo![]
}
