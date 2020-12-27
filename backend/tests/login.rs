extern crate backend;
extern crate rocket;
use rocket::local::Client;
use rocket::http::{ContentType, Header};

use backend::database::DbConn;

/// The json body for signing up
const SIGNUPBODY: &'static str = r#"
{
    username: "Capnhawkbill",
    password: "Easypassword",
}
"#;

const AUTHHEADER: &'static str = "Q2Fwbmhhd2tiaWxsOkVhc3lwYXNzd29yZA==";

fn main() {
    // Construct application
    let rocket = rocket::ignite()
        .attach(DbConn::fairing());
    let rocket = backend::routes::student::mount(rocket);

    let client = Client::new(rocket).expect("Failed to initialize client");

    // Signup
    let signup = client
        .post("/api/student/signup")
        .body(SIGNUPBODY)
        .header(ContentType::JSON);
    let _id = signup.dispatch();

    // Access protected path
    let auth = Header::new("Authorization", AUTHHEADER);
    let check = client.get("/api/student/student")
        .header(auth);
    let mut result = check.dispatch();
    assert_eq!("Hello student Capnhawkbill".to_string(), result.body_string().unwrap());
}
