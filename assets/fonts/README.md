# Japanese fallback font

Noto Sans CJK JP Regular, unmodified OpenType font from the Noto CJK project.
Retrieved 9 September 2026 from:

- https://raw.githubusercontent.com/notofonts/noto-cjk/main/Sans/OTF/Japanese/NotoSansCJKjp-Regular.otf
- https://raw.githubusercontent.com/notofonts/noto-cjk/main/Sans/LICENSE

The complete upstream SIL Open Font Licence 1.1 is in OFL.txt. The unmodified
font's embedded copyright notice is © 2014-2021 Adobe (http://www.adobe.com/).
Its version is 2.004; the decoded name-table metadata and original font identity
are retained in evidence/requalification/font-metadata.json.
The font is bundled locally under those terms; no installed system font
or font service is required. Exact downloaded bytes are identified by the
submitted file artefacts and implementation manifest. The source URL names a
moving branch; the retained SHA-256 identity pins the actual delivered font.

The original 16,467,736-byte OpenType file is losslessly zlib-compressed and
stored as thirteen ordered binary parts to fit the evidence store's per-file
and aggregate source bounds. Concatenate zlib0 through zlib12 and decompress
with zlib to recreate the unmodified font; the application does this in memory.

Pagefold adds this face at lowest priority in Polyorama's regular/semibold
named families and egui's proportional/monospace families. Existing Latin
typography stays with Polyorama; the fallback supplies Japanese glyphs.
