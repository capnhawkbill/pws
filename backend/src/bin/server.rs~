extern crate backend;
#[macro_use]
extern crate rocket;

use backend::config::AppState;
use backend::routes::*;

fn main() {
    rocket::ignite()
        .mount("/api", routes![login, signup, student])
        .attach(UserDataBase::fairing())
        .attach(AppState::manage())
        .launch();
}
