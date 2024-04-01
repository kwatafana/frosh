use super::ID;
use crate::version;
use chrono::{Datelike, NaiveDate};
use rust_decimal::Decimal as Money;
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Course input from user
#[derive(Serialize, Deserialize, Clone)]
pub struct CourseInput {
    /// The name of the course
    pub name: String,
    /// The description of the course
    pub description: String,
    /// Topics covered in the course
    pub topics: Vec<String>,
    /// Goals of the course
    pub goals: Vec<String>,
    /// Textbooks for the course
    pub textbooks: Vec<String>,
    /// Grading information
    pub grading: Option<String>,
    /// The weekly calendar of the course
    pub calendar: Value,
    /// The level of the course
    pub level: Option<ID>,
    /// The price of the course
    pub price: Option<Money>,
    /// The duration of the course
    pub duration: Option<String>,
    /// Unique identification code for the course
    pub code: String,
}

/// Course databse input built from user input
#[derive(Serialize, Deserialize, Clone)]
pub struct CourseDatabseInput {
    /// The name of the course
    pub name: String,
    /// The description of the course
    pub description: String,
    /// Topics covered in the course
    pub topics: Vec<String>,
    /// Goals of the course
    pub goals: Vec<String>,
    /// Textbooks for the course
    pub textbooks: Vec<String>,
    /// Grading information
    pub grading: Option<String>,
    /// The weekly calendar of the course
    pub calendar: Value,
    /// The level of the course
    pub level: Option<ID>,
    /// The price of the course
    pub price: Option<Money>,
    /// The duration of the course
    pub duration: Option<String>,
    /// Unique identification code for the course
    pub code: String,
    /// The day the course was added
    pub added: NaiveDate,
    /// Last time the course was updated
    pub last_updated: NaiveDate,
    /// Data type schema version
    pub version: i16,
}

impl From<CourseInput> for CourseDatabseInput {
    fn from(input: CourseInput) -> Self {
        let current_date = chrono::Utc::now();
        let year = current_date.year();
        let month = current_date.month();
        let day = current_date.day();
        if let Some(date) = chrono::NaiveDate::from_ymd_opt(year, month, day) {
            Self {
                name: input.name,
                description: input.description,
                topics: input.topics,
                goals: input.goals,
                textbooks: input.textbooks,
                grading: input.grading,
                calendar: input.calendar,
                level: input.level,
                price: input.price,
                duration: input.duration,
                code: input.code,
                added: date,
                last_updated: date,
                version: version::COURSE_SCHEMA,
            }
        } else {
            panic!("Date error");
        }
    }
}
