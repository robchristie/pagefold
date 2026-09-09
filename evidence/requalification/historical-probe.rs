use egui_commonmark::{CommonMarkCache, CommonMarkViewer};
use std::collections::BTreeMap;

pub struct Probe {
    pages: BTreeMap<String, String>,
    page: String,
    status: String,
    cache: CommonMarkCache,
}

impl Probe {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        polyorama_ui_egui::apply_design_system_with_typography(
            &cc.egui_ctx,
            polyorama_ui_egui::UiPreferences::default(),
            polyorama_ui_egui::TypographyProfile::Reading,
        );
        egui_extras::install_image_loaders(&cc.egui_ctx);
        cc.egui_ctx.include_bytes(
            "bytes://gradient.png",
            include_bytes!(concat!(env!("OUT_DIR"), "/gradient.png")),
        );
        Self {
            pages: serde_json::from_str(include_str!(concat!(env!("OUT_DIR"), "/pages.json")))
                .unwrap(),
            page: "Home.md".into(),
            status: "Read-only synthetic fixture · calibration probe".into(),
            cache: CommonMarkCache::default(),
        }
    }

    fn navigate(&mut self, link: &str) {
        match resolve(&self.page, link) {
            Some(path) if self.pages.contains_key(&path) => {
                self.page = path;
                self.status = "Read-only synthetic fixture · calibration probe".into();
            }
            Some(path) if path.ends_with(".md") => self.status = format!("Missing page: {path}"),
            Some(path) if path == "attachments/example.dat" => {
                self.status = format!("Unsupported attachment: {path} (binary data; no preview)")
            }
            Some(path) => self.status = format!("Missing or unsupported attachment: {path}"),
            None => self.status = format!("Blocked outside or external path: {link}"),
        }
    }
}

/// Calibration resolver; the full filesystem policy belongs to the next increment.
fn resolve(page: &str, link: &str) -> Option<String> {
    if link.starts_with('/') || link.contains(':') || link.contains('\\') {
        return None;
    }
    let mut parts: Vec<&str> = page.split('/').collect();
    parts.pop();
    for part in link.split('/') {
        match part {
            ".." => {
                parts.pop()?;
            }
            "." | "" => {}
            other => parts.push(other),
        }
    }
    Some(parts.join("/"))
}

impl eframe::App for Probe {
    fn ui(&mut self, root: &mut egui::Ui, _: &mut eframe::Frame) {
        let ctx = root.ctx().clone();
        egui::Panel::top("probe-navigation").show(root, |ui| {
            ui.heading("Pagefold · reader calibration");
            ui.horizontal_wrapped(|ui| {
                for page in ["Home.md", "guides/Reading.md", "Unicode.md"] {
                    if ui.selectable_label(self.page == page, page).clicked() {
                        self.page = page.into();
                        self.status = "Read-only synthetic fixture · calibration probe".into();
                    }
                }
            });
            ui.add(egui::Label::new(&self.status).wrap().selectable(true));
        });
        egui::CentralPanel::default().show(root, |ui| {
            ui.style_mut().interaction.selectable_labels = true;
            let source =
                self.pages[&self.page].replace("attachments/gradient.png", "bytes://gradient.png");
            egui::ScrollArea::vertical()
                .id_salt((&self.page, "reader"))
                .show(ui, |ui| {
                    CommonMarkViewer::new()
                        .explicit_image_uri_scheme(true)
                        .max_image_width(Some(ui.available_width().max(1.0) as usize))
                        .show(ui, &mut self.cache, &source);
                });
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
    }
}

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub async fn start(canvas: web_sys::HtmlCanvasElement) -> Result<(), JsValue> {
    console_error_panic_hook::set_once();
    eframe::WebRunner::new()
        .start(
            canvas,
            eframe::WebOptions::default(),
            Box::new(|cc| Ok(Box::new(Probe::new(cc)))),
        )
        .await
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn relative_navigation_is_contained() {
        assert_eq!(
            resolve("guides/Reading.md", "../Home.md"),
            Some("Home.md".into())
        );
        for bad in [
            "../outside.md",
            "/etc/passwd",
            "https://example.invalid/",
            "javascript:alert(1)",
            "a\\b",
        ] {
            assert_eq!(resolve("Home.md", bad), None);
        }
    }
}
