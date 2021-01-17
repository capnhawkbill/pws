extern crate backend;
#[macro_use]
extern crate log;
extern crate rocket;

use backend::{
    database::{self, DbConn},
    routes::{self, class, homework, student, teacher},
    testhelp::{get_id_student, get_id_teacher, init_logger, init_test_db},
};

use chrono::NaiveDate;
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
    let homework = routes::Homework {
        name: "TestHomework".into(),
        date: NaiveDate::parse_from_str("2020-12-12", "%Y-%m-%d").unwrap(),
        description: "Very Hard Homework".into(),
        points: 2,
    };
    let homework_str = serde_json::to_string(&homework).unwrap();

    client
        .post(format!("/api/homework/add?class={}", class))
        .body(&homework_str)
        .header(authteacher.clone())
        .header(ContentType::JSON)
        .dispatch();

    let homework2 = routes::Homework {
        name: "CoolHomework".into(),
        date: NaiveDate::parse_from_str("2021-10-10", "%Y-%m-%d").unwrap(),
        description: "Very Cool Homework".into(),
        points: 2,
    };
    let homework2_str = serde_json::to_string(&homework2).unwrap();

    client
        .post(format!("/api/homework/add?class={}", class2))
        .body(&homework2_str)
        .header(authteacher.clone())
        .header(ContentType::JSON)
        .dispatch();

    // signup student
    let _student = get_id_student(&client, SIGNUPSTUDENT);
    let authstudent = Header::new("Authorization", AUTHSTUDENT);

    // add student to both classes
    client
        .get(format!("/api/class/join?id={}", class))
        .header(authstudent.clone())
        .dispatch();
    client
        .get(format!("/api/class/join?id={}", class2))
        .header(authstudent.clone())
        .dispatch();

    // get homework student
    let studenthwids = client
        .get(format!("/api/homework/get?class={}", class))
        .header(authstudent.clone())
        .dispatch()
        .body_string()
        .unwrap();
    trace!("{:?}", studenthwids);
    let studenthwids: Vec<String> = serde_json::from_str(&studenthwids).unwrap();
    // For removing later
    let homeworkid = studenthwids[1].clone();

    let mut studenthw = Vec::new();
    // resolve id's
    for id in studenthwids {
        if id.is_empty() {
            continue;
        }
        let hw = client
            .get(format!("/api/homework/get?id={}", id))
            .header(authstudent.clone())
            .dispatch()
            .body_string()
            .unwrap();

        trace!("{}", hw);
        let hw: database::models::Homework = serde_json::from_str(&hw).unwrap();
        studenthw.push(hw);
    }
    let studenthw = 

    // check
    assert_eq!(
        studenthw
            .iter_mut()
            .map(|x| -> routes::Homework { x.to_owned().into() })
            .next()
            .unwrap(),
        homework.clone()
    );

    // get all the homework
    let allstudenthwids = client
        .get("/api/homework/get")
        .header(authstudent.clone())
        .dispatch()
        .body_string()
        .unwrap();
    let allstudenthwids: Vec<String> = serde_json::from_str(&allstudenthwids).unwrap();

    let mut allstudenthw = Vec::new();
    // resolve id's
    for id in allstudenthwids {
        if id.is_empty() {
            continue;
        }
        let hw = client
            .get(format!("/api/homework/get?id={}", id))
            .header(authstudent.clone())
            .dispatch()
            .body_string()
            .unwrap();

        trace!("{}", hw);
        let hw: database::models::Homework = serde_json::from_str(&hw).unwrap();
        allstudenthw.push(hw);
    }

    let comparer = |acc, x: &database::Homework| -> bool {
        x.name == homework.name
            && x.description == homework.description
            && x.points == homework.points
            && x.date == homework.date
            || acc
    };

    let comparer2 = |acc, x: &database::Homework| -> bool {
        x.name == homework2.name
            && x.description == homework2.description
            && x.points == homework2.points
            && x.date == homework2.date
            || acc
    };

    // check
    assert!(allstudenthw.iter().fold(false, comparer));
    assert!(allstudenthw.iter().fold(false, comparer2));

    // remove homework
    client
        .get(format!(
            "/api/homework/remove?class={}&homework={}",
            class, homeworkid
        ))
        .header(authteacher)
        .header(ContentType::JSON)
        .dispatch();

    // get homework student
    let newstudenthw = client
        .get(format!("/api/homework/get?class={}", class))
        .header(authstudent)
        .dispatch()
        .body_string()
        .unwrap();
    // check
    assert_eq!(newstudenthw, "[\"\"]");
    // maybe multiple classes for the student
}
