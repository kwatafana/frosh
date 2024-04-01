use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(PartialEq, Debug, Serialize, Deserialize, Clone)]
pub struct Calendar {
    weeks: Vec<Week>,
}

#[derive(PartialEq, Debug, Serialize, Deserialize, Clone)]
pub struct Week {
    days: Vec<Day>,
}

#[derive(PartialEq, Debug, Serialize, Deserialize, Clone)]
pub struct Day {
    date: NaiveDate,
}
