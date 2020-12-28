extern crate backend;
extern crate rocket;
#[macro_use]
extern crate serde;
#[macro_use]
extern crate log;

use rocket::http::{ContentType, Header};
use rocket::local::Client;
use std::collections::HashMap;
use rocket::config::{Config, Environment, Value};

use backend::database::{self, Student, DbConn, Id};
use backend::testhelp::*;

/// The json body for signing up
const SIGNUPBODY: &'static str = r#"
{
    "username": "Capnhawkbill",
    "password": "Easypassword"
}
"#;
const AUTHHEADER: &'static str = "Q2Fwbmhhd2tiaWxsOkVhc3lwYXNzd29yZA==";

/// The second json body for signing up
const SIGNUPBODY2: &'static str = r#"
{
    "username": "William",
    "password": "Shakespear"
}
"#;
const AUTHHEADER2: &'static str = "V2lsbGlhbTpTaGFrZXNwZWFy";

#[derive(Debug, Deserialize, PartialEq)]
pub struct SafeStudent {
    name: String,
    classes: Vec<Id>,
    badges: Vec<Id>,
}

impl From<Student> for SafeStudent {
    fn from(student: Student) -> Self {
        SafeStudent {
            name: student.name,
            classes: student.classes,
            badges: student.badges,
        }
    }
}

#[test]
fn id_test() {
    init_logger();
    let config = init_test_db();

    // Construct application
    let rocket = rocket::custom(config).attach(DbConn::fairing());
    let rocket = backend::routes::student::mount(rocket);

    let client = Client::new(rocket).expect("Failed to initialize client");

    // signup both
    let id = get_id(&client, SIGNUPBODY);
    let id2 = get_id(&client, SIGNUPBODY2);

    // get info from itself
    let selfinfo = get_self_info(&client, AUTHHEADER);
    // get info from the other
    let info = get_info(&client, AUTHHEADER, id2);

    // do it again
    // get info from itself
    let selfinfo2 = get_self_info(&client, AUTHHEADER2);
    // get info from the other
    let info2 = get_info(&client, AUTHHEADER2, id);

    // compare
    assert_eq!(selfinfo, info2);
    assert_eq!(selfinfo2, info);
}

fn get_self_info(client: &Client, auth: &'static str) -> SafeStudent {
    let auth = Header::new("Authorization", auth);
    let req = client
        .get("/api/student/info")
        .header(auth);

    let body = &req.dispatch().body_string().unwrap();
    trace!("{}", body);
    serde_json::from_str(&body).unwrap()
}

fn get_info(client: &Client, auth: &'static str, lookup: Id) -> SafeStudent {
    let auth = Header::new("Authorization", auth);
    let req = client
        .get(format!("/api/student/id?id={}", lookup))
        .header(auth);

    let body = &req.dispatch().body_string().unwrap();
    trace!("{}", body);
    serde_json::from_str(&body).unwrap()
}

fn get_id(client: &Client, body: &'static str) -> Id {
    // Signup
    let signup = client
        .post("/api/student/signup")
        .body(body)
        .header(ContentType::JSON);

    signup.dispatch().body_string().unwrap()
}
