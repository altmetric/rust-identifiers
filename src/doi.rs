//! This module handles Digital Object Identifiers (DOIs).
//!
//! # Example
//!
//! A DOI is represented by the struct `Doi` which implements `FromStr` meaning that they can be
//! created by calling `parse` on a string slice:
//!
//! ```
//! use identifiers::Doi;
//!
//! let doi: Doi = "10.1038/nplants.2015.3".parse().unwrap();
//! ```
//!
//! Alternatively, a string slice can be scanned for zero or more DOIs using `extract`:
//!
//! ```
//! use identifiers::Doi;
//!
//! let dois = Doi::extract("I love 10.1234/foo and 10.5678/bar");
//! ```

use regex::Regex;
use std::fmt;
use std::str;

use Error;

/// A single DOI.
#[derive(Clone, Debug, PartialEq)]
pub struct Doi(String);

lazy_static! {
    static ref DOI_RE: Regex = Regex::new(r"\b10\.\d{4,9}/\S+\b").unwrap();
}

impl Doi {
    /// Extract valid DOIs from a given string.
    ///
    /// # Example
    ///
    /// ```
    /// use identifiers::Doi;
    ///
    /// let dois = Doi::extract("I like 10.1234/foobar and 10.1234/bazquux");
    /// ```
    pub fn extract(text: &str) -> Vec<Doi> {
        DOI_RE.find_iter(text).map(|mat| Doi(mat.as_str().into())).collect()
    }
}

impl str::FromStr for Doi {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match DOI_RE.find(s) {
            Some(mat) => Ok(Doi(mat.as_str().into())),
            None => Err(Error::InvalidDoi(s.into())),
        }
    }
}

impl fmt::Display for Doi {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use Error;
    use super::*;

    #[test]
    fn can_parse_a_valid_doi() {
        let doi: Doi = "10.1038/nplants.2015.3".parse().unwrap();

        assert_eq!(doi, Doi("10.1038/nplants.2015.3".into()));
    }

    #[test]
    fn can_parse_an_invalid_doi() {
        let doi = "not a DOI".parse::<Doi>();

        match doi {
            Err(Error::InvalidDoi(text)) => assert_eq!(text, "not a DOI"),
            _ => panic!("Expected an invalid DOI"),
        }
    }

    #[test]
    fn extract_returns_a_doi() {
        let dois = Doi::extract("I love 10.1038/nplants.2015.3");

        assert_eq!(dois, vec![Doi("10.1038/nplants.2015.3".into())]);
    }

    #[test]
    fn extract_returns_multiple_dois() {
        let dois = Doi::extract("I love 10.1038/nplants.2015.3 and 10.1038/nplants.2015.4");

        assert_eq!(dois,
                   vec![Doi("10.1038/nplants.2015.3".into()),
                        Doi("10.1038/nplants.2015.4".into())]);
    }

    #[test]
    fn extract_returns_an_empty_vec() {
        let dois = Doi::extract("No DOIs here");

        assert_eq!(dois, Vec::new());
    }
}
