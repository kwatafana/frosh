use super::DatabaseError;
use crate::data::{Course, CourseDatabseInput, ID};
use postgres::error::SqlState;
use postgres::Client;
use serde_json::json;

/// SQL queries for the student_photos database table
pub struct CoursesSql;

impl CoursesSql {
    /// Create a new student_photo row
    pub fn create(client: &mut Client, course: CourseDatabseInput) -> Result<(), DatabaseError> {
        let row = client
            .execute(
                "INSERT INTO public.courses (
                  name,
                  description,
                  topics,
                  goals,
                  textbooks,
                  grading,
                  calendar,
                  level,
                  price,
                  duration,
                  code,
                  added,
                  last_updated,
                  version
                 ) 
                 VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14);",
                &[
                    &course.name,
                    &course.description,
                    &course.topics,
                    &course.goals,
                    &course.textbooks,
                    &course.grading,
                    &course.calendar,
                    &course.level,
                    &course.price,
                    &course.duration,
                    &course.code,
                    &course.added,
                    &course.last_updated,
                    &course.version,
                ],
            )
            .map_err(|_| DatabaseError::SQL)?;
        Ok(())
    }

    pub fn read(client: &mut Client, id: ID) -> Result<Option<Course>, DatabaseError> {
        let row = client
            .query("SELECT * FROM courses WHERE id = $1", &[&id])
            .map_err(|_| DatabaseError::SQL)?;

        if row.is_empty() {
            Ok(None)
        } else {
            Ok(Some(Course {
                id: row[0].get("id"),
                name: row[0].get("name"),
                description: row[0].get("description"),
                topics: row[0].get("topics"),
                goals: row[0].get("goals"),
                textbooks: row[0].get("textbooks"),
                grading: row[0].get("grading"),
                calendar: row[0].get("calendar"),
                level: row[0].get("level"),
                price: row[0].get("price"),
                duration: row[0].get("duration"),
                code: row[0].get("code"),
                added: row[0].get("added"),
                last_updated: row[0].get("last_updated"),
                version: row[0].get("version"),
            }))
        }
    }

    pub fn read_by_code(client: &mut Client, code: &str) -> Result<Option<Course>, DatabaseError> {
        let row = client
            .query("SELECT * FROM courses WHERE code = $1", &[&code])
            .map_err(|_| DatabaseError::SQL)?;

        if row.is_empty() {
            Ok(None)
        } else {
            Ok(Some(Course {
                id: row[0].get("id"),
                name: row[0].get("name"),
                description: row[0].get("description"),
                topics: row[0].get("topics"),
                goals: row[0].get("goals"),
                textbooks: row[0].get("textbooks"),
                grading: row[0].get("grading"),
                calendar: row[0].get("calendar"),
                level: row[0].get("level"),
                price: row[0].get("price"),
                duration: row[0].get("duration"),
                code: row[0].get("code"),
                added: row[0].get("added"),
                last_updated: row[0].get("last_updated"),
                version: row[0].get("version"),
            }))
        }
    }

    pub fn update(
        client: &mut Client,
        code: &str,
        course: &CourseDatabseInput,
    ) -> Result<(), DatabaseError> {
        let mut transaction = client.transaction().map_err(|_| DatabaseError::SQL)?;
        transaction
            .execute(
                "UPDATE courses SET name = $1 WHERE code = $2",
                &[&course.name, &code],
            )
            .map_err(|_| DatabaseError::SQL)?;
        transaction
            .execute(
                "UPDATE courses SET description = $1 WHERE code = $2",
                &[&course.description, &code],
            )
            .unwrap();
        //.map_err(|_| DatabaseError::SQL)?;
        transaction
            .execute(
                "UPDATE courses SET topics = $1 WHERE code = $2",
                &[&course.topics, &code],
            )
            .map_err(|_| DatabaseError::SQL)?;
        transaction
            .execute(
                "UPDATE courses SET goals = $1 WHERE code = $2",
                &[&course.goals, &code],
            )
            .map_err(|_| DatabaseError::SQL)?;
        transaction
            .execute(
                "UPDATE courses SET textbooks = $1 WHERE code = $2",
                &[&course.textbooks, &code],
            )
            .map_err(|_| DatabaseError::SQL)?;
        transaction
            .execute(
                "UPDATE courses SET grading = $1 WHERE code = $2",
                &[&course.grading, &code],
            )
            .map_err(|_| DatabaseError::SQL)?;
        transaction
            .execute(
                "UPDATE courses SET calendar = $1 WHERE code = $2",
                &[&course.calendar, &code],
            )
            .map_err(|_| DatabaseError::SQL)?;
        transaction
            .execute(
                "UPDATE courses SET level = $1 WHERE code = $2",
                &[&course.level, &code],
            )
            .map_err(|_| DatabaseError::SQL)?;
        transaction
            .execute(
                "UPDATE courses SET price = $1 WHERE code = $2",
                &[&course.price, &code],
            )
            .map_err(|_| DatabaseError::SQL)?;
        transaction
            .execute(
                "UPDATE courses SET duration = $1 WHERE code = $2",
                &[&course.duration, &code],
            )
            .map_err(|_| DatabaseError::SQL)?;
        transaction
            .execute(
                "UPDATE courses SET code = $1 WHERE code = $2",
                &[&course.code, &code],
            )
            .map_err(|_| DatabaseError::SQL)?;
        transaction
            .execute(
                "UPDATE courses SET added = $1 WHERE code = $2",
                &[&course.added, &code],
            )
            .map_err(|_| DatabaseError::SQL)?;
        transaction
            .execute(
                "UPDATE courses SET code = $1 WHERE code = $2",
                &[&course.code, &code],
            )
            .map_err(|_| DatabaseError::SQL)?;

        transaction
            .execute(
                "UPDATE courses SET last_updated = $1 WHERE code = $2",
                &[&course.last_updated, &code],
            )
            .map_err(|_| DatabaseError::SQL)?;
        transaction
            .execute(
                "UPDATE courses SET version = $1 WHERE code = $2",
                &[&course.version, &code],
            )
            .map_err(|_| DatabaseError::SQL)?;

        transaction.commit().map_err(|_| DatabaseError::SQL)?;

        Ok(())
    }

    pub fn delete(client: &mut Client, id: ID) -> Result<(), DatabaseError> {
        client
            .execute("DELETE FROM courses WHERE id = $1", &[&id])
            .unwrap();
        //.map_err(|_| DatabaseError::SQL)?;
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::config::FroshConfig;
    use crate::database::FroshDatabase;

    #[test]
    fn test_create_read() {
        let config = FroshConfig::from_file("example-config.toml").unwrap();
        let db = FroshDatabase::new(
            &config.database_name,
            &config.database_user,
            &config.database_password,
        )
        .unwrap();

        if let Some(date) = chrono::NaiveDate::from_ymd_opt(2020, 02, 2) {
            let course = CourseDatabseInput {
                name: "Computer Science".to_string(),
                description: "Computer Science 101".to_string(),
                topics: vec![],
                goals: vec![],
                textbooks: vec![],
                grading: None,
                calendar: json!(null),
                level: None,
                price: None,
                duration: None,
                code: "CS".to_string(),
                added: date,
                last_updated: date,
                version: 0,
            };

            if let Some(mut client) = db.client {
                super::CoursesSql::create(&mut client, course.clone()).unwrap();
                let course1 = super::CoursesSql::read_by_code(&mut client, &course.code).unwrap();

                if let Some(course1) = course1 {
                    assert_eq!(course.name, course1.name);

                    if let Some(course2) = super::CoursesSql::read(&mut client, course1.id).unwrap()
                    {
                        assert_eq!(course.name, course2.name);
                        assert_eq!(course1, course2);
                    }
                } else {
                    panic!("wiip");
                }
            } else {
                panic!("woop");
            }
        } else {
            panic!("woo");
        }
    }

    #[test]
    fn test_update_delete() {
        let config = FroshConfig::from_file("example-config.toml").unwrap();
        let db = FroshDatabase::new(
            &config.database_name,
            &config.database_user,
            &config.database_password,
        )
        .unwrap();

        if let Some(date) = chrono::NaiveDate::from_ymd_opt(2020, 02, 2) {
            let course = CourseDatabseInput {
                name: "Physics".to_string(),
                description: "Computer Science 101".to_string(),
                topics: vec![],
                goals: vec![],
                textbooks: vec![],
                grading: None,
                calendar: json!(null),
                level: None,
                price: None,
                duration: None,
                code: "phy".to_string(),
                added: date,
                last_updated: date,
                version: 0,
            };

            let course_update = CourseDatabseInput {
                name: "Education".to_string(),
                description: "Computer Science 2".to_string(),
                topics: vec![],
                goals: vec![],
                textbooks: vec![],
                grading: None,
                calendar: json!(null),
                level: None,
                price: None,
                duration: None,
                code: "edu".to_string(),
                added: date,
                last_updated: date,
                version: 0,
            };

            if let Some(mut client) = db.client {
                super::CoursesSql::create(&mut client, course.clone()).unwrap();
                super::CoursesSql::update(&mut client, &course.code, &course_update.clone())
                    .unwrap();
                if let Some(updated) =
                    super::CoursesSql::read_by_code(&mut client, &course_update.code).unwrap()
                {
                    assert_eq!(course_update.name, updated.name);

                    // delete course
                    super::CoursesSql::delete(&mut client, updated.id).unwrap();
                    let deleted_course = super::CoursesSql::read(&mut client, updated.id).unwrap();
                    assert_eq!(deleted_course, None);
                } else {
                    panic!("tring");
                }
            } else {
                panic!("woop");
            }
        } else {
            panic!("woo");
        }
    }
}
