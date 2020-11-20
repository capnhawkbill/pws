use rocket_contrib::databases::rustqlite;

pub mod models;

#[database("sqlite_logs")]
pub struct DbConn(rustqlite::Connection);
