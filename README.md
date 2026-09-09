# Pagefold

A local, read-only Markdown workspace using Polyorama's egui reading typography,
action controls and font configuration. Choose a directory, browse or search its
pages, follow relative links and refresh after editing files in external tools.

Requirements: Linux/POSIX Python 3.11+, Rust, the wasm32-unknown-unknown target,
wasm-bindgen CLI 0.2.127 and a sibling Polyorama checkout. Registry dependencies
are pinned in Cargo.lock. The current Polyorama dependency is commit
`fba3db87aaeb4a7a9f1bbfd0ba59fa0aac713b21`, tree
`f9fb7201c5cb32dd41a476de2c9770a8d707b7b5`; it is consumed read-only.

Run `sh tools/verify.sh` to test the filesystem service and Rust content boundary,
check formatting and Clippy, build native/WASM clients and generate web/pkg.
This task's script uses the writable `.runtime-scratch/cargo` Cargo cache.
On another machine use a writable CARGO_HOME and the same commands in the script.
The supplied synthetic fixture path is required by this task's preservation test.

Create a **separate, existing** state directory, then start the local service:

```sh
mkdir -p .runtime-scratch/pagefold-state
python3 tools/server.py --state .runtime-scratch/pagefold-state
```

Open **http://127.0.0.1:3817/**. Enter an absolute knowledge-directory path and
choose **Open directory**. For the authorised synthetic example, enter
`/nvme/development/pagefold-supervision-20260909-a/knowledge`.
Choose a page from the list, or type in **Search page text** to filter results.
Click a result to read it. Scroll inside the reader; the controls remain visible.
The path field accepts pasted directory paths; there is no operating-system
folder chooser. Tab/Shift-Tab move focus and Enter/Space activate action buttons.

The native client uses the same local service: run `cargo run --locked` while
the service runs. If changing the service port, set PAGEFOLD_ENDPOINT to
`http://127.0.0.1:PORT/api/snapshot` for the native client. The browser uses its
own origin. Native compilation is verified; browser interaction is the inspected
runtime route. This is a local single-user service, bound only to 127.0.0.1.
Host, Origin and JSON-content checks prevent arbitrary websites using it as a
filesystem reader. It is not a multi-user server or remote hosting interface.

**Refresh / rebuild** rereads the currently open directory, reloads images and
replaces the complete search index. External additions, edits, renames and
deletions become visible together when the new snapshot arrives. The open page
is reread, or shows an unavailable state if removed. Search stays applied to
the new index. A failed refresh leaves the previous snapshot visible with an
explicit stale warning that persists through navigation and search until a
successful open/refresh. Changes during a scan are not an atomic filesystem
transaction: finish external changes and refresh again for a settled view.
There is no background watcher.

Source files are never opened for writing. The separate state directory contains
only rebuildable `index.json` (UTF-8 JSON of lowercased Markdown page text).
Search is literal, case-insensitive substring matching of page text or paths;
Markdown syntax is included. The client searches the same index returned by the
service. The index is recreated on every open/refresh, including after deletion
or corruption; no irreplaceable derived state is required.
Selection, search and scroll state remain in memory. State locations within,
equal to, or containing the selected directory are rejected before writes.
State directories and their ancestors must not be symlinks.

See [the content contract](docs/content-contract.md) for path and preview limits,
and the [current qualification](evidence/requalification-repair/report.md) for checks,
physical browser journeys, opened desktop/narrow images, refresh failure and
recovery, exact source/build identities and independent review evidence.
Earlier calibration, implementation and qualification directories are historical
evidence from an interrupted outcome; their increment acceptance statements do
not establish current product acceptance. Bokkie's product assessment remains
separate from engineering review. The active state is recorded in
[the plan](docs/active-plan.md).
