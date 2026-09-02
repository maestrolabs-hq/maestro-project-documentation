# Workspace Intelligence Documentation Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Add a provider-neutral public workspace-intelligence capability and complete the six-lane Maestro estate view without promoting any capability beyond its evidence.

**Architecture:** Keep the public book relational rather than duplicative. One capability page explains stable Maestro behavior and links to primary owner documents. Three new estate pages complete the six repository lanes. Current-reality, roadmap, ADR, source, and claim indexes expose status and immutable evidence while executable policy and provider facts remain in their owning repositories.

**Tech Stack:** Markdown, existing dependency-free `sitecheck`, mdBook 0.5.4, and the repository's existing Rust and prose gates.

**Specs:**

- `docs/superpowers/specs/2026-09-01-maestro-documentation-vitrine-design.md`
- `../maestro-core/docs/superpowers/specs/2026-09-01-workspace-intelligence-facades.md`
- `../maestro-pi-config/docs/superpowers/specs/2026-09-01-workspace-intelligence-providers.md`

## Blocking prerequisites

- Complete and merge Tasks 7-8 of `2026-09-01-maestro-documentation-vitrine.md` first. The remote documentation repository, required contexts, tracked-file scopes, and post-merge `governance plan` must be green. A local onboarding diff is not sufficient.
- Use only merged, reviewed owner documents as public sources. Resolve immutable GitHub links from their 40-character merged commit IDs; never publish a branch link or placeholder digest.
- Keep all four workspace-intelligence capabilities at `designed` until their individual acceptance evidence has merged. Active work may become `in-progress`; `built` requires the immutable evidence contract in Task 2.
- Public documentation and core-facing links use only stable roles, commands, status, and artifact references. Provider names, versions, release URLs, digests, native schemas, and installation details remain only in Pi provider documentation.
- Do not copy executable policy, retry algorithms, trust schemas, or repository-local implementation descriptions into this book.
- Do not commit a RED state. Each task observes the stated failure, completes the matching content, runs GREEN, and commits once.

## File map

**Create:**

- `src/capabilities/workspace-intelligence.md`
- `src/estate/maestro-manifests.md`
- `src/estate/dot-github.md`
- `src/estate/maestro-project-documentation.md`

**Modify:**

- `src/SUMMARY.md`
- `src/estate/index.md`
- `src/estate/maestro-core.md`
- `src/estate/maestro-pi-config.md`
- `src/estate/maestro-governance.md`
- `src/overture/what-is-maestro.md`
- `src/overture/current-reality.md`
- `src/delivery/roadmap.md`
- `src/evidence/sources.md`
- `src/evidence/adrs.md`
- `src/evidence/claims.md`

---

### Task 1: Add the capability and complete all six estate lanes

**Files:**

- Create: `src/capabilities/workspace-intelligence.md`
- Create: `src/estate/maestro-manifests.md`
- Create: `src/estate/dot-github.md`
- Create: `src/estate/maestro-project-documentation.md`
- Modify: `src/SUMMARY.md`
- Modify: `src/estate/index.md`
- Modify: `src/estate/maestro-core.md`
- Modify: `src/estate/maestro-pi-config.md`
- Modify: `src/estate/maestro-governance.md`
- Modify: `src/overture/what-is-maestro.md`
- Modify: `src/evidence/sources.md`

**Interfaces:**

- Produces one **Capabilities → Workspace intelligence** navigation entry.
- Produces estate pages for `maestro-manifests`, `maestro-core`, `maestro-pi-config`, `dot-github`, `maestro-governance`, and `maestro-project-documentation`.
- Keeps each estate page to role, ownership, non-ownership, relationships, current status, and primary links.

- [ ] **Step 1: Add navigation first and observe focused sitecheck RED**

Add these links to `src/SUMMARY.md` without creating their pages yet:

```markdown
# Capabilities

- [Workspace intelligence](capabilities/workspace-intelligence.md)
```

Add the three missing estate links:

```markdown
- [maestro-manifests](estate/maestro-manifests.md)
- [dot-github](estate/dot-github.md)
- [maestro-project-documentation](estate/maestro-project-documentation.md)
```

Keep all six estate links together in ownership-flow order. Run:

```bash
cargo run -p sitecheck -- .
```

Expected RED: `sitecheck` reports each newly referenced missing page with its path and SUMMARY line. Do not commit this state.

- [ ] **Step 2: Create the focused workspace-intelligence page**

Create `src/capabilities/workspace-intelligence.md` with exactly these capability sections:

1. trusted workspace discovery;
2. independent `repository_graph` and `workspace_context` results;
3. architecture validate, render, deliver, status, and artifact retrieval; and
4. durable post-commit and pre-push refresh.

Explain only stable `maestro workspace ...`, `maestro graph ...`, and `maestro architecture ...` commands, separate source envelopes, bounded artifact references, durable acknowledgement, and honest failure/degradation behavior. Link to immutable merged core facade/runtime documents and the Pi provider-integration document as primary sources. Do not copy provider facts from that Pi document.

- [ ] **Step 3: Create three estate pages and bring the three existing pages under the same contract**

Create the pages for `maestro-manifests`, `dot-github`, and
`maestro-project-documentation`. Modify the existing `maestro-core`,
`maestro-pi-config`, and `maestro-governance` pages in the same change. Check
all six pages against the same required content:

- the repository's single ownership lane;
- what it must not own;
- its dependencies on the other five lanes;
- current status using the approved vocabulary; and
- immutable links to its README, relevant accepted ADRs/specifications, and delivery documents.

Replace every mutable branch link in the three existing pages with a permalink
to the reviewed merged source using its full 40-character commit ID. Apply the
same permalink rule to the three new pages. Do not leave a `/blob/main/`,
branch, tag, placeholder, or abbreviated revision in any of the six estate
pages.

`maestro-project-documentation` describes itself as the estate/public view and explicitly refuses executable policy. `maestro-manifests` is the settings and local-membership authority. `dot-github` publishes shared lifecycle definitions and reusable workflows but does not own runtime failure policy.

- [ ] **Step 4: Complete estate and authority indexes**

Update:

- `src/estate/index.md` to show all six lanes and their directed ownership relationships;
- `src/overture/what-is-maestro.md` to name the manifests settings authority without copying its schema; and
- `src/evidence/sources.md` to list all six authoritative repositories and the facts each owns.

- [ ] **Step 5: Run focused GREEN and commit once**

```bash
cargo run -p sitecheck -- .
mdbook build
```

Expected: no missing navigation or local-link diagnostics; `build/capabilities/workspace-intelligence.html` and all six estate pages exist.

```bash
git add src/SUMMARY.md src/capabilities src/estate \
  src/overture/what-is-maestro.md src/evidence/sources.md
git commit -m "docs: explain workspace intelligence across the estate"
```

---

### Task 2: Publish honest status and immutable evidence boundaries

**Interfaces:**

- Produces four separately tracked capabilities: trusted workspace, graph intelligence, architecture, and lifecycle refresh.
- Produces no `built` claim without merged acceptance evidence for that capability.
- Keeps `repository_graph` and `workspace_context` evidence separate.

- [ ] **Step 1: Observe deterministic status-content RED**

Before editing, run this assertion:

```bash
python3 - <<'PY'
from pathlib import Path

capabilities = (
    "Trusted workspace",
    "Graph intelligence",
    "Architecture",
    "Lifecycle refresh",
)
authorities = (
    Path("src/overture/current-reality.md"),
    Path("src/delivery/roadmap.md"),
)
failures = []
for path in authorities:
    text = path.read_text(encoding="utf-8")
    marker = "## Designed\n"
    if marker not in text:
        failures.append(f"{path}: missing Designed section")
        continue
    designed = text.split(marker, 1)[1].split("\n## ", 1)[0]
    for capability in capabilities:
        if capability not in designed:
            failures.append(f"{path}: Designed missing {capability}")
if failures:
    raise SystemExit("\n".join(failures))
PY
```

Expected RED: the assertion exits nonzero and identifies every capability absent
from the Designed section of each authority file.

- [ ] **Step 2: Add the four designed capabilities**

Update `src/overture/current-reality.md` and `src/delivery/roadmap.md` so each capability appears separately under `designed`:

- **Trusted workspace:** nested discovery plus explicit trust and refusal boundaries.
- **Graph intelligence:** separate ordered native-source envelopes and reconstructable receipts.
- **Architecture:** validate, render, deliver, status, and exact artifact retention.
- **Lifecycle refresh:** durable asynchronous post-commit/pre-push scheduling with the approved warning/blocking split.

Do not combine these into one status that could hide partial delivery. Change only an active capability to `in-progress` and only from reviewed work evidence.

- [ ] **Step 3: Link decisions and define promotion evidence**

Update `src/evidence/adrs.md` only after the governance lifecycle amendment and Pi boundary ADR have merged; use immutable links to both accepted decisions.

Update `src/evidence/claims.md` to state the evidence required before each capability may become `built`:

- trusted-workspace refusal and native platform evidence;
- three-repository graph refresh, source separation, degradation, and receipt reconstruction;
- exact architecture input/output retention plus retained browser artifacts and manual review; and
- one immutable hook revision across every parsed governed repository with zero governance drift.

Add an actual `.maestro-claim` only when its merged source proves the complete named capability. Each claim uses a 40-character commit permalink and line anchor. Until then, the claim index describes the required evidence without asserting completion.

- [ ] **Step 4: Run the exact status assertion and evidence GREEN**

Run the same assertion as RED, unchanged:

```bash
python3 - <<'PY'
from pathlib import Path

capabilities = (
    "Trusted workspace",
    "Graph intelligence",
    "Architecture",
    "Lifecycle refresh",
)
authorities = (
    Path("src/overture/current-reality.md"),
    Path("src/delivery/roadmap.md"),
)
failures = []
for path in authorities:
    text = path.read_text(encoding="utf-8")
    marker = "## Designed\n"
    if marker not in text:
        failures.append(f"{path}: missing Designed section")
        continue
    designed = text.split(marker, 1)[1].split("\n## ", 1)[0]
    for capability in capabilities:
        if capability not in designed:
            failures.append(f"{path}: Designed missing {capability}")
if failures:
    raise SystemExit("\n".join(failures))
PY
cargo run -p sitecheck -- .
```

Expected: the assertion proves all four capabilities appear inside the Designed
section of each authority file; statuses use only approved values, and claim
metadata remains valid.

- [ ] **Step 5: Commit**

```bash
git add src/overture/current-reality.md src/delivery/roadmap.md \
  src/evidence/adrs.md src/evidence/claims.md
git commit -m "docs: track workspace intelligence evidence"
```

---

### Task 3: Verify provider neutrality, prose, standards, and rendering

- [ ] **Step 1: Review ownership boundaries**

Read every changed paragraph against its immutable owner source. Confirm that public/core material contains no provider name or pin and that provider-specific facts remain confined to the Pi provider specification and implementation plan. Confirm no public page copies settings schemas, retry internals, executable policy, or native response schemas.

- [ ] **Step 2: Run repository-specific prose and standards checks**

```bash
cargo fmt --all --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --test standards
cargo test --all-targets
cargo run -p sitecheck -- .
mdbook test
mdbook build
just check
git diff --check
git status --short
git diff --cached --name-only
```

Expected: all commands pass; the built book has no broken navigation or claim diagnostics; the cached-file list is empty after commits.

- [ ] **Step 3: Review the generated pages**

Serve the book and inspect the capability, estate, current-reality, roadmap, ADR, source, and claim pages at narrow and wide widths. Verify keyboard navigation, visible focus, status distinction, source-link clarity, and no horizontal overflow. A rendered page is not evidence that a designed capability is built.

## Completion evidence

- `src/SUMMARY.md` exposes Workspace intelligence and all six repository lanes.
- The capability page covers exactly the four approved behaviors through stable Maestro interfaces.
- Current reality and roadmap track all four capabilities independently and honestly.
- Evidence pages link merged decisions and refuse placeholder or mutable claims.
- Public/core prose is provider-neutral; provider details remain in Pi-owned documents.
- `sitecheck`, mdBook, standards, full repository gates, and `git diff --check` pass with no staged files.
