use rocket_contrib::databases::rusqlite;

pub mod models;

#[database("sqlite_logs")]
pub struct DbConn(rusqlite::Connection);
