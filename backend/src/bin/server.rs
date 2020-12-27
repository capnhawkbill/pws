extern crate backend;
#[macro_use]
extern crate rocket;
extern crate env_logger;

use backend::database::DbConn;
use backend::routes;

fn main() {
    env_logger::init();
    let rocket = rocket::ignite().attach(DbConn::fairing());
    let rocket = routes::student::mount(rocket);
    let rocket = routes::teacher::mount(rocket);
    rocket.launch();
}
