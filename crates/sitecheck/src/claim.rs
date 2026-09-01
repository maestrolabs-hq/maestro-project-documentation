//! Status and immutable-source validation for explicit Maestro claim blocks.

use crate::Diagnostic;
use std::path::Path;

#[must_use]
pub fn check(path: &Path, markdown: &str) -> Vec<Diagnostic> {
    let lines: Vec<&str> = markdown.lines().collect();
    let mut diagnostics = Vec::new();
    let mut index = 0;
    while index < lines.len() {
        let line = lines[index];
        let claim = line.contains("class=\"maestro-claim\"");
        if !claim && !line.contains("class=\"maestro-status\"") {
            index += 1;
            continue;
        }
        let line_number = index + 1;
        let marker = if claim {
            let (opening_tag, end) = collect_opening_tag(&lines, index);
            index = end;
            opening_tag
        } else {
            line.to_owned()
        };
        match attribute(&marker, "data-status") {
            Some(status) if valid_status(status) => {}
            Some(status) => diagnostics.push(diagnostic(
                path,
                line_number,
                format!("unknown Maestro status `{status}`"),
            )),
            None => diagnostics.push(diagnostic(
                path,
                line_number,
                "Maestro status marker is missing `data-status`",
            )),
        }
        if claim {
            validate_claim(path, line_number, &marker, &mut diagnostics);
        }
        index += 1;
    }
    diagnostics
}

fn collect_opening_tag(lines: &[&str], start: usize) -> (String, usize) {
    let mut tag = lines[start].to_owned();
    let mut end = start;
    while !tag.contains('>') && end + 1 < lines.len() {
        end += 1;
        tag.push('\n');
        tag.push_str(lines[end]);
    }
    (tag, end)
}

fn validate_claim(path: &Path, line_number: usize, line: &str, diagnostics: &mut Vec<Diagnostic>) {
    match attribute(line, "data-source") {
        Some(source) if valid_source(source) => {}
        Some(_) => diagnostics.push(diagnostic(
            path,
            line_number,
            "Maestro claim source must be an immutable GitHub permalink with a 40-character commit hash and line anchor",
        )),
        None => diagnostics.push(diagnostic(
            path,
            line_number,
            "Maestro claim is missing `data-source`",
        )),
    }
    match attribute(line, "data-verified") {
        Some(date) if valid_date(date) => {}
        Some(_) => diagnostics.push(diagnostic(
            path,
            line_number,
            "Maestro claim `data-verified` must use YYYY-MM-DD",
        )),
        None => diagnostics.push(diagnostic(
            path,
            line_number,
            "Maestro claim is missing `data-verified`",
        )),
    }
}

fn diagnostic(path: &Path, line: usize, message: impl Into<String>) -> Diagnostic {
    Diagnostic {
        path: path.to_path_buf(),
        line: Some(line),
        message: message.into(),
    }
}

fn valid_status(status: &str) -> bool {
    matches!(status, "built" | "in-progress" | "designed" | "exploring")
}

fn valid_source(source: &str) -> bool {
    let Some(path) = source.strip_prefix("https://github.com/") else {
        return false;
    };
    let Some((repository, revision_path)) = path.split_once("/blob/") else {
        return false;
    };
    let mut repository = repository.split('/');
    if repository.next().is_none_or(str::is_empty)
        || repository.next().is_none_or(str::is_empty)
        || repository.next().is_some()
    {
        return false;
    }
    let Some((revision, source_path)) = revision_path.split_once('/') else {
        return false;
    };
    revision.len() == 40
        && revision.bytes().all(|byte| byte.is_ascii_hexdigit())
        && valid_source_path(source_path)
}

fn valid_source_path(source_path: &str) -> bool {
    let Some((file, anchor)) = source_path.rsplit_once('#') else {
        return false;
    };
    !file.is_empty() && valid_line_anchor(anchor)
}

fn valid_line_anchor(anchor: &str) -> bool {
    let Some(lines) = anchor.strip_prefix('L') else {
        return false;
    };
    let mut lines = lines.split("-L");
    let Some(first) = lines.next() else {
        return false;
    };
    valid_line_number(first) && lines.next().is_none_or(valid_line_number) && lines.next().is_none()
}

fn valid_line_number(line: &str) -> bool {
    !line.is_empty() && line.bytes().all(|byte| byte.is_ascii_digit())
}

fn valid_date(date: &str) -> bool {
    date.len() == 10
        && date.bytes().enumerate().all(|(index, byte)| match index {
            4 | 7 => byte == b'-',
            _ => byte.is_ascii_digit(),
        })
}

fn attribute<'a>(line: &'a str, name: &str) -> Option<&'a str> {
    let needle = format!("{name}=\"");
    let mut remainder = line;
    while let Some(index) = remainder.find(&needle) {
        let boundary = index == 0
            || remainder[..index]
                .chars()
                .next_back()
                .is_some_and(char::is_whitespace);
        let value = &remainder[index + needle.len()..];
        if boundary {
            return value.split_once('"').map(|(value, _)| value);
        }
        remainder = value;
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_the_four_status_values() {
        for status in ["built", "in-progress", "designed", "exploring"] {
            assert!(valid_status(status));
        }
    }

    #[test]
    fn rejects_an_unknown_status_with_its_line() {
        let diagnostics = check(
            Path::new("roadmap.md"),
            "<span class=\"maestro-status\" data-status=\"planned\">Planned</span>\n",
        );

        assert_eq!(diagnostics[0].line, Some(1));
        assert_eq!(diagnostics[0].message, "unknown Maestro status `planned`");
    }

    #[test]
    fn does_not_treat_an_attribute_suffix_as_the_status() {
        let diagnostics = check(
            Path::new("roadmap.md"),
            "<span class=\"maestro-status\" other-data-status=\"built\">Unknown</span>\n",
        );

        assert_eq!(
            diagnostics[0].message,
            "Maestro status marker is missing `data-status`"
        );
    }

    #[test]
    fn rejects_a_status_marker_without_a_status() {
        let diagnostics = check(
            Path::new("roadmap.md"),
            "<span class=\"maestro-status\">Unknown</span>\n",
        );

        assert_eq!(
            diagnostics[0].message,
            "Maestro status marker is missing `data-status`"
        );
    }

    #[test]
    fn accepts_claim_metadata_across_the_opening_tag() {
        const SOURCE: &str = "https://github.com/maestrolabs-hq/maestro-core/blob/0123456789abcdef0123456789abcdef01234567/README.md#L1-L4";
        let markdown = format!(
            "<div class=\"maestro-claim\"\n     data-status=\"built\"\n     data-source=\"{SOURCE}\"\n     data-verified=\"2026-09-01\">\n  <p>The factual assertion.</p>\n</div>\n"
        );

        let diagnostics = check(Path::new("claims.md"), &markdown);

        assert!(diagnostics.is_empty(), "{diagnostics:?}");
    }

    #[test]
    fn reports_multiline_claim_errors_on_the_opening_line() {
        let diagnostics = check(
            Path::new("claims.md"),
            "first line\n<div class=\"maestro-claim\"\n     data-status=\"planned\"\n     data-source=\"https://github.com/maestrolabs-hq/maestro-core/blob/main/README.md#L1\"\n     data-verified=\"September 1\">\n",
        );

        assert_eq!(diagnostics.len(), 3);
        assert!(
            diagnostics
                .iter()
                .all(|diagnostic| diagnostic.line == Some(2))
        );
    }

    #[test]
    fn rejects_a_claim_without_a_source() {
        let diagnostics = check(
            Path::new("claims.md"),
            "<div class=\"maestro-claim\" data-status=\"built\" data-verified=\"2026-09-01\">\n",
        );

        assert!(
            diagnostics
                .iter()
                .any(|diagnostic| diagnostic.message == "Maestro claim is missing `data-source`")
        );
    }

    #[test]
    fn rejects_branch_and_short_commit_sources() {
        for source in [
            "https://github.com/maestrolabs-hq/maestro-core/blob/main/README.md#L1-L4",
            "https://github.com/maestrolabs-hq/maestro-core/blob/0123456/README.md#L1-L4",
        ] {
            let markdown = format!(
                "<div class=\"maestro-claim\" data-status=\"built\" data-source=\"{source}\" data-verified=\"2026-09-01\">\n"
            );

            let diagnostics = check(Path::new("claims.md"), &markdown);

            assert!(diagnostics.iter().any(|diagnostic| diagnostic.message
                == "Maestro claim source must be an immutable GitHub permalink with a 40-character commit hash and line anchor"));
        }
    }

    #[test]
    fn accepts_an_immutable_source_and_verification_date_on_one_line() {
        const SOURCE: &str = "https://github.com/maestrolabs-hq/maestro-core/blob/0123456789abcdef0123456789abcdef01234567/README.md#L1-L4";
        let markdown = format!(
            "<div class=\"maestro-claim\" data-status=\"built\" data-source=\"{SOURCE}\" data-verified=\"2026-09-01\">\n"
        );

        let diagnostics = check(Path::new("claims.md"), &markdown);

        assert!(diagnostics.is_empty(), "{diagnostics:?}");
    }

    #[test]
    fn rejects_a_repeated_line_range() {
        const SOURCE: &str = "https://github.com/maestrolabs-hq/maestro-core/blob/0123456789abcdef0123456789abcdef01234567/README.md#L1-L2-L3";
        let markdown = format!(
            "<div class=\"maestro-claim\" data-status=\"built\" data-source=\"{SOURCE}\" data-verified=\"2026-09-01\">\n"
        );

        let diagnostics = check(Path::new("claims.md"), &markdown);

        assert!(diagnostics.iter().any(|diagnostic| diagnostic.message
            == "Maestro claim source must be an immutable GitHub permalink with a 40-character commit hash and line anchor"));
    }

    #[test]
    fn rejects_missing_and_malformed_verification_dates() {
        const SOURCE: &str = "https://github.com/maestrolabs-hq/maestro-core/blob/0123456789abcdef0123456789abcdef01234567/README.md#L1-L4";
        let missing = format!(
            "<div class=\"maestro-claim\" data-status=\"built\" data-source=\"{SOURCE}\">\n"
        );
        let malformed = format!(
            "<div class=\"maestro-claim\" data-status=\"built\" data-source=\"{SOURCE}\" data-verified=\"September 1\">\n"
        );

        let missing_diagnostics = check(Path::new("claims.md"), &missing);
        let malformed_diagnostics = check(Path::new("claims.md"), &malformed);

        assert!(
            missing_diagnostics
                .iter()
                .any(|diagnostic| diagnostic.message == "Maestro claim is missing `data-verified`")
        );
        assert!(
            malformed_diagnostics
                .iter()
                .any(|diagnostic| diagnostic.message
                    == "Maestro claim `data-verified` must use YYYY-MM-DD")
        );
    }
}
