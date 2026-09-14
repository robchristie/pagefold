# Content and path contract

Pages are ordinary UTF-8 .md or .markdown files, recursively browsed in path
order. No front matter, generated page identity or metadata is required.
CommonMark includes fenced code, tables and strikethrough. Embedded HTML is
escaped into literal text before rendering; no HTML renderer or script engine
is used for content.

Links resolve against the containing page. Dot segments are normalised; parent
traversal is allowed only while it remains within the selected directory.
Percent-encoded UTF-8 filenames are decoded once. Absolute paths, backslashes,
control characters, URL schemes (including http, file, data and javascript)
and query strings are blocked. Encoded `%3F` denotes a literal filename question
mark; a raw `?` starts an unsupported query. Heading fragments open the containing page with
an explicit notice that heading scrolling is unsupported. Names containing
colons, backslashes or control characters are unsupported and reported.
Case follows the source filesystem.

The selected root and every source/state directory component must be a real
directory. **All symlinks are excluded**, including links to targets within
the selected directory; blocked links appear in directory warnings. Traversal
and reads use directory descriptors and O_NOFOLLOW. Only regular files are read;
devices, sockets and FIFOs are not opened as content. Browsing, indexing,
navigation and images all use the same contained snapshot, so there is no
separate attachment endpoint that can escape the boundary.

Relative PNG, JPEG, GIF and WebP images are previewed from registered in-memory
bytes, including images referenced from nested pages and reference-style
Markdown images. Images embedded as HTML are inert HTML text. Remote images
are blocked. SVG and other attachments receive an unsupported-preview message;
open them using an appropriate external application. Missing links and missing
images are named explicitly. Clicking a supported image attachment shows it in
the reader; choose a page to return to text. Corrupt/unreadable content produces
a warning or renderer error instead of an empty success state.

The milestone targets modest local collections: at most 10,000 directory
entries, 64 nested directory levels, 4 MiB per previewed file and 32 MiB total
preview content per snapshot. Images must have dimensions at most 4096 × 4096.
Oversized individual files are reported unavailable; a workspace-wide limit
fails the refresh and keeps the previous snapshot with a stale warning.
Browsing widgets are virtualised; the bounded index and snapshot are held in
memory. The index includes raw Markdown text for predictable literal search.

The service atomically replaces a fresh exclusive index file in the separate
state directory. Existing index symlinks or hardlinks cannot cause source
content to be overwritten. The service does not create a requested state
directory; the caller prepares it separately. Root and state containment checks
precede all generated-state writes. Source files and attachments remain the
authority; deleting the generated index loses no knowledge.

The UI is a single fixed reading surface with a browser above it; there is no
second docking tree. It uses Polyorama Reading typography and action recipes.
The Noto fallback extends the supported application font API without changing
Polyorama. Source editing, semantic search, ingestion, synchronisation, graph
visualisation and authoring tools are outside this milestone.

## Page addresses

Browser links use `/#workspace=<absolute-path>&page=<relative-page>` on the local
service origin. Both values use UTF-8 percent encoding, decoded exactly once by
the Pagefold browser address parser. `+` is literal, not a form-encoded space.
Encode `%`, `&`, `=`, `?`, `#`, spaces and non-ASCII characters with ordinary
`encodeURIComponent` or Python `quote(value, safe='')`; slashes may be encoded.
The fragment is not sent as an HTTP path. The page must end in `.md` or
`.markdown` (case-insensitive) and is looked up in the contained snapshot.

The fragment is at most 16384 characters; decoded fields are each at most 4096
JavaScript UTF-16 code units. Exactly one `workspace` and one `page` are required.
Malformed percent encoding/UTF-8, extra/repeated fields, empty components, dot
segments, absolute pages, colons, backslashes and control characters are rejected.
Use an absolute workspace without a trailing slash. There is no second decode,
root-escape normalisation or serving endpoint. Existing directory, symlink,
regular-file and snapshot limits apply to all addressed pages.

Page addresses are separate from relative Markdown links: the latter still
resolve against their containing page and reject external URL schemes.
Browser page-list, search and internal page navigation update the address;
passage reveal and repeated current-page selection do not create entries.
Browser and application traversal preserve the current search query. Opening a
different workspace resets application page history, as before. Browser entries
can still return to the named workspace. A reload/restart rereads the addressed
page and starts new in-memory search/history state. The browser copy controls
are ordinary DOM controls; native compilation does not qualify this browser UI.

The browser publishes bounded current control rectangles and state through
`window.pagefoldObservation()`. This is an observation, not a mutation API.
It includes measured Polyorama action text and explicit native/reader exclusions.
It is not the framework gallery hook or an AccessKit tree. Eframe's browser
AccessKit adapter is unavailable; no browser screen-reader support is claimed.
Text-input and document-reader internal label geometry are unmeasured, and no
native operating-system assistive-technology workflow has been qualified here.

Search excerpts show at most 24 source characters on each side and 48 matched
characters, with ellipses for omitted text. Whitespace is displayed as spaces.
Matching retains whole-string lowercase semantics, including Unicode expansion;
a partial lowercase expansion highlights the original character.

The reader marks the first match's enclosing top-level passage rather than
highlighting individual rendered words. Lists, tables and code blocks are marked
as a unit. Markdown syntax, link destinations, image alt text and transformed
text may not appear as written; the interface explains this and retains the
highlighted source excerpt. Definitions without a rendered block and blocks over
2,000 source characters use the source excerpt alone, without an invented target.
Reference resolution and escaped user HTML still use the complete document.
