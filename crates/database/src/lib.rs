//! Frosh database management

#![doc(html_favicon_url = "https://kwatafana.org/frosh/logo.jpeg")]
#![doc(html_logo_url = "https://kwatafana.org/frosh/logo.jpeg")]
#![warn(missing_docs, unreachable_pub, future_incompatible, rust_2018_idioms)]

pub use students::Database;
pub use error::Error;

mod students;
mod error;
