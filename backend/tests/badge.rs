extern crate backend;
extern crate rocket;

use backend::database::DbConn;
use backend::routes::*;
use backend::testhelp::*;

use rocket::http::{ContentType, Header};
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

const BADGE: &'static str = r#"
{
    "name": "TestBadge",
    "description": "A very cool badge for testing"
}
"#;

#[test]
fn test_badge() {
    init_logger();
    let config = init_test_db();

    // Construct application
    let rocket = rocket::custom(config).attach(DbConn::fairing());
    let rocket = class::mount(rocket);
    let rocket = student::mount(rocket);
    let rocket = teacher::mount(rocket);
    let rocket = badge::mount(rocket);

    let client = Client::new(rocket).expect("Failed to initialize client");

    let _teacher = get_id_teacher(&client, SIGNUPTEACHER);
    let student = get_id_student(&client, SIGNUPSTUDENT);

    let authTeacher = Header::new("Authorization", AUTHTEACHER);
    let class = client
        .get("/api/class/create?name=testclass")
        .header(authTeacher.clone())
        .dispatch()
        .body_string()
        .unwrap();

    let authStudent = Header::new("Authorization", AUTHSTUDENT);
    client
        .get(format!("/api/class/join?id={}", class))
        .header(authStudent.clone())
        .dispatch();

    // create badge
    let badge = client
        .post("/api/badge/create")
        .body(BADGE)
        .header(authTeacher.clone())
        .header(ContentType::JSON)
        .dispatch()
        .body_string()
        .unwrap();

    // award the badge
    client
        .get(format!(
            "/api/badge/award?student={}&badge={}",
            student, badge
        ))
        .header(authTeacher)
        .dispatch();

    let student_info = get_self_info_student(&client, AUTHSTUDENT);
    let teacher_info = get_self_info_teacher(&client, AUTHTEACHER);

    assert_eq!(student_info.badges, vec!["".to_string(), badge.clone()]);
    assert_eq!(teacher_info.badges, vec!["".to_string(), badge]);
}
