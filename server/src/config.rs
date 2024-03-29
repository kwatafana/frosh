//! Server configuration

use crate::error::FroshErrors;
use serde::Deserialize;
use std::{env, fs};

#[derive(Deserialize)]
pub struct FroshConfig {
    /// Postgres database user name
    pub database_user: String,
    /// Postgres database name
    pub database_name: String,
    /// Postgres database password
    pub database_password: String,
}

impl FroshConfig {
    /// Read server config file from path provided as an argument when
    /// the program was started.
    pub fn read() -> Result<Self, FroshErrors> {
        let arg = env::args().nth(2);
        match arg {
            Some(a) => {
                let toml_str = fs::read_to_string(a).map_err(|_| FroshErrors::Config)?;
                let config: FroshConfig =
                    toml::from_str(&toml_str).map_err(|_| FroshErrors::Config)?;
                Ok(config)
            }
            None => panic!("Path to config file is not provided!"),
        }
    }

    pub fn from_file(path: &str) -> Result<Self, FroshErrors> {
        let toml_str = fs::read_to_string(path).map_err(|_| FroshErrors::Config)?;
        let config: FroshConfig = toml::from_str(&toml_str).map_err(|_| FroshErrors::Config)?;
        Ok(config)
    }
}
