//! 🚨 frosh errors

use std::fmt;

/// frosh Errors
#[derive(Debug)]
pub enum FroshErrors {
    /// Error with config
    Config,
}

impl std::error::Error for FroshErrors {}

impl fmt::Display for FroshErrors {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Config => write!(f, "Error with configuration"),
        }
    }
}
