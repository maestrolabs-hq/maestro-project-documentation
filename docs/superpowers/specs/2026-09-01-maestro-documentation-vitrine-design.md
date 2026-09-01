# Maestro documentation vitrine design

Status: Approved

Approval basis: the user explicitly approved including `maestro-project-documentation` as the sixth Maestro repository lane and directed this design to proceed.

## Purpose

Build one repository that serves two readers without maintaining two bodies of truth:

- people and agents use the Markdown as cross-project knowledge;
- outsiders use the generated mdBook site as the Maestro vitrine.

The site presents the engineering method as the evidence behind Maestro. It distinguishes current reality from future direction and never replaces repository-local documentation.

## Design principles

1. **Repository-local facts stay local.** Each Maestro repository remains authoritative for its implementation, decisions, and work.
2. **This repository owns the estate view.** It explains how the repositories fit together, records Maestro-level vocabulary, and presents the central roadmap.
3. **Actual and future are visibly different.** Every roadmap item is `built`, `in-progress`, `designed`, or `exploring`.
4. **Claims carry evidence.** Factual assertions that need machine verification use explicit claim blocks linked to immutable GitHub permalinks pinned to commit SHAs. Human review remains responsible for ordinary prose.
5. **The site follows the standard it presents.** The repository is governed, tested, cross-platform, and publishable only when its checks pass.
6. **Rust-native and dependency-light.** mdBook provides the site; one small Rust binary provides repository-specific checks. There is no npm toolchain.

## Repository architecture

```text
maestro-project-documentation/
├── .github/
│   ├── release-please/
│   │   ├── config.json
│   │   └── manifest.json
│   ├── workflows/
│   │   ├── ci.yml
│   │   ├── heavy.yml
│   │   └── pages.yml
│   ├── CODEOWNERS
│   └── dependabot.yml
├── crates/
│   └── sitecheck/
│       ├── src/
│       │   └── main.rs
│       └── Cargo.toml
├── docs/
│   ├── adr/
│   └── superpowers/specs/
├── src/
│   ├── SUMMARY.md
│   ├── index.md
│   ├── overture/
│   ├── method/
│   ├── capabilities/
│   ├── estate/
│   ├── delivery/
│   ├── evidence/
│   └── images/
├── theme/
│   ├── css/
│   ├── fonts/
│   ├── favicon.svg
│   └── maestro.js
├── .editorconfig
├── .gitattributes
├── .gitignore
├── .pre-commit-config.yaml
├── AGENTS.md
├── BACKLOG.md
├── CHANGELOG.md
├── CONTEXT.md
├── Cargo.lock
├── Cargo.toml
├── LICENSE
├── NORTHSTAR.md
├── README.md
├── TODO.md
├── book.toml
├── clippy.toml
├── deny.toml
├── justfile
└── rust-toolchain.toml
```

Only one Rust crate is created. `sitecheck` has an immediate job; no speculative crates are scaffolded.

## Governance contract

### Files copied byte-for-byte

The following files use the estate versions and remain tracked by blob hash in `maestro-governance/baseline.txt`:

- `.editorconfig`
- `.gitattributes`
- `.gitignore`
- `LICENSE`
- `NORTHSTAR.md`
- `clippy.toml`
- `deny.toml`
- `rust-toolchain.toml`
- `.pre-commit-config.yaml`
- `CHANGELOG.md`
- `.github/CODEOWNERS`
- `.github/dependabot.yml`
- `.github/release-please/config.json`
- `.github/release-please/manifest.json`
- `.github/workflows/heavy.yml`

The new repository is added to every applicable scoped `file` directive in the governance baseline. Shared files are copied, not recreated from memory.

### Files specific to this repository

- `README.md` explains the repository and local commands.
- `AGENTS.md` records local working rules.
- `CONTEXT.md` defines domain vocabulary only.
- `BACKLOG.md` defines the delivery backlog and its readiness requirements.
- `TODO.md` remains limited to trivial local cleanup.
- `book.toml`, `src/`, and `theme/` define the site.
- `Cargo.toml`, `crates/sitecheck/`, and `justfile` define validation and local tasks.
- `ci.yml` calls the shared common and Rust fast workflows and adds a site job.
- `pages.yml` publishes the already-checked site artifact.

GitHub supplies organization-level community files such as `SECURITY.md`, `CONTRIBUTING.md`, and issue templates from the `.github` repository. They are not duplicated here.

### Repository settings and rules

The GitHub repository is:

- public;
- squash-merge only;
- configured to delete merged branches;
- configured to require signed web commits;
- configured with issues enabled and repository wiki/projects disabled;
- protected by organization rules plus a repository ruleset naming its required CI contexts.

`maestro-governance/baseline.txt` records the repository and all applicable tracked files. The organization-level GitHub Project remains the portfolio view; repository Projects stay disabled.

## Content architecture

### Overture

- What Maestro is
- What exists today
- How to read the site

### Method

- Principles
- Proved gates
- Decisions beside code
- Rebuilding from source

### Capabilities

- Workspace intelligence

The workspace-intelligence page explains trusted workspace discovery, separate repository-graph and workspace-context results, architecture validation/render/delivery, and durable lifecycle refresh through stable Maestro commands and artifact references. It links to core and Pi-config primary documents without copying provider names, versions, pins, native schemas, retry internals, or executable policy.

### Estate

- Ecosystem map
- maestro-manifests
- maestro-core
- maestro-pi-config
- dot-github
- maestro-governance
- maestro-project-documentation

These are the six repository ownership lanes. Repository pages explain roles, non-ownership boundaries, relationships, and current status. They link to repository-local documents instead of copying them. The estate index and source-authority table must list all six even when a capability remains only `designed`.

### Delivery

- Idea pipeline
- Backlog model
- Roadmap

The roadmap presents `built`, `in-progress`, `designed`, and `exploring` work. It is a portfolio narrative, not a second task tracker.

### Evidence

- Verified claims
- ADR index
- Source repositories

A machine-verified factual assertion uses an explicit claim block containing an immutable GitHub permalink. The page may state when it was last verified, but must not imply that old evidence is current. Assertions outside claim blocks are reviewed as prose rather than presented as machine-verified.

## Idea-to-production model

The site documents, but does not initially automate, this flow:

```text
Idea
  → brainstorm
  → grill
  → define functional and non-functional requirements
  → architecture review when needed
  → acceptance criteria and definition of done
  → split into initiatives, epics, and tasks
  → human approval
  → backlog
  → development
  → verification
  → released
```

Canonical terms:

- **Idea:** an uncommitted thought.
- **Proposal:** an idea undergoing discovery.
- **Initiative:** an approved Maestro-level outcome.
- **Epic:** a delivery slice assigned to one repository.
- **Task:** a development-ready implementation unit.
- **Backlog:** prioritized initiatives and epics that meet their readiness definition.
- **Roadmap:** the public projection of approved direction.

Future `maestro idea add|list|show|promote|delete` commands may wrap the approved workflow. `promote` starts processing and pauses for human approval at major gates. No command or autonomous agent pipeline is part of the initial site bootstrap.

## Visual design

The selected direction is **Obsidian Score**:

- ink-black surfaces;
- warm brass accents;
- restrained editorial serif headlines;
- compact sans-serif interface text;
- subtle orchestral language without decorative excess;
- responsive layouts with strong keyboard and screen-reader behavior.

The site uses a customized mdBook theme rather than replacing mdBook with a custom generator.

### Wordmark

The selected identity is **The Institution**, a wordmark rather than a forced standalone symbol:

- `MAESTRO` in an open-licensed, self-hosted editorial serif;
- generous spacing and classical proportions;
- the final `O` as the only brass accent, representing completion and accountability;
- no monogram, invented glyph, conductor icon, or generated AI mark;
- monochrome and small-size variants that remain legible without the accent color;
- SVG exports for the site, social preview, and repository identity.

The first implementation uses Cormorant Garamond under the SIL Open Font License. The required font files, license, and final SVG are stored locally so the published identity does not depend on a third-party font service.

### Features

- native mdBook full-text search;
- a small `⌘K` command palette over site navigation;
- status badges for roadmap and implementation state;
- source links and last-verified markers;
- accessible estate and idea-pipeline diagrams;
- breadcrumbs and related-page links;
- heading permalinks and reading progress;
- polished code blocks and copy feedback;
- print styles;
- reduced-motion support.

Theme behavior uses minimal vanilla JavaScript. There is no npm dependency, animation framework, custom search backend, account system, comments, or analytics dashboard.

## Validation

`sitecheck` validates rules specific to this repository:

- all required repository files exist;
- `src/SUMMARY.md` and referenced pages agree;
- internal links resolve;
- status values belong to the approved set;
- every explicit claim block has a GitHub permalink pinned to a commit SHA;
- required claim metadata is present;
- generated-site inputs do not contain machine-specific paths.

Failures identify the path, line, and violated rule. The first implementation uses the standard library unless a real parser requirement proves a dependency necessary.

`mdbook test` checks Rust code examples. `mdbook build` proves the complete book renders.

## CI and deployment

### Pull requests and pushes

`.github/workflows/ci.yml` runs:

- `common`, using the organization common-fast reusable workflow;
- `fast`, using the organization Rust fast reusable workflow;
- `site`, running `sitecheck`, `mdbook test`, and `mdbook build`.

The repository ruleset requires the exact resulting contexts.

### Heavy evidence

`.github/workflows/heavy.yml` remains the estate-standard weekly evidence workflow. It is not a required merge context.

### Pages

`.github/workflows/pages.yml` publishes only from `main`. It reruns the same validation and build commands used by CI, then deploys with least-privilege Pages permissions. A failed validation or build prevents publication.

## Error posture

The site is not published when:

- a required file is missing;
- navigation references a missing page;
- an internal link is broken;
- a status is unknown;
- an explicit claim block lacks immutable evidence;
- Rust validation fails;
- mdBook examples or the site build fail.

External claims are presented as verified at a specific source revision. Freshness is deliberate work, not silently inferred from an old passing build.

## Testing

The smallest durable checks are:

- unit tests for each non-trivial `sitecheck` parser or rule;
- one invalid-book fixture proving structural failures are caught;
- one valid fixture proving the minimum book passes;
- `mdbook test` for compilable Rust examples;
- `mdbook build` for rendering;
- CI on Linux, macOS, and Windows through the shared Rust workflow.

Visual behavior is checked responsively and with reduced motion before the first release. Accessibility basics are not optional: semantic landmarks, visible focus, keyboard navigation, sufficient contrast, and useful alternative text are part of the definition of done.

## Initial acceptance criteria

The bootstrap is complete when:

1. the repository contains the full Maestro file contract;
2. the Rust workspace has exactly one useful crate, `sitecheck`;
3. all local and CI checks pass;
4. mdBook builds the Obsidian Score site without npm;
5. The Institution wordmark ships as local SVG and font assets with its license;
6. the site contains the approved top-level navigation and honest status vocabulary;
7. invalid structure or unsourced claim blocks fail with actionable diagnostics;
8. the Pages workflow can publish from `main`;
9. governance onboarding is merged and verified before any cross-repository lifecycle rollout depends on this lane, including baseline coverage, required contexts, and a zero-drift audit;
10. no repository-local fact is duplicated as a second source of truth;
11. the site works on narrow screens, by keyboard, and with reduced motion.

## Out of scope for the bootstrap

- implementing the autonomous idea-to-production pipeline;
- implementing `maestro idea` commands;
- maintaining a second internal documentation tree;
- generating live repository statistics;
- comments, accounts, analytics dashboards, or a CMS;
- a custom static-site generator;
- speculative Rust crates.
