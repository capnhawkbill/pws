use super::super::{getcsv, mkcsv, Id};
use super::{get_student, get_teacher};
use anyhow::{anyhow, Result};
use chrono::naive::NaiveDate;
use rocket_contrib::databases::rusqlite::Connection;

/// A Class
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Class {
    /// The id of the class
    pub id: Id,
    /// The name of the class
    pub name: String,
    /// The id's of the teachers in the class
    pub teachers: Vec<Id>,
    /// The id's of the students in the class
    pub students: Vec<Id>,
    /// The homework of this class
    pub homework: Vec<Homework>
}

/// A homework assignment
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Homework {
    /// Name/Title of the homework
    pub name: String,
    /// Due date of the homework
    pub date: NaiveDate,
    /// Description of the homework
    pub description: String,
}


pub fn create_table(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE class (
                id          TEXT NOT NULL PRIMARY KEY,
                name        TEXT NOT NULL,
                teachers    TEXT NOT NULL,
                students    TEXT NOT NULL,
                homework    TEXT NOT NULL
        )",
        &[],
    )?;

    Ok(())
}

/// Insert a class into the database
pub fn insert_class(conn: &Connection, class: &Class) -> Result<()> {
    trace!("Inserting class {}", class.name);
    let teachers = mkcsv(&class.teachers)?;
    let students = mkcsv(&class.students)?;
    let homework = serde_json::to_string(&class.homework)?;
    conn.execute(
        "INSERT INTO class (id, name, teachers, students, homework) VALUES (?1, ?2, ?3, ?4, ?5)",
        &[&class.id, &class.name, &teachers, &students, &homework],
    )?;
    Ok(())
}

/// Get a class from the database
pub fn get_class(conn: &Connection, id: Id) -> Result<Class> {
    trace!("Getting class with id {}", id);
    let mut stmt = conn.prepare("SELECT * FROM class where id = ?1")?;
    let mut classes = stmt.query_map(&[&id], |row| {
        // Parse from csv
        let teachers = getcsv(row.get(2));
        if let Err(e) = teachers {
            return Err(e);
        }
        let students = getcsv(row.get(3));
        if let Err(e) = students {
            return Err(e);
        }
        let homework: std::result::Result<Vec<Homework>, serde_json::Error> = serde_json::from_str(row.get::<_, String>(4).as_str());
        if let Err(e) = homework {
            return Err(e.into());
        }

        Ok(Class {
            id: row.get(0),
            name: row.get(1),
            teachers: teachers.unwrap(),
            students: students.unwrap(),
            homework: homework.unwrap(),
        })
    })?;

    // Check if exactly one class is found
    if let Some(class) = classes.next() {
        if classes.next().is_some() {
            Err(anyhow!("Multiple classes found with this id: {}", id))
        } else {
            Ok(class??)
        }
    } else {
        Err(anyhow!("No classes found with this id: {}", id))
    }
}

/// Add homework to a class
pub fn add_homework(conn: &Connection, homework: Homework, class: Id) -> Result<()> {
    trace!("Adding homework {:?} to class {:?}", homework, class);
    // Get the current homework
    let mut stmt = conn.prepare("SELECT homework FROM class WHERE id = ?1")?;
    let mut things = stmt.query_map(&[&class], |row| {
        serde_json::from_str(&row.get::<_, String>(0))
    })?;
    // Checks
    if let Some(current) = things.next() {
        if things.next().is_some() {
            return Err(anyhow!("Multiple classes with the same id"))
        }

        let mut new: Vec<Homework> = current??;
        new.push(homework);
        let new = serde_json::to_string(&new)?;

        conn.execute("UPDATE class SET homework = ?1 WHERE id = ?2", &[&new])?;
        Ok(())
    } else {
        Err(anyhow!("No class with this id"))
    }
}

/// Add a student or teacher to a class
pub fn add_to_class(conn: &Connection, id: Id, class: Id) -> Result<()> {
    trace!("Adding {:?} to class {:?}", id, class);
    // make sure the class exists
    let dbclass = get_class(&conn, class.clone())?;
    if let Ok(student) = get_student(&conn, id.clone()) {
        // add the class to the student
        let mut classes = student.classes;
        classes.push(class.clone());
        let classes = mkcsv(&classes)?;
        conn.execute("UPDATE student SET classes = ?1 WHERE id = ?2", &[&classes, &id])?;

        // add the student to the class
        let mut students = dbclass.students;
        students.push(id);
        let students = mkcsv(&students)?;
        conn.execute("UPDATE class SET students = ?1 WHERE id = ?2", &[&students, &class])?;
        Ok(())
    } else if let Ok(teacher) = get_teacher(&conn, id.clone()) {
        // add the class to the teacher
        let mut classes = teacher.classes;
        classes.push(class.clone());
        let classes = mkcsv(&classes)?;
        conn.execute("UPDATE teacher SET classes = ?1 WHERE id = ?2", &[&classes, &id])?;

        // add the teacher to the class
        let mut teachers = dbclass.teachers;
        teachers.push(id);
        let teachers = mkcsv(&teachers)?;
        conn.execute("UPDATE class SET teachers = ?1 WHERE id = ?2", &[&teachers, &class])?;
        Ok(())
    } else {
        Err(anyhow!("User {:?} doesn't exist", id))
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    fn init() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[test]
    fn test_class_db() {
        init();
        // create mock class
        let class = Class {
            id: "ID".into(),
            name: "Coolest".into(),
            teachers: vec!["TeacherA".into(), "TeacherB".into()],
            students: vec!["StudentA".into(), "StudentB".into()],
        };

        // create mock database
        let conn = &Connection::open_in_memory().unwrap();
        create_table(&conn).unwrap();

        // test if the inserted class can be retrieved
        insert_class(conn, &class).unwrap();
        let gotten = get_class(conn, "ID".into()).unwrap();
        assert_eq!(class, gotten);
    }
}
