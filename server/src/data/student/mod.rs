use super::ID;
use chrono::{DateTime, Utc};
use kommon::Gender;
use serde::{Deserialize, Serialize};

pub use student_photos::StudentPhotos;
pub mod input;
mod student_photos;

/// Student data type
#[derive(PartialEq, Debug, Serialize, Deserialize, Clone)]
pub struct Student {
    /// Student unique identifier
    pub id: i64,
    /// Students account username
    pub username: String,
    /// Students first name
    pub firstname: String,
    /// Students middle names
    pub middlenames: Option<Vec<String>>,
    /// Students last name
    pub lastname: String,
    /// Students email address
    pub email: String,
    /// Photo of the student
    pub photo: Option<ID>,
    /// Short bio of student
    pub bio: Option<String>,
    /// Student's date of birth
    pub dob: Option<DateTime<Utc>>,
    /// Student's number
    pub student_number: Option<String>,
    /// Student national ID number
    pub national_id: Option<String>,
    /// Student physical address
    pub physical_address: Option<String>,
    /// Students mobile phone number
    pub mobile: Option<String>,
    #[serde(deserialize_with = "Gender::deserialize")]
    #[serde(serialize_with = "Gender::serialize")]
    /// Students gender
    pub gender: Gender,
    /// Last time the student logged into Frosh
    pub last_login: Option<DateTime<Utc>>,
    /// Date when student joined institution
    pub joined: DateTime<Utc>,
    /// Data type schema version
    pub version: i16,
}
