# Addressable pages

Owner: Pagefold. Phase: candidate prepared after provisional browser qualification;
see `calibration/addressable-pages/report.md`. Exact committed-candidate browser
qualification and independent review are the next gates, recorded on the PR.

Use `/#workspace=<encoded absolute path>&page=<encoded relative Markdown path>`.
An absolute path is inspectable and constructible from filesystem knowledge,
survives service restart without stored identity, and needs no registry. It
exposes local directory names and is portable only between clients of the same
service with unchanged paths. A durable opaque identifier could hide names but
would require a separate persistent registry and agent discovery; it would not
make the filesystem available on another machine. That complexity is unnecessary
for this outcome. Knowledge directories receive no metadata.

Calibration question: can ordinary browser fragment/history operations drive the
existing validated directory/page operations while keeping session page history
and search intact? Smallest probe: two disposable synthetic roots, a nested page
named with spaces, Unicode, `%`, `?` and `#`, and a second ordinary page. Exercise
fresh navigation, reload/restart, browser/app history, unavailable and malformed
targets and root isolation before broad control work. Pagefold owns the probe
and observations under `calibration/addressable-pages`; generated runtime files
live under ignored task scratch. Exit when the browser mechanism has observed
round-trips and an explicit history policy; change approach if it needs framework
mutation or bypasses snapshot validation. These observations are provisional.

Then implement copy feedback/fallback and full navigation/error semantics, run
canonical verification and all candidate browser journeys, obtain independent
review of the exact commit and prepare one PR. Bokkie owns assessment, merge,
post-merge CI, cleanup and final acceptance. Preserve historical plans/evidence.

The browser probe is `tools/addressable-pages-browser.mjs`. It accepts a new,
disposable scratch path, starts/stops its own loopback service, and uses an
already owned browser through `PAGEFOLD_CDP`. `PAGEFOLD_PLAYWRIGHT` can name an
installed Playwright module; browser inspection is additional to the portable
canonical `sh tools/verify.sh`. No historical fixture directory is required.
It records exact source/build and synthetic inventories, intentional fixture
edits, browser state, Lantern flow/layout and desktop/narrow PNGs. Open those
PNGs before judging visual usability. Clipboard denial uses a real browser
Permissions Policy applied by the harness; it does not mock application APIs.
