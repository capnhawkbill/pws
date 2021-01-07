extern crate backend;
extern crate rocket;

use backend::database::DbConn;
use backend::routes::*;
use backend::testhelp::*;

use rocket::http::Header;
use rocket::local::Client;

const SIGNUPTEACHER: &'static str = r#"
{
    "username": "TestTeacher",
    "password": "CoolPassword"
}
"#;
const AUTHTEACHER: &'static str = "VGVzdFRlYWNoZXI6Q29vbFBhc3N3b3Jk";

const SIGNUPSTUDENT: &'static str = r#"
{
    "username": "TestStudent",
    "password": "CoolerPassword"
}
"#;
const AUTHSTUDENT: &'static str = "VGVzdFN0dWRlbnQ6Q29vbGVyUGFzc3dvcmQ=";

#[test]
fn test_class() {
    init_logger();
    let config = init_test_db();

    // Construct application
    let rocket = rocket::custom(config).attach(DbConn::fairing());
    let rocket = class::mount(rocket);
    let rocket = student::mount(rocket);
    let rocket = teacher::mount(rocket);

    let client = Client::new(rocket).expect("Failed to initialize client");

    let _teacher = get_id_teacher(&client, SIGNUPTEACHER);
    let _student = get_id_student(&client, SIGNUPSTUDENT);

    let auth = Header::new("Authorization", AUTHTEACHER);
    let class = client
        .get("/api/class/create?name=testclass")
        .header(auth)
        .dispatch()
        .body_string()
        .unwrap();

    let auth = Header::new("Authorization", AUTHSTUDENT);
    let _ = client
        .get(format!("/api/class/join?id={}", class))
        .header(auth)
        .dispatch();

    let student_info = get_self_info_student(&client, AUTHSTUDENT);
    let teacher_info = get_self_info_teacher(&client, AUTHTEACHER);

    assert!(!student_info.classes.is_empty() && !teacher_info.classes.is_empty());
    assert_eq!(teacher_info.classes, student_info.classes);
}
