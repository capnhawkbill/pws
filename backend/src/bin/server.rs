extern crate backend;
extern crate env_logger;
extern crate rocket;

use backend::database::DbConn;
use backend::routes;

use rocket::config::{Config, Environment, Value};
use std::collections::HashMap;

/// Create config for the database
pub fn init_db() -> Config {
    // Create the database
    let path = "db.sqlite";
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
fn main() {
    env_logger::init();

    let config = init_db();

    let rocket = rocket::custom(config).attach(DbConn::fairing());
    let rocket = routes::student::mount(rocket);
    let rocket = routes::teacher::mount(rocket);
    let rocket = routes::class::mount(rocket);
    let rocket = routes::badge::mount(rocket);
    rocket.launch();
}
