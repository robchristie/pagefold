use egui_commonmark::{CommonMarkCache, CommonMarkViewer};
use polyorama_ui_egui::{self as design, ActionKey};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::sync::{Arc, Mutex};

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Snapshot {
    pub root: String,
    pub state: String,
    pub generation: String,
    pub pages: BTreeMap<String, String>,
    pub images: BTreeMap<String, Vec<u8>>,
    pub attachments: BTreeMap<String, String>,
    pub index: BTreeMap<String, String>,
    pub warnings: Vec<String>,
}

/// All URL interpretation happens here, before lookup in the contained snapshot.
pub fn resolve(page: &str, link: &str) -> Result<String, String> {
    let path = link.split('#').next().unwrap_or_default();
    let decoded = percent_encoding::percent_decode_str(path)
        .decode_utf8()
        .map_err(|_| "Invalid UTF-8 in link")?;
    if decoded.starts_with('/')
        || decoded.contains([':', '\\', '?'])
        || decoded.chars().any(char::is_control)
    {
        return Err("Blocked absolute, external or unsafe path".into());
    }
    if decoded.is_empty() {
        return Ok(page.into());
    }
    let mut parts: Vec<&str> = page.split('/').collect();
    parts.pop();
    for part in decoded.split('/') {
        match part {
            ".." => {
                parts
                    .pop()
                    .ok_or("Blocked path outside the knowledge directory")?;
            }
            "." | "" => {}
            other => parts.push(other),
        }
    }
    Ok(parts.join("/"))
}

fn literal(text: &str) -> String {
    text.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('[', "&#91;")
        .replace(']', "&#93;")
}

/// Rewrite parsed image spans (including reference images), never textual substrings.
/// The renderer receives only registered byte URIs; HTML is explicitly escaped.
pub fn prepare(page: &str, source: &str, images: &BTreeMap<String, String>) -> String {
    use pulldown_cmark::{Event, Options, Parser, Tag};
    let mut replacements = Vec::new();
    let mut covered_until = 0;
    for (event, range) in Parser::new_ext(
        source,
        Options::ENABLE_TABLES | Options::ENABLE_STRIKETHROUGH,
    )
    .into_offset_iter()
    {
        if range.start < covered_until {
            continue;
        }
        let replacement = match event {
            Event::Start(Tag::Image { dest_url, .. }) => {
                let resolved = resolve(page, &dest_url);
                match resolved.as_ref().ok().and_then(|p| images.get(p)) {
                    Some(uri) => format!("![Local image]({uri})"),
                    None => format!(
                        "**Image unavailable:** {}",
                        literal(&resolved.unwrap_or_else(|e| format!("{dest_url} ({e})")))
                    ),
                }
            }
            Event::Html(html) | Event::InlineHtml(html) => literal(&html),
            _ => continue,
        };
        covered_until = range.end;
        replacements.push((range, replacement));
    }
    let mut output = source.to_owned();
    for (range, replacement) in replacements.into_iter().rev() {
        output.replace_range(range, &replacement);
    }
    output
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize)]
enum Action {
    Open,
    Refresh,
}
impl design::ActionKey for Action {
    fn stable_id(self) -> &'static str {
        match self {
            Self::Open => "pagefold.open",
            Self::Refresh => "pagefold.refresh",
        }
    }
    fn specification(self) -> design::ActionSpec<Self> {
        design::ActionSpec {
            id: self,
            label: match self {
                Self::Open => "Open directory",
                Self::Refresh => "Refresh / rebuild",
            },
            description: match self {
                Self::Open => "Read a directory and build a separate search index",
                Self::Refresh => "Reread all pages and attachments and rebuild the search index",
            },
            compact_label: None,
            shortcut: None,
            scope: design::ActionScope::Application,
        }
    }
}

type Pending = Arc<Mutex<Option<Result<Snapshot, String>>>>;

pub struct Pagefold {
    snapshot: Snapshot,
    directory: String,
    page: String,
    query: String,
    status: String,
    snapshot_stale: bool,
    source: String,
    image_uris: BTreeMap<String, String>,
    cache: CommonMarkCache,
    pending: Option<Pending>,
    endpoint: String,
}

fn font_bytes() -> Vec<u8> {
    use std::io::Read;
    let compressed = [
        include_bytes!("../assets/fonts/NotoSansCJKjp-Regular.zlib0").as_slice(),
        include_bytes!("../assets/fonts/NotoSansCJKjp-Regular.zlib1").as_slice(),
        include_bytes!("../assets/fonts/NotoSansCJKjp-Regular.zlib2").as_slice(),
        include_bytes!("../assets/fonts/NotoSansCJKjp-Regular.zlib3").as_slice(),
        include_bytes!("../assets/fonts/NotoSansCJKjp-Regular.zlib4").as_slice(),
        include_bytes!("../assets/fonts/NotoSansCJKjp-Regular.zlib5").as_slice(),
        include_bytes!("../assets/fonts/NotoSansCJKjp-Regular.zlib6").as_slice(),
        include_bytes!("../assets/fonts/NotoSansCJKjp-Regular.zlib7").as_slice(),
        include_bytes!("../assets/fonts/NotoSansCJKjp-Regular.zlib8").as_slice(),
        include_bytes!("../assets/fonts/NotoSansCJKjp-Regular.zlib9").as_slice(),
        include_bytes!("../assets/fonts/NotoSansCJKjp-Regular.zlib10").as_slice(),
        include_bytes!("../assets/fonts/NotoSansCJKjp-Regular.zlib11").as_slice(),
        include_bytes!("../assets/fonts/NotoSansCJKjp-Regular.zlib12").as_slice(),
    ]
    .concat();
    let mut font = Vec::with_capacity(16_467_736);
    flate2::read::ZlibDecoder::new(compressed.as_slice())
        .read_to_end(&mut font)
        .expect("bundled font is valid zlib");
    font
}

pub fn install_fonts(ctx: &egui::Context) {
    design::apply_design_system_with_typography(
        ctx,
        design::UiPreferences::default(),
        design::TypographyProfile::Reading,
    );
    ctx.add_font(egui::epaint::text::FontInsert {
        name: "Pagefold Noto Sans CJK JP".into(),
        data: egui::FontData::from_owned(font_bytes()),
        families: [
            egui::FontFamily::Proportional,
            egui::FontFamily::Monospace,
            egui::FontFamily::Name(design::REGULAR_FONT_FAMILY.into()),
            egui::FontFamily::Name(design::SEMIBOLD_FONT_FAMILY.into()),
        ]
        .into_iter()
        .map(|family| egui::epaint::text::InsertFontFamily {
            family,
            priority: egui::epaint::text::FontPriority::Lowest,
        })
        .collect(),
    });
}

impl Pagefold {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        install_fonts(&cc.egui_ctx);
        egui_extras::install_image_loaders(&cc.egui_ctx);
        Self {
            snapshot: Snapshot::default(),
            directory: String::new(),
            page: String::new(),
            query: String::new(),
            status: "Enter the absolute path of a Markdown directory to begin.".into(),
            snapshot_stale: false,
            source: String::new(),
            image_uris: BTreeMap::new(),
            cache: CommonMarkCache::default(),
            pending: None,
            endpoint: if cfg!(target_arch = "wasm32") {
                "/api/snapshot".into()
            } else {
                std::env::var("PAGEFOLD_ENDPOINT")
                    .unwrap_or("http://127.0.0.1:3817/api/snapshot".into())
            },
        }
    }

    fn request(&mut self, directory: String, ctx: &egui::Context) {
        let pending: Pending = Arc::new(Mutex::new(None));
        self.pending = Some(pending.clone());
        self.status = "Reading directory and rebuilding index…".into();
        let mut request = ehttp::Request::post(
            &self.endpoint,
            serde_json::to_vec(&serde_json::json!({"directory": directory})).unwrap(),
        );
        request.headers.insert("Content-Type", "application/json");
        let ctx = ctx.clone();
        ehttp::fetch(request, move |response| {
            let result = response.and_then(|r| {
                if r.ok {
                    serde_json::from_slice(&r.bytes).map_err(|e| e.to_string())
                } else {
                    let error: serde_json::Value =
                        serde_json::from_slice(&r.bytes).unwrap_or_default();
                    Err(error["error"]
                        .as_str()
                        .unwrap_or("Local service rejected the request")
                        .into())
                }
            });
            *pending.lock().unwrap() = Some(result);
            ctx.request_repaint();
        });
    }

    fn accept(&mut self, mut snapshot: Snapshot, ctx: &egui::Context) {
        self.snapshot_stale = false;
        for uri in self.image_uris.values() {
            ctx.forget_image(uri);
        }
        self.image_uris.clear();
        // Header dimensions bound decompression; malformed images have honest error states.
        for (path, bytes) in std::mem::take(&mut snapshot.images) {
            let valid = image::ImageReader::new(std::io::Cursor::new(&bytes))
                .with_guessed_format()
                .ok()
                .and_then(|r| r.into_dimensions().ok())
                .is_some_and(|(w, h)| w > 0 && h > 0 && w <= 4096 && h <= 4096);
            if valid {
                let uri = format!(
                    "bytes://{}/{}",
                    snapshot.generation,
                    percent_encoding::utf8_percent_encode(
                        &path,
                        percent_encoding::NON_ALPHANUMERIC
                    )
                );
                ctx.include_bytes(uri.clone(), bytes);
                self.image_uris.insert(path, uri);
            } else {
                snapshot.attachments.insert(
                    path.clone(),
                    "Unsupported or corrupt image, or dimensions exceed 4096 × 4096".into(),
                );
                snapshot
                    .warnings
                    .push(format!("Image preview unavailable: {path}"));
            }
        }
        let same_root = snapshot.root == self.snapshot.root;
        self.snapshot = snapshot;
        self.directory = self.snapshot.root.clone();
        self.cache = CommonMarkCache::default();
        if !same_root {
            self.page.clear();
        }
        if self.page.is_empty() {
            self.page = self
                .snapshot
                .pages
                .keys()
                .next()
                .cloned()
                .unwrap_or_default();
        }
        self.status = format!(
            "{} pages · refreshed from source · {} warnings",
            self.snapshot.pages.len(),
            self.snapshot.warnings.len()
        );
        self.open_page(self.page.clone());
    }

    fn open_page(&mut self, path: String) {
        self.page = path;
        self.source = self.snapshot.pages.get(&self.page)
            .map(|s| prepare(&self.page, s, &self.image_uris))
            .unwrap_or_else(|| {
                if self.page.is_empty() { "No readable Markdown pages in this directory.".into() }
                else { format!("**Page unavailable:** {}\n\nThe page was removed, renamed or could not be read. Choose another page.",
                    literal(&self.page)) }
            });
    }

    fn record_failure(&mut self, error: &str) {
        self.snapshot_stale = !self.snapshot.root.is_empty();
        self.status = format!("Open/refresh failed: {error}.");
    }

    fn visible_status(&self) -> String {
        if self.snapshot_stale {
            format!(
                "{} Previous snapshot retained; it may be stale.",
                self.status
            )
        } else {
            self.status.clone()
        }
    }

    fn navigate(&mut self, link: &str) {
        match resolve(&self.page, link) {
            Ok(path) if self.snapshot.pages.contains_key(&path) => {
                self.open_page(path);
                self.status = if link.contains('#') {
                    "Page opened. Heading fragments are not supported; scroll to the heading."
                        .into()
                } else {
                    "Read-only snapshot · use Refresh after external changes".into()
                };
            }
            Ok(path) => {
                self.status = self.snapshot.attachments.get(&path).map_or_else(
                    || format!("Missing page or attachment: {path}"),
                    |message| format!("{path}: {message}"),
                );
                if let Some(uri) = self.image_uris.get(&path) {
                    self.source = format!("![Local image]({uri})");
                }
            }
            Err(error) => self.status = format!("{error}: {link}"),
        }
    }
}

thread_local! {
    static OBSERVATION: std::cell::RefCell<String> = const { std::cell::RefCell::new(String::new()) };
}

fn observe(nodes: &mut Vec<serde_json::Value>, id: &str, label: &str, response: &egui::Response) {
    nodes.push(serde_json::json!({
        "id": id, "label": label, "rect": [response.rect.min.x, response.rect.min.y, response.rect.width(), response.rect.height()],
        "enabled": response.enabled(), "focused": response.has_focus()
    }));
}

impl eframe::App for Pagefold {
    fn ui(&mut self, root: &mut egui::Ui, _: &mut eframe::Frame) {
        let ctx = root.ctx().clone();
        let completed = self.pending.as_ref().and_then(|p| p.lock().unwrap().take());
        if let Some(result) = completed {
            self.pending = None;
            match result {
                Ok(snapshot) => self.accept(snapshot, &ctx),
                Err(error) => self.record_failure(&error),
            }
        }
        let mut nodes = Vec::new();
        let mut text_layouts = Vec::new();
        let tokens = design::ApplicationTheme::analytical().resolve(
            if root.visuals().dark_mode {
                design::ThemeVariant::Dark
            } else {
                design::ThemeVariant::Light
            },
            design::UiPreferences::default().density_variant(),
            design::TypographyProfile::Reading,
        );
        let mut action = None;
        egui::Panel::top("workspace-controls")
            .resizable(false)
            .show(root, |ui| {
                ui.heading("Pagefold");
                ui.label("Knowledge directory");
                let response = ui.add(
                    egui::TextEdit::singleline(&mut self.directory)
                        .id_salt("directory")
                        .desired_width(f32::INFINITY)
                        .hint_text("/absolute/path/to/knowledge"),
                );
                observe(&mut nodes, "directory", "Knowledge directory", &response);
                ui.horizontal_wrapped(|ui| {
                    for key in [Action::Open, Action::Refresh] {
                        let enabled = self.pending.is_none()
                            && match key {
                                Action::Open => !self.directory.is_empty(),
                                Action::Refresh => !self.snapshot.root.is_empty(),
                            };
                        let response = design::action_button(
                            ui,
                            design::ActionButtonSpec {
                                target: design::ActionTarget::application(key),
                                availability: if enabled {
                                    design::Availability::Enabled
                                } else {
                                    design::Availability::Disabled {
                                    reason:
                                        "Enter a directory, or wait for the current read to finish"
                                            .into(),
                                }
                                },
                                state: design::ActionButtonState::Momentary,
                                emphasis: design::ActionEmphasis::Normal,
                                compact: false,
                            },
                            &tokens,
                            1.0,
                            &mut text_layouts,
                        );
                        observe(
                            &mut nodes,
                            key.stable_id(),
                            key.specification().label,
                            &response,
                        );
                        if let Some(node) = nodes.last_mut() {
                            node["enabled"] = enabled.into();
                        }
                        if response.clicked() {
                            action = Some(key);
                        }
                    }
                });
                ui.add(
                    egui::Label::new(self.visible_status())
                        .wrap()
                        .selectable(true),
                );
            });
        if let Some(action) = action {
            let directory = match action {
                Action::Open => self.directory.clone(),
                Action::Refresh => self.snapshot.root.clone(),
            };
            self.request(directory, &ctx);
        }
        egui::Panel::top("page-browser")
            .resizable(false)
            .min_size(if self.snapshot.warnings.is_empty() {
                190.0
            } else {
                310.0
            })
            .show(root, |ui| {
                let response = ui.add(
                    egui::TextEdit::singleline(&mut self.query)
                        .id_salt("search")
                        .hint_text("Search page text")
                        .desired_width(f32::INFINITY),
                );
                observe(&mut nodes, "search", "Search page text", &response);
                let query = self.query.to_lowercase();
                let matches: Vec<String> = self
                    .snapshot
                    .index
                    .iter()
                    .filter(|(p, text)| {
                        query.is_empty()
                            || text.contains(&query)
                            || p.to_lowercase().contains(&query)
                    })
                    .map(|(p, _)| p.clone())
                    .collect();
                ui.label(format!("{} matching pages", matches.len()));
                egui::ScrollArea::vertical()
                    .id_salt("browse")
                    .max_height(120.0)
                    .show_rows(
                        ui,
                        ui.spacing().interact_size.y,
                        matches.len(),
                        |ui, range| {
                            for path in &matches[range] {
                                ui.push_id(path, |ui| {
                                    let response = ui
                                        .add_sized(
                                            [ui.available_width(), ui.spacing().interact_size.y],
                                            egui::Button::new(path)
                                                .selected(self.page == *path)
                                                .truncate(),
                                        )
                                        .on_hover_text(path);
                                    design::record_native_text_control(
                                        &response,
                                        design::NativeTextControlKind::Selectable,
                                    );
                                    observe(&mut nodes, &format!("page:{path}"), path, &response);
                                    if response.clicked() {
                                        self.open_page(path.clone());
                                    }
                                });
                            }
                        },
                    );
                if !self.snapshot.warnings.is_empty() {
                    egui::CollapsingHeader::new("Directory warnings").show(ui, |ui| {
                        egui::ScrollArea::vertical()
                            .max_height(100.0)
                            .show(ui, |ui| {
                                for warning in &self.snapshot.warnings {
                                    ui.add(egui::Label::new(warning).wrap().selectable(true));
                                }
                            });
                    });
                }
            });
        egui::CentralPanel::default().show(root, |ui| {
            ui.style_mut().interaction.selectable_labels = true;
            let result = egui::ScrollArea::vertical().id_salt((&self.page, "reader")).show(ui, |ui| {
                ui.set_max_width(860.0_f32.min(ui.available_width()));
                ui.label(&self.page);
                CommonMarkViewer::new().explicit_image_uri_scheme(true)
                    .max_image_width(Some(ui.available_width().max(1.0) as usize))
                    .show(ui, &mut self.cache, &self.source);
            });
            nodes.push(serde_json::json!({"id":"reader", "rect":[result.inner_rect.min.x,result.inner_rect.min.y,
                result.inner_rect.width(),result.inner_rect.height()], "scroll_y":result.state.offset.y}));
        });
        let links = ctx.output_mut(|out| {
            let mut links = Vec::new();
            out.commands.retain(|command| {
                if let egui::OutputCommand::OpenUrl(url) = command {
                    links.push(url.url.clone());
                    false
                } else {
                    true
                }
            });
            links
        });
        for link in links {
            self.navigate(&link);
            ctx.request_repaint();
        }
        let coverage = design::text_audit_coverage(&ctx, &text_layouts);
        OBSERVATION.with(|o| *o.borrow_mut() = serde_json::json!({
            "page":self.page, "root":self.snapshot.root, "generation":self.snapshot.generation,
            "status":self.visible_status(), "snapshot_stale":self.snapshot_stale,
            "query":self.query, "pending":self.pending.is_some(),
            "nodes":nodes, "text_coverage":coverage, "text_layouts":text_layouts,
            "limitations":["Reader and native label layout are unmeasured", "Text inputs have no framework coverage category", "Browser AccessKit adapter unavailable"],
        }).to_string());
    }
}

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn observation() -> String {
    OBSERVATION.with(|o| o.borrow().clone())
}
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub async fn start(canvas: web_sys::HtmlCanvasElement) -> Result<(), JsValue> {
    console_error_panic_hook::set_once();
    eframe::WebRunner::new()
        .start(
            canvas,
            eframe::WebOptions::default(),
            Box::new(|cc| Ok(Box::new(Pagefold::new(cc)))),
        )
        .await
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn stale_warning_survives_navigation_until_a_successful_snapshot() {
        let mut app = Pagefold {
            snapshot: Snapshot::default(),
            directory: String::new(),
            page: String::new(),
            query: String::new(),
            status: String::new(),
            snapshot_stale: false,
            source: String::new(),
            image_uris: BTreeMap::new(),
            cache: CommonMarkCache::default(),
            pending: None,
            endpoint: String::new(),
        };
        app.record_failure("directory unavailable");
        assert!(!app.snapshot_stale); // No previous snapshot exists on an initial failure.
        let snapshot = Snapshot {
            root: "/synthetic".into(),
            pages: BTreeMap::from([
                ("Home.md".into(), "Home".into()),
                ("Other.md".into(), "Other".into()),
            ]),
            ..Snapshot::default()
        };
        let ctx = egui::Context::default();
        app.accept(snapshot.clone(), &ctx);
        app.record_failure("directory unavailable");
        for link in [
            "Other.md",
            "Missing.md",
            "../outside.md",
            "https://example.invalid",
        ] {
            app.navigate(link);
            assert!(app.snapshot_stale);
            assert!(
                app.visible_status()
                    .contains("Previous snapshot retained; it may be stale.")
            );
        }
        app.open_page("Home.md".into());
        assert!(app.visible_status().contains("may be stale"));
        app.accept(snapshot, &ctx);
        assert!(!app.snapshot_stale);
        assert!(!app.visible_status().contains("may be stale"));
    }
    #[test]
    fn paths_and_encoded_urls_are_contained() {
        assert_eq!(
            resolve("guides/Reading.md", "../Home.md").unwrap(),
            "Home.md"
        );
        assert_eq!(resolve("Home.md", "a%20b.md#heading").unwrap(), "a b.md");
        for link in [
            "../outside",
            "%2e%2e/outside",
            "/etc/passwd",
            "%2fetc/passwd",
            "https://example.org",
            "javascript:alert(1)",
            "data:image/png,x",
            "file:///etc/passwd",
            "a\\b",
            "a%00b",
        ] {
            assert!(resolve("Home.md", link).is_err(), "{link}");
        }
    }
    #[test]
    fn images_and_html_cannot_escape_the_renderer() {
        let images = BTreeMap::from([("img.png".into(), "bytes://safe".into())]);
        let source = "![x](img.png)\n![ref][r]\n\n[r]: https://evil.invalid/a.png\n\n<script>alert(1)</script>\n\n`![code](img.png)`";
        let out = prepare("Home.md", source, &images);
        assert!(out.contains("![Local image](bytes://safe)"));
        assert!(out.contains("Image unavailable"));
        assert!(!out.contains("<script>"));
        assert!(out.contains("`![code](img.png)`"));
        for event in pulldown_cmark::Parser::new(&out) {
            if let pulldown_cmark::Event::Start(pulldown_cmark::Tag::Image { dest_url, .. }) = event
            {
                assert!(dest_url.starts_with("bytes://"));
            }
        }
    }
    #[test]
    fn japanese_font_contains_the_calibrated_glyphs() {
        let ctx = egui::Context::default();
        install_fonts(&ctx);
        let mut output = ctx.run_ui(egui::RawInput::default(), |ui| {
            let ctx = ui.ctx();
            ctx.fonts_mut(|fonts| {
                let font = egui::FontId::new(
                    18.0,
                    egui::FontFamily::Name(design::REGULAR_FONT_FAMILY.into()),
                );
                for c in "日本語".chars() {
                    assert!(fonts.has_glyph(&font, c), "{c}");
                }
            });
        });
        output.textures_delta.clear();
    }
}
