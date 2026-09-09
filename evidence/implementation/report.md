# Filesystem-backed implementation evidence

This increment implements workspace_reader, attachments_and_paths,
source_preservation, search_and_refresh and local_delivery. It follows Bokkie's
accepted reader calibration. Comprehensive ui_qualification and Bokkie's product
acceptance are separate subsequent decisions; neither is claimed here.

The fixed embedded fixture map has been removed. Native and WASM clients use
the same loopback Python service, which owns directory-descriptor traversal and
read-only source snapshots. The client owns relative Markdown navigation and
registered byte-image rendering. The service persists a fully rebuildable text
index outside source; the client searches that returned index. README.md and
docs/content-contract.md define use, containment, state, refresh and limits.

## Actual verification

The final canonical command, `sh tools/verify.sh`, completed successfully.
verification.log retains the complete final output: six Python filesystem and
transport tests, three Rust path/HTML/image/font tests, formatting, Clippy with
warnings denied, native build, WASM build, wasm-bindgen and fixture comparison.
The independent reviewer also ran the Rust and Python tests.

Tests cover external page addition/edit/rename/deletion, image replacement and
deletion, index deletion/regeneration, supplied-fixture preservation and isolated
copy edits. They exercise source/state overlap before writes, source/state
symlinks, source special files, index hardlinks/symlinks, invalid UTF-8,
oversized content and cross-origin/Host/content-type request rejection.
Rust tests exercise encoded traversal/unsafe schemes, reference and inline
image rewriting, code preservation, inert HTML and Japanese glyph availability.

fixtures-after.json compares equal to the retained calibration
fixtures-before.json, including every supplied file's size and content hash.
The supplied directory was only read. Tests create disposable synthetic content
under .runtime-scratch, separately from application source. No private content
was used. Source revision is the exact uncommitted file set in
review-artefacts.json and Bokkie's measured file artefacts; no commit was made.
The final Git diff whitespace check passed. The pre-existing .gitignore entries
were preserved; only a Python cache exclusion was added by this increment.

## Focused runtime smoke

`node tools/implementation-smoke.mjs` completed successfully against the actual
filesystem service. Its physical canvas input targets current rectangles from
the read-only Pagefold observation hook. It selects the supplied directory,
opens all three pages, searches café, opens the result and refreshes it.
smoke.log records completion; step-state JSON records postconditions.
Lantern collected the navigation interval, screenshots and narrow body layout.
All seven final PNGs were opened and assessed.

| Image | Observed result |
| --- | --- |
| empty.png | Directory prompt and disabled actions visible; intentional empty reader |
| home.png | Three source pages browsable; local gradient renders; embedded script appears as literal text |
| unicode.png | Japanese 日本語, Latin accents and Greek are legible; smiley has monochrome fallback |
| reading.png | Fenced Rust code, table, inline code and long-page content render in the width-constrained reader |
| search.png | café returns Unicode.md, and the result opens the corresponding page |
| refresh.png | Refresh finishes, preserves the query and current page, and shows the refreshed status |
| narrow.png | At 390 × 760 pixels, controls and reading text fit; path input scrolls horizontally |

The ordinary PNGs are 1100 × 800 pixels. The reading width is capped at 860
egui points; source text stays selectable with pane-local scrolling. The page
browser has a bounded height, fixed row sizing and virtualised rows. Chrome is
deliberately dense; comprehensive design and keyboard journeys remain for the
separate UI qualification package.

Behavioural smoke: passed. Presentation smoke: useful, nonblank content and
legible normal/narrow layouts; no observed overlap. Design decision: retain this
candidate for comprehensive qualification, with the reader and stable controls
using the calibrated Polyorama style. This is not a comprehensive visual audit.

Earlier development smoke attempts exposed a disposable-browser font setup
problem and clipped page-list rows. The browser was restarted with task-local
fontconfig; fixed panel minimums and row sizes now allow the physical journey.
The final retained captures describe the repaired source, not those attempts.
An earlier navigation reported a favicon 404; the final retained navigation
interval has no HTTP errors or JavaScript exceptions. No content execution was
observed. document-state.json records no injected dataset
value. This bounded observation does not claim that all runtime intervals are
clean.

## Runtime and dependency identity

runtime-identities.txt records the final native and browser build hashes plus
the concatenated upstream font hash. Cargo.lock records registry dependency
identities. Polyorama remained clean at commit
`fba3db87aaeb4a7a9f1bbfd0ba59fa0aac713b21`, tree
`f9fb7201c5cb32dd41a476de2c9770a8d707b7b5`. Its canonical owner check is
`cargo xtask verify`; it was read, not rerun or claimed for an unchanged sibling.
Applicable guidance: supplied personal AGENTS instructions; Polyorama AGENTS
and UI guides (components, panes, interactions, accessibility and review);
installed Lantern skill with browser, GPU/canvas and Polyorama references.
The landing skill excludes local-only work; this saved package prohibits commits
and publication, so engineering review is retained locally.

Browser: task-owned Playwright chromium-1234 executable at
/home/rob/.cache/ms-playwright/chromium-1234/chrome-linux64/chrome, loopback CDP
9317, headless ANGLE SwiftShader/WebGL. Lantern is clean build
`b41e999113e0563024990a949288f749e2e8ae29`, version 0.1.0.
This is software-renderer browser evidence, not hardware WebGPU or native
interactive qualification. Browser startup diagnostics include unavailable
DBus/NSS host integrations; no credentials or host configuration were changed.

For reproduction in this sandbox, run the service as described in README.md.
Use a disposable Chromium profile, the existing task ALSA libraries and a local
fontconfig pointing at Polyorama's bundled Source Sans fonts. The exact launch
recipe is in tools/start-inspection-browser.sh. Start it, then run the smoke
script. Browser/service processes are task-owned and may be stopped at package
submission; the next package should launch its own runtime.

The read-only observation hook includes current control geometry, page/root,
generation, status, query, measured action text and coverage exclusions.
Document-reader and native text-input layouts are not measured. This custom
hook is not the Polyorama gallery adapter or browser accessibility tree.
Browser AccessKit and native OS assistive-technology workflows are unqualified.
No claim of complete canvas accessibility or zero-finding text coverage is made.

Noto Sans CJK JP Regular is bundled unmodified under its retained SIL OFL licence.
Its ordered zlib parts decompress to the 16,467,736-byte upstream font with SHA-256
`68a3fc98800b2a27b371f2fb79991daf3633bd89309d4ffaa6946fd587f375b5`.
Lossless compression and splitting fit Bokkie's source bounds; no glyph subset
or reusable framework repair is introduced.

Independent review is commissioned on the exact manifest. The registered
machine-attributed report and reviewer identity will be retained in
review-evidence.json and review-report.json, separately from this candidate
description. The supervisor can discover the registered review in durable state.
