extern crate backend;
extern crate env_logger;
extern crate rocket;

use backend::database::DbConn;
use backend::routes;

fn main() {
    env_logger::init();
    let rocket = rocket::ignite().attach(DbConn::fairing());
    let rocket = routes::student::mount(rocket);
    let rocket = routes::teacher::mount(rocket);
    rocket.launch();
}
