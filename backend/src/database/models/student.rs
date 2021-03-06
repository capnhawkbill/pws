use super::remove_from_class;
use super::{
    super::{getcsv, mkcsv, Id},
    Homework,
};
use anyhow::{anyhow, Result};
use rocket_contrib::databases::rusqlite::Connection;

/// The student
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Student {
    /// The id of the student
    pub id: Id,
    /// The name of the student
    pub name: String,
    /// The password of the student TODO plaintext lol
    pub password: String,
    /// Id's of the classes the student is in
    /// Stored as csv
    pub classes: Vec<Id>,
    /// Id's of the badges the student has
    /// Stored as csv
    pub badges: Vec<Id>,
    /// Finished homework
    pub homework: Vec<Id>,
    /// Total amount of points
    pub points: i32,
}

/// Create a table for the students
pub fn create_table(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE student (
            id          varchar(50),
            name        TEXT NOT NULL,
            password    TEXT NOT NULL,
            classes     TEXT,
            badges      TEXT,
            homework    TEXT,
            points      INTEGER
        )",
        &[],
    )?;
    Ok(())
}

/// Insert a student into the database
pub fn insert_student(conn: &Connection, student: &Student) -> Result<()> {
    trace!("Inserting student {}", student.name);
    // Convert to csv
    let classes = mkcsv(&student.classes)?;
    let badges = mkcsv(&student.badges)?;
    let homework = mkcsv(&student.homework)?;

    conn.execute(
        "INSERT INTO student (id, name, password, classes, badges, homework, points) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
        &[
            &student.id,
            &student.name,
            &student.password,
            &classes,
            &badges,
            &homework,
            &student.points,
        ],
    )?;
    Ok(())
}

/// Update a student in the database
pub fn update_student(conn: &Connection, student: &Student) -> Result<()> {
    trace!("Updating {:?}", student.name);

    // Convert to csv
    let classes = mkcsv(&student.classes)?;
    let badges = mkcsv(&student.badges)?;
    let homework = mkcsv(&student.homework)?;

    conn.execute(
        "UPDATE student SET name = ?2, password = ?3, classes = ?4, badges = ?5, homework = ?6, points = ?7 WHERE id = ?1",
        &[
            &student.id,
            &student.name,
            &student.password,
            &classes,
            &badges,
            &homework,
            &student.points,
        ],
    )?;

    Ok(())
}

/// Get a student from the database
pub fn get_student(conn: &Connection, id: Id) -> Result<Student> {
    trace!("Getting student with id {}", id);
    let mut stmt = conn.prepare("SELECT * FROM student where id = ?1")?;
    let mut students = stmt.query_map(&[&id], |row| {
        // Parse from csv
        let csvclasses = getcsv(row.get(3));
        if let Err(e) = csvclasses {
            return Err(e);
        }
        let csvclasses = csvclasses.unwrap();

        let mut classes: Vec<_>;
        // remove empty entry
        if csvclasses.len() == 1 && csvclasses[0].is_empty() {
            classes = Vec::new();
        } else {
            classes = csvclasses;
            if classes[0].is_empty() {
                classes.remove(0);
            }
        }

        let csvbadges = getcsv(row.get(4));
        if let Err(e) = csvbadges {
            return Err(e);
        }
        let csvbadges = csvbadges.unwrap();

        let mut badges: Vec<_>;
        // remove empty entry
        if csvbadges.len() == 1 && csvbadges[0].is_empty() {
            badges = Vec::new();
        } else {
            badges = csvbadges;
            if badges[0].is_empty() {
                badges.remove(0);
            }
        }

        let csvhomework = getcsv(row.get(5));
        if let Err(e) = csvhomework {
            return Err(e);
        }
        let csvhomework = csvhomework.unwrap();

        let mut homework: Vec<_>;
        // remove empty entry
        if csvhomework.len() == 1 && csvhomework[0].is_empty() {
            homework = Vec::new();
        } else {
            homework = csvhomework;
            if homework[0].is_empty() {
                homework.remove(0);
            }
        }
        Ok(Student {
            id: row.get(0),
            name: row.get(1),
            password: row.get(2),
            classes,
            badges,
            homework,
            points: row.get(6),
        })
    })?;

    // Check if exactly one student is found
    if let Some(student) = students.next() {
        if students.next().is_some() {
            Err(anyhow!("Multiple students found with this id: {}", id))
        } else {
            Ok(student??)
        }
    } else {
        Err(anyhow!("No students found with this id: {}", id))
    }
}

/// Remove a student from the database
pub fn remove_student(conn: &Connection, id: Id) -> Result<()> {
    trace!("Removing student {:?}", id);
    let student = get_student(&conn, id.clone())?;

    // Remove student from all classes
    for class in student.classes {
        remove_from_class(&conn, id.clone(), class)?;
    }

    conn.execute("DELETE FROM student WHERE id = ?1", &[&id])?;
    Ok(())
}

/// Get a student from the database using the username
pub fn get_student_by_name(conn: &Connection, name: &str) -> Result<Student> {
    trace!("Getting student {}", name);
    let mut stmt = conn.prepare("SELECT * FROM student where name = ?1")?;
    let mut students = stmt.query_map(&[&name], |row| {
        // Parse from csv
        let csvclasses = getcsv(row.get(3));
        if let Err(e) = csvclasses {
            return Err(e);
        }
        let csvclasses = csvclasses.unwrap();

        let mut classes: Vec<_>;
        // remove empty entry
        if csvclasses.len() == 1 && csvclasses[0].is_empty() {
            classes = Vec::new();
        } else {
            classes = csvclasses;
            if classes[0].is_empty() {
                classes.remove(0);
            }
        }

        let csvbadges = getcsv(row.get(4));
        if let Err(e) = csvbadges {
            return Err(e);
        }
        let csvbadges = csvbadges.unwrap();

        let mut badges: Vec<_>;
        // remove empty entry
        if csvbadges.len() == 1 && csvbadges[0].is_empty() {
            badges = Vec::new();
        } else {
            badges = csvbadges;
            if badges[0].is_empty() {
                badges.remove(0);
            }
        }

        let csvhomework = getcsv(row.get(5));
        if let Err(e) = csvhomework {
            return Err(e);
        }
        let csvhomework = csvhomework.unwrap();

        let mut homework: Vec<_>;
        // remove empty entry
        if csvhomework.len() == 1 && csvhomework[0].is_empty() {
            homework = Vec::new();
        } else {
            homework = csvhomework;
            if homework[0].is_empty() {
                homework.remove(0);
            }
        }
        // Parse from json
        Ok(Student {
            id: row.get(0),
            name: row.get(1),
            password: row.get(2),
            classes,
            badges,
            homework,
            points: row.get(6),
        })
    })?;

    // Check if exactly one student is found
    if let Some(student) = students.next() {
        if students.next().is_some() {
            Err(anyhow!(
                "Multiple students found with this username: {}",
                name
            ))
        } else {
            Ok(student??)
        }
    } else {
        Err(anyhow!("No students found with this username: {}", name))
    }
}

/// Award a badge to a student
pub fn award_badge(conn: &Connection, student: Id, badge: Id) -> Result<()> {
    trace!("Awarding badge {:?} to student {:?}", badge, student);
    let mut student = get_student(&conn, student)?;
    let mut badges = student.badges;
    badges.push(badge);

    student.badges = badges;

    update_student(&conn, &student)
}

/// Mark homework as finished and calculate points
pub fn finish_homework(conn: &Connection, student: Id, homework: &Homework) -> Result<()> {
    let mut student = get_student(&conn, student)?;
    if student.homework.contains(&homework.id) {
        return Err(anyhow!("Homework already finished"));
    }
    student.homework.push(homework.id.clone());
    student.points += homework.points;

    update_student(&conn, &student)
}

/// Unmark homework as finished and calculate points
pub fn unfinish_homework(conn: &Connection, student: Id, homework: &Homework) -> Result<()> {
    let mut student = get_student(&conn, student)?;
    student.homework = student
        .homework
        .iter_mut()
        .filter(|x| x != &&mut homework.id.clone())
        .map(|x| x.to_owned())
        .collect();
    student.points -= homework.points;

    update_student(&conn, &student)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn init() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[test]
    fn test_student_db() {
        init();
        // create mock student
        let student = Student {
            id: "ID".into(),
            name: "Elias".into(),
            password: "very secure".into(),
            classes: vec!["ClassId".into(), "Second ClassId".into()],
            badges: vec!["BadgeId".into(), "Second BadgeId".into()],
            homework: vec!["HomeworkId".into()],
            points: 4,
        };

        // create mock database
        let conn = &Connection::open_in_memory().unwrap();
        create_table(&conn).unwrap();

        // test if the inserted student can be retrieved
        insert_student(conn, &student).unwrap();
        let gotten = get_student(conn, "ID".into()).unwrap();
        trace!("gotten: {:?}", gotten);
        assert_eq!(student, gotten);
    }
}
