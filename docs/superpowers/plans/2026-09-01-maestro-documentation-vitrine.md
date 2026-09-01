# Maestro Documentation Vitrine Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Bootstrap a governance-compliant Rust repository that publishes the Obsidian Score mdBook site, validates its structure and evidence, and presents Maestro's shared knowledge and roadmap.

**Architecture:** Plain Markdown is the source for both agents and the public site. mdBook 0.5.4 renders it; one dependency-free Rust crate named `sitecheck` validates the repository contract, navigation, local links, status vocabulary, and explicitly marked claim blocks. Shared governance files remain byte-identical to `maestro-core`, while repository-specific CI adds site validation and GitHub Pages deployment.

**Tech Stack:** Rust 2024, Rust 1.98.0 toolchain, MSRV 1.85, standard library only, mdBook 0.5.4, HTML, CSS, minimal vanilla JavaScript, GitHub Actions, GitHub Pages.

**Spec:** `docs/superpowers/specs/2026-09-01-maestro-documentation-vitrine-design.md`

## Global Constraints

- Copy baseline-pinned files byte-for-byte from `../maestro-core`; do not recreate or edit them locally.
- Use exactly one Rust crate: `crates/sitecheck`.
- Add no Rust application dependency unless a standard-library implementation has been proved insufficient.
- Add no npm, Node package manifest, JavaScript framework, analytics, account system, comments, or custom search backend.
- Keep repository-specific implementation facts authoritative in their original repositories.
- Use only `built`, `in-progress`, `designed`, and `exploring` as status values.
- Machine-verified assertions use explicit `.maestro-claim` blocks with immutable GitHub commit permalinks; ordinary prose remains a review responsibility.
- Derive paths from the repository root; never commit a path naming a developer machine.
- Every Rust source file starts with a `//!` brief explaining what it does and why it exists.
- English only, conventional commits, failing test first, and no weakened gate.
- Accessibility basics are required: semantic landmarks, visible focus, keyboard navigation, sufficient contrast, alternative text, and reduced-motion behavior.
- Keep the generated mdBook output in `build/`, which the shared `.gitignore` already excludes.
- Keep `.superpowers/` local through `.git/info/exclude`; do not change the baseline-pinned `.gitignore`.

---

## Planned File Map

### Repository contract

- `.editorconfig`, `.gitattributes`, `.gitignore`, `.pre-commit-config.yaml`, `CHANGELOG.md`, `LICENSE`, `NORTHSTAR.md`, `clippy.toml`, `deny.toml`, `rust-toolchain.toml`: byte-identical estate files.
- `.github/CODEOWNERS`, `.github/dependabot.yml`, `.github/release-please/*`, `.github/workflows/heavy.yml`: byte-identical estate GitHub files.
- `README.md`: public repository entry point and local commands.
- `AGENTS.md`: instructions specific to editing site content and claim blocks.
- `CONTEXT.md`: glossary only.
- `BACKLOG.md`: definition and readiness contract for initiatives and epics.
- `TODO.md`: trivial repository-local cleanup only.
- `justfile`: standalone-equivalent install, setup, check, build, serve, format, and doctor recipes.

### Rust checker

- `Cargo.toml`: one-member workspace and estate lint configuration.
- `crates/sitecheck/Cargo.toml`: dependency-free binary/library package.
- `crates/sitecheck/src/diagnostic.rs`: path-and-line diagnostics.
- `crates/sitecheck/src/required.rs`: required repository-file validation.
- `crates/sitecheck/src/markdown.rs`: focused inline-link extraction.
- `crates/sitecheck/src/book.rs`: book traversal, SUMMARY agreement, and local-link validation.
- `crates/sitecheck/src/claim.rs`: status and claim-block validation.
- `crates/sitecheck/src/lib.rs`: public `check_repository` orchestration.
- `crates/sitecheck/src/main.rs`: terminal adapter and exit status.
- `crates/sitecheck/tests/common/mod.rs`: repository-root and source-file discovery shared by standards tests.
- `crates/sitecheck/tests/standards.rs`: module-size, English-only, and accounted-duplication gates required by the shared hook.
- `crates/sitecheck/tests/fixtures/valid/`: smallest passing repository/book fixture.
- `crates/sitecheck/tests/fixtures/invalid/`: focused failing fixture inputs.
- `crates/sitecheck/tests/sitecheck.rs`: integration coverage for valid and invalid fixtures.

### mdBook

- `book.toml`: mdBook 0.5.4 configuration and local theme assets.
- `src/SUMMARY.md`: the only navigation order.
- `src/index.md`: public overture.
- `src/overture/*`: purpose, current reality, and reading guide.
- `src/method/*`: principles and evidence-driven engineering method.
- `src/estate/*`: ecosystem and repository roles.
- `src/delivery/*`: idea pipeline, backlog model, and roadmap.
- `src/evidence/*`: verified claim blocks, ADR links, and source repositories.
- `theme/css/maestro.css`: Obsidian Score design tokens, layout, components, print, and reduced motion.
- `theme/maestro.js`: command palette, reading progress, claim-source rendering, and copy feedback.
- `src/images/maestro-wordmark.svg`: The Institution wordmark, copied by mdBook as source content.
- `theme/favicon.svg`: the accented final `O`, used only where a wide wordmark cannot fit.
- `theme/fonts/CormorantGaramond.ttf`: locally hosted source face.
- `theme/fonts/OFL.txt`: SIL Open Font License.

### Automation

- `.github/workflows/ci.yml`: shared common/Rust checks plus site validation.
- `.github/workflows/pages.yml`: checked build and least-privilege Pages deployment.
- `../maestro-governance/baseline.txt`: repository registration and tracked-file scope.

---

### Task 1: Establish the Governed Rust Repository

**Files:**

- Create/copy: all repository-contract files listed above
- Create: `Cargo.toml`
- Create: `crates/sitecheck/Cargo.toml`
- Create: `crates/sitecheck/src/{lib.rs,main.rs,diagnostic.rs,required.rs}`
- Create: `README.md`, `AGENTS.md`, `CONTEXT.md`, `BACKLOG.md`, `TODO.md`, `justfile`
- Create: `crates/sitecheck/tests/common/mod.rs`, `crates/sitecheck/tests/standards.rs`
- Test: unit tests in `crates/sitecheck/src/required.rs`; estate standards in `crates/sitecheck/tests/standards.rs`

**Interfaces:**

- Produces: `Diagnostic { path: PathBuf, line: Option<usize>, message: String }`
- Produces: `required::check(root: &Path) -> Vec<Diagnostic>`
- Produces: `check_repository(root: &Path) -> Vec<Diagnostic>`
- Produces: CLI `sitecheck [ROOT]`, where ROOT defaults to `.` and any diagnostic returns exit code 1

- [ ] **Step 1: Copy every baseline-pinned file from the existing Rust estate**

Run from the documentation repository:

```bash
mkdir -p .github/release-please .github/workflows
for file in \
  .editorconfig .gitattributes .gitignore .pre-commit-config.yaml \
  CHANGELOG.md LICENSE NORTHSTAR.md clippy.toml deny.toml rust-toolchain.toml
do
  cp "../maestro-core/$file" "$file"
done
cp ../maestro-core/.github/CODEOWNERS .github/CODEOWNERS
cp ../maestro-core/.github/dependabot.yml .github/dependabot.yml
cp ../maestro-core/.github/release-please/config.json .github/release-please/config.json
cp ../maestro-core/.github/release-please/manifest.json .github/release-please/manifest.json
cp ../maestro-core/.github/workflows/heavy.yml .github/workflows/heavy.yml
printf '\n/.superpowers/\n' >> .git/info/exclude
```

Verify byte identity:

```bash
for file in \
  .editorconfig .gitattributes .gitignore .pre-commit-config.yaml \
  CHANGELOG.md LICENSE NORTHSTAR.md clippy.toml deny.toml rust-toolchain.toml \
  .github/CODEOWNERS .github/dependabot.yml \
  .github/release-please/config.json .github/release-please/manifest.json \
  .github/workflows/heavy.yml
do
  cmp "$file" "../maestro-core/$file"
done
```

Expected: no output and exit code 0.

- [ ] **Step 2: Write the failing required-file test**

Create `crates/sitecheck/src/required.rs` with the test first:

```rust
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

pub fn check(_root: &Path) -> Vec<Diagnostic> {
    Vec::new()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{fs, time::{SystemTime, UNIX_EPOCH}};

    #[test]
    fn reports_a_missing_required_file() {
        let root = std::env::temp_dir().join(format!(
            "maestro-sitecheck-required-{}",
            SystemTime::now().duration_since(UNIX_EPOCH).expect("clock").as_nanos()
        ));
        fs::create_dir_all(&root).expect("fixture directory");
        for path in REQUIRED_FILES.iter().filter(|path| **path != "README.md") {
            let path = root.join(path);
            fs::create_dir_all(path.parent().expect("required file has parent"))
                .expect("fixture parent");
            fs::write(path, "fixture\n").expect("fixture file");
        }

        let diagnostics = check(&root);

        assert_eq!(diagnostics.len(), 1);
        assert_eq!(diagnostics[0].path, root.join("README.md"));
        fs::remove_dir_all(root).expect("remove fixture");
    }
}
```

- [ ] **Step 3: Create the minimum Cargo workspace and run the test to prove failure**

Create root `Cargo.toml`:

```toml
# The documentation workspace: one checker, because it is the only custom code.

[workspace]
resolver = "3"
members = ["crates/sitecheck"]

[workspace.package]
edition = "2024"
rust-version = "1.85"
license = "MIT"

[workspace.lints.clippy]
pedantic = { level = "warn", priority = -1 }
too_many_lines = "warn"
cognitive_complexity = "warn"
too_many_arguments = "warn"
```

Create `crates/sitecheck/Cargo.toml`:

```toml
# The repository-specific documentation validator.

[package]
name = "sitecheck"
version = "0.1.0"
edition.workspace = true
rust-version.workspace = true
license.workspace = true

[lints]
workspace = true
```

Create `crates/sitecheck/src/diagnostic.rs` and `lib.rs` sufficiently for compilation:

```rust
//! Actionable validation failures with an optional source line.

use std::path::PathBuf;

#[derive(Debug, PartialEq, Eq)]
pub struct Diagnostic {
    pub path: PathBuf,
    pub line: Option<usize>,
    pub message: String,
}
```

```rust
//! Validation for the governed repository and the mdBook it publishes.

mod diagnostic;
mod required;

pub use diagnostic::Diagnostic;

use std::path::Path;

#[must_use]
pub fn check_repository(root: &Path) -> Vec<Diagnostic> {
    required::check(root)
}
```

Create `crates/sitecheck/src/main.rs`:

```rust
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
```

Run:

```bash
cargo test -p sitecheck required::tests::reports_a_missing_required_file
```

Expected: FAIL because `check` returns no diagnostics.

- [ ] **Step 4: Implement required-file validation minimally**

Replace `required::check` with:

```rust
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
```

Add this exact `Display` implementation to `diagnostic.rs`:

```rust
use std::fmt;

impl fmt::Display for Diagnostic {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.line {
            Some(line) => write!(formatter, "{}:{line}: {}", self.path.display(), self.message),
            None => write!(formatter, "{}: {}", self.path.display(), self.message),
        }
    }
}
```

`main.rs` already prints each diagnostic to stderr and returns exit code 1.

- [ ] **Step 5: Create repository-specific contract documents**

Use these exact boundaries:

- `README.md`: repository purpose, `just install`, `just setup`, `just check`, `just build`, and `just serve`.
- `AGENTS.md`: read the design and glossary first; keep repo facts linked rather than copied; use claim blocks only for machine-verified assertions; run `just check` before proposing a change.
- `CONTEXT.md`: define Idea, Proposal, Initiative, Epic, Task, Backlog, Roadmap, Claim, and Evidence without implementation details.
- `BACKLOG.md`: declare entry criteria: approved initiative, owner repository, outcome, FRs, NFRs, acceptance criteria, definition of done, evidence plan, and dependencies. No raw ideas enter the backlog.
- `TODO.md`: state that it is only for trivial local cleanup and begin empty.

Create `crates/sitecheck/tests/common/mod.rs` by adapting the estate source walk from `../maestro-core/crates/cli/tests/common/mod.rs` to a crate at `crates/sitecheck`. Create `crates/sitecheck/tests/standards.rs` from the estate standard with `ACCEPTED_DUPLICATION` empty. Keep the 250-line module limit, English-only scan, similarity-rs invocation, and stale-allowlist test. This is required because the byte-identical pre-push hook runs `cargo test --test standards`.

Create `justfile` with standalone-equivalent recipes:

```just
# Optional convenience tasks; every command remains runnable without just.

path_sep := if os_family() == "windows" { ";" } else { ":" }
export PATH := home_directory() / ".cargo" / "bin" + path_sep + home_directory() / ".local" / "bin" + path_sep + env('PATH')

install:
    rustup toolchain install --profile minimal 1.98.0
    rustup component add clippy rustfmt llvm-tools
    cargo binstall -y prek cargo-deny cargo-machete cargo-llvm-cov similarity-rs
    cargo install mdbook --version 0.5.4 --locked

setup:
    prek install --install-hooks

check:
    cargo fmt --all --check
    cargo clippy --all-targets --all-features -- -D warnings
    cargo test --all-targets
    cargo machete
    cargo deny check
    cargo run -p sitecheck -- .
    mdbook test
    mdbook build

build:
    cargo run -p sitecheck -- .
    mdbook build

serve:
    mdbook serve --open

fmt:
    cargo fmt --all

doctor:
    @echo "just    $(command -v just)"
    @echo "cargo   $(command -v cargo)"
    @echo "mdbook  $(mdbook --version)"
    @echo "prek    $(command -v prek)"
    @echo "rustc   $(rustc --version)"
```

- [ ] **Step 6: Run the unit test and baseline identity checks**

Run:

```bash
cargo test -p sitecheck required::tests::reports_a_missing_required_file
cargo fmt --all --check
cargo clippy --all-targets --all-features -- -D warnings
```

Expected: PASS.

- [ ] **Step 7: Commit the governed foundation**

```bash
git add \
  .editorconfig .gitattributes .gitignore .pre-commit-config.yaml \
  .github AGENTS.md BACKLOG.md CHANGELOG.md CONTEXT.md Cargo.lock Cargo.toml \
  LICENSE NORTHSTAR.md README.md TODO.md clippy.toml crates deny.toml justfile \
  rust-toolchain.toml
git commit -m "chore: establish the governed documentation repository"
```

### Task 2: Validate Book Navigation and Local Links

**Files:**

- Create: `crates/sitecheck/src/markdown.rs`
- Create: `crates/sitecheck/src/book.rs`
- Modify: `crates/sitecheck/src/lib.rs`
- Test: unit tests in both new modules

**Interfaces:**

- Produces: `markdown::inline_targets(markdown: &str) -> Vec<(usize, String)>`
- Produces: `book::check(root: &Path) -> Vec<Diagnostic>`
- Consumes: `Diagnostic`

- [ ] **Step 1: Write failing link-extraction tests**

Test these exact cases:

```rust
#[test]
fn extracts_inline_links_with_source_lines() {
    let links = inline_targets("first\n[Estate](estate/index.md)\n[Source](https://example.com)\n");
    assert_eq!(links, vec![(2, "estate/index.md".to_owned()), (3, "https://example.com".to_owned())]);
}

#[test]
fn ignores_images_and_fragment_only_links() {
    let links = inline_targets("![Map](map.svg) [Heading](#heading)\n");
    assert!(links.is_empty());
}
```

Run `cargo test -p sitecheck markdown::tests`; expect FAIL because the function does not exist.

- [ ] **Step 2: Implement the focused inline-link scanner**

Implement a line-by-line scanner that:

1. finds `](`;
2. rejects matches preceded by `!` at the opening `[`;
3. reads until the next `)`;
4. trims an optional quoted title after the target;
5. returns non-empty targets except fragment-only links.

Do not implement reference-style links; `AGENTS.md` explicitly requires inline links so validation stays deterministic without a Markdown dependency.

Run `cargo test -p sitecheck markdown::tests`; expect PASS.

- [ ] **Step 3: Write failing SUMMARY and local-link tests**

Create fixture files in a temporary directory and assert:

```rust
#[test]
fn reports_a_summary_page_that_does_not_exist() {
    let root = fixture("[Missing](missing.md)\n", "# Home\n");
    let diagnostics = check(&root);
    assert!(diagnostics.iter().any(|d| d.message == "SUMMARY references a missing page"));
}

#[test]
fn reports_a_broken_relative_link() {
    let root = fixture("[Home](index.md)\n", "[Missing](missing.md)\n");
    let diagnostics = check(&root);
    assert!(diagnostics.iter().any(|d| d.message == "relative Markdown link does not resolve"));
}
```

Run `cargo test -p sitecheck book::tests`; expect FAIL.

- [ ] **Step 4: Implement recursive book checking**

`book::check` must:

- require `src/SUMMARY.md` to be readable;
- resolve SUMMARY targets relative to `src/`;
- recursively visit `.md` files under `src/`;
- resolve relative Markdown links from each source file's parent;
- ignore `http:`, `https:`, `mailto:`, and fragment-only targets;
- strip `#fragment` before testing filesystem existence;
- reject absolute and machine-specific paths.

Add `book::check(root)` to `check_repository` after `required::check(root)`.

- [ ] **Step 5: Verify and commit**

```bash
cargo test -p sitecheck
cargo clippy --all-targets --all-features -- -D warnings
git add crates/sitecheck/src
git commit -m "feat: validate book navigation and local links"
```

Expected: all tests pass and Clippy emits no warnings.

### Task 3: Validate Statuses and Explicit Claim Blocks

**Files:**

- Create: `crates/sitecheck/src/claim.rs`
- Modify: `crates/sitecheck/src/lib.rs`
- Test: unit tests in `claim.rs`; integration cases under `crates/sitecheck/tests/fixtures/`

**Interfaces:**

- Produces: `claim::check(path: &Path, markdown: &str) -> Vec<Diagnostic>`
- Consumes HTML markers:

```html
<div class="maestro-claim"
     data-status="built"
     data-source="https://github.com/maestrolabs-hq/maestro-core/blob/0123456789abcdef0123456789abcdef01234567/README.md#L1-L4"
     data-verified="2026-09-01">
  <p>The factual assertion.</p>
</div>
```

- [ ] **Step 1: Write failing status tests**

```rust
#[test]
fn accepts_the_four_status_values() {
    for status in ["built", "in-progress", "designed", "exploring"] {
        assert!(valid_status(status));
    }
}

#[test]
fn rejects_an_unknown_status_with_its_line() {
    let diagnostics = check(Path::new("roadmap.md"), "<span class=\"maestro-status\" data-status=\"planned\">Planned</span>\n");
    assert_eq!(diagnostics[0].line, Some(1));
    assert_eq!(diagnostics[0].message, "unknown Maestro status `planned`");
}
```

Run `cargo test -p sitecheck claim::tests`; expect FAIL.

- [ ] **Step 2: Implement exact status validation**

Implement `valid_status` with `matches!` and scan lines containing `class="maestro-status"` or `class="maestro-claim"` for a quoted `data-status` attribute. Missing or unknown attributes produce line-specific diagnostics.

Run status tests; expect PASS.

- [ ] **Step 3: Write failing claim-source tests**

Cover:

- missing `data-source`;
- a branch URL containing `/blob/main/`;
- a short SHA;
- a valid 40-character hexadecimal SHA and `#L` anchor;
- missing or malformed ISO `data-verified` date.

The valid source test uses a synthetic value, not a machine path:

```rust
const SOURCE: &str = "https://github.com/maestrolabs-hq/maestro-core/blob/0123456789abcdef0123456789abcdef01234567/README.md#L1-L4";
```

- [ ] **Step 4: Implement immutable-source checks**

The standard-library validator must require:

- `https://github.com/` prefix;
- `/blob/` segment;
- exactly 40 ASCII hexadecimal characters between `/blob/` and the next `/`;
- a non-empty repository path;
- `#L` line anchor;
- `data-verified` in `YYYY-MM-DD` ASCII shape.

It validates metadata, not remote content and not free-form prose. Add `claim::check` to every Markdown file visited by `book::check`.

- [ ] **Step 5: Add integration fixtures and verify the CLI exit contract**

The valid fixture must return no diagnostics. The invalid fixture must contain one broken link, one unknown status, and one mutable claim source; assert all three diagnostics and CLI exit code 1.

Run:

```bash
cargo test --all-targets
cargo run -p sitecheck -- crates/sitecheck/tests/fixtures/valid
```

Expected: tests pass and the valid fixture exits 0.

- [ ] **Step 6: Commit**

```bash
git add crates/sitecheck
git commit -m "feat: validate roadmap states and sourced claims"
```

### Task 4: Build the Honest Maestro Knowledge Book

**Files:**

- Create: `book.toml`
- Create: `src/SUMMARY.md`, `src/index.md`
- Create: `src/overture/{what-is-maestro,current-reality,reading-guide}.md`
- Create: `src/method/{principles,proved-gates,decisions,rebuilding}.md`
- Create: `src/estate/{index,maestro-core,maestro-governance,maestro-pi-config}.md`
- Create: `src/delivery/{idea-pipeline,backlog,roadmap}.md`
- Create: `src/evidence/{claims,adrs,sources}.md`
- Remove after migration: `ecosystem-map.md`
- Test: `cargo run -p sitecheck -- .`, `mdbook test`, `mdbook build`

**Interfaces:**

- Produces: complete navigation in `src/SUMMARY.md`
- Produces: `build/` static site
- Consumes: validated inline links, statuses, and `.maestro-claim` markup

- [ ] **Step 1: Create pinned mdBook configuration**

```toml
# The public rendering of Maestro's shared Markdown knowledge.

[book]
title = "Maestro"
description = "Agent orchestration, and the machinery that keeps it honest."
authors = ["MaestroLabs"]
language = "en"
src = "src"

[build]
build-dir = "build"
create-missing = false

[output.html]
default-theme = "navy"
preferred-dark-theme = "navy"
smart-punctuation = true
git-repository-url = "https://github.com/maestrolabs-hq/maestro-project-documentation"
edit-url-template = "https://github.com/maestrolabs-hq/maestro-project-documentation/edit/main/{path}"
additional-css = ["theme/css/maestro.css"]
additional-js = ["theme/maestro.js"]
no-section-label = true

[output.html.search]
enable = true
limit-results = 20
use-boolean-and = true
boost-title = 3
boost-hierarchy = 2
boost-paragraph = 1
```

- [ ] **Step 2: Prove the empty book fails before adding navigation**

Run:

```bash
cargo run -p sitecheck -- .
mdbook build
```

Expected: FAIL because `src/SUMMARY.md` and referenced pages do not exist.

- [ ] **Step 3: Create the exact navigation tree**

`src/SUMMARY.md` contains:

```markdown
# Summary

[Overture](index.md)

- [What Maestro is](overture/what-is-maestro.md)
- [What exists today](overture/current-reality.md)
- [How to read this site](overture/reading-guide.md)

# Method

- [Principles](method/principles.md)
- [Proved gates](method/proved-gates.md)
- [Decisions beside code](method/decisions.md)
- [Rebuilding from source](method/rebuilding.md)

# Estate

- [Ecosystem map](estate/index.md)
- [maestro-core](estate/maestro-core.md)
- [maestro-governance](estate/maestro-governance.md)
- [maestro-pi-config](estate/maestro-pi-config.md)

# Delivery

- [Idea pipeline](delivery/idea-pipeline.md)
- [Backlog model](delivery/backlog.md)
- [Roadmap](delivery/roadmap.md)

# Evidence

- [Verified claims](evidence/claims.md)
- [ADR index](evidence/adrs.md)
- [Source repositories](evidence/sources.md)
```

- [ ] **Step 4: Write the overture and method pages from primary repository sources**

Use the organization profile, the four `NORTHSTAR.md` files, and accepted ADRs as sources. Keep each page focused:

- `index.md`: “Agent orchestration, and the machinery that keeps it honest” plus links into Current reality, Method, and Roadmap.
- `what-is-maestro.md`: system purpose and accountability boundary.
- `current-reality.md`: explicit built/designed/exploring sections, never a blended feature list.
- `reading-guide.md`: explain status badges, claim evidence, and repository authority.
- method pages: one principle per section, followed by immutable evidence rather than copied implementation details.

Every machine-verified statement uses the exact claim-block contract from Task 3. Pin links with `git -C ../REPO rev-parse HEAD` and line anchors.

- [ ] **Step 5: Write the estate pages as relationships, not duplicated READMEs**

Migrate the useful intent of `ecosystem-map.md` into `src/estate/index.md`, then remove the old root draft. Each repository page contains only:

1. its role in the estate;
2. what it owns;
3. what it explicitly does not own;
4. current status;
5. links to its README, CONTEXT, NORTHSTAR, TODO/BACKLOG if present, and ADRs.

- [ ] **Step 6: Write delivery pages using the approved domain model**

`idea-pipeline.md` renders:

```text
Idea → brainstorm → grill → FR/NFR → architecture review when needed
     → acceptance criteria + definition of done → split → human approval
     → backlog → development → verification → released
```

`backlog.md` repeats the readiness contract from root `BACKLOG.md` by linking to it and explaining it publicly; it does not create a second prioritized list.

`roadmap.md` groups capabilities under the four exact statuses. It does not duplicate repository TODO items.

- [ ] **Step 7: Write evidence pages and run the complete content checks**

`claims.md` indexes explicit claim blocks. `adrs.md` links to accepted ADRs in their owning repositories. `sources.md` names each authoritative repository and its role.

Run:

```bash
cargo run -p sitecheck -- .
mdbook test
mdbook build
```

Expected: all commands exit 0 and `build/index.html` exists.

- [ ] **Step 8: Commit**

```bash
git add book.toml src ecosystem-map.md
git commit -m "docs: establish the Maestro knowledge book"
```

### Task 5: Implement Obsidian Score and The Institution

**Files:**

- Create: `theme/css/maestro.css`
- Create: `theme/maestro.js`
- Create: `src/images/maestro-wordmark.svg`, `theme/favicon.svg`
- Create: `theme/fonts/{CormorantGaramond.ttf,OFL.txt}`
- Modify: `src/index.md` and claim-bearing pages for theme components
- Test: built HTML inspection and browser accessibility review

**Interfaces:**

- Consumes: `.maestro-status[data-status]` and `.maestro-claim[data-status][data-source][data-verified]`
- Produces: `window.MaestroTheme.init()` for one idempotent theme initialization
- Produces CSS variables: `--maestro-ink`, `--maestro-panel`, `--maestro-paper`, `--maestro-muted`, `--maestro-brass`, `--maestro-line`

- [ ] **Step 1: Vendor the approved open font and license**

```bash
mkdir -p theme/fonts theme/css src/images
curl --fail --location \
  'https://raw.githubusercontent.com/google/fonts/45b0855d499c093e4d1bd08926fec4e1a582e225/ofl/cormorantgaramond/CormorantGaramond%5Bwght%5D.ttf' \
  --output theme/fonts/CormorantGaramond.ttf
curl --fail --location \
  'https://raw.githubusercontent.com/google/fonts/45b0855d499c093e4d1bd08926fec4e1a582e225/ofl/cormorantgaramond/OFL.txt' \
  --output theme/fonts/OFL.txt
```

Verify `OFL.txt` contains `SIL OPEN FONT LICENSE Version 1.1`, then commit both files so publication never depends on Google Fonts at runtime.

- [ ] **Step 2: Create The Institution wordmark as local SVG**

Create `src/images/maestro-wordmark.svg` with a transparent view box, accessible title, widely tracked `MAESTRO`, and only the final `O` in brass. Create a monochrome-compatible CSS fallback using `currentColor`. Create `theme/favicon.svg` as the final accented `O`, not a new symbol or monogram.

The SVG must contain no raster data, remote URL, filter, gradient, script, or machine-specific font path.

- [ ] **Step 3: Write the Obsidian Score token and typography layer**

Start `maestro.css` with:

```css
/* Obsidian Score: Maestro's restrained editorial mdBook theme. */
@font-face {
  font-family: "Cormorant Garamond";
  src: url("../fonts/CormorantGaramond.ttf") format("truetype");
  font-display: swap;
  font-weight: 300 700;
}

:root {
  --maestro-ink: #090b10;
  --maestro-panel: #0d1017;
  --maestro-paper: #f5f0e8;
  --maestro-muted: #a7abb5;
  --maestro-brass: #c8a75a;
  --maestro-line: #292d35;
}

.navy {
  --bg: var(--maestro-ink);
  --fg: var(--maestro-paper);
  --sidebar-bg: var(--maestro-panel);
  --sidebar-fg: var(--maestro-muted);
  --links: var(--maestro-brass);
  --inline-code-color: var(--maestro-paper);
  --quote-bg: rgba(200, 167, 90, 0.08);
  --quote-border: var(--maestro-brass);
}

.content main h1,
.content main h2 {
  font-family: "Cormorant Garamond", Georgia, serif;
  letter-spacing: -0.02em;
}

:focus-visible {
  outline: 2px solid var(--maestro-brass);
  outline-offset: 3px;
}
```

Then add responsive hero, navigation, claim, status, roadmap, diagram, code, print, and reduced-motion rules. Do not hide native mdBook controls or visible focus.

- [ ] **Step 4: Implement theme behavior without dependencies**

`theme/maestro.js` exposes one idempotent initializer that:

- adds reading progress with `requestAnimationFrame` throttling;
- opens a native `<dialog>` command palette on `Ctrl/Cmd+K`;
- indexes links already present in the mdBook sidebar rather than creating a second search backend;
- appends visible source and verified-date links to `.maestro-claim` from validated data attributes;
- gives code-copy actions clear success text;
- closes the dialog with Escape and restores focus;
- does nothing when JavaScript is unavailable except omit progressive enhancements.

Guard motion behind `matchMedia('(prefers-reduced-motion: reduce)')` and never remove native mdBook search.

- [ ] **Step 5: Build and inspect the generated contract**

```bash
mdbook build
rg -n 'maestro.css|maestro.js|maestro-claim|maestro-status' build
rg -n 'fonts.googleapis.com|fonts.gstatic.com|node_modules' build && exit 1 || true
```

Expected: local assets and components are present; no remote font or npm reference exists.

- [ ] **Step 6: Perform the visual and accessibility check**

Serve with `mdbook serve`. Check at 360, 768, 1280, and 1600 CSS pixels:

- wordmark legibility;
- sidebar and content hierarchy;
- command palette by keyboard only;
- visible focus;
- claim source visibility;
- reduced-motion mode;
- print preview;
- no horizontal overflow.

Record screenshots under a temporary path, not in the repository.

- [ ] **Step 7: Commit**

```bash
git add theme src/index.md src
git commit -m "feat: create the Obsidian Score documentation theme"
```

### Task 6: Wire Local Gates, CI, and Pages

**Files:**

- Modify: `justfile`
- Create: `.github/workflows/ci.yml`
- Create: `.github/workflows/pages.yml`
- Modify only if dependency evidence requires it: none of the baseline-pinned files
- Test: local `just check`; workflow syntax and action pin checks

**Interfaces:**

- Produces required CI jobs: `common`, `fast`, `site`
- Produces Pages jobs: `build`, `deploy`
- Consumes mdBook version `0.5.4`

- [ ] **Step 1: Write the CI workflow**

Create `.github/workflows/ci.yml`:

```yaml
# Fast repository and site gates. Long evidence remains in heavy.yml.
name: CI

on:
  push:
    branches: [main]
  pull_request:
  workflow_dispatch:

permissions:
  contents: read

concurrency:
  group: ${{ github.workflow }}-${{ github.ref }}
  cancel-in-progress: true

jobs:
  common:
    uses: maestrolabs-hq/.github/.github/workflows/common-fast.yml@main

  fast:
    uses: maestrolabs-hq/.github/.github/workflows/rust-fast.yml@main
    with:
      duplication-test: standards

  site:
    runs-on: ubuntu-latest
    steps:
      - name: Check out the repository
        uses: actions/checkout@fbc6f3992d24b796d5a048ff273f7fcc4a7b6c09 # v5
      - name: Install mdBook
        uses: peaceiris/actions-mdbook@ee69d230fe19748b7abf22df32acaa93833fad08 # v2
        with:
          mdbook-version: "0.5.4"
      - name: Validate and build
        run: |
          cargo run -p sitecheck -- .
          mdbook test
          mdbook build
```

- [ ] **Step 2: Write the least-privilege Pages workflow**

Create `.github/workflows/pages.yml` using these pinned actions:

- `actions/checkout@fbc6f3992d24b796d5a048ff273f7fcc4a7b6c09`
- `peaceiris/actions-mdbook@ee69d230fe19748b7abf22df32acaa93833fad08`
- `actions/configure-pages@983d7736d9b0ae728b81ab479565c72886d7745b`
- `actions/upload-pages-artifact@56afc609e74202658d3ffba0e8f6dda462b719fa`
- `actions/deploy-pages@d6db90164ac5ed86f2b6aed7e0febac5b3c0c03e`

Trigger only on pushes to `main` and manual dispatch. Give the build job `contents: read`; give the deploy job only `pages: write` and `id-token: write`. Use environment `github-pages`, upload `build/`, and set deployment concurrency to cancel an obsolete publication.

- [ ] **Step 3: Prove local parity**

Install mdBook 0.5.4 and run:

```bash
just check
```

Expected: formatting, Clippy, tests, machete, deny, sitecheck, mdBook tests, and mdBook build all pass.

- [ ] **Step 4: Check workflow policy before pushing**

```bash
rg -n 'uses:' .github/workflows
rg -n 'uses: [^#]+@(main|master|v[0-9])' .github/workflows && exit 1 || true
```

Expected: external actions are full-SHA pinned. Reusable Maestro workflows may remain `@main` because they are organization-owned shared gates.

- [ ] **Step 5: Commit**

```bash
git add .github/workflows/ci.yml .github/workflows/pages.yml justfile
git commit -m "ci: validate and publish the Maestro book"
```

### Task 7: Register the Repository in Maestro Governance

**Files:**

- Modify: `../maestro-governance/baseline.txt`
- Test: `cargo test` in `../maestro-governance`; blob-hash comparison in this repository

**Interfaces:**

- Produces governance repository name: `maestro-project-documentation`
- Produces required merge contexts from the first successful CI run

- [ ] **Step 1: Write the failing governance expectation**

In `../maestro-governance`, add a parser test asserting that a baseline containing the new repo and its Rust-scoped files keeps the repository in scope. Run the focused test before editing `baseline.txt`; expect FAIL because the repository is absent.

- [ ] **Step 2: Register the repository and extend every applicable scope**

Add:

```text
repo maestro-project-documentation
```

Extend the scoped directives for:

- `clippy.toml`
- `deny.toml`
- `rust-toolchain.toml`
- `.github/dependabot.yml`
- `.github/release-please/config.json`
- `.github/release-please/manifest.json`
- `CHANGELOG.md`
- `.pre-commit-config.yaml`
- `.github/workflows/heavy.yml`

Do not add the repository to a scoped directive unless `cmp` proves its local file is byte-identical to the tracked source.

- [ ] **Step 3: Recompute and verify blob hashes**

For each tracked file:

```bash
git hash-object PATH
```

The output must match the hash already recorded in `baseline.txt`. A mismatch means the file was copied or edited incorrectly; fix the repository file rather than changing the shared hash.

- [ ] **Step 4: Run governance tests**

```bash
cargo test --manifest-path ../maestro-governance/Cargo.toml --all-targets
```

Expected: PASS.

- [ ] **Step 5: Commit governance separately**

From `../maestro-governance`:

```bash
git add baseline.txt src tests
git commit -m "feat: govern the documentation repository"
```

Keep this as a separate repository commit and pull request.

### Task 8: Create the Remote, Apply Required Contexts, and Run Final Acceptance

**Files:**

- No new product files unless verification exposes a defect
- External: GitHub repository settings and repository ruleset

**Interfaces:**

- Consumes: passing local checks and commits from Tasks 1-7
- Produces: public repository, protected `main`, passing CI, and published Pages site

- [ ] **Step 1: Run the complete local acceptance gate**

```bash
just check
git status --short
```

Expected: every check passes. Only intentionally untracked local visual-companion data may exist, and `.git/info/exclude` keeps it out of status.

- [ ] **Step 2: Create and push the public repository**

```bash
gh repo create maestrolabs-hq/maestro-project-documentation \
  --public \
  --source=. \
  --remote=origin \
  --push
```

If the repository already exists, add its remote and push `main` instead. Do not change organization settings by hand.

- [ ] **Step 3: Verify inherited and audited settings**

Read settings with:

```bash
gh api repos/maestrolabs-hq/maestro-project-documentation \
  --jq '{visibility,has_issues,has_projects,has_wiki,allow_squash_merge,allow_merge_commit,allow_rebase_merge,delete_branch_on_merge,web_commit_signoff_required}'
```

Expected values are the governance baseline: public, issues true, projects/wiki false, squash true, merge/rebase false, delete branch true, signed web commits true. Use `maestro-governance apply` only for writable setting drift; do not hand-edit rules or tracked files through the settings endpoint.

- [ ] **Step 4: Let CI report the exact required contexts**

```bash
gh run watch --repo maestrolabs-hq/maestro-project-documentation --exit-status
```

Record the successful check names returned by GitHub. Create the repository-level required-status-checks ruleset using those exact names, alongside the organization deletion, non-fast-forward, and pull-request floor. Do not guess context names before the first run.

- [ ] **Step 5: Enable GitHub Pages from Actions and verify deployment**

Configure Pages source as GitHub Actions, dispatch `pages.yml`, and run:

```bash
gh workflow run pages.yml --repo maestrolabs-hq/maestro-project-documentation
gh run watch --repo maestrolabs-hq/maestro-project-documentation --exit-status
```

Expected: the deployment succeeds and the Pages URL returns HTTP 200.

- [ ] **Step 6: Run the fleet audit**

From `../maestro-governance`:

```bash
cargo run -- plan
```

Expected after the remote ruleset and settings exist: no drift for `maestro-project-documentation`. If GitHub exposes a control that governance records as pending, report it rather than weakening the baseline.

- [ ] **Step 7: Final acceptance review**

Confirm all eleven acceptance criteria from the design spec with evidence:

- full governed file contract;
- exactly one useful Rust crate;
- local and CI checks green;
- npm-free mdBook build;
- local The Institution assets and license;
- approved navigation and status vocabulary;
- actionable invalid-book diagnostics;
- Pages publication;
- governance registration and rules;
- no duplicated repository source of truth;
- responsive, keyboard, contrast, and reduced-motion checks.
