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

fn book_toml() -> String {
    fs::read_to_string(repo_root().join("book.toml")).expect("book.toml is readable")
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

#[test]
fn code_blocks_follow_the_theme_without_javascript_and_in_print() {
    let css = theme_css();
    let print = css.split_once("@media print").expect("print rules").1;

    assert!(css.contains(".content main pre {\n  background: var(--theme-popup-bg);"));
    assert!(css.contains(".content main pre > code {\n  color: var(--fg);"));
    assert!(print.contains(".content main pre,"));
    assert!(print.contains(".content main pre code {"));
    assert!(print.contains("background: #fff !important;"));
    assert!(print.contains("color: #111 !important;"));
}

/// DESIGN.md is a light-canvas system by default; the book's default theme
/// must not silently revert to the old dark-first `navy` default.
#[test]
fn default_theme_follows_design_md_light_canvas() {
    let toml = book_toml();

    assert!(toml.contains("default-theme = \"light\""));
    assert!(toml.contains("preferred-dark-theme = \"navy\""));
}

/// DESIGN.md names Geist and Geist Mono as "the two custom faces [that] carry
/// the entire system"; the retired Cormorant Garamond face must not return.
#[test]
fn headings_and_body_use_geist_not_cormorant() {
    let css = theme_css();

    assert!(
        css.contains("font-family: \"Geist\", \"Inter\", system-ui, -apple-system, sans-serif;")
    );
    assert!(css.contains(
        "font-family: \"Geist Mono\", ui-monospace, SFMono-Regular, Menlo, Monaco, monospace;"
    ));
    assert!(!css.contains("Cormorant Garamond"));
}

/// The four status colours (docs/design-mapping.md) are filled chips, not
/// currentColor-on-page-background text, so a coloured background must not
/// survive into print -- otherwise print wastes ink on decoration a
/// currentColor pill never needed resetting for.
#[test]
fn status_chip_fills_reset_to_transparent_in_print() {
    let css = theme_css();
    let print = css.split_once("@media print").expect("print rules").1;

    assert!(print.contains(
        ".maestro-status,\n  .maestro-claim__status {\n    background: transparent !important;"
    ));
    assert!(print.contains("border: 1px solid #777 !important;"));
}

/// mdBook's own `.left-buttons` siblings (`#mdbook-theme-toggle`,
/// `#mdbook-search-toggle`) have no explicit height, so they stretch to fill
/// the menu bar row and centre their glyph via line-height. This button sets
/// an explicit height, which drops the flex fallback for that axis to the
/// row's cross-start (its top) instead of its centre -- align-self restores
/// the shared centreline (see PR report for the measured before/after box).
#[test]
fn command_trigger_shares_the_left_buttons_centerline() {
    let css = theme_css();

    assert!(
        css.contains(".maestro-command-trigger {\n  align-items: center;\n  align-self: center;")
    );
}

/// DESIGN.md requires the mesh gradient to read as a "full-bleed" wash,
/// "never cropped to a frame" -- but `.maestro-hero` clips its overflow, and
/// the pseudo-element's own box is offset so most of it sits outside that
/// clip, cutting straight through the gradient's still-visible colour. This
/// overlay fades the same box to `--bg` before each clip seam, so the cut
/// falls across pixels that already match what sits beyond it.
#[test]
fn hero_gradient_fades_to_bg_before_its_own_clip_seams() {
    let css = theme_css();

    assert!(css.contains(
        ".maestro-hero::after {\n  background:\n    linear-gradient(to right, transparent 0%, transparent 48%, var(--bg) 64%),\n    linear-gradient(to bottom, var(--bg) 0%, var(--bg) 44%, transparent 64%);"
    ));
}

/// `built` uses `{colors.link-deep}`, not `{colors.success}`: both are the
/// same blue family, but success/link (#0070f3) on white chip text is only
/// 4.55:1, a hair over the 4.5 AA floor for small text -- link-deep
/// (#0761d1) gives 5.77:1 of headroom for the same hue.
#[test]
fn built_status_chip_clears_aa_with_headroom() {
    let css = theme_css();

    assert!(css.contains("--design-status-built: var(--design-link-deep);"));
}
