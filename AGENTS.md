# Working in Maestro Project Documentation

The shared view of the Maestro estate.

Written for an agent, and true for a person.

## Before anything else

Read `README.md`, `CONTEXT.md`, and the approved design in
`docs/superpowers/specs/`. Decisions belong in `docs/adr/` when they meet the
repository's ADR bar.

## Content boundaries

Repository-specific facts remain authoritative in their owning repositories.
Explain relationships here and link to primary sources; do not copy a README,
ADR, backlog, or implementation description into a second source of truth.

Use an explicit `maestro-claim` block only when a factual assertion needs
machine verification. Its source must be an immutable GitHub permalink with a
40-character commit hash and line anchor. Ordinary prose remains a review
responsibility and must not imply machine verification.

Use only these status values:

- `built`
- `in-progress`
- `designed`
- `exploring`

## Working rules

Think before editing. Prefer the smallest change that keeps the estate view
true. Do not create abstractions, navigation sections, or automation for
future needs. Keep changes surgical and define the evidence that will prove
them complete.

Every Rust source starts with a `//!` brief. Prose and identifiers are English
only. Paths are derived from the repository root and never name a developer
machine. External actions are pinned by full commit hash.

Write a failing test before implementing non-trivial validation. Never weaken a
gate to make a change pass.

## Before proposing a change

Run:

```text
just check
```

Report the commands and their output. A green claim without evidence is not a
result.
