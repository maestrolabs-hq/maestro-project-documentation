//! Theme contracts that keep every mdBook color mode and print output legible.

use std::fs;
use std::path::{Path, PathBuf};

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("sitecheck has a workspace root")
        .to_path_buf()
}

fn theme_css() -> String {
    fs::read_to_string(repo_root().join("theme/css/maestro.css")).expect("theme CSS is readable")
}

#[test]
fn content_colors_follow_the_selected_mdbook_theme() {
    let css = theme_css();

    assert!(css.contains("body {\n  background: var(--bg);"));
    assert!(css.contains(".navy.js body {\n  background:"));
    assert!(css.contains(".content main {\n  color: var(--fg);"));
    assert!(css.contains(".content main h3 {\n  color: var(--links);"));
    assert!(css.contains(".light .maestro-hero__wordmark"));
    assert!(css.contains(".rust .maestro-hero__wordmark"));
    assert!(css.contains("html:not(.js) .maestro-hero__wordmark"));
}

#[test]
fn print_forces_the_wordmark_to_monochrome() {
    let css = theme_css();
    let print = css.split_once("@media print").expect("print rules").1;

    assert!(print.contains(".maestro-hero__wordmark"));
    assert!(print.contains("filter: brightness(0) contrast(100%);"));
    assert!(print.contains("--bg: #fff !important;"));
    assert!(print.contains(".page-wrapper"));
}
