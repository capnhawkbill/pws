//! This module contains the models for the database and the functions te get, insert and modify them.
//! Lots of the models use lists of id's, to resolve them use the get functions
mod badge;
mod class;
mod homework;
mod student;
mod teacher;

pub use badge::*;
pub use class::{add_to_class, create_table, get_class, insert_class, remove_from_class, Class};
pub use homework::*;
pub use student::*;
pub use teacher::*;

use anyhow::Result;
use rocket_contrib::databases::rusqlite::Connection;

/// Create the tables in the database
pub fn create_tables(conn: &Connection) -> Result<()> {
    student::create_table(&conn)?;
    teacher::create_table(&conn)?;
    badge::create_table(&conn)?;
    class::create_table(&conn)?;
    homework::create_table(&conn)?;
    Ok(())
}
