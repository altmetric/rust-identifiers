#![warn(missing_docs)]

//! A library for extracting scholarly identifiers from text.
//!
//! # Example
//!
//! ```
//! use identifiers::Doi;
//!
//! let doi: Doi = "10.1234/foobar".parse().unwrap();
//! let dois = Doi::extract("This article has a DOI of 10.1234/foobar");
//! ```

#[macro_use]
extern crate lazy_static;
extern crate regex;

use std::error;
use std::fmt;

pub use doi::Doi;

mod doi;

/// Error enumerates the possible error conditions when initialising identifier types.
#[derive(Clone, Debug, PartialEq)]
pub enum Error {
    /// The given string did not conform to the standard DOI format.
    InvalidDoi(String),
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::InvalidDoi(_) => "invalid DOI",
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::InvalidDoi(ref text) => write!(f, "{} is not a valid DOI", text),
        }
    }
}
