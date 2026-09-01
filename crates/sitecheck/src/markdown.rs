//! Focused Markdown link extraction for the inline-link convention used here.

#[must_use]
pub fn inline_targets(markdown: &str) -> Vec<(usize, String)> {
    let mut targets = Vec::new();
    for (line_index, line) in markdown.lines().enumerate() {
        let mut cursor = 0;
        while let Some(relative_close) = line[cursor..].find("](") {
            let close = cursor + relative_close;
            let Some(open) = line[..close].rfind('[') else {
                cursor = close + 2;
                continue;
            };
            let target_start = close + 2;
            let Some(relative_end) = line[target_start..].find(')') else {
                break;
            };
            let end = target_start + relative_end;
            let is_image = open > 0 && line.as_bytes()[open - 1] == b'!';
            let target = line[target_start..end]
                .split_whitespace()
                .next()
                .unwrap_or_default();
            if !is_image && !target.is_empty() && !target.starts_with('#') {
                targets.push((line_index + 1, target.to_owned()));
            }
            cursor = end + 1;
        }
    }
    targets
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extracts_inline_links_with_source_lines() {
        let links =
            inline_targets("first\n[Estate](estate/index.md)\n[Source](https://example.com)\n");
        assert_eq!(
            links,
            vec![
                (2, "estate/index.md".to_owned()),
                (3, "https://example.com".to_owned())
            ]
        );
    }

    #[test]
    fn ignores_images_and_fragment_only_links() {
        let links = inline_targets("![Map](map.svg) [Heading](#heading)\n");
        assert!(links.is_empty());
    }

    #[test]
    fn trims_quoted_titles_and_extracts_multiple_links() {
        let links = inline_targets("[One](one.md \"First\") and [Two](two.md 'Second')\n");
        assert_eq!(
            links,
            vec![(1, "one.md".to_owned()), (1, "two.md".to_owned())]
        );
    }
}
