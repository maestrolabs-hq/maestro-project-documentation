//! End-to-end validation of passing and failing documentation repositories.

use sitecheck::check_repository;
use std::{path::PathBuf, process::Command};

fn fixture(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures")
        .join(name)
}

#[test]
fn valid_fixture_has_no_diagnostics() {
    let diagnostics = check_repository(&fixture("valid"));

    assert!(diagnostics.is_empty(), "{diagnostics:#?}");
}

#[test]
fn invalid_fixture_reports_links_status_and_mutable_sources() {
    let diagnostics = check_repository(&fixture("invalid"));
    let messages: Vec<&str> = diagnostics
        .iter()
        .map(|diagnostic| diagnostic.message.as_str())
        .collect();

    assert!(messages.contains(&"relative Markdown link does not resolve"));
    assert!(messages.contains(&"unknown Maestro status `planned`"));
    assert!(messages.contains(&"Maestro claim source must be an immutable GitHub permalink with a 40-character commit hash and line anchor"));
}

#[test]
fn invalid_fixture_makes_the_cli_fail() {
    let output = Command::new(env!("CARGO_BIN_EXE_sitecheck"))
        .arg(fixture("invalid"))
        .output()
        .expect("run sitecheck");

    assert!(!output.status.success());
    let stderr = String::from_utf8(output.stderr).expect("UTF-8 diagnostics");
    assert!(stderr.contains("relative Markdown link does not resolve"));
    assert!(stderr.contains("unknown Maestro status `planned`"));
    assert!(stderr.contains("40-character commit hash and line anchor"));
}
