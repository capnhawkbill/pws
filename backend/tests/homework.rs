extern crate backend;
extern crate rocket;

use backend::database::DbConn;
use backend::testhelp::*;
use backend::routes::*;

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

#[test]
fn test_homework() {
    init_logger();
    let config = init_test_db();

    // Construct application
    let rocket = rocket::custom(config).attach(DbConn::fairing());
    let rocket = class::mount(rocket);
    let rocket = student::mount(rocket);
    let rocket = teacher::mount(rocket);
    let rocket = homework::mount(rocket);

    let client = Client::new(rocket).expect("Failed to initialize client");

    // Signup teacher
    let _teacher = get_id_teacher(&client, SIGNUPTEACHER);
    let authteacher = Header::new("Authorization", AUTHTEACHER);

    // Create class
    let class = client
        .get("/api/class/create?name=testclass")
        .header(authteacher.clone())
        .dispatch()
        .body_string()
        .unwrap();

    let class2 = client
        .get("/api/class/create?name=cooltestclass")
        .header(authteacher.clone())
        .dispatch()
        .body_string()
        .unwrap();

    // add homework
    let homework = Homework {
        name: "TestHomework".into(),
        date: "2020-12-12".into(),
        description: "Very Hard Homework".into(),
    };
    let homework_str = serde_json::to_string(&homework).unwrap();

    client.post(format!("/api/homework/add?class={}", class))
          .body(&homework_str)
          .header(authteacher.clone())
          .header(ContentType::JSON)
          .dispatch();

    let homework2 = Homework {
        name: "CoolHomework".into(),
        date: "2021-10-10".into(),
        description: "Very Cool Homework".into(),
    };
    let homework2_str = serde_json::to_string(&homework2).unwrap();

    client.post(format!("/api/homework/add?class={}", class2))
          .body(&homework2_str)
          .header(authteacher.clone())
          .header(ContentType::JSON)
          .dispatch();

    // signup student
    let _student = get_id_student(&client, SIGNUPSTUDENT);
    let authstudent = Header::new("Authorization", AUTHSTUDENT);

    // add student to both classes
    client.get(format!("/api/class/join?id={}", class))
          .header(authstudent.clone())
          .dispatch();
    client.get(format!("/api/class/join?id={}", class2))
          .header(authstudent.clone())
          .dispatch();

    // get homework student
    let studenthw = client.get(format!("/api/homework/get?class={}", class))
                          .header(authstudent.clone())
                          .dispatch()
                          .body_string()
                          .unwrap();
    let studenthw = serde_json::from_str::<Vec<Homework>>(&studenthw).unwrap();

    // check
    assert_eq!(studenthw, vec![homework.clone()]);

    // get all the homework
    let allstudenthw = client.get("/api/homework/get")
                             .header(authstudent.clone())
                             .dispatch()
                             .body_string()
                             .unwrap();
    let allstudenthw = serde_json::from_str::<Vec<Homework>>(&allstudenthw).unwrap();

    // check
    assert!(allstudenthw.contains(&homework));
    assert!(allstudenthw.contains(&homework2));

    // remove homework
    client.post(format!("/api/homework/remove?class={}", class))
          .body(&homework_str)
          .header(authteacher)
          .header(ContentType::JSON)
          .dispatch();

    // get homework student
    let newstudenthw = client.get(format!("/api/homework/get?class={}", class))
                             .header(authstudent)
                             .dispatch()
                             .body_string()
                             .unwrap();
    // check
    assert_eq!(newstudenthw, "[]");
    // maybe multiple classes for the student
}
