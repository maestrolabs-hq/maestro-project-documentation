//! Size, language and duplication gates for the documentation repository.

use std::collections::BTreeSet;
use std::fs;
use std::process::Command;

mod common;
use common::{repo_root, sources};

/// A dumping-ground tripwire, not a design rule. Function-level Clippy lints
/// enforce the corresponding local constraints.
const MAX_MODULE_LINES: usize = 250;

/// Latin-1 accented letters plus the OE ligatures, built from code points so
/// this file stays accent-free and cannot fail its own test.
const ACCENT_RANGES: &[(u32, u32)] = &[
    (0x00C0, 0x00D6),
    (0x00D8, 0x00F6),
    (0x00F8, 0x00FF),
    (0x0152, 0x0153),
];

/// Duplicate pairs looked at and kept, with the reason. The initial repository
/// has no accepted duplication.
const ACCEPTED_DUPLICATION: &[(&str, &str)] = &[];

#[test]
fn no_module_becomes_a_dumping_ground() {
    let over: Vec<String> = sources()
        .iter()
        .filter(|path| path.extension().and_then(|extension| extension.to_str()) == Some("rs"))
        .filter_map(|path| {
            let text = fs::read_to_string(path).ok()?;
            let lines = text
                .lines()
                .position(|line| line.trim_start().starts_with("#[cfg(test)]"))
                .unwrap_or_else(|| text.lines().count());
            (lines > MAX_MODULE_LINES).then(|| format!("  {}: {lines} lines", path.display()))
        })
        .collect();
    assert!(
        over.is_empty(),
        "Module over {MAX_MODULE_LINES} lines:\n{}\n",
        over.join("\n")
    );
}

#[test]
fn all_prose_is_english() {
    let accented = |character: char| {
        ACCENT_RANGES
            .iter()
            .any(|(low, high)| (*low..=*high).contains(&(character as u32)))
    };
    let root = repo_root();
    let mut found = Vec::new();
    for path in sources() {
        let Ok(text) = fs::read_to_string(&path) else {
            continue;
        };
        for (line_number, line) in text.lines().enumerate() {
            if line.chars().any(accented) {
                let relative = path.strip_prefix(&root).unwrap_or(&path);
                found.push(format!(
                    "  {}:{}: {}",
                    relative.display(),
                    line_number + 1,
                    line.trim()
                ));
            }
        }
    }
    assert!(
        found.is_empty(),
        "Accented characters, which usually means French:\n\n{}\n",
        found.join("\n")
    );
}

#[test]
fn a_failed_similarity_process_cannot_report_green() {
    let error = duplication_from_process(false, b"", b"invalid arguments\n")
        .expect_err("a failed process must be an error");

    assert_eq!(error, "similarity-rs failed: invalid arguments");
}

fn duplication_from_process(
    success: bool,
    stdout: &[u8],
    stderr: &[u8],
) -> Result<BTreeSet<String>, String> {
    if !success {
        return Err(format!(
            "similarity-rs failed: {}",
            String::from_utf8_lossy(stderr).trim()
        ));
    }

    Ok(String::from_utf8_lossy(stdout)
        .lines()
        .filter(|line| !line.trim_start().starts_with("Classes:"))
        .filter_map(|line| {
            let (left, right) = line.split_once(" <-> ")?;
            let name = |side: &str| side.split_whitespace().last().map(str::to_owned);
            Some(format!("{} <-> {}", name(left)?, name(right)?))
        })
        .collect())
}

fn detected_duplication() -> BTreeSet<String> {
    let output = Command::new("similarity-rs")
        .args(["--threshold", "0.85", "crates"])
        .current_dir(repo_root())
        .output()
        .expect(
            "similarity-rs must be installed: cargo binstall similarity-rs. \
             A gate that skips when its tool is missing reports green while \
             looking at nothing.",
        );

    duplication_from_process(output.status.success(), &output.stdout, &output.stderr)
        .unwrap_or_else(|error| panic!("{error}"))
}

#[test]
fn no_duplication_is_unaccounted_for() {
    let accepted: BTreeSet<&str> = ACCEPTED_DUPLICATION
        .iter()
        .map(|(pair, _reason)| *pair)
        .collect();
    let unexplained: Vec<String> = detected_duplication()
        .into_iter()
        .filter(|pair| !accepted.contains(pair.as_str()))
        .collect();
    assert!(
        unexplained.is_empty(),
        "Duplication with no recorded decision:\n\n{}\n\n\
         Factor out what is shared, or record the pair with its reason.\n",
        unexplained.join("\n")
    );
}

/// An allowlist nobody prunes becomes excuses for code that no longer exists.
#[test]
fn no_accepted_duplication_has_gone_stale() {
    let found = detected_duplication();
    let stale: Vec<&str> = ACCEPTED_DUPLICATION
        .iter()
        .map(|(pair, _reason)| *pair)
        .filter(|pair| !found.contains(*pair))
        .collect();
    assert!(
        stale.is_empty(),
        "No longer duplicated, remove from the list:\n\n  {}\n",
        stale.join("\n  ")
    );
}
