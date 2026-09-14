# Search passage calibration

Owner: Pagefold. Base: fe4d076c7dc4d522419381c8fec4427ecf935dba.
Polyorama fba3db87aaeb4a7a9f1bbfd0ba59fa0aac713b21 is read-only.

Question: can the existing CommonMark viewer reveal a first source match through
its HTML callback at a parsed top-level block boundary, without splitting the
document, changing source files or refactoring the renderer?

Smallest probe: a feature-gated synthetic browser surface using the application's
fonts, preparation function and actual CommonMark viewer. One fixture covers
headings, paragraphs, lists, links, code, tables, repeated and Unicode matches,
invisible syntax and destinations, and a distant paragraph. Executable checks
cover source byte ranges, lowercase expansion and block placement. A separate
path-only case has no invented body target.

Exit: retain only if tests prove first-source-match ranges and safe marker
placement, and actual browser observations plus opened pixels prove the distant
passage is marked and revealed. Otherwise reject or narrow the mechanism with
explicit observations. This package does not qualify result/history/refresh UI.

Evidence is owned here and in task-local target/calibration output. Historical
evidence and docs/active-plan.md stay unchanged. A final report records exact
inputs/builds, supported behaviour, remaining work and the retain/reject decision.
