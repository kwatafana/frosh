use super::ID;
use chrono::NaiveDate;
pub use input::*;
use rust_decimal::Decimal as Money;
mod input;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
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
