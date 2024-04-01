use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct StudentPhotos {
    pub data: Vec<u8>,
    pub mime: String,
    pub filename: Option<String>,
    pub added: NaiveDate,
    pub version: i16,
}
