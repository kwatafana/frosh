use super::ID;
use chrono::{DateTime, Utc};
use kommon::Gender;
use kong::{
    inputs::UserInput, json, json_from_str, validate::ValidationError, JsonError, JsonValue,
};
use serde::{Deserialize, Serialize};

/// ## ⌨️ Student creation input
#[derive(Serialize, Deserialize, Clone)]
pub struct StudentInput {
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
    /// Year the student was born
    pub birth_year: Option<u8>,
    /// Month the student was born
    pub birth_month: Option<u8>,
    /// Day the student was born
    pub birth_date: Option<u8>,
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
}

impl StudentInput {
    /// new generic resource
    pub fn as_json(&self) -> JsonValue {
        json!({
            "username": self.username,
            "firstname": self.firstname,
            "middlenames": self.middlenames,
            "lastname": self.lastname,
            "email": self.email,
            "photo": self.photo,
            "bio": self.bio,
            "birth_year": self.birth_year,
            "birth_month": self.birth_month,
            "birth_date": self.birth_date,
            "national_id": self.national_id,
            "physical_address": self.physical_address,
            "mobile": self.mobile,
            "gender": self.gender.to_string()
        })
    }

    /// from json
    pub fn from_json_str(json_str: String) -> Result<StudentInput, JsonError> {
        let a: StudentInput = json_from_str(&json_str)?;
        Ok(a)
    }
}

impl UserInput for StudentInput {
    fn is_valid(&self) -> Result<(), ValidationError> {
        // TODO: validate input properly
        Ok(())
    }
}
