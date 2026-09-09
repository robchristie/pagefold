# Final filesystem-backed UI qualification

The accepted Pagefold source passed the physical browser journeys below.
Application source, tests, dependencies and font assets remain byte-identical to
the accepted inputs. Only qualification tools/evidence and final use/plan prose
were added or updated. Bokkie product acceptance is pending and is separate from
this engineering judgement and the registered independent review.

## Task, reference and judgement

The task is to select a local directory, find and read Markdown, follow contained
links and see external changes after refresh without changing source knowledge.
The reference is the calibrated Polyorama Reading appearance and the presentation
contract in docs/active-plan.md: stable controls, bounded page list, selectable
width-constrained reader and pane-local scrolling.

Behavioural checks: passed. Presentation checks: passed within the stated canvas
coverage. Design decision: retain for this milestone. The reader is legible,
link colour is distinct, selected pages are visible, controls remain reachable,
and missing/loading/stale states explain what happened. The dense control and
page-list area occupies roughly the upper 328 pixels at the normal width; it
reduces visible reading space and would benefit from later product refinement.
The heading hierarchy is restrained. At 390 × 760 the page wraps and images scale;
the directory field intentionally scrolls its long path. Code fits this fixture,
tables retain their columns and long content scrolls inside the reader. A partly
visible fourth page-list row is intentional list scrolling, not lost content.
No overlap or unwanted viewport overflow was observed in the opened images.
Lantern's narrow body heuristic nevertheless reports 406 CSS pixels of document
scroll width for a 390-pixel viewport; this is retained as a structural finding,
not silently treated as a clean canvas audit.
Follow-up dom-geometry.json shows the canvas exactly matching 390 × 760 while
eframe's transparent one-pixel text input retains an off-viewport caret position
after resize. This accounts for document scroll-width risk outside the canvas;
the body clips it and physical narrow typing/search and visible rendering passed.

These are separate behavioural, presentation and design judgements. Body DOM
layout audits cannot certify canvas layout. The custom observation hook reports
current control rectangles and selected state; it does not measure reader text
geometry, provide a browser AccessKit tree or prove native assistive-technology
support. Hardware GPU/WebGPU and native interaction are unqualified. Software
WebGL browser operation and native compilation are the retained routes.

## Physical journeys and opened images

All PNG files in this directory were opened and assessed at their actual size.
Desktop images are 1100 × 800; narrow images are 390 × 760. Expected content,
state clarity, hierarchy, alignment, clipping and intentional scrolling were
assessed against the contract above.

| Captures | Observation |
| --- | --- |
| empty, keyboard-focus, home | Initial prompt and disabled actions; Tab from the path field visibly focuses Open directory, Enter opens the supplied directory; three pages and local gradient appear |
| missing-page, missing-image-link, unsupported | Physical reader links show the precise missing path or unsupported-attachment explanation |
| blocked, external | Parent escape and external URL clicks stay in Pagefold and show blocked-path explanations; embedded script is displayed literally |
| unicode, back-home | Relative Unicode link opens Unicode.md; Japanese, accented Latin and Greek render; the back-link returns to Home.md |
| reading, reading-scrolled, reading-return | Relative nested link opens Reading.md with fenced highlighted Rust, table and inline code; wheel reaches sections 19–21 and returns to offset zero |
| narrow-reading, narrow-home, narrow-search, no-results | Narrow rendering, scaled image, wrapped text; café search returns Unicode.md and its physical result opens it; unmatched query visibly returns zero pages while the open reader remains |
| loading, second-directory | A separate synthetic copy is selected; a delayed real response exposes loading and disabled action controls; Change.md opens through search and names a missing embedded image |
| edited-open, added-result | Refresh replaces the open text and removes the old search match; a newly added page is found by text and opened |
| renamed-open, deleted-open | Rename updates search to Renamed.md while the old open path becomes unavailable; deleting that page removes its result and explains the unavailable open page |
| attachment-replaced, attachment-removed | Replacing the isolated gradient with the synthetic empty-state PNG changes the displayed image; removing it displays Image unavailable with its path |
| failed-refresh, refresh-recovered | Temporarily moving the isolated directory makes refresh fail, keeps the previous generation and reader, and visibly warns of staleness; restoring and refreshing recovers |
| empty-directory | Selecting another empty synthetic directory clears the old page and explicitly reports no readable Markdown |

tools/qualification-action.mjs records each physical action's before/after hook
state in actions.jsonl. Canvas controls use current observed rectangles. Reader
link coordinates were taken from opened current images, with the resulting
page/status captured after each click. tools/qualification-refresh.mjs creates
a fresh isolated copy under .runtime-scratch, applies filesystem changes and
uses physical UI controls to refresh. It asserts index, page-list, selected-page,
generation, loading/disabled and failure/recovery postconditions. It deletes only
its isolated synthetic files and the separately generated qualification index.
The regenerated index contains the current edited text without relying on the
old index. The loading probe delays the actual request by 1.8 seconds through
the browser harness; it does not replace the service response or application data.

The final refresh.log is a successful complete run. Two earlier harness failures
are retained in refresh-initial-attempt.log and refresh-generation-attempt.log:
empty insertText did not delete an egui selection, and an unchanged-content rebuild
correctly retained its content-derived generation. The harness now sends Backspace
and only requires a generation change after content changes. No application
repair was needed. An initial inventory invocation supplied the wrong destination
argument, which failed with IsADirectoryError before writing; the supplied source
inventory remains exactly equal to the calibration baseline.

## Exact evidence and reproduction

accepted-inputs.json records the exact Bokkie inputs. check-qualification.py checks
every accepted byte identity except the two owned final documentation files, plus
the retained behavioural postconditions and fixture-history equality. Source
identities are uncommitted file artefacts: the local-only package prohibits commits.
The final independent reviewer receives review-artefacts.json; machine-attributed
review evidence and the actual report are retained separately after registration.

runtime-identities.json records native/WASM/browser binary identities, service and
browser recipes, and unchanged Polyorama commit/tree. Registry versions remain in
Cargo.lock; font provenance/licence remains in assets/fonts. The existing task-local
browser recipe uses a disposable profile and local fontconfig, Chrome 151 with
ANGLE SwiftShader, loopback CDP 9317 and application port 3817. Lantern identity is
in lantern-capabilities.json, doctor.json confirms the browser endpoint, and flow,
final-flow and page JSON retain bounded runtime observations. The observed flow
intervals have no JavaScript exceptions. Initial navigation records an incidental
favicon.ico 404. The final interval retains that console entry and the expected
400 from the deliberately failed refresh; its network attachment has a preceding
collection gap. These are bounded observations, not a clean-runtime claim.
The intentionally failed refresh is separately asserted and visibly documented.
document-state.json records no injected body marker.

Run the README build/service recipe and tools/start-inspection-browser.sh. Use
qualification-action.mjs with viewport, fill, click, point, key, wheel, state or
capture arguments for bounded inspection. qualification-refresh.mjs expects that
runtime and the dedicated .runtime-scratch/qualification-state service directory;
each run creates a new synthetic copy. It also uses this directory's empty.png
as a clearly distinct synthetic replacement image. Screenshots must be opened
after reproduction; script completion alone does not establish presentation.

verification.log retains successful canonical sh tools/verify.sh: six Python
filesystem/transport tests, three Rust path/HTML/font tests, formatting, Clippy,
native/WASM builds, bindgen and fixture comparison. Both qualification scripts
pass node --check. Final checks retain source equality, before/after supplied
fixture equality against calibration and the qualification postconditions.
Supplied source was never intentionally modified, and only synthetic content
was used. Applicable guidance was the supplied personal agreements, Polyorama
AGENTS/UI guides and snapshot review guidance, and the installed Lantern skill
with browser, GPU, Polyorama and functional-action references. No Polyorama edits,
publication, credentials, service installation or future milestone features occur.
