//! Actionable validation failures with an optional source line.

use std::{fmt, path::PathBuf};

#[derive(Debug, PartialEq, Eq)]
pub struct Diagnostic {
    pub path: PathBuf,
    pub line: Option<usize>,
    pub message: String,
}

impl fmt::Display for Diagnostic {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.line {
            Some(line) => write!(
                formatter,
                "{}:{line}: {}",
                self.path.display(),
                self.message
            ),
            None => write!(formatter, "{}: {}", self.path.display(), self.message),
        }
    }
}
