<div align="center">

# Maestro Project Documentation

**The shared view of the Maestro estate**

Cross-project knowledge for people and agents, rendered as the public Maestro vitrine.

</div>

---

This repository explains how Maestro's repositories fit together without
copying the implementation facts they own. Markdown is the source; mdBook is
the public rendering; `sitecheck` verifies the repository and book contracts.

## Commands

Every command works directly. `just` is an optional shorthand.

```text
just install   install the pinned Rust and documentation tools
just setup     install local git hooks
just check     run the same gates as CI
just build     validate and build the book
just serve     serve the book locally and open it
```

## Layout

```text
crates/sitecheck   repository-specific documentation validation
src                mdBook source and cross-project knowledge
theme              Obsidian Score presentation assets
docs/adr           decisions owned by this repository
docs/superpowers   approved designs and implementation plans
```

Repository-specific implementation details remain authoritative in their own
repositories. This repository owns only the estate view, shared vocabulary,
and public roadmap.
