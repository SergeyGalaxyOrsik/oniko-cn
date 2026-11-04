//! `oniko build`: compiles `registry/` into the published JSON layout
//!
//! ```text
//! <out>/registry.json                    index of styles, themes and items
//! <out>/styles/<style>/index.json        registry:style (base CSS) for `init`
//! <out>/styles/<style>/<item>.json       compiled items for `add`
//! <out>/themes/<theme>.json              registry:theme (CSS variables)
//! ```

use std::collections::BTreeSet;
use std::fs;
use std::path::Path;

use anyhow::{Context, Result, bail};

use crate::schema::*;
use crate::{style_map, transform};

#[derive(Debug, Default)]
pub struct BuildReport {
    pub styles: Vec<String>,
    pub themes: Vec<String>,
    pub items: usize,
    /// `(style, selector)` pairs that the style compiler could not use.
    pub skipped_selectors: Vec<(String, String)>,
}

pub fn build(registry_dir: &Path, out_dir: &Path) -> Result<BuildReport> {
    let registry: Registry = read_json(&registry_dir.join("registry.json"))?;
    check_dependencies(&registry)?;
    let base_css = fs::read_to_string(registry_dir.join("base.css")).context("reading base.css")?;
    let mut report = BuildReport::default();

    for (style, css) in read_styles(&registry_dir.join("styles"))? {
        let map = style_map::parse(&css).with_context(|| format!("compiling style `{style}`"))?;
        report
            .skipped_selectors
            .extend(map.skipped.iter().map(|s| (style.clone(), s.clone())));
        let style_dir = out_dir.join("styles").join(&style);
        fs::create_dir_all(&style_dir)?;

        let index = RegistryItem {
            schema: Some(ITEM_SCHEMA.into()),
            name: style.clone(),
            kind: ItemType::Style,
            title: None,
            description: Some(format!("The `{style}` style.")),
            dependencies: vec!["oniko-cn@0.1".into()],
            registry_dependencies: vec![],
            files: vec![],
            css_vars: None,
            css: Some(base_css.clone()),
        };
        write_json(&style_dir.join("index.json"), &index)?;

        for item in &registry.items {
            let mut compiled = item.clone();
            compiled.schema = Some(ITEM_SCHEMA.into());
            for file in &mut compiled.files {
                let source = fs::read_to_string(registry_dir.join(&file.path))
                    .with_context(|| format!("reading {}", file.path))?;
                let compiled_source = transform::apply_style_map(&source, &map);
                let formatted = rustfmt(&compiled_source)
                    .with_context(|| format!("formatting {} ({style})", file.path))?;
                file.content = Some(formatted);
            }
            write_json(&style_dir.join(format!("{}.json", item.name)), &compiled)?;
        }
        report.styles.push(style);
    }

    let themes_out = out_dir.join("themes");
    fs::create_dir_all(&themes_out)?;
    for entry in sorted_entries(&registry_dir.join("themes"))? {
        let theme: RegistryItem = read_json(&entry)?;
        if theme.kind != ItemType::Theme || theme.css_vars.is_none() {
            bail!("{} is not a registry:theme with cssVars", entry.display());
        }
        write_json(&themes_out.join(format!("{}.json", theme.name)), &theme)?;
        report.themes.push(theme.name);
    }

    report.items = registry.items.len();
    let index = PublishedIndex {
        name: registry.name.clone(),
        styles: report.styles.clone(),
        themes: report.themes.clone(),
        items: registry
            .items
            .iter()
            .map(|item| IndexEntry {
                name: item.name.clone(),
                kind: item.kind,
                description: item.description.clone(),
            })
            .collect(),
    };
    write_json(&out_dir.join("registry.json"), &index)?;
    Ok(report)
}

/// Inlined classes make lines longer, so the compiled file is re-formatted:
/// users get code that `cargo fmt` leaves untouched.
fn rustfmt(source: &str) -> Result<String> {
    use std::io::Write;
    use std::process::{Command, Stdio};

    let mut child = Command::new("rustfmt")
        .args(["--edition", "2024", "--emit", "stdout"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .context("running rustfmt (install it with `rustup component add rustfmt`)")?;
    child
        .stdin
        .take()
        .context("rustfmt stdin")?
        .write_all(source.as_bytes())?;
    let output = child.wait_with_output()?;
    if !output.status.success() {
        bail!(
            "rustfmt failed:\n{}",
            String::from_utf8_lossy(&output.stderr)
        );
    }
    Ok(String::from_utf8(output.stdout)?)
}

fn check_dependencies(registry: &Registry) -> Result<()> {
    let names: BTreeSet<&str> = registry.items.iter().map(|i| i.name.as_str()).collect();
    for item in &registry.items {
        for dep in &item.registry_dependencies {
            let external = dep.contains('/') || dep.contains(':');
            if !external && !names.contains(dep.as_str()) {
                bail!("item `{}` depends on unknown item `{dep}`", item.name);
            }
        }
    }
    Ok(())
}

fn read_styles(dir: &Path) -> Result<Vec<(String, String)>> {
    let mut styles = Vec::new();
    for path in sorted_entries(dir)? {
        let file = path
            .file_name()
            .and_then(|f| f.to_str())
            .unwrap_or_default();
        if let Some(name) = file
            .strip_prefix("style-")
            .and_then(|f| f.strip_suffix(".css"))
        {
            styles.push((name.to_string(), fs::read_to_string(&path)?));
        }
    }
    if styles.is_empty() {
        bail!("no style-<name>.css files in {}", dir.display());
    }
    Ok(styles)
}

fn sorted_entries(dir: &Path) -> Result<Vec<std::path::PathBuf>> {
    let mut entries = fs::read_dir(dir)
        .with_context(|| format!("reading {}", dir.display()))?
        .map(|e| e.map(|e| e.path()))
        .collect::<std::io::Result<Vec<_>>>()?;
    entries.sort();
    Ok(entries)
}

fn read_json<T: serde::de::DeserializeOwned>(path: &Path) -> Result<T> {
    let text = fs::read_to_string(path).with_context(|| format!("reading {}", path.display()))?;
    serde_json::from_str(&text).with_context(|| format!("parsing {}", path.display()))
}

fn write_json(path: &Path, value: &impl serde::Serialize) -> Result<()> {
    let mut text = serde_json::to_string_pretty(value)?;
    text.push('\n');
    fs::write(path, text).with_context(|| format!("writing {}", path.display()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builds_shipped_registry_without_placeholders() {
        let registry = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../registry");
        let out = tempfile::tempdir().unwrap();
        let report = build(&registry, out.path()).unwrap();
        assert_eq!(report.styles.len(), 8);
        assert_eq!(report.themes.len(), 7);

        for style in &report.styles {
            let item: RegistryItem =
                read_json(&out.path().join(format!("styles/{style}/button.json"))).unwrap();
            let content = item.files[0].content.as_deref().unwrap();
            assert!(
                !content.contains("\"cn-"),
                "{style}: placeholder left in button.rs"
            );
            assert!(
                content.contains("bg-primary"),
                "{style}: default variant missing"
            );
            assert_eq!(
                rustfmt(content).unwrap(),
                content,
                "{style}: not rustfmt-stable"
            );
        }
        let index: PublishedIndex = read_json(&out.path().join("registry.json")).unwrap();
        assert!(index.items.iter().any(|i| i.name == "button"));
    }
}
