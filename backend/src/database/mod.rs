use rocket_contrib::databases::rusqlite;

pub mod models;

#[database("sqlite_database")]
pub struct DbConn(rusqlite::Connection);
