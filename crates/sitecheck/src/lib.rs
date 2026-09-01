//! Validation for the governed repository and the mdBook it publishes.

mod diagnostic;
mod required;

pub use diagnostic::Diagnostic;

use std::path::Path;

#[must_use]
pub fn check_repository(root: &Path) -> Vec<Diagnostic> {
    required::check(root)
}
