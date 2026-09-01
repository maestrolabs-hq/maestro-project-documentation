//! The repository contract files that must exist before the site can be trusted.

use crate::diagnostic::Diagnostic;
use std::path::Path;

pub const REQUIRED_FILES: &[&str] = &[
    ".editorconfig",
    ".gitattributes",
    ".gitignore",
    ".pre-commit-config.yaml",
    ".github/CODEOWNERS",
    ".github/dependabot.yml",
    ".github/release-please/config.json",
    ".github/release-please/manifest.json",
    ".github/workflows/ci.yml",
    ".github/workflows/heavy.yml",
    ".github/workflows/pages.yml",
    "AGENTS.md",
    "BACKLOG.md",
    "CHANGELOG.md",
    "CONTEXT.md",
    "Cargo.lock",
    "Cargo.toml",
    "LICENSE",
    "NORTHSTAR.md",
    "README.md",
    "TODO.md",
    "book.toml",
    "clippy.toml",
    "deny.toml",
    "justfile",
    "rust-toolchain.toml",
    "src/SUMMARY.md",
];

#[must_use]
pub fn check(root: &Path) -> Vec<Diagnostic> {
    REQUIRED_FILES
        .iter()
        .map(|path| root.join(path))
        .filter(|path| !path.is_file())
        .map(|path| Diagnostic {
            path,
            line: None,
            message: "required repository file is missing".to_owned(),
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{
        fs,
        time::{SystemTime, UNIX_EPOCH},
    };

    #[test]
    fn reports_a_missing_required_file() {
        let root = std::env::temp_dir().join(format!(
            "maestro-sitecheck-required-{}",
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("clock")
                .as_nanos()
        ));
        fs::create_dir_all(&root).expect("fixture directory");
        for path in REQUIRED_FILES.iter().filter(|path| **path != "README.md") {
            let path = root.join(path);
            fs::create_dir_all(path.parent().expect("required file has parent"))
                .expect("fixture parent");
            fs::write(path, "fixture\n").expect("fixture file");
        }

        let diagnostics = check(&root);
        fs::remove_dir_all(&root).expect("remove fixture");

        assert_eq!(diagnostics.len(), 1);
        assert_eq!(diagnostics[0].path, root.join("README.md"));
    }
}
