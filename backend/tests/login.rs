extern crate backend;
extern crate rocket;
use rocket::local::Client;

use backend::{database::DbConn, routes::login::*};

/// The json body for signing up
const SIGNUPBODY: &'static str = r#"
{
    username: "Capnhawkbill",
    password: "Easypassword",
}
"#;

fn main() {
    // Construct application
    let rocket = rocket::ignite()
        .mount("/api", routes![signup])
        .attach(DbConn::fairing());

    let client = Client::new(rocket).expect("Failed to initialize client");

    // Signup
    let signup = client
        .post("/api/signup")
        .body(SIGNUPBODY)
        .header(ContentType::Json);
    // TODO check the response
    // TODO check if signup succeeded by accessing a protected path
}
