use std::path::PathBuf;

use ructe::{Result, Ructe};

fn main() -> Result<()> {
    let templates_path = "templates";
    println!("cargo:rerun-if-changed={templates_path}");

    Ructe::new(PathBuf::from("src"))?.compile_templates(templates_path)
}
