# Pagefold first milestone

## Calibration phase — candidate complete, awaiting independent review

Question: can a Polyorama reader render and navigate representative Markdown
with usable layout and long-page scrolling?

Smallest probe: one egui reading pane using Polyorama reading typography,
three supplied Markdown pages, relative links and the supplied PNG. A fixed
read-only fixture bundle replaces directory selection during this probe.

Evidence owner: `evidence/calibration/`, with probe source in `src/` and
reproduction tooling in `tools/`. Record source and fixture hashes, build
results, Lantern observations and opened screenshots here.

Exit condition: observe long-page scrolling, fenced code, tables, Unicode,
local images and broken-link behaviour in a running browser; compare fixture
inventories before and after; retain or reject the approach with explicit
constraints and an independent review. Missing glyphs or layout limitations
must be named rather than hidden.

Presentation contract: the page heading and selectable reading surface lead;
page navigation is secondary. Use Polyorama's reading style as the reference.
Inspect ordinary and narrow widths, long content and error states. Search,
directory selection, refresh and final product qualification are excluded.

| Increment | State | Evidence / next action |
| --- | --- | --- |
| Reader calibration | Candidate complete | `evidence/calibration/report.md`; retain CommonMark approach, repair font coverage before final qualification |
| Application implementation | Pending | Directory access, containment, attachments, rebuildable search, refresh, Japanese glyph coverage and instrumentation |
| Final qualification | Pending | Canonical checks, Lantern journeys, source preservation and independent exact-source review |
