mod schema;
mod models;

/// Struct for database pooling
#[database("sqlite_users")]
pub struct UserDataBase(diesel::SqliteConnection);
