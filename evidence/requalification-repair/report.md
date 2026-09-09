# Requalified candidate after independent review repair

This is the current candidate for outcome 20a8e12c-7e60-46c9-8b27-bc8d3ce61d20.
The preceding evidence/requalification/report.md records historical reconciliation
and the first current engineering pass. The independent reviewer found that
navigation could overwrite its failed-refresh status and hide staleness. That
candidate is superseded. This directory contains fresh captures of the repaired
native/WASM source; product acceptance remains Bokkie's separate decision.

## Repair and current verification

Pagefold now retains snapshot_stale separately from transient status. A failed
open/refresh marks an existing snapshot stale; every visible status includes the
warning while that flag is set. Relative links, missing/blocked links and page-list
navigation cannot clear it. Only accepting a successful snapshot clears it.
An initial open failure does not falsely claim that a previous snapshot exists.
README.md describes this behaviour. The Rust regression exercises all these
transitions, including successful recovery. No content, index or path policy changed.

Canonical sh tools/verify.sh passed: formatting, six Python filesystem/transport
tests, FOUR Rust path/HTML/font/persistent-staleness tests, Clippy, native build,
WASM build, bindgen and fixture comparison. verification.log retains the run.
The original corruption-recovery assertions remain in Python and browser checks.
Runtime identities in this directory identify the repaired native/WASM/generated
JS bytes, unchanged browser and clean Polyorama dependency. Source-artefacts and
review-artefacts manifests identify the exact uncommitted deliverable.

The physical reading journey was replayed after reloading the rebuilt WASM.
reader-actions.json and tools/requalification-reader.mjs reproduce directory
selection, Tab/Enter, relative links, missing/unsupported/blocked targets,
ordinary/narrow code/table/image/Unicode reading and search. The first replay
plan omitted the two Lantern wheel commands because they were outside the action
log; those captures were corrected with actual Lantern hover/wheel and the plan
now includes physical wheel steps. scroll.json, scroll-top.json and current
reading-scrolled/reading-return state retain the successful real observations.
No application response or content was mocked.

The isolated refresh journey then passed all external addition/edit/rename/delete,
image replacement/removal, index deletion/corruption rebuild, loading/disabled,
failure/recovery and empty-directory assertions. Its added stale-navigation and
stale-search assertions prove that the warning survives a physical relative link
to guides/Reading.md and opening the café search result. The recovery assertion
requires snapshot_stale=false. refresh-actions.json and all state JSON are current;
the new stale captures are separately retained. The 1.8-second request delay is
only a loading observation; it does not replace the service response.

fixtures-before.json and fixtures-after.json exactly match the original
evidence/calibration/fixtures-before.json baseline. Only isolated synthetic copies
under .runtime-scratch were mutated. The supplied five knowledge files were read
without modification. Derived state remains separate and overlap is rejected.

## Current visual and behavioural judgement

Task and reference remain the calibrated Polyorama Reading presentation and
the active plan's stable controls, bounded page list and selectable reader.
All 30 PNGs in this directory were opened. Desktop images are 1100 × 800;
narrow-reading, narrow-home, narrow-search and no-results are 390 × 760.

Behavioural checks: passed. Presentation checks: passed within the documented
canvas coverage. Design decision: retain the repaired candidate. The original
presentation observations were reproduced: readable Japanese 日本語, Latin accents
and Greek; fenced Rust, table and inline code; scaled local image; pane-local
long scrolling and return to top; clear selected, missing, unsupported, loading,
empty and failed/recovered states. Embedded script text stays literal. The stale
warning remains visibly present in stale-navigation.png and stale-search.png,
and is absent after successful recovery. The visible fourth page-list row is
part of intentional list scrolling. Dense upper chrome still reduces reading
space, but controls stay reachable and no unwanted canvas overlap was observed.

The fresh body layout observations are retained in desktop-layout.json and
narrow-layout.json. This run reports 509 CSS pixels of document scroll width
at the final 390-pixel empty view; dom-geometry.json again records an off-viewport
eframe input while the canvas fits and body clips overflow. They do not measure
canvas text. The prior 406-pixel narrow
body heuristic and off-viewport text-input diagnosis remain historical evidence;
no browser accessibility claim is inferred from either layout result. Current
accessibility.json remains bounded canvas/textbox evidence. Action text coverage
and explicit reader/native label exclusions are retained in current state JSON.
Browser AccessKit/screen-reader operation, native interactive/OS assistive-technology
behaviour and hardware GPU/WebGPU are unqualified. Native compilation is verified.
The actual browser route remains Chrome 151, headless ANGLE SwiftShader/WebGL.
final-flow.json observes no JavaScript exception in its bounded interval and
retains expected request-error history/collection gaps; it is not a global clean log.

## Criterion and provenance map

| Criterion | Current supporting evidence |
| --- | --- |
| reader_calibration | Retained exact historical plan/probe/build, calibration manifest/decision and original fixture baseline in evidence/requalification and evidence/calibration; current four Rust tests plus opened reading, scroll, image, Unicode and broken-link captures |
| workspace_reading | Physical directory selection and ordinary relative navigation, page/search controls, local gradient, missing/unsupported/blocked captures; README and docs/content-contract.md |
| search_refresh | Current refresh journey, persistent-staleness regression, corrupt/deleted index recovery, external-change and stale navigation/search/recovery captures |
| source_preservation | Current before/after inventories equal original baseline; isolated-copy Python/physical tests and overlap/symlink/hardlink checks |
| content_safety | Unchanged contained snapshot service, path/HTML/image tests, Host/Origin/content-type boundary tests and blocked/external/inert-content UI observations |
| ui_qualification | All current opened ordinary/narrow captures, physical actions, states, Lantern wheel/layout/accessibility/flow and explicit coverage limits |
| local_delivery | Current source/build manifests, successful canonical checks, content/run policy, unchanged Polyorama identity and font licence/provenance, separately registered repair review |

The Noto font binary parts, licence and metadata are unchanged. Exact decompressed
identity and embedded copyright/licence were inspected and remain in
evidence/requalification/font-metadata.json; the origin and licence are documented
in assets/fonts. Raw compressed font bytes were not dumped into review context.
Historical source identities, calibration decision and input inventory remain
inspectable; they are not replaced by summary-only claims of acceptance.

For reproduction, follow README.md, start the local service with the dedicated
.runtime-scratch/qualification-state directory and launch the task-local browser.
Set PAGEFOLD_EVIDENCE to an absolute fresh output directory containing the retained
reader-actions.json, run tools/requalification-reader.mjs, then arrange 1100 × 800
and clear search through qualification-action.mjs before qualification-refresh.mjs.
Keep the captured empty.png for the synthetic image replacement. After collecting
inventories and source-artefacts.json, run tools/check-qualification.py with the
same PAGEFOLD_EVIDENCE. Open generated images; script success is not visual review.

The exact candidate is reviewed separately from Bokkie's product assessment.
Review registration and its report are retained alongside the submission after
completion. No commit, remote publication, deployment, credentials, global
configuration change, service installation or future milestone feature is included.
