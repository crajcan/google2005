use std::fmt::Display;

use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Google2005Error {
    report: String,
}

impl Google2005Error {
    pub fn new(report: String) -> Google2005Error {
        Google2005Error { report: report }
    }
}

impl Display for Google2005Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.report)
    }
}