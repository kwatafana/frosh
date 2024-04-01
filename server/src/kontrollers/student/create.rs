//! # Create student kontroller
//!
//! This __kontroller__ is used to create a new student
//! It gets the input from the HTTP request validates it
//! and creates the Student from the validated input. The Student
//! is stored in an SQLite database.

use crate::data::student::{Student, StudentInput};
use crate::database::{error::DatabaseError, student::StudentDatabase};
use kommon::Gender;
use kong::{inputs::UserInput, server, ErrorResponse, JsonValue, Kong, Kontrol, Method};
use kong_kontrollers::accounts::database::Database as AccountsDatabase;
use kong_kontrollers::login::is_admin;
use std::sync::{Arc, Mutex};

/// ✨ Create student kontroller
pub struct CreateStudentKontroller {
    /// Address to kontroller
    pub address: String,
    /// HTTP method supported by the kontroller
    pub method: Method,
    /// SQLite database handle
    pub database: Arc<Mutex<StudentDatabase>>,
    /// Accounts database
    pub accounts_database: Arc<Mutex<AccountsDatabase>>,
}

impl CreateStudentKontroller {
    /// Store uploaded student photos
    fn store_cover_photo(photo: server::input::post::BufferedFile) -> std::io::Result<String> {
        let filename: Option<String> = if let Some(filename) = &photo.filename {
            Some(filename.clone())
        } else {
            None
        };
        let timestamp = chrono::Utc::now().timestamp();
        let file = format!("{directory}/{timestamp}-{filename}");

        // Store photo in directory
        std::fs::write(&file, &photo.data)?;

        // remeber file path
        let directory = format!("uploads/student_photos/{dir_name}");
        let file = format!("{directory}/{timestamp}-{filename}");

        Ok(file)
    }
}

impl Kontrol for CreateStudentKontroller {
    /// Endpoint's address
    fn address(&self) -> String {
        self.address.clone()
    }

    /// Method supported by endpoint
    fn method(&self) -> Method {
        self.method
    }

    fn get_input(&self, request: &server::Request) -> Option<JsonValue> {
        if let Ok(input) = server::post_input!(request, {
        username: String,
        firstname: String,
        middlenames: Option<Vec<String>>,
        lastname: String,
        email: String,
        photo: Option<server::input::post::BufferedFile>,
        bio: Option<String>,
        courses: Vec<u32>,
        birth_year: Option<u8>,
        birth_month: Option<u8>,
        birth_date: Option<u8>,
        national_id: Option<String>,
        physical_address: Option<String>,
        mobile: Option<String>,
        gender: String,
        }) {
            let gender = input.gender;
            let gender: Result<Gender, kommon::Error> = gender.as_str().parse();

            if let Ok(gender) = gender {
                // store photo
                if let Some(photo) = input.photo {
                    if let Ok(photo) =
                        CreateStudentKontroller::store_cover_photo(&input.username, photo)
                    {
                        let input = StudentInput {
                            username: input.username,
                            firstname: input.firstname,
                            middlenames: input.middlenames,
                            lastname: input.lastname,
                            email: input.email,
                            photo: Some(photo),
                            bio: input.bio,
                            courses: input.courses,
                            birth_year: input.birth_year,
                            birth_month: input.birth_month,
                            birth_date: input.birth_date,
                            national_id: input.national_id,
                            physical_address: input.physical_address,
                            mobile: input.mobile,
                            gender,
                        };

                        Some(input.as_json())
                    } else {
                        None
                    }
                } else {
                    let input = StudentInput {
                        username: input.username,
                        firstname: input.firstname,
                        middlenames: input.middlenames,
                        lastname: input.lastname,
                        email: input.email,
                        photo: None,
                        bio: input.bio,
                        courses: input.courses,
                        birth_year: input.birth_year,
                        birth_month: input.birth_month,
                        birth_date: input.birth_date,
                        national_id: input.national_id,
                        physical_address: input.physical_address,
                        mobile: input.mobile,
                        gender,
                    };
                    Some(input.as_json())
                }
            } else {
                None
            }
        } else {
            None
        }
    }

    /// Validate user input
    fn validate(&self, input: Option<JsonValue>) -> Result<Option<JsonValue>, ()> {
        if let Some(input) = input {
            if let Ok(input) = StudentInput::from_json_str(input.to_string()) {
                if input.is_valid().is_ok() {
                    Ok(Some(input.as_json()))
                } else {
                    // TODO: proper error handling
                    Err(())
                }
            } else {
                // TODO: proper error handling
                Err(())
            }
        } else {
            // TODO: proper error handling
            Err(())
        }
    }
    /// Add student
    fn kontrol(&self, kong: &Kong) -> server::Response {
        if let Some(k) = &kong.kpassport {
            if let Ok(admin) = is_admin(k, self.accounts_database.clone()) {
                if admin {
                    if let Some(input) = &kong.input {
                        let input = StudentInput::from_json_str(input.to_string());

                        // Derive student from string
                        let student: Student = if let Ok(input) = input {
                            input.into()
                        } else {
                            return ErrorResponse::bad_request();
                        };

                        // Store student into the database
                        let res = self.database.lock().unwrap().create(&student);

                        match res {
                            Ok(()) => server::Response::json(&student).with_status_code(201),
                            Err(err) => match err {
                                DatabaseError::Field => ErrorResponse::bad_request(),
                                _ => ErrorResponse::internal(),
                            },
                        }
                    } else {
                        ErrorResponse::unauthorized()
                    }
                } else {
                    ErrorResponse::unauthorized()
                }
            } else {
                ErrorResponse::internal()
            }
        } else {
            ErrorResponse::unauthorized()
        }
    }
}
