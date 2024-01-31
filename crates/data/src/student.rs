//! Frosh Student Data Type

use crate::gender::Gender;
use chrono::{DateTime, Utc};

/// Student data type
pub struct Student{
    /// Students account username
    pub username: String,
    /// Students first name
    pub firstname: String,
    /// Students middle names
    pub middlenames: String,
    /// Students last name
    pub lastname: String,
    /// Students email address
    pub email: String,
    /// Short bio of student
    pub bio: String,
    /// Courses, it is u32 because it represents the course ID in
    /// the courses database
    pub courses: Vec<u32>,
    /// Student's number
    pub student_number: String,
    /// Student national ID number
    pub national_id: String,
    /// Students mobile phone number
    pub mobile: String,
    /// Students gender
    pub gender: Gender,
    /// Last time the student logged into Frosh
    pub last_login: DateTime<Utc>,
    /// Data type schema version
    pub version: u32,
}
