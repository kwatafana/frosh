mod course;
mod instructor;
mod student;

pub use course::*;
pub use student::{Student, StudentPhotos};

/// Frosh entity unique identifier
pub type ID = i32;
