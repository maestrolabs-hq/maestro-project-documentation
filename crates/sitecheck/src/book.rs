//! mdBook navigation and relative-link checks rooted at the repository.

use crate::{Diagnostic, claim, markdown::inline_targets};
use std::{
    fs,
    path::{Component, Path, PathBuf},
};

#[must_use]
pub fn check(root: &Path) -> Vec<Diagnostic> {
    let source = root.join("src");
    let summary = source.join("SUMMARY.md");
    let mut diagnostics = Vec::new();
    let summary_markdown = match fs::read_to_string(&summary) {
        Ok(markdown) => markdown,
        Err(error) => {
            diagnostics.push(Diagnostic {
                path: summary,
                line: None,
                message: format!("cannot read book summary: {error}"),
            });
            return diagnostics;
        }
    };

    diagnostics.extend(claim::check(&summary, &summary_markdown));
    diagnostics.extend(check_targets(
        &summary,
        &source,
        &source,
        &summary_markdown,
        "SUMMARY references a missing page",
    ));

    let mut pages = Vec::new();
    collect_markdown(&source, &mut pages, &mut diagnostics);
    for page in pages {
        if page == summary {
            continue;
        }
        match fs::read_to_string(&page) {
            Ok(markdown) => {
                diagnostics.extend(check_targets(
                    &page,
                    &source,
                    page.parent().unwrap_or(&source),
                    &markdown,
                    "relative Markdown link does not resolve",
                ));
                diagnostics.extend(claim::check(&page, &markdown));
            }
            Err(error) => diagnostics.push(Diagnostic {
                path: page,
                line: None,
                message: format!("cannot read Markdown page: {error}"),
            }),
        }
    }
    diagnostics
}

fn check_targets(
    source: &Path,
    source_root: &Path,
    base: &Path,
    markdown: &str,
    missing_message: &str,
) -> Vec<Diagnostic> {
    let mut diagnostics = Vec::new();
    for (line, target) in inline_targets(markdown) {
        if is_external(&target) {
            continue;
        }
        if is_machine_specific(&target) {
            diagnostics.push(Diagnostic {
                path: source.to_path_buf(),
                line: Some(line),
                message: "absolute or machine-specific Markdown path is not allowed".to_owned(),
            });
            continue;
        }
        let target = target.split('#').next().unwrap_or_default();
        if target.is_empty() {
            continue;
        }
        if !stays_within_source(source_root, base, target) {
            diagnostics.push(Diagnostic {
                path: source.to_path_buf(),
                line: Some(line),
                message: "Markdown path escapes the book source directory".to_owned(),
            });
            continue;
        }
        let path = base.join(target);
        let exists = path.is_file()
            || (path.extension().and_then(|extension| extension.to_str()) == Some("html")
                && path.with_extension("md").is_file());
        if !exists {
            diagnostics.push(Diagnostic {
                path: source.to_path_buf(),
                line: Some(line),
                message: missing_message.to_owned(),
            });
        }
    }
    diagnostics
}

fn stays_within_source(source_root: &Path, base: &Path, target: &str) -> bool {
    let Ok(relative_base) = base.strip_prefix(source_root) else {
        return false;
    };
    let mut depth = relative_base
        .components()
        .filter(|component| matches!(component, Component::Normal(_)))
        .count();
    for component in Path::new(target).components() {
        match component {
            Component::ParentDir if depth == 0 => return false,
            Component::ParentDir => depth -= 1,
            Component::Normal(_) => depth += 1,
            Component::CurDir => {}
            Component::Prefix(_) | Component::RootDir => return false,
        }
    }
    true
}

fn collect_markdown(directory: &Path, pages: &mut Vec<PathBuf>, diagnostics: &mut Vec<Diagnostic>) {
    let entries = match fs::read_dir(directory) {
        Ok(entries) => entries,
        Err(error) => {
            diagnostics.push(Diagnostic {
                path: directory.to_path_buf(),
                line: None,
                message: format!("cannot read book source directory: {error}"),
            });
            return;
        }
    };
    let mut paths = Vec::new();
    for entry in entries {
        match entry {
            Ok(entry) => paths.push(entry.path()),
            Err(error) => diagnostics.push(Diagnostic {
                path: directory.to_path_buf(),
                line: None,
                message: format!("cannot read book source entry: {error}"),
            }),
        }
    }
    paths.sort();
    for path in paths {
        if path.is_dir() {
            collect_markdown(&path, pages, diagnostics);
        } else if path.extension().and_then(|extension| extension.to_str()) == Some("md") {
            pages.push(path);
        }
    }
}

fn is_external(target: &str) -> bool {
    ["http:", "https:", "mailto:"].iter().any(|scheme| {
        target
            .get(..scheme.len())
            .is_some_and(|prefix| prefix.eq_ignore_ascii_case(scheme))
    })
}

fn is_machine_specific(target: &str) -> bool {
    let bytes = target.as_bytes();
    target.starts_with('/')
        || target.contains('\\')
        || target.starts_with("~/")
        || (bytes.len() > 1 && bytes[0].is_ascii_alphabetic() && bytes[1] == b':')
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{
        fs,
        path::PathBuf,
        time::{SystemTime, UNIX_EPOCH},
    };

    fn fixture(summary: &str, index: &str) -> PathBuf {
        let root = std::env::temp_dir().join(format!(
            "maestro-sitecheck-book-{}",
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("clock")
                .as_nanos()
        ));
        fs::create_dir_all(root.join("src")).expect("fixture source directory");
        fs::write(root.join("src/SUMMARY.md"), summary).expect("fixture summary");
        fs::write(root.join("src/index.md"), index).expect("fixture index");
        root
    }

    #[test]
    fn reports_a_summary_page_that_does_not_exist() {
        let root = fixture("[Missing](missing.md)\n", "# Home\n");

        let diagnostics = check(&root);

        assert!(
            diagnostics
                .iter()
                .any(|diagnostic| diagnostic.message == "SUMMARY references a missing page")
        );
        fs::remove_dir_all(root).expect("remove fixture");
    }

    #[test]
    fn reports_a_broken_relative_link() {
        let root = fixture("[Home](index.md)\n", "[Missing](missing.md)\n");

        let diagnostics = check(&root);

        assert!(
            diagnostics.iter().any(|diagnostic| {
                diagnostic.message == "relative Markdown link does not resolve"
            })
        );
        fs::remove_dir_all(root).expect("remove fixture");
    }

    #[test]
    fn accepts_existing_fragments_and_external_links() {
        let root = fixture(
            "[Home](index.md)\n",
            "[Section](target.md#section) [Web](https://example.com) [Mail](mailto:a@example.com)\n",
        );
        fs::write(root.join("src/target.md"), "# Section\n").expect("target page");

        let diagnostics = check(&root);

        assert!(diagnostics.is_empty(), "{diagnostics:?}");
        fs::remove_dir_all(root).expect("remove fixture");
    }

    #[test]
    fn accepts_external_schemes_case_insensitively() {
        let root = fixture(
            "[Home](index.md)\n",
            "[Web](HTTPS://example.com) [Other](HtTp://example.com) [Mail](MAILTO:a@example.com)\n",
        );

        let diagnostics = check(&root);

        assert!(diagnostics.is_empty(), "{diagnostics:?}");
        fs::remove_dir_all(root).expect("remove fixture");
    }

    #[test]
    fn rejects_backslashes_anywhere_in_local_targets() {
        let root = fixture("[Home](index.md)\n", "[Windows](nested\\page.md)\n");
        fs::create_dir(root.join("src/nested")).expect("nested fixture directory");
        fs::write(root.join("src/nested/page.md"), "# Page\n").expect("nested fixture page");

        let diagnostics = check(&root);

        assert_eq!(diagnostics.len(), 1);
        assert_eq!(
            diagnostics[0].message,
            "absolute or machine-specific Markdown path is not allowed"
        );
        fs::remove_dir_all(root).expect("remove fixture");
    }

    #[test]
    fn rejects_machine_specific_paths() {
        let root = fixture(
            "[Home](index.md)\n",
            "[Unix](/etc/passwd) [Windows](C:\\Users\\person\\file.md) [Drive](D:relative.md)\n",
        );

        let diagnostics = check(&root);

        assert_eq!(
            diagnostics
                .iter()
                .filter(|diagnostic| diagnostic.message.contains("machine-specific"))
                .count(),
            3
        );
        fs::remove_dir_all(root).expect("remove fixture");
    }

    #[test]
    fn rejects_machine_specific_html_targets() {
        let root = fixture(
            "[Home](index.md)\n",
            "<a href=\"/etc/passwd\">Absolute</a>\n",
        );

        let diagnostics = check(&root);

        assert_eq!(diagnostics.len(), 1);
        assert_eq!(diagnostics[0].line, Some(1));
        assert_eq!(
            diagnostics[0].message,
            "absolute or machine-specific Markdown path is not allowed"
        );
        fs::remove_dir_all(root).expect("remove fixture");
    }

    #[test]
    fn rejects_targets_that_escape_the_book_source() {
        let root = fixture("[Home](index.md)\n", "[Outside](../outside.md)\n");
        fs::write(root.join("outside.md"), "# Outside\n").expect("outside fixture page");

        let diagnostics = check(&root);

        assert_eq!(diagnostics.len(), 1);
        assert_eq!(diagnostics[0].line, Some(1));
        assert_eq!(
            diagnostics[0].message,
            "Markdown path escapes the book source directory"
        );
        fs::remove_dir_all(root).expect("remove fixture");
    }

    #[test]
    fn accepts_parent_navigation_within_the_book_source() {
        let root = fixture("[Home](index.md)\n", "# Home\n");
        fs::create_dir(root.join("src/nested")).expect("nested fixture directory");
        fs::write(root.join("src/nested/page.md"), "[Home](../index.md)\n")
            .expect("nested fixture page");

        let diagnostics = check(&root);

        assert!(diagnostics.is_empty(), "{diagnostics:?}");
        fs::remove_dir_all(root).expect("remove fixture");
    }

    #[test]
    fn checks_nested_markdown_pages() {
        let root = fixture("[Home](index.md)\n", "# Home\n");
        fs::create_dir(root.join("src/nested")).expect("nested fixture directory");
        fs::write(root.join("src/nested/page.md"), "[Missing](missing.md)\n")
            .expect("nested fixture page");

        let diagnostics = check(&root);

        assert!(diagnostics.iter().any(|diagnostic| {
            diagnostic.path.ends_with("nested/page.md")
                && diagnostic.message == "relative Markdown link does not resolve"
        }));
        fs::remove_dir_all(root).expect("remove fixture");
    }
}
