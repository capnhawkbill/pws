//! Functions to help with integration tests
use crate::database;
use rocket_contrib::databases::rusqlite::Connection;
use std::collections::HashMap;
use rocket::config::{Config, Environment, Value};

/// Create an empty test database and return a config for it
pub fn init_test_db() -> Config {
    // Create the database
    let path = "test.sqlite";
    let _ = std::fs::remove_file(&path);
    let db = Connection::open(&path).unwrap();
    database::create_tables(&db).unwrap();

    // Create the config
    let mut database_config = HashMap::new();
    let mut databases = HashMap::new();

    database_config.insert("url", Value::from(path));
    databases.insert("sqlite_database", Value::from(database_config));
    Config::build(Environment::Development)
        .extra("databases", databases)
        .finalize()
        .unwrap()
}

/// Initialize logger for tests
pub fn init_logger() {
    let _ = env_logger::builder().is_test(true).try_init();
}
