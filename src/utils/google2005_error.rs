use askama;
use serde::Serialize;
use std::fmt::Display;

#[derive(Debug, Serialize)]
pub struct Google2005Error {
    user_error: Option<String>,
    internal_error: Option<String>,
}

impl Google2005Error {
    pub fn new(user_error: Option<&str>, internal_error: Option<&str>) -> Google2005Error {
        Google2005Error {
            user_error: user_error.map(|s| s.to_string()),
            internal_error: internal_error.map(|s| s.to_string()),
        }
    }
}

impl Display for Google2005Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.user_error {
            Some(e) => write!(f, "{}", e),
            None => write!(f, "An internal error occurred, please try again"),
        }
    }
}

impl From<askama::Error> for Google2005Error {
    fn from(e: askama::Error) -> Google2005Error {
        Google2005Error::new(None, Some(&e.to_string()))
    }
}

impl From<reqwest::Error> for Google2005Error {
    fn from(e: reqwest::Error) -> Google2005Error {
        Google2005Error::new(None, Some(&e.to_string()))
    }
}
