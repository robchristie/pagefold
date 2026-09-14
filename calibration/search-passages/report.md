# Source-to-passage calibration decision

Retain the Pagefold-only generated block marker mechanism for implementation.
This is a provisional calibration hand-off, not product qualification or a PR.
The default application remains unchanged; `passage-calibration` replaces only
the browser entry surface with the synthetic probe.

The question and exit condition are in [plan.md](plan.md). The executable probe
is [probe.rs](probe.rs), its complete synthetic input is [fixture.md](fixture.md),
and [run-browser.py](run-browser.py) owns the physical browser checks.

## Mechanism and inspected owner APIs

Pagefold currently calls `prepare` on the complete source, then
`CommonMarkViewer::show` within an egui `ScrollArea`. Polyorama supplies Reading
typography, fonts and actions, not the Markdown parser or source-range geometry.
The prepared dependency remains fba3db87aaeb4a7a9f1bbfd0ba59fa0aac713b21.
Its `polyorama-ui-egui/src/lib.rs` exports the typography/text/action APIs; no
Polyorama source change is needed.

Inspected egui_commonmark 0.25.0 `src/lib.rs` and `src/parsers/pulldown.rs`, and
egui_commonmark_backend 0.25.0 `src/lib.rs` and `src/pulldown.rs`. The public
`show` result covers the whole document. There is no exact text range highlight
API. `render_html_fn` receives a UI at the parsed HTML block position and can
create a labelled marker whose response supports `scroll_to_me`. The callback
requires an owned `'static` closure environment; an Rc/RefCell records geometry.
Use an ordinary `div` marker: an initial custom hyphenated tag produced no HTML
block callback and was rejected by the executable probe.

Find the first substring in **whole-string lowercased source**, then map its
folded byte interval to original Unicode scalar boundaries using each scalar's
lowercase expansion length. Do not slice original source at folded offsets.
This preserves contextual final sigma values while handling `İ` expansion,
combining-dot-only queries, accented text and Japanese. A partial expansion hit
maps to the full original scalar. This does not add case folding, normalisation,
fuzzy matching or ranking. The product must still use existing index/path match
semantics and derive locations from the current original page body.

Parse offsets with the renderer's actual options (tables, task lists,
strikethrough, footnotes and definition lists). Select the first source match's
enclosing top-level block. Insert a unique plain placeholder at its boundary,
prepare the **whole document**, then replace only that placeholder with trusted
generated HTML. User HTML remains escaped; references remain in the document.
The callback marks the corresponding block and scrolls its start into view.
There is no DOM search, offset guessing from lowercase bytes, or search for a
later matching rendered occurrence.

## Observed result and supported limits

Ten Rust tests passed, including three focused calibration tests. The browser
runner exercised 15 queries at each of 1100×900 and 390×900: heading, paragraph,
list, inline and reference link labels, fenced code, table, Unicode, repeated
text, destination, reference definition, Markdown delimiters, path-only and a
distant paragraph. The first distant match is original bytes 6807..6820.
Desktop reveal moved 2113 points; narrow reveal moved over 3300 points. Both
opened images show the first corresponding paragraph directly below the marker.

Opened PNG judgement: code/table markers sit directly above the intact code
block/table; Unicode glyphs and the matched source spelling are readable;
the destination marker names the absence of an exact rendered-text mapping;
the reference-definition fallback shows the original source match without an
invented target; the path-only state labels its path. Desktop and narrow distant
images have readable controls, marker and paragraph, with intentional reader
clipping/scrolling and no horizontal overlap. Canvas internals are not DOM
layout coverage: Lantern body layout is supplemental, and the pixels and
Pagefold observation hook establish the probe result.

This is a **block marker**, not exact in-paragraph highlighting. Lists are marked
as a whole list, tables as a whole table, and fenced code as a whole code block.
Very large compound blocks may require additional manual scrolling within the
marked passage; do not promise exact word positioning. The product should keep
the bounded highlighted source excerpt visible as a useful fallback. Pure
reference definitions have no rendered block. Syntax, destinations and matches
crossing transformed/entity/style event boundaries must be described as lacking
an exact rendered-text mapping, not as a successfully highlighted word. The
probe conservatively classifies raw text/code event spans; image alt text and
other preparation rewrites need explicit conservative handling during product
integration. No support for arbitrary image-alt-to-pixel location is implied.

Resizing alone retains the reader's previous scroll offset. Fresh narrow query
selection was tested; automatic target tracking on resize was not qualified.
The future product must keep reveal requests separate from page-history writes
and clear/recompute source targets when the snapshot changes. Query persistence,
compact highlighted result excerpts, refresh/removal/stale-state handling and
the complete read-only application journey remain the implementation package.

## Reproduction and evidence

Canonical check: `sh tools/verify.sh` with prepared Rust 1.97.1, locked Cargo
dependencies and wasm-bindgen 0.2.127. Probe checks/build:

```sh
cargo test --locked --features passage-calibration
cargo build --locked --target wasm32-unknown-unknown --lib --features passage-calibration
wasm-bindgen --target web --out-dir web/pkg target/bokkie-dependencies/target/wasm32-unknown-unknown/debug/pagefold_probe.wasm
python3 tools/server.py --state target/calibration/state --port 3827
python3 calibration/search-passages/run-browser.py
```

The disposable state directory must already exist. The probe fixture is compiled
into the optional browser surface; it does not open private knowledge or call the
snapshot API. The runner compares fixture hashes before/after all interactions.
This establishes probe read-only behaviour; the full app's filesystem journey
remains a subsequent requirement.

Browser: local Chromium 151.0.7922.34, headless SwiftShader, CDP 9317. Harness
setup sets viewport and focuses measured canvas input geometry using Playwright;
query text is physically dispatched by Lantern into eframe's input. State reads
only call `window.pagefoldObservation()`. Lantern build is
b41e999113e0563024990a949288f749e2e8ae29. No hardware GPU or screen-reader claim.

Use process-local `TMPDIR=target` (relative), `FONTCONFIG_FILE` pointing to the
task's `.runtime-scratch/fonts.conf`, and `LD_LIBRARY_PATH` including Pixi's lib
directory and `.runtime-scratch/alsa/usr/lib/x86_64-linux-gnu`. The fontconfig
file names the prepared pinned Polyorama fonts and a task-local cache. Chromium
flags: `--headless --no-sandbox --disable-dev-shm-usage
--disable-background-networking --disable-breakpad --disable-crash-reporter
--use-gl=angle --use-angle=swiftshader --enable-unsafe-swiftshader
--remote-debugging-port=9317 --user-data-dir=<workspace>/.runtime-scratch/calibration-chrome`.
No global configuration was changed. The missing runtime library was extracted
locally from Debian `libasound2t64_1.2.14-1+deb13u1_amd64.deb` at
https://deb.debian.org/debian/pool/main/a/alsa-lib/ (package copyright retained
under the extracted `usr/share/doc`; no dependency is added to Pagefold).

Retained failed setup observations: absent ALSA library; an overlong absolute
Chromium socket path; missing browser fonts causing beforeinput without committed
input; and an ambiguous restored tab. These were resolved with local library
extraction, relative ignored temp directory, explicit fontconfig and removal of
the disposable blank tab. Initial navigation reported only a favicon 404, not a
WASM/app exception. Early browser commands had unavailable source captures because
Chromium put a socket symlink in the source root. Those observations are
provisional; final checks run with browser temporary files under ignored target.

Exact candidate, build, fixture, browser output and PNG identities are retained
in `target/calibration/manifest.json`; final command logs and
`browser-observations.json` are alongside it. Bokkie retains the exact submitted
Git artefact and file identities, source-bound command validations and separately
registered independent review. Historical evidence and active-plan records were
not changed. The supervisor owns the decision to proceed to implementation.
