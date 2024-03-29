use super::ID;
use chrono::{DateTime, Utc};
use rust_decimal::Decimal as Money;

pub struct Course {
    /// Unique identifier for the course
    pub id: ID,
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
    pub calendar: Option<Calendar>,
    /// The level of the course
    pub level: Option<ID>,
    /// The price of the course
    pub price: Option<Money>,
    /// The duration of the course
    pub duration: Option<std::time::Duration>,
    /// The day the course was added
    pub added: DateTime<Utc>,
    /// Data type schema version
    pub version: u16,
}

pub struct Calendar;
pub struct Module;
