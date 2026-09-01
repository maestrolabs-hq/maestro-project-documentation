//! Focused target extraction for Markdown links and quoted raw HTML attributes.

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
        targets.extend(
            html_targets(line)
                .into_iter()
                .map(|target| (line_index + 1, target.to_owned())),
        );
    }
    targets
}

fn html_targets(line: &str) -> Vec<&str> {
    let bytes = line.as_bytes();
    let mut targets = Vec::new();
    let mut cursor = 0;
    let mut in_tag = false;
    while cursor < bytes.len() {
        match bytes[cursor] {
            b'<' => in_tag = true,
            b'>' => in_tag = false,
            _ if in_tag => {
                let name_length = if bytes[cursor..]
                    .get(..4)
                    .is_some_and(|name| name.eq_ignore_ascii_case(b"href"))
                {
                    4
                } else if bytes[cursor..]
                    .get(..3)
                    .is_some_and(|name| name.eq_ignore_ascii_case(b"src"))
                {
                    3
                } else {
                    cursor += 1;
                    continue;
                };
                if cursor > 0 && !bytes[cursor - 1].is_ascii_whitespace() {
                    cursor += name_length;
                    continue;
                }
                let mut value_start = cursor + name_length;
                while bytes.get(value_start).is_some_and(u8::is_ascii_whitespace) {
                    value_start += 1;
                }
                if bytes.get(value_start) != Some(&b'=') {
                    cursor += name_length;
                    continue;
                }
                value_start += 1;
                while bytes.get(value_start).is_some_and(u8::is_ascii_whitespace) {
                    value_start += 1;
                }
                let Some(quote @ (b'\'' | b'"')) = bytes.get(value_start).copied() else {
                    cursor += name_length;
                    continue;
                };
                value_start += 1;
                let Some(relative_end) =
                    bytes[value_start..].iter().position(|byte| *byte == quote)
                else {
                    break;
                };
                let end = value_start + relative_end;
                if end > value_start {
                    targets.push(&line[value_start..end]);
                }
                cursor = end;
            }
            _ => {}
        }
        cursor += 1;
    }
    targets
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_targets(markdown: &str, expected: &[(usize, &str)]) {
        assert_eq!(
            inline_targets(markdown),
            expected
                .iter()
                .map(|(line, target)| (*line, (*target).to_owned()))
                .collect::<Vec<_>>()
        );
    }

    #[test]
    fn extracts_inline_links_with_source_lines() {
        assert_targets(
            "first\n[Estate](estate/index.md)\n[Source](https://example.com)\n",
            &[(2, "estate/index.md"), (3, "https://example.com")],
        );
    }

    #[test]
    fn extracts_quoted_html_targets_with_source_lines() {
        assert_targets(
            "first\n<a href=\"guide.html\">Guide</a>\n<img src='images/map.svg' alt=''>\n",
            &[(2, "guide.html"), (3, "images/map.svg")],
        );
    }

    #[test]
    fn ignores_images_and_fragment_only_links() {
        let links = inline_targets("![Map](map.svg) [Heading](#heading)\n");
        assert!(links.is_empty());
    }

    #[test]
    fn trims_quoted_titles_and_extracts_multiple_links() {
        assert_targets(
            "[One](one.md \"First\") and [Two](two.md 'Second')\n",
            &[(1, "one.md"), (1, "two.md")],
        );
    }
}
