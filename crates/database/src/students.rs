//! Frosh -- students database management

use crate::Error;
use froshdata::Student;
use rusqlite::{params, Connection};

mod sql {
    pub(super) const CREATE_STUDENTS_TABLE: &str = "
      CREATE TABLE IF NOT EXISTS students (
        id INTEGER PRIMARY KEY,                       -- The Identifier of the student, the Rust Type is `i64`
        username TEXT NOT NULL,                       -- Students account username
        firstname TEXT NOT NULL,                      -- Students first name
        middlenames TEXT,                             -- Students middle names
        lastname TEXT NOT NULL,                       -- Students last name
        email TEXT NOT NULL,                          -- Students email address
        bio TEXT,                                     -- Short bio of student
        courses TEXT DEFAULT('[]') NOT NULL,          -- JSON array string of the Students course IDs
        student_number TEXT,                          -- Student's number
        national_id TEXT,                             -- Student national ID number
        mobile TEXT,                                  -- Students mobile phone number
        gender TEXT,                                  -- Students gender
        last_login TEXT,                              -- Last time the student logged into Frosh
        joined TEXT NOT NULL,                         -- Date when student joined institution
        version INTEGER NOT NULL)                     -- Data type schema version";

    /// Add a student
    pub const ADD_STUDENT: &str = "
      INSERT INTO students (username,
        firstname,
        middlenames,
        lastname,
        email,
        bio,
        courses,
        student_number,
        national_id,
        mobile,
        gender,
        joined,
        version
       )
      VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9,?10,?11,?12,?13)";

    /// Get student by username
    pub const GET_STUDENT_BY_USERNAME: &str = "SELECT * FROM students WHERE username = ?;";

    /// Get all students
    pub const GET_ALL_STUDENTS: &str = "SELECT * FROM students;";
}

/// database controller
pub struct Database {
    /// Database file path
    path: String,
    /// An SQLite connection handle
    conn: Option<Connection>,
}

impl Database {
    /// Create new database
    pub fn new(path: &str) -> Self {
        Database {
            path: path.to_string(),
            conn: None,
        }
    }

    /// Connect database
    pub fn connect(&mut self) -> Result<(), Error> {
        // Open database connection
        let conn = Connection::open(self.path.clone()).map_err(|_| Error::Connection)?;
        self.conn = Some(conn);

        // Create database tables if they do not already exist
        match &mut self.conn {
            Some(conn) => {
                let tx = conn.transaction().map_err(|_| Error::Transaction)?;

                tx.execute(sql::CREATE_STUDENTS_TABLE, ())
                    .map_err(|_| Error::TableCreation)?;

                tx.commit().map_err(|_| Error::TableCreation)?;

                Ok(())
            }
            None => Err(Error::Connection),
        }
    }

    pub fn add_student(&self, student: &Student) -> Result<(), Error> {
        match &self.conn {
            Some(conn) => {
                if let Ok(courses) = serde_json::to_string(&student.courses) {
                    conn.execute(
                        sql::ADD_STUDENT,
                        params![
                            &student.username,
                            &student.firstname,
                            &student.middlenames,
                            &student.lastname,
                            &student.email,
                            &student.bio,
                            courses,
                            &student.student_number,
                            &student.national_id,
                            &student.mobile,
                            &student.gender,
                            &student.joined,
                            &student.version
                        ],
                    )
                    .map_err(|_| Error::Field)?;
                    Ok(())
                } else {
                    Err(Error::Field)
                }
            }
            None => Err(Error::Connection),
        }
    }

    pub fn get_student_by_username(&self, username: &str) -> Result<Option<Student>, Error> {
        match &self.conn {
            Some(conn) => {
                let mut stmt = conn
                    .prepare(sql::GET_STUDENT_BY_USERNAME)
                    .map_err(|_| Error::Field)?;
                let mut students: Vec<Student> = vec![];

                let property_iter = stmt
                    .query_map(params![username], |s| {
                        let courses_str: String = s.get(7).map_err(|_| Error::Field).unwrap();
                        let courses: Vec<u32> = serde_json::from_str(&courses_str)
                            .map_err(|_| Error::Field)
                            .unwrap();

                        Ok(Student {
                            username: s.get(1).map_err(|_| Error::Field).unwrap(),
                            firstname: s.get(2).map_err(|_| Error::Field).unwrap(),
                            middlenames: s.get(3).map_err(|_| Error::Field).unwrap(),
                            lastname: s.get(4).map_err(|_| Error::Field).unwrap(),
                            email: s.get(5).map_err(|_| Error::Field).unwrap(),
                            bio: s.get(6).map_err(|_| Error::Field).unwrap(),
                            courses,
                            student_number: s.get(8).map_err(|_| Error::Field).unwrap(),
                            national_id: s.get(9).map_err(|_| Error::Field).unwrap(),
                            mobile: s.get(10).map_err(|_| Error::Field).unwrap(),
                            gender: s.get(11).map_err(|_| Error::Field).unwrap(),
                            last_login: s.get(12).map_err(|_| Error::Field).unwrap(),
                            joined: s.get(13).map_err(|_| Error::Field).unwrap(),
                            version: s.get(14).map_err(|_| Error::Field).unwrap(),
                        })
                    })
                    .map_err(|_| Error::Field)?;

                for c in property_iter {
                    students.push(c.unwrap());
                }

                if students.is_empty() {
                    Ok(None)
                } else {
                    Ok(Some(students[0].clone()))
                }
            }
            None => Err(Error::Connection),
        }
    }

    /// Get all students
    pub fn get_all_properties(&self) -> Result<Vec<Student>, Error> {
        match &self.conn {
            Some(conn) => {
                let mut students: Vec<Student> = vec![];
                let mut stmt = conn.prepare(sql::GET_ALL_STUDENTS).unwrap();
                let property_iter = stmt
                    .query_map([], |s| {
                        let courses_str: String = s.get(7).map_err(|_| Error::Field).unwrap();
                        let courses: Vec<u32> = serde_json::from_str(&courses_str)
                            .map_err(|_| Error::Field)
                            .unwrap();

                        Ok(Student {
                            username: s.get(1).map_err(|_| Error::Field).unwrap(),
                            firstname: s.get(2).map_err(|_| Error::Field).unwrap(),
                            middlenames: s.get(3).map_err(|_| Error::Field).unwrap(),
                            lastname: s.get(4).map_err(|_| Error::Field).unwrap(),
                            email: s.get(5).map_err(|_| Error::Field).unwrap(),
                            bio: s.get(6).map_err(|_| Error::Field).unwrap(),
                            courses,
                            student_number: s.get(8).map_err(|_| Error::Field).unwrap(),
                            national_id: s.get(9).map_err(|_| Error::Field).unwrap(),
                            mobile: s.get(10).map_err(|_| Error::Field).unwrap(),
                            gender: s.get(11).map_err(|_| Error::Field).unwrap(),
                            last_login: s.get(12).map_err(|_| Error::Field).unwrap(),
                            joined: s.get(13).map_err(|_| Error::Field).unwrap(),
                            version: s.get(14).map_err(|_| Error::Field).unwrap(),
                        })
                    })
                    .unwrap();

                for student in property_iter {
                    students.push(student.unwrap());
                }

                Ok(students)
            }
            None => Err(Error::Connection),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_DB_PATH: &str = "STUDENTS_TEST_DATABASE.sqlite";

    #[test]
    fn connect_db() {
        let mut db = Database::new(TEST_DB_PATH);

        // Connect to database
        db.connect().unwrap();

        match db.conn {
            Some(_conn) => assert!(true),
            _ => assert!(false),
        }
    }

    #[test]
    fn test_store_get_student() {
        remove_test_db();
        let mut db = Database::new(TEST_DB_PATH);

        let student = Student {
            username: "jon".to_string(),
            firstname: "Jon".to_string(),
            middlenames: None,
            lastname: "Smith".to_string(),
            email: "jon@gmail.com".to_string(),
            bio: None,
            courses: vec![1, 2],
            student_number: None,
            national_id: None,
            mobile: None,
            gender: kommon::Gender::Female,
            last_login: None,
            joined: chrono::Utc::now(),
            version: 0,
        };

        db.connect().unwrap();
        db.add_student(&student).unwrap();

        let st = db.get_student_by_username("jon").unwrap();

        if let Some(s) = st {
            assert_eq!(s, student);
        } else {
            panic!("Could not get student from database");
        }
    }

    fn remove_test_db() {
        let test_db_path = std::path::Path::new(TEST_DB_PATH);
        if std::path::Path::exists(test_db_path) {
            std::fs::remove_file(test_db_path).unwrap();
        }
    }
}
