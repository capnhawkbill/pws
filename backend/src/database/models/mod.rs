//! This module contains the models for the database and the functions te get, insert and modify them.
//! Lots of the models use lists of id's, to resolve them use the get functions
mod badge;
mod class;
mod student;
mod teacher;

pub use badge::*;
pub use class::*;
pub use student::*;
pub use teacher::*;

use anyhow::Result;
use rocket_contrib::databases::rusqlite::Connection;

/// Create the tables in the database
pub fn create_tables(conn: &Connection) -> Result<()> {
    student::create_table(&conn)?;
    teacher::create_table(&conn)?;
    // TODO badges and classes
    Ok(())
}

//#[macro_export]
//macro_rules! params {
//    () => {
//        rocket_contrib::databases::rusqlite::NO_PARAMS
//    };
//    ($($param:expr),+ $(,)?) => {
//        &[$(&$param as &dyn rocket_contrib::databases::rusqlite::ToSql),+] as &[&dyn rocket_contrib::databases::rusqlite::ToSql]
//    };
//}
