//! Frosh -- students database management
mod error;
pub mod student_photos;
pub use error::DatabaseError;
use postgres::{Client, NoTls};

/// Students database management
pub struct FroshDatabase {
    /// postgres client
    pub client: Option<Client>,
}

impl FroshDatabase {
    /// create new database
    pub fn new(db: &str, user: &str, password: &str) -> Result<Self, DatabaseError> {
        let params = format!("postgresql://localhost/{db}?user={user}&password={password}");
        let client = Client::connect(&params, NoTls).map_err(|_| DatabaseError::SQL)?;

        Ok(FroshDatabase {
            client: Some(client),
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::config::FroshConfig;

    #[test]
    fn connect_db() {
        let config = FroshConfig::from_file("example-config.toml").unwrap();
        let db = FroshDatabase::new(
            &config.database_name,
            &config.database_user,
            &config.database_password,
        )
        .unwrap();
        match db.client {
            Some(_conn) => assert!(true),
            _ => assert!(false),
        }
    }
}
