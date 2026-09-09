# Current milestone requalification

Outcome 20a8e12c-7e60-46c9-8b27-bc8d3ce61d20, package
cfc23196-fc31-4d23-8d2f-26ca5d67be3b. This report records the current engineering
candidate, not Bokkie's product acceptance. Earlier reports are preserved as
history from the interrupted outcome.

## Reconciliation and candidate

The current application Rust, service, web entry points, dependency lock and
font bytes match the earlier final qualification manifest. The starting identity
comparison is retained in history-reconciliation.json. This run adds explicit
corrupt-index recovery assertions to the existing Python and browser checks,
allows PAGEFOLD_EVIDENCE to select a fresh evidence directory, generalises the
qualification checker to a current source manifest, and updates current-status
and font provenance prose. No application behaviour repair was necessary.

The original calibration question, smallest three-page/one-image probe, evidence
owner and exit condition were inspected in the retained plan by its exact digest.
The exact old probe and build source were retrieved from retained evidence and
are copied here as historical-plan.md, historical-probe.rs and historical-build.rs.
The historical manifest is evidence/calibration/artefacts.json; the original
five-file fixture identities are evidence/calibration/fixtures-before.json.
The calibration report records its long-page/code/table/image/Unicode/broken-link
observations and decision: retain CommonMark with Polyorama Reading typography,
repair Japanese glyphs. This is now requalified through current canonical tests
and actual opened browser captures, including legible 日本語. Historical evidence
is not substituted for current execution.

The bundled font remains the unmodified Noto Sans CJK JP Regular 2.004, under the
retained SIL OFL. font-metadata.json records its embedded copyright/licence names,
16,467,736-byte decompressed identity and SHA-256, matching the implementation
baseline. The thirteen zlib parts are retained by exact binary artefact identity;
their raw compressed bytes were not dumped into review context.

## Actual checks

verification.log is this run's successful sh tools/verify.sh: formatting, six
Python filesystem/transport tests, three Rust containment/HTML/font tests,
Clippy with warnings denied, native/WASM builds, bindgen and supplied inventory
comparison. The Python regeneration test now also corrupts index.json and
asserts the same source snapshot and rebuilt index. Source/state overlap,
symlink components and targets, FIFOs, hardlinked/symlinked generated indexes,
invalid UTF-8, limits and Host/Origin/content-type boundaries are exercised.
Blocked Markdown paths are rejected before contained-snapshot lookup; there is
no attachment filesystem-read endpoint. External links cannot read outside the
selected root. HTML and remote/reference image handling are covered by Rust tests.

fixtures-before.json and fixtures-after.json equal the original calibration
baseline byte for byte. Tests and refresh mutations used isolated synthetic
copies under .runtime-scratch, with generated state in a separate sibling
directory. The supplied knowledge was only read. No private content was used.

The completed physical refresh journey is refresh.log and refresh-actions.json.
It asserts second-directory selection, loading/disabled controls, external page
edits/additions/renames/deletions, search updates, image replacement/removal,
index deletion AND corruption recovery, failed refresh with unchanged generation
and stale warning, recovery and empty-directory selection. The loading probe
delays the real request, without replacing data. The replacement image is this
run's synthetic empty.png. The observation hook is read-only; input uses physical
Playwright pointer/keyboard events, while Lantern provides captures and the
long-page hover/wheel journey. Every capture has a current state JSON.

## Opened-image judgement

Task/reference: select a directory, browse/search/read it and refresh external
changes using the retained Polyorama Reading presentation contract in the plan.
Every PNG in this directory was opened and assessed against expected content,
state clarity, hierarchy, spacing, overlap, clipping and intentional scrolling.
Ordinary captures are 1100 × 800; narrow captures are 390 × 760.

| Captures | Current observed result |
| --- | --- |
| empty, keyboard-focus, home | Prompt and disabled actions; visible Tab focus on Open directory; Enter opens all three supplied pages and the local gradient |
| missing-page, missing-image-link, unsupported, blocked, external | Exact missing/unsupported/blocked path explanations; current page stays readable, script text remains literal |
| unicode, back-home | Relative link renders Japanese, accented Latin and Greek; physical Back home link returns to Home.md |
| reading, reading-scrolled, reading-return | Highlighted fenced Rust, table, emphasis and inline code; Lantern wheel reaches sections 16–18; return reaches offset zero |
| narrow-reading, narrow-home, narrow-search, no-results | Code/table fit the supplied fixture; image scales; café returns and opens Unicode.md; no-match query preserves the open reader |
| loading, second-directory, edited-open, added-result | Loading is explicit and actions disabled; missing embedded image is named; refresh updates the open text and results; new page opens |
| renamed-open, deleted-open | Browser/search update; old open paths explicitly become unavailable |
| attachment-replaced, attachment-removed | Replaced image visibly changes; removal names the unavailable image |
| failed-refresh, refresh-recovered, empty-directory | Stale warning and old reader retained; recovery succeeds; empty directory clears old content and explains no readable pages |

Behavioural checks: passed. Presentation checks: passed for the inspected canvas
route. Design decision: retain. The controls are stable and reachable, links are
distinct, the image and body wrap at narrow width, and the error states explain
what changed. The upper controls/page list occupy about 328 pixels, leaving less
reading space; this remains a visible design trade-off, not a functional blocker.
The partly visible fourth page-list row is within an intentionally scrolling list.

desktop-layout.json has no body overflow finding. narrow-layout.json reports
406 CSS pixels of document width for a 390-pixel viewport. dom-geometry.json
records the canvas fitting the viewport and the off-viewport eframe text input;
the body clips overflow. Opened pixels and narrow typing pass. This body heuristic
does not audit canvas text, and is retained rather than relabelled a clean audit.

## Provenance, coverage and reproduction

runtime-identities.json identifies actual native/WASM/generated JS/browser bytes,
the clean read-only Polyorama commit/tree, local service and browser recipes.
Registry dependencies remain pinned in Cargo.lock. The actual route is task-owned
Chrome 151, headless ANGLE SwiftShader/WebGL on loopback ports 9317 and 3817.
lantern-capabilities.json identifies the installed build; doctor.json records
the endpoint. flow.json observes fresh navigation with no JavaScript exception
and an incidental favicon 404. final-flow.json retains the expected failed-refresh
400 console entry and preceding collection gaps. These are bounded observations,
not an assertion that every interval was clean.

The accessibility observation is truncated and exposes canvas/textbox only.
Current action text layouts and coverage/exclusions are present in state JSON.
Reader/native label geometry, browser AccessKit/screen-reader operation, native
interactive behaviour and hardware GPU/WebGPU remain unqualified. Native build
success is compilation evidence only. These documented coverage limits are not
waived or converted into zero-finding audits.

Use README.md for the app recipe. For inspection start tools/start-inspection-browser.sh
and the service with .runtime-scratch/qualification-state. Set PAGEFOLD_EVIDENCE
to an absolute fresh directory before qualification-action.mjs and
qualification-refresh.mjs; capture empty.png there before the refresh journey.
The action log retains the physical journey and current postconditions.
Run PAGEFOLD_EVIDENCE=... python3 tools/check-qualification.py against the retained
source-artefacts.json and state files. Open screenshots after reproducing them.

review-artefacts.json is the exact local deliverable manifest supplied to the
independent reviewer. Registration evidence and the actual report are retained
separately after review. No commit, publication, deployment, credential access,
global configuration change or service installation was performed. Bokkie decides
product acceptance separately from this engineering submission.
