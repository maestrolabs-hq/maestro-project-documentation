//! Terminal adapter for documentation validation.

use sitecheck::check_repository;
use std::{env, path::PathBuf, process::ExitCode};

fn main() -> ExitCode {
    let root = env::args_os()
        .nth(1)
        .map_or_else(|| PathBuf::from("."), PathBuf::from);
    let diagnostics = check_repository(&root);
    if diagnostics.is_empty() {
        ExitCode::SUCCESS
    } else {
        for diagnostic in diagnostics {
            eprintln!("{diagnostic}");
        }
        ExitCode::FAILURE
    }
}
