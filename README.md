# Pagefold

A local, read-only Markdown workspace using Polyorama's egui reading typography,
action controls and font configuration. Choose a directory, browse or search its
pages, follow relative links and refresh after editing files in external tools.

## Development and verification

Supported verification environment: Linux with Python 3.11 or newer, rustup,
and the native build libraries below. `rust-toolchain.toml` pins Rust 1.97.1,
Clippy, rustfmt and the WebAssembly target. Cargo fetches Polyorama directly
from Git at revision `fba3db87aaeb4a7a9f1bbfd0ba59fa0aac713b21`;
no sibling checkout is required. `Cargo.lock` retains exact dependency resolution.

From a fresh checkout on Ubuntu 24.04:

```sh
sudo apt-get update
sudo apt-get install -y pkg-config libx11-dev libxi-dev libxrandr-dev libxcursor-dev libxinerama-dev libgl1-mesa-dev libegl1-mesa-dev libwayland-dev libxkbcommon-dev
rustup show
cargo install wasm-bindgen-cli --version 0.2.127 --locked
sh tools/verify.sh
```

The canonical command runs disposable Python filesystem/HTTP tests, Rust tests,
formatting, Clippy, native and WebAssembly builds, and generates `web/pkg` using
the matching bindings generator. It can be invoked from another working directory
and honours `CARGO_HOME` and `CARGO_TARGET_DIR`. Initial toolchain and dependency
installation requires network access. GitHub Actions runs the same command on
pull requests and pushes to `main`, with read-only repository permissions.

Tests use the synthetic pages and image in `tests/fixtures/knowledge` and temporary
copies. They require no original campaign directory, existing cache, private
content or generated evidence. Source preservation is checked by comparing hashes
before and after opening/indexing and changing an isolated copy.

## Local use

Create a **separate, existing** state directory, then start the local service:

```sh
mkdir -p .runtime-scratch/pagefold-state
python3 tools/server.py --state .runtime-scratch/pagefold-state
```

Open **http://127.0.0.1:3817/**. Enter an absolute knowledge-directory path and
choose **Open directory**. For the bundled synthetic example, enter the absolute path to
`tests/fixtures/knowledge` in your checkout.
Choose a page from the list, or type in **Search page text** to filter results.
Results show a bounded, highlighted excerpt of the first Markdown source match.
Path-only matches are labelled separately. Click a result to read it and reveal
the corresponding passage; selecting the same result reveals it again.
**Back** and **Forward** revisit pages opened through
the list, search or internal links. Opening a different page after Back replaces
the forward branch. Reopening the current page adds no duplicate. History stays
in memory for the selected directory and resets when a different directory opens;
the browser address follows page navigation, and browser Back/Forward revisit
the same pages. Search stays in the current session. Reload starts a new session
at the addressed page; earlier in-memory page history does not persist.
Scroll inside the reader; the controls remain visible.
The path field accepts pasted directory paths; there is no operating-system
folder chooser. Tab/Shift-Tab move focus and Enter/Space activate action buttons.

### Page links (browser)

Choose **Copy page link** above the reader. Success is confirmed; if clipboard
access is unavailable or denied, a selected text field provides the complete
link for manual copying with Ctrl+C or ⌘C. The link identifies the current page,
without search text, passage highlight or scroll position.

For example, a workspace `/home/me/Notes` and page `guides/Reading.md` use:

```text
http://127.0.0.1:3817/#workspace=%2Fhome%2Fme%2FNotes&page=guides%2FReading.md
```

An agent can construct a link using known absolute directory and relative page
paths, independently of any browser state:

```python
from urllib.parse import quote
link = ('http://127.0.0.1:3817/#workspace=' + quote('/home/me/Notes', safe='')
        + '&page=' + quote('guides/日本 %?#.md', safe=''))
```

Open the link in a fresh tab, reload it, or reopen it after restarting the same
local service. It reads the named workspace through the normal directory loader.
Missing pages name the intended target; an unavailable workspace reports its
path and allows another directory or a retry. It never substitutes a previously
open workspace. Refresh checks current files and can recover a restored page.

Links expose local path names. They work only for clients with access to the same
service/filesystem, while the service origin and directory/page locations remain
unchanged. Moving or renaming files breaks old links. This is local read-only
access, not publication or a grant of access from another machine. There is no
registry or knowledge-directory metadata. See the exact limits below in the
[content contract](docs/content-contract.md#page-addresses).

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
is reread, or shows an unavailable state if removed. Refresh preserves page
history; revisiting a removed page names its unavailable path, while Back and
Forward still reach the remaining entries. Search stays applied to
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
Search text persists through Back and Forward. Refresh recomputes excerpts and
passage targets; a removed match is explained and its old target discarded.
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
separate from engineering review. The [original milestone plan](docs/active-plan.md) is a historical submission-time
record. `evidence/` and the original inspection scripts retain historical inputs,
paths and tool assumptions; they are not prerequisites for canonical verification
and are not regenerated by it. Current verification is owned by `tools/verify.sh`
and `.github/workflows/ci.yml`.
