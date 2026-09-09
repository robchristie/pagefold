# Reader calibration decision

Question, smallest probe, evidence owner and exit condition were recorded in
`docs/active-plan.md` before source implementation. This report concludes only
the `reader_calibration` increment. The application is not product-qualified.

Decision: retain egui_commonmark 0.25.0 with Polyorama Reading typography and
a pane-local selectable ScrollArea as the candidate reader approach. Repair
Japanese font coverage before final qualification. The existing fonts paint
Japanese as three missing-glyph boxes; Latin accents and Greek are legible,
and the smiley is a monochrome glyph. This calibration discovered a requirement
for the next increment rather than concealing it as a successful Unicode test.
Do not copy reusable font-framework repairs into Pagefold; resolve the repair
at its owner or choose a supported application font configuration.

## Evidence and observations

The final Rust source was formatted and rebuilt before `tools/capture.mjs`
captured the final running WASM. All ten PNGs listed below were opened and
visually assessed. The probe has one fixed reading pane, so introduces no dock
tree. It consumes the existing Polyorama style and font initialisation; this
calibration does not qualify an application shell or new framework component.

| Surface | Actual observation | Evidence |
| --- | --- | --- |
| Home, 1100 × 657 | Heading, relative links, supplied colour gradient and literal script text visible; no overlap | `home.png` |
| Reading, 1100 × 657 | Fenced Rust code, two-column table, emphasis and inline code visible | `reading.png` |
| Long-page scroll | Lantern wheel acknowledged; opened image shows sections 21–24, with navigation fixed above the pane | `scroll.json`, `scrolled.png` |
| Unicode | Latin/Greek render; Japanese missing-glyph boxes; monochrome smiley | `unicode.png` |
| Relative links | Home's reading link opens Reading; Unicode's Back home opens Home | `reading.png`, `back-home.png`, physical input in `tools/capture.mjs` |
| Missing page | Current page stays readable; status names Missing.md | `missing.png` |
| Narrow Home, 390 × 700 | Text wraps, gradient scales to pane, no visible clipping | `narrow.png` |
| Unsupported attachment | Status explains binary data and no preview; wraps at narrow width | `unsupported.png` |
| Narrow long page | Paragraphs and inline code wrap; scroll position is retained across page changes | `narrow-reading.png` |
| Narrow code/table | After hovering the canvas and scrolling upwards, code and table fit | `narrow-reading-top.png`, `hover-reader.json`, `scroll-top.json` |

The first upwards wheel after a tab click was acknowledged but did not move
the reader, because egui still had the pointer over chrome. Explicit Lantern
hover into the canvas followed by wheel produced the observed top view. The
reproduction harness now includes that hover. Dispatch acknowledgement alone
was not treated as a successful application action.

Behavioural judgement: the bounded reading/navigation/scrolling tasks work.
Presentation judgement: ordinary and narrow layouts are usable, with a known
Japanese glyph defect. The dense heading/body hierarchy should gain a more
comfortable reading width and stronger page heading in the application.
Design decision: retain the mechanism with these constraints; no further
visual alternative is needed to answer this calibration question.

Embedded HTML remains visible inert text. `browser-state.json` records a ready
document with no injected dataset value and the local URL unchanged.
`flow.json` retains the fresh-navigation console/network observation. The
initial navigation had a favicon 404; the final recapture reports its actual
bounded interval. No claim is made about unobserved earlier/later events.
The ordinary DOM accessibility observation sees the canvas and textbox, not
the internal reader text. Polyorama semantic/text snapshot and native-control
coverage are unavailable in this minimal probe, not a zero-finding audit.
The next increment needs explicit accessibility and text-observation coverage.

## Reproduction and identities

`sh tools/verify.sh` passed: formatting, one focused relative-containment test,
Clippy with warnings denied, native build, WASM build, wasm-bindgen generation
and before/after fixture comparison. Full output is `verification.log`.
Polyorama's own canonical command is `cargo xtask verify`; it was read from
its guidance but not run or claimed for this unmodified read-only dependency.
Pagefold's bounded canonical command is `sh tools/verify.sh`.

`fixtures-before.json` and `fixtures-after.json` identify all five supplied
files by path, bytes and SHA-256 and compare equal. The successful baseline
inventory was recorded before the first successful probe build and before
runtime inspection; an earlier shell heredoc attempt failed due to read-only
temporary storage. No supplied file was written. Build output embeds derived
snapshots under target; the fixture source remains separately stored.

Polyorama dependency: clean commit
`fba3db87aaeb4a7a9f1bbfd0ba59fa0aac713b21`, tree
`f9fb7201c5cb32dd41a476de2c9770a8d707b7b5`.
Applicable guidance read: supplied personal AGENTS instructions; Polyorama
AGENTS.md, docs/ui-guides/README.md, components.md, panes.md and ui-review.md;
installed Lantern skill and its browser-session, GPU/canvas and Polyorama
references. No ancestor/Pagefold AGENTS file was present.

The browser was task-owned Chromium from Playwright chromium-1234, with a
disposable scratch profile, loopback CDP port 9317 and local HTTP port 3817.
Rendering was headless WebGL via ANGLE SwiftShader; this is software-renderer
evidence, not hardware WebGPU qualification. Host Pixi shared libraries plus
alsa-lib 1.2.16.1 extracted into scratch supplied missing runtime libraries;
no global installation or service installation occurred. Browser startup
reported unavailable DBus and host fonts; visible egui text uses embedded fonts.
Lantern build: b41e999113e0563024990a949288f749e2e8ae29, clean, version 0.1.0.
`runtime-identities.txt` identifies the actual build and browser bytes.

Exact uncommitted source/evidence identities are retained by Bokkie file
artefacts and `artefacts.json`. No commit or remote publication was made.
Registered independent review evidence is retained separately after review.

## Constraints for the application increment

Implement directory selection, browsing, safe filesystem path/symlink handling,
general attachment resolution, missing/unreadable states, source preservation,
separate derived state, rebuildable search and consistent refresh. Replace the
fixed fixture map/image substitution; it is explicitly not a general content
boundary. Add font coverage and reader observation contracts. Keep the source
directory authoritative and unchanged, and use isolated synthetic copies for
external-change tests. Final UI qualification and final independent engineering
review remain separate packages and separate from Bokkie's product acceptance.
