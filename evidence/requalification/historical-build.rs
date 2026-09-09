use std::{collections::BTreeMap, fs, path::PathBuf};
fn main() {
    let root = PathBuf::from("/nvme/development/pagefold-supervision-20260909-a/knowledge");
    let out = PathBuf::from(std::env::var_os("OUT_DIR").unwrap());
    let mut pages = BTreeMap::new();
    for name in ["Home.md", "Unicode.md", "guides/Reading.md"] {
        println!("cargo:rerun-if-changed={}", root.join(name).display());
        pages.insert(name, fs::read_to_string(root.join(name)).unwrap());
    }
    fs::write(out.join("pages.json"), serde_json::to_vec(&pages).unwrap()).unwrap();
    let image = root.join("attachments/gradient.png");
    println!("cargo:rerun-if-changed={}", image.display());
    fs::copy(image, out.join("gradient.png")).unwrap();
}
