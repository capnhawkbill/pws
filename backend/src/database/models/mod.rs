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

use anyhow::{anyhow, Result};
use rocket_contrib::databases::rusqlite::{types::ToSql, Connection, Row};

use super::Id;

/// Create the tables in the database
pub fn create_tables(conn: &Connection) -> Result<()> {
    student::create_table(&conn)?;
    teacher::create_table(&conn)?;
    badge::create_table(&conn)?;
    class::create_table(&conn)?;
    homework::create_table(&conn)?;
    Ok(())
}

/// A type that is in the database
pub trait DatabaseModel {
    /// Define the get query together with the values in here
    fn get_query() -> &'static str;
    /// Construct Self using a database row
    fn construct<T>(row: &Row<'_, '_>) -> Result<T>;
    /// Get the struct from a database using the get query
    fn get(conn: &Connection, id: Id) -> Result<Self>
    where
        Self: Sized,
    {
        let query = Self::get_query();
        let mut stmt = conn.prepare(query)?;
        let mut models = stmt.query_map(&[&id], Self::construct)?;

        // Check that there is only one
        if let Some(model) = models.next() {
            if models.next().is_some() {
                Err(anyhow!("Multiple models found"))
            } else {
                let model = model??;
                Ok(model)
            }
        } else {
            Err(anyhow!("No model found"))
        }
    }

    /// Define the table creation query in here
    fn table() -> &'static str;
    /// Creates the table using the query defined in `table()`
    fn create_table(&self, conn: &Connection) -> Result<()> {
        conn.execute(Self::table(), &[])?;
        Ok(())
    }

    /// Define the insert query together with the values in here
    fn insert_query() -> &'static str;
    /// Insert the struct into the database using the insert query and the values
    fn insert(&self, conn: &Connection) -> Result<()>;

    /// Define the remove query together with the values in here
    fn remove_query() -> &'static str;
    /// Remove the struct from the database using the insert query and the values
    fn remove(&self, conn: &Connection, id: Id) -> Result<()> {
        let query = Self::remove_query();
        conn.execute(query, &[&id])?;
        Ok(())
    }
}
