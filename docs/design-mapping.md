# DESIGN.md mapping

`DESIGN.md` describes a marketing site: a nav bar with sign-up pills, pricing
cards, a template grid, a customer-logo strip, a full-bleed hero mesh
gradient. This repository publishes an mdBook: a sidebar, a table of
contents, long-form prose, code blocks, and search. A literal application of
one to the other is not possible. This page records the honest
transposition, in the same spirit as the vocabulary-mapping tables used
elsewhere in the estate: what each `DESIGN.md` element means, and what it
becomes here, including where nothing carries over.

## Element mapping

| `DESIGN.md` element | What it means | mdBook counterpart |
| --- | --- | --- |
| `{colors.canvas}` / `{colors.canvas-soft}` / `{colors.canvas-soft-2}` | White-to-near-white page and card surface ladder | Book background and code-block/table surface ladder |
| `{colors.ink}` / `{colors.body}` / `{colors.mute}` | Heading, body, and low-priority text | Book heading, body, and muted (sidebar/caption) text |
| `{colors.hairline}` / `{colors.hairline-strong}` | 1px dividers and borders | Table borders, rules, sidebar dividers |
| `{colors.link}` / `{colors.link-deep}` | Primary link color and its pressed/visited tone | Inline link color and visited/active state |
| `{colors.primary}` / `{colors.on-primary}` | Ink-black CTA fill and its white text | Dark-variant page background and its text (see Decisions) |
| Geist / Geist Mono | The two faces that carry the entire system | Book heading/body face and code-block face |
| `typography.display-*` scale, negative tracking | Marketing headline sizes | mdBook `h1`-`h3` sizes and letter-spacing |
| `typography.body-*` / `typography.caption*` / `typography.code` | Body, caption, and code type scale | Book prose, sidebar labels, and code-block type |
| `spacing.*` (4px base) | The spacing scale | Vertical rhythm between headings, paragraphs, and blocks |
| `rounded.sm` / `rounded.md` / `rounded.lg` | UI, card, and large-card corner radii | Code block, table, and callout corner radii |
| Elevation levels 1-3 (inset hairline, soft/subtle stacked shadow) | Card elevation | Code block and table elevation |
| `nav-bar`, `nav-link` | Top marketing nav | mdBook sidebar and its link rows |
| `link-inline` | Body-copy inline link | Book prose link |
| `badge-secondary`, `.maestro-status` pill | Small inline metadata pill | Existing `maestro-status` pill (colors re-mapped; see below) |
| Mesh gradient (hero scale only) | The brand's only decorative chrome | Restricted to `index.md`'s hero band only, per `DESIGN.md`'s own "hero scale only" rule |
| `nav-cta-signup` / `nav-cta-login` / `nav-cta-ask-ai` | Auth and AI-assistant nav buttons | No equivalent -- this book has no accounts, sign-up, or in-book assistant |
| `pricing-card`, `pricing-card-featured` | Pricing-tier cards | No equivalent -- nothing here is sold |
| Template grid (`template-card`, 5-up/3-up/2-up/1-up) | Deploy-template gallery | No equivalent -- no template catalogue exists |
| `logo-strip` | Customer-logo row | No equivalent -- no customer roster to display |
| `code-editor-mockup` | Dark code-preview marketing surface | No equivalent -- the book's own code blocks already show real, runnable code, not a mockup |
| `ex-*` illustrative examples (subscription-summary, product-selector, auth-form surfaces) | Auto-derived kit-mirror demonstration surfaces | No equivalent -- illustrative only in `DESIGN.md` itself, not part of the captured brand system |

## Decisions already taken

**Light canvas by default, with a dark variant derived from the same
tokens.** `DESIGN.md` is a light-canvas system (`{colors.canvas}` /
`{colors.canvas-soft}`); this book's current theme defaults to dark
(`default-theme = "navy"` in `book.toml`). The book will default to a light
theme built from `{colors.canvas}`, `{colors.ink}`, and the rest of the
light-side token set, with a dark variant derived by substituting
`{colors.primary}` as the page background and `{colors.on-primary}` as text --
the same polarity-flip `DESIGN.md` itself documents for
`showcase-band-dark` and `pricing-card-featured`. No colour outside the
`DESIGN.md` token set is introduced for either variant.

**Geist and Geist Mono, self-hosted, replacing Cormorant Garamond.** The
current theme's serif display face (`theme/fonts/`) is replaced by Geist for
headings and body, and Geist Mono for code, matching `DESIGN.md`'s "two
custom faces carry the entire system." Both are OFL-licensed and
self-hostable, consistent with how the current fonts are already served.

Neither decision is implemented in this change. Both are recorded here for
the theme implementation that follows.

## Status-colour mapping (proposed)

The book's four status values (`built`, `in-progress`, `designed`,
`exploring`) are currently rendered as `color: var(--maestro-*)` text inside
a pill with `border: 1px solid currentColor` (`theme/css/maestro.css`,
`.maestro-status[data-status="*"]`) -- the status colour is the text colour
itself, read directly against the page canvas. Any replacement must clear
WCAG AA for small text (4.5:1) against `{colors.canvas}` (`#ffffff`) under
that same rendering, and must draw only from tokens already declared in
`DESIGN.md` -- no colour is invented for this mapping.

| Status | Candidate token | Hex | Contrast vs `{colors.canvas}` | WCAG AA small text (4.5:1) |
| --- | --- | --- | --- | --- |
| `built` | `{colors.success}` | `#0070f3` | 4.55:1 | Pass (marginal) |
| `in-progress` | `{colors.warning}` | `#f5a623` | 2.03:1 | **Fail** |
| `in-progress` (revised) | `{colors.warning-deep}` | `#ab570a` | 5.12:1 | Pass |
| `designed` | `{colors.violet}` | `#7928ca` | 7.07:1 | Pass |
| `exploring` | `{colors.cyan}` | `#50e3c2` | 1.60:1 | **Fail** |
| `exploring` (revised) | `{colors.cyan-deep}` | `#29bc9b` | 2.40:1 | **Fail** |

Proposed mapping: `built` -> `{colors.success}`, `in-progress` ->
`{colors.warning-deep}` (not the bare `{colors.warning}` token, which fails
outright), `designed` -> `{colors.violet}`. `{colors.warning}` and
`{colors.cyan}` both read as fills or accents inside `DESIGN.md`'s own
components (`warning` as a status-indicator swatch, `cyan` inside gradient
stops and spotlight accents), not as small body-scale text on a white page,
which is why they fail here despite being legitimate brand tokens.

**Open question for the theme implementation (lot 2):** no cyan-family token
clears AA as text for `exploring`. The nearby alternatives were checked and
rejected: `{colors.link-deep}` (`#0761d1`, 5.77:1) passes but repeats the
blue hue already assigned to `built`, which would make two of four statuses
read as the same colour family; `{colors.mute}` (`#888888`, 3.54:1) and
`{colors.hairline-strong}` (`#a1a1a1`, 2.58:1) both still fail 4.5:1. This
page does not resolve that gap -- the theme implementation must either pick
one of these tradeoffs, or render `exploring` as a filled chip (colour as
background, not as text) rather than as `currentColor` text, which changes
the component rather than the token choice.
