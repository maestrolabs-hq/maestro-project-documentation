//! Validation for the governed repository and the mdBook it publishes.

mod book;
mod claim;
mod diagnostic;
mod markdown;
mod required;

pub use diagnostic::Diagnostic;

use std::path::Path;

#[must_use]
pub fn check_repository(root: &Path) -> Vec<Diagnostic> {
    let mut diagnostics = required::check(root);
    diagnostics.extend(book::check(root));
    diagnostics
}
