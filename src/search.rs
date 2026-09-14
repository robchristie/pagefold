use pulldown_cmark::{Event, Options, Parser};
use std::{collections::BTreeMap, ops::Range};
pub const MARKER: &str = "<div data-pagefold-search=marker></div>";

#[derive(Clone, Debug, serde::Serialize)]
pub struct Excerpt {
    pub before: String,
    pub matched: String,
    pub after: String,
}

pub fn excerpt(source: &str, range: Range<usize>) -> Excerpt {
    let before: String = source[..range.start]
        .chars()
        .rev()
        .take(24)
        .collect::<Vec<_>>()
        .into_iter()
        .rev()
        .collect();
    let matched: String = source[range.clone()].chars().take(48).collect();
    let after: String = source[range.end..].chars().take(24).collect();
    let clean = |s: &str| {
        s.chars()
            .map(|c| if c.is_whitespace() { ' ' } else { c })
            .collect::<String>()
    };
    Excerpt {
        before: format!(
            "{}{}",
            if before.len() < range.start {
                "…"
            } else {
                ""
            },
            clean(&before)
        ),
        matched: format!(
            "{}{}",
            clean(&matched),
            if matched.len() < range.len() {
                "…"
            } else {
                ""
            }
        ),
        after: format!(
            "{}{}",
            clean(&after),
            if after.len() < source.len() - range.end {
                "…"
            } else {
                ""
            }
        ),
    }
}

pub fn job(excerpt: &Excerpt, ui: &egui::Ui) -> egui::text::LayoutJob {
    let mut job = egui::text::LayoutJob::default();
    for (text, highlight) in [
        (&excerpt.before, false),
        (&excerpt.matched, true),
        (&excerpt.after, false),
    ] {
        job.append(
            text,
            0.0,
            egui::TextFormat {
                font_id: egui::TextStyle::Body.resolve(ui.style()),
                color: if highlight {
                    egui::Color32::WHITE
                } else {
                    ui.visuals().text_color()
                },
                background: if highlight {
                    egui::Color32::from_rgb(100, 70, 0)
                } else {
                    egui::Color32::TRANSPARENT
                },
                ..Default::default()
            },
        );
    }
    job
}
// Match the renderer's actual parser options, including its additional structures.
fn options() -> Options {
    Options::ENABLE_TABLES
        | Options::ENABLE_TASKLISTS
        | Options::ENABLE_STRIKETHROUGH
        | Options::ENABLE_FOOTNOTES
        | Options::ENABLE_DEFINITION_LIST
}

/// Match whole-string lowercase semantics, then translate folded bytes to source.
/// Lowercase expansion changes lengths; contextual sigma changes values, not length.
pub fn first_match(source: &str, query: &str) -> Option<Range<usize>> {
    if query.is_empty() {
        return None;
    }
    let folded = source.to_lowercase();
    let query = query.to_lowercase();
    let start = folded.find(&query)?;
    let end = start + query.len();
    let mut offset = 0;
    let mut original_start = None;
    let mut original_end = 0;
    for (byte, ch) in source.char_indices() {
        let next = offset + ch.to_lowercase().map(char::len_utf8).sum::<usize>();
        if offset < end && next > start {
            original_start.get_or_insert(byte);
            original_end = byte + ch.len_utf8();
        }
        offset = next;
    }
    Some(original_start?..original_end)
}

#[derive(Clone, Debug, serde::Serialize)]
pub struct Target {
    pub matched: Range<usize>,
    pub block: Option<Range<usize>>,
    pub visible: bool,
}

pub fn target(source: &str, query: &str) -> Option<Target> {
    let matched = first_match(source, query)?;
    let mut depth = 0;
    let mut block = None;
    let mut visible = false;
    let mut image_depth = 0;
    for (event, range) in Parser::new_ext(source, options()).into_offset_iter() {
        match event {
            Event::Start(tag) => {
                if matches!(tag, pulldown_cmark::Tag::Image { .. }) {
                    image_depth += 1;
                }
                if depth == 0 && range.start <= matched.start && range.end >= matched.end {
                    block = Some(range.clone());
                }
                depth += 1;
            }
            Event::End(tag) => {
                depth -= 1;
                if matches!(tag, pulldown_cmark::TagEnd::Image) {
                    image_depth -= 1;
                }
            }
            Event::Text(text) | Event::Code(text) => {
                // Conservative: transformed entities/escapes and cross-event matches
                // use the honest source fallback, never a later rendered occurrence.
                if let Some(start) = source[range.clone()]
                    .find(text.as_ref())
                    .filter(|_| image_depth == 0)
                {
                    let visible_range = range.start + start..range.start + start + text.len();
                    visible |=
                        visible_range.start <= matched.start && visible_range.end >= matched.end;
                }
            }
            _ => {}
        }
    }
    // Large compound blocks use the source fallback rather than an imprecise reveal.
    if block
        .as_ref()
        .is_some_and(|r| source[r.clone()].chars().count() > 2000)
    {
        block = None;
    }
    Some(Target {
        matched,
        block,
        visible,
    })
}

pub fn prepared(
    page: &str,
    source: &str,
    images: &BTreeMap<String, String>,
    target: &Target,
) -> String {
    let Some(block) = &target.block else {
        return crate::prepare(page, source, images);
    };
    // A unique plain placeholder survives the normal whole-document preparation.
    // Only afterwards is it replaced by trusted generated HTML. User HTML remains
    // escaped and reference definitions remain in the full document.
    let mut placeholder = "PAGEFOLD_SEARCH_SENTINEL".to_owned();
    while source.contains(&placeholder) {
        placeholder.push('_');
    }
    let mut marked = source.to_owned();
    marked.insert_str(block.start, &format!("\n\n{placeholder}\n\n"));
    crate::prepare(page, &marked, images).replacen(&placeholder, MARKER, 1)
}

#[cfg(test)]
mod tests {
    use super::*;
    const FIXTURE: &str = include_str!("../calibration/search-passages/fixture.md");
    #[test]
    fn original_ranges_and_first_occurrences() {
        for (source, query, expected) in [
            ("İ after", "after", "after"),
            ("İ", "i", "İ"),
            ("İ", "\u{307}", "İ"),
            ("ΟΣ", "ος", "ΟΣ"),
            ("café 日本語", "日本", "日本"),
            ("Echo Echo", "ECHO", "Echo"),
        ] {
            let range = first_match(source, query).unwrap();
            assert_eq!(&source[range.clone()], expected);
            assert_eq!(range.start, source.find(expected).unwrap());
        }
        assert!(first_match("abc", "").is_none());
    }
    #[test]
    fn representative_blocks_and_invisible_matches() {
        for query in [
            "Heading Beacon",
            "Alpha",
            "List Beacon",
            "Linklabel",
            "Reference",
            "CodeBeacon",
            "TableBeacon",
            "İSTANBUL",
            "日本語",
            "Echo",
            "DistantBeacon",
        ] {
            let t = target(FIXTURE, query).unwrap();
            assert!(t.visible, "{query}");
            assert!(t.block.is_some(), "{query}");
            let marked = prepared("Fixture.md", FIXTURE, &BTreeMap::new(), &t);
            let callbacks: Vec<_> = Parser::new_ext(&marked, options())
                .filter_map(|e| {
                    if let Event::Html(h) = e {
                        Some(h.into_string())
                    } else {
                        None
                    }
                })
                .collect();
            assert_eq!(callbacks.len(), 1, "{query}: {callbacks:?}");
            assert_eq!(callbacks[0].trim(), MARKER);
        }
        for query in ["**", "invisible-destination", "```", "ReferenceTarget"] {
            assert!(!target(FIXTURE, query).unwrap().visible, "{query}");
        }
        assert!(target(FIXTURE, "ReferenceTarget").unwrap().block.is_none());
        assert!(target(FIXTURE, "PathOnly").is_none());
        assert_eq!(
            target(FIXTURE, "Echo").unwrap().matched.start,
            FIXTURE.find("Echo").unwrap()
        );
        assert!(target(FIXTURE, "DistantBeacon").unwrap().matched.start > 6000);
    }
    #[test]
    fn marker_preserves_reference_links_and_escapes_user_html() {
        let source = "<script>bad</script>\n\nA [reference][r].\n\n[r]: local.md\n";
        let marked = prepared(
            "Fixture.md",
            source,
            &BTreeMap::new(),
            &target(source, "reference").unwrap(),
        );
        assert!(!marked.contains("<script>"));
        assert!(Parser::new_ext(&marked, options()).any(|e| matches!(e, Event::Start(pulldown_cmark::Tag::Link {dest_url, ..}) if dest_url.as_ref() == "local.md")));
    }
}

#[cfg(test)]
mod limits {
    use super::*;
    #[test]
    fn rewritten_images_and_large_blocks_are_honest() {
        assert!(
            !target("![AltNeedle](local.png)", "AltNeedle")
                .unwrap()
                .visible
        );
        assert!(
            !target("![alt][r]\n\n[r]: local.png", "alt")
                .unwrap()
                .visible
        );
        assert!(!target("<b>Needle</b>", "<b>").unwrap().visible);
        assert!(
            target(&format!("- {} Needle", "padding ".repeat(400)), "Needle")
                .unwrap()
                .block
                .is_none()
        );
        let source = format!("{}İ{}", "a".repeat(200), "b".repeat(200));
        let excerpt = excerpt(&source, first_match(&source, "i").unwrap());
        assert_eq!(excerpt.matched, "İ");
        assert!(excerpt.before.chars().count() <= 46);
        assert!(excerpt.after.chars().count() <= 46);
    }
}
