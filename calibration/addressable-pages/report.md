# Provisional browser calibration

The question, probe and exit condition are in `docs/addressable-pages-plan.md`.
On the minimal route bridge, actual canvas clicks and browser traversal verified
fresh special-character selection, list address updates, browser Back/app Forward,
reload, missing page/root handling, traversal rejection and root isolation.
The same journey passed again after stopping and restarting the local service.
`results.json`, `restart.json` and `identities.txt` retain the observations and
source/build identities. The opened 1100 × 900 `target/bokkie-tmp/calibration.png`
showed the intended nested page, legible Japanese filename and separated controls
without overlap. Generated fixtures and images are disposable, synthetic only.

The first Lantern flow detected an empty initial observation causing JSON.parse
to throw before the first frame; the bridge now tolerates that initial state.
This is a calibration repair, not a claim that the initial probe passed.
Selected mechanism: browser fragment plus browser entry state for session cursor;
browser traversal reuses application history and application traversal reuses
browser entries. Normal page selection pushes only when the page changes.
Proceed to candidate work and qualify again; these records are not acceptance.

Environment: Chromium headless shell 1234, SwiftShader software rendering,
loopback service 3828/CDP 9328. Existing cached ALSA library resolved Chromium's
missing runtime library. Full Chrome hit a Unix socket path-length limit; the
available headless shell avoids its singleton socket. No native/hardware GPU
qualification is claimed. No dependency or system configuration was changed.
