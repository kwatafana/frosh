//! Frosh Student Data Type

use chrono::{DateTime, Utc};
use kommon::Gender;
use serde::{Deserialize, Serialize};

/// Student data type
#[derive(PartialEq, Debug, Serialize, Deserialize)]
pub struct Student {
    /// Students account username
    pub username: String,
    /// Students first name
    pub firstname: String,
    /// Students middle names
    pub middlenames: Option<String>,
    /// Students last name
    pub lastname: String,
    /// Students email address
    pub email: String,
    /// Short bio of student
    pub bio: Option<String>,
    /// Courses, it is u32 because it represents the course ID in
    /// the courses database
    pub courses: Vec<u32>,
    /// Student's number
    pub student_number: Option<String>,
    /// Student national ID number
    pub national_id: Option<String>,
    /// Students mobile phone number
    pub mobile: Option<String>,
    #[serde(deserialize_with = "Gender::deserialize")]
    #[serde(serialize_with = "Gender::serialize")]
    /// Students gender
    pub gender: Gender,
    /// Last time the student logged into Frosh
    pub last_login: DateTime<Utc>,
    /// Data type schema version
    pub version: u32,
}
