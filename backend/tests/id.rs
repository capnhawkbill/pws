extern crate backend;
extern crate rocket;
#[macro_use]
extern crate serde;
#[macro_use]
extern crate log;

use backend::database::{self, Student, DbConn, Id};
use backend::testhelp::*;
use backend::routes::{class, student, teacher};

use rocket::local::Client;

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


#[test]
fn id_test() {
    init_logger();
    let config = init_test_db();

    // Construct application
    let rocket = rocket::custom(config).attach(DbConn::fairing());
    let rocket = backend::routes::student::mount(rocket);

    let client = Client::new(rocket).expect("Failed to initialize client");

    // signup both
    let id = get_id_student(&client, SIGNUPBODY);
    let id2 = get_id_student(&client, SIGNUPBODY2);

    // get info from itself
    let selfinfo = get_self_info_student(&client, AUTHHEADER);
    // get info from the other
    let info = get_info(&client, AUTHHEADER, id2);

    // do it again
    // get info from itself
    let selfinfo2 = get_self_info_student(&client, AUTHHEADER2);
    // get info from the other
    let info2 = get_info(&client, AUTHHEADER2, id);

    // compare
    assert_eq!(selfinfo, info2);
    assert_eq!(selfinfo2, info);
}

