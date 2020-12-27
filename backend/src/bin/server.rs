extern crate backend;
#[macro_use]
extern crate rocket;

use backend::database::DbConn;
use backend::routes;

fn main() {
    let rocket = rocket::ignite().attach(DbConn::fairing());
    let rocket = routes::student::mount(rocket);
    let rocket = routes::teacher::mount(rocket);
    rocket.launch();
}
