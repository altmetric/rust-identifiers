//! This module handles Digital Object Identifiers (DOIs).

use regex::Regex;
use Error;

/// A single DOI.
#[derive(Clone, Debug, PartialEq)]
pub struct Doi(String);

lazy_static! {
    static ref DOI_RE: Regex = Regex::new(r"\b10\.\d{4,9}/\S+\b").unwrap();
}

impl Doi {
    /// Try to create a new DOI from a string.
    ///
    /// # Example
    ///
    /// ```
    /// use identifiers::Doi;
    ///
    /// let doi = Doi::new("10.1234/foobar");
    /// ```
    pub fn new(text: &str) -> Result<Doi, Error> {
        match DOI_RE.find(text) {
            Some(mat) => Ok(Doi(mat.as_str().into())),
            None => Err(Error::InvalidDoi(text.into())),
        }
    }

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

#[cfg(test)]
mod tests {
    use Error;
    use super::*;

    #[test]
    fn new_returns_a_valid_doi() {
        let doi = Doi::new("10.1038/nplants.2015.3").unwrap();

        assert_eq!(doi, Doi("10.1038/nplants.2015.3".into()));
    }

    #[test]
    fn new_returns_an_err_if_given_an_invalid_doi() {
        match Doi::new("not a DOI") {
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
