#![warn(missing_docs)]

//! A library for extracting scholarly identifiers from text.
//!
//! # Example
//!
//! ```
//! use identifiers::Doi;
//!
//! let dois = Doi::extract("This article has a DOI of 10.1234/foobar");
//! ```

#[macro_use]
extern crate lazy_static;
extern crate regex;

pub use doi::Doi;

mod doi;

/// Error enumerates the possible error conditions when initialising identifier types.
#[derive(Clone, Debug, PartialEq)]
pub enum Error {
    /// The given string did not conform to the standard DOI format.
    InvalidDoi(String),
}
