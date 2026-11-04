//! Embeds every `registry:ui` item, compiled for the site's style, so the
//! component pages list and show exactly what `oniko add` installs.

use std::error::Error;
use std::fmt::Write;
use std::path::Path;
use std::{env, fs};

const STYLE: &str = "mira";

fn main() -> Result<(), Box<dyn Error>> {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let registry_json = root.join("registry/registry.json");
    let compiled_dir = root.join("public/r/styles").join(STYLE);
    println!("cargo::rerun-if-changed={}", registry_json.display());
    println!("cargo::rerun-if-changed={}", compiled_dir.display());

    let registry: serde_json::Value = serde_json::from_str(&fs::read_to_string(&registry_json)?)?;
    let items = registry["items"]
        .as_array()
        .ok_or("registry.json has no items")?;
    let mut out = String::from("pub const COMPONENTS: &[(&str, &str)] = &[\n");
    for item in items.iter().filter(|i| i["type"] == "registry:ui") {
        let name = item["name"]
            .as_str()
            .ok_or("registry item without a name")?;
        let path = compiled_dir.join(format!("{name}.json"));
        writeln!(
            out,
            "    ({name:?}, include_str!({:?})),",
            path.display().to_string()
        )?;
    }
    out.push_str("];\n");
    fs::write(Path::new(&env::var("OUT_DIR")?).join("components.rs"), out)?;
    Ok(())
}
