fn main() -> eframe::Result {
    eframe::run_native(
        "Pagefold",
        eframe::NativeOptions::default(),
        Box::new(|cc| Ok(Box::new(pagefold_probe::Pagefold::new(cc)))),
    )
}
