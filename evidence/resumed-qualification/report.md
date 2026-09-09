# Resumed exact-candidate qualification

Outcome 20a8e12c-7e60-46c9-8b27-bc8d3ce61d20; worker execution
26d76e4d-c009-469f-89ed-bf630f1ba85c. Product acceptance remains pending Bokkie.
This execution completes evidence and independent review for the unchanged
repaired candidate described in evidence/requalification-repair/report.md.
All 38 source/configuration/tool/font entries match that candidate's source
manifest. The review manifest additionally identifies the calibration records,
font metadata, repaired report and dependency/build record. No commit was made.

Canonical `sh tools/verify.sh` passed in this execution: six Python filesystem
and transport tests, four Rust containment/HTML/Japanese/persistent-staleness
tests, formatting, Clippy, native/WASM builds, bindgen and fixture comparison.
The complete output is ../requalification-repair/resumed-verification.log.
runtime-identities.json records that rebuilt native/WASM/JS and browser bytes
match the repaired baseline and that the Polyorama dependency remains clean.

The original calibration question, three-page/one-image probe, exact source and
input identities and retain/repair decision were inspected in the retained
historical plan, Rust probe/build, calibration manifest and report. They justify
the retained reader mechanism; current behaviour is established separately by
this execution's tests and physical journeys. Font licence and provenance were
inspected; all thirteen binary parts were identity-verified without dumping
compressed data. Current glyph tests and opened pixels show readable 日本語.

reader-actions.json was replayed through physical pointer/keyboard/wheel input
against the freshly loaded WASM. reader.log records completion. The separate
refresh journey passed with isolated synthetic copies: edits, additions,
renames, deletions, image replacement/removal, deleted/corrupt index rebuilding,
loading/disabled controls, failure with old generation retained, stale navigation
and stale search, successful recovery and empty-directory selection. The loading
probe delays the real request by 1.8 seconds without replacing its response.
refresh-actions.json and refresh.log retain the actions and assertions.
fixtures-before.json and fixtures-after.json both equal the original five-file
calibration baseline. Source/state overlap, symlinks, special files, index
hardlinks, unsafe links and Host/Origin/content-type boundaries pass canonical
tests. Supplied knowledge was only read; generated state stays separate.

All 30 PNGs in this directory were opened. Expected presentation: stable top
controls, bounded page list, selectable width-constrained reader, readable text,
clear errors, intentional reader/list scrolling and wrapping at narrow width.
Ordinary images are 1100 × 800; narrow-reading, narrow-home, narrow-search and
no-results are 390 × 760. Behavioural judgement: pass. Presentation judgement:
pass for the inspected canvas route. Retain the candidate.

The images show relative navigation and return, local gradient and replacement
image, explicit missing/unsupported/blocked targets, literal embedded script,
Japanese/Latin/Greek, fenced code and table, sections 16–18 after scrolling,
return to top, keyboard focus, search and no-result states. The remaining
captures show external edits in the open reader, rename/deletion explanations,
loading, empty-directory content, stale warning during navigation/search and
its removal after recovery. Dense upper controls reduce reading space, but stay
reachable. The fourth list row is intentionally within a scrolling list.

Lantern capabilities, doctor, fresh-navigation flow, final-flow, ordinary/narrow
body layout and accessibility observations are retained here. Desktop body
layout has no finding. Narrow body layout reports 512 CSS pixels of document
scroll width at 390 pixels; dom-geometry.json records the off-viewport eframe
input, fitting canvas and clipped body. Opened narrow text, table and image fit.
This DOM heuristic does not measure canvas reader geometry. Accessibility is a
truncated canvas/textbox observation; reader/native text geometry, browser
AccessKit/screen readers, native interaction/OS assistive technology and hardware
GPU/WebGPU are unqualified. Native compilation is verified. The observed route
is Chrome 151 with headless ANGLE SwiftShader/WebGL. Fresh flow has an incidental
favicon 404; final flow retains the expected failed-refresh 400 and collection
gaps. Neither interval reports a JavaScript exception; no global clean-log claim
is inferred.

Use README.md and tools/start-inspection-browser.sh for local reproduction.
Set PAGEFOLD_EVIDENCE to an absolute output directory containing reader-actions.json;
run tools/requalification-reader.mjs, arrange 1100 × 800 and clear search with
qualification-action.mjs, then run qualification-refresh.mjs. Keep empty.png for
the isolated replacement image. Collect inventory and source manifest, run
tools/check-qualification.py and open the generated screenshots.

The exact independently reviewed candidate and evidence identities are retained
in review-artefacts.json. Registration and report bytes are recorded separately
after review. These are engineering evidence, not product acceptance. No
publication, deployment, credentials, global configuration change or future
milestone feature is part of this deliverable.
