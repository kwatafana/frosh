use super::DatabaseError;
use crate::data::student::StudentPhotos;
use postgres::Client;

/// SQL queries for the student_photos database table
pub struct StudentPhotosSql;

impl StudentPhotosSql {
    /// Create a new student_photo row
    pub fn create(client: &mut Client, student_photos: StudentPhotos) -> Result<(), DatabaseError> {
        client
            .execute(
                "INSERT INTO public.student_photos (
                  data, mime, filename, added, version
                 ) 
                 VALUES ($1, $2, $3, $4, $5)",
                &[
                    &student_photos.data,
                    &student_photos.mime,
                    &student_photos.filename,
                    &student_photos.added,
                    &student_photos.version,
                ],
            )
            .unwrap();
        //            .map_err(|_| DatabaseError::Connection)?;
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use crate::config::FroshConfig;
    use crate::data::student::StudentPhotos;
    use crate::database::FroshDatabase;

    #[test]
    fn test_create() {
        let config = FroshConfig::from_file("example-config.toml").unwrap();
        let db = FroshDatabase::new(
            &config.database_name,
            &config.database_user,
            &config.database_password,
        )
        .unwrap();
        if let Some(added) = chrono::NaiveDate::from_ymd_opt(2020, 02, 2) {
            let student_photos = StudentPhotos {
                data: vec![],
                mime: "TEST".to_string(),
                filename: None,
                added,
                version: 0,
            };

            if let Some(mut client) = db.client {
                let res = super::StudentPhotosSql::create(&mut client, student_photos);
                if let Ok(_res) = res {
                    assert!(true);
                } else {
                    panic!("waap");
                }
            } else {
                panic!("woop");
            }
        } else {
            panic!("woo");
        }
    }
}
