//! Provisional source-to-block calibration; not the product search interface.
use egui_commonmark::{CommonMarkCache, CommonMarkViewer};
use pulldown_cmark::{Event, Options, Parser};
use std::{cell::RefCell, collections::BTreeMap, ops::Range};

const FIXTURE: &str = include_str!("fixture.md");
const MARKER: &str = "<div data-pagefold-calibration=marker></div>";

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
fn first_match(source: &str, query: &str) -> Option<Range<usize>> {
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

#[derive(Debug)]
struct Target {
    matched: Range<usize>,
    block: Option<Range<usize>>,
    visible: bool,
}

fn target(source: &str, query: &str) -> Option<Target> {
    let matched = first_match(source, query)?;
    let mut depth = 0;
    let mut block = None;
    let mut visible = false;
    for (event, range) in Parser::new_ext(source, options()).into_offset_iter() {
        match event {
            Event::Start(_) => {
                if depth == 0 && range.start <= matched.start && range.end >= matched.end {
                    block = Some(range.clone());
                }
                depth += 1;
            }
            Event::End(_) => depth -= 1,
            Event::Text(text) | Event::Code(text) => {
                // Conservative: transformed entities/escapes and cross-event matches
                // use the honest source fallback, never a later rendered occurrence.
                if let Some(start) = source[range.clone()].find(text.as_ref()) {
                    let visible_range = range.start + start..range.start + start + text.len();
                    visible |=
                        visible_range.start <= matched.start && visible_range.end >= matched.end;
                }
            }
            _ => {}
        }
    }
    Some(Target {
        matched,
        block,
        visible,
    })
}

fn prepared(source: &str, target: &Target) -> String {
    let Some(block) = &target.block else {
        return crate::prepare("Fixture.md", source, &BTreeMap::new());
    };
    // A unique plain placeholder survives the normal whole-document preparation.
    // Only afterwards is it replaced by trusted generated HTML. User HTML remains
    // escaped and reference definitions remain in the full document.
    let mut placeholder = "PAGEFOLD_CALIBRATION_SENTINEL".to_owned();
    while source.contains(&placeholder) {
        placeholder.push('_');
    }
    let mut marked = source.to_owned();
    marked.insert_str(block.start, &format!("\n\n{placeholder}\n\n"));
    crate::prepare("Fixture.md", &marked, &BTreeMap::new()).replacen(&placeholder, MARKER, 1)
}

pub struct Probe {
    query: String,
    previous: String,
    cache: CommonMarkCache,
}
impl Probe {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        crate::install_fonts(&cc.egui_ctx);
        Self {
            query: String::new(),
            previous: String::new(),
            cache: CommonMarkCache::default(),
        }
    }
}
impl eframe::App for Probe {
    fn ui(&mut self, root: &mut egui::Ui, _: &mut eframe::Frame) {
        let mut nodes = Vec::new();
        egui::Panel::top("calibration").show(root, |ui| {
            ui.heading("Passage calibration — synthetic fixture");
            ui.label("Provisional block marker; exact text highlighting is unavailable.");
            let response = ui.text_edit_singleline(&mut self.query);
            crate::observe(&mut nodes, "search", "Calibration query", &response);
        });
        let reveal = self.query != self.previous;
        self.previous = self.query.clone();
        let target = target(FIXTURE, &self.query);
        let source = target.as_ref().map_or_else(
            || crate::prepare("Fixture.md", FIXTURE, &BTreeMap::new()),
            |t| prepared(FIXTURE, t),
        );
        let marker_rect = std::rc::Rc::new(RefCell::new(None));
        let callback_rect = marker_rect.clone();
        let visible = target.as_ref().is_some_and(|t| t.visible);
        let callback = move |ui: &mut egui::Ui, html: &str| {
            if html.trim() == MARKER {
                let label = if visible {
                    "Matching passage below"
                } else {
                    "No exact rendered-text mapping; corresponding block below"
                };
                let response = ui.label(
                    egui::RichText::new(label)
                        .strong()
                        .background_color(egui::Color32::from_rgb(90, 65, 0))
                        .color(egui::Color32::WHITE),
                );
                *callback_rect.borrow_mut() = Some(response.rect);
                if reveal {
                    response.scroll_to_me(Some(egui::Align::TOP));
                }
            } else {
                ui.label(html);
            }
        };
        let mut scroll_y = 0.0;
        egui::CentralPanel::default().show(root, |ui| {
            if let Some(t) = &target {
                ui.label(format!("First source match: {:?} · {}", t.matched, &FIXTURE[t.matched.clone()]));
                if t.block.is_none() { ui.label("Source-only match; no rendered block. Source excerpt above is the fallback."); }
            } else if !self.query.is_empty() {
                ui.label(if "PathOnly.md".to_lowercase().contains(&self.query.to_lowercase()) { "Path-only match: PathOnly.md; no body passage" } else { "No body match" });
            }
            let result = egui::ScrollArea::vertical().id_salt("probe-reader").show(ui, |ui| {
                ui.set_max_width(860.0_f32.min(ui.available_width()));
                CommonMarkViewer::new().explicit_image_uri_scheme(true).render_html_fn(Some(&callback)).show(ui, &mut self.cache, &source);
            });
            scroll_y = result.state.offset.y;
            nodes.push(serde_json::json!({"id":"reader", "rect":[result.inner_rect.min.x,result.inner_rect.min.y,result.inner_rect.width(),result.inner_rect.height()], "scroll_y":scroll_y}));
        });
        let rect = marker_rect
            .borrow()
            .map(|r| [r.min.x, r.min.y, r.width(), r.height()]);
        crate::OBSERVATION.with(|o| *o.borrow_mut() = serde_json::json!({"query":self.query,"nodes":nodes,"scroll_y":scroll_y,"marker_rect":rect,"visible_source_text":visible,"match":target.as_ref().map(|t|vec![t.matched.start,t.matched.end]),"block":target.as_ref().and_then(|t|t.block.as_ref()).map(|r|vec![r.start,r.end])}).to_string());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
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
            let marked = prepared(FIXTURE, &t);
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
        let marked = prepared(source, &target(source, "reference").unwrap());
        assert!(!marked.contains("<script>"));
        assert!(Parser::new_ext(&marked, options()).any(|e| matches!(e, Event::Start(pulldown_cmark::Tag::Link {dest_url, ..}) if dest_url.as_ref() == "local.md")));
    }
}
