//! `oniko.toml`: per-project configuration (the counterpart of shadcn's
//! `components.json`).

use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result, bail};
use toml_edit::{DocumentMut, Item, Table, value};

pub const FILE_NAME: &str = "oniko.toml";

#[derive(Debug, Clone)]
pub struct Config {
    /// Directory containing `oniko.toml` and `Cargo.toml`.
    pub root: PathBuf,
    /// Registry base: an `http(s)://` URL or a path relative to `root`.
    pub registry: String,
    pub style: String,
    pub base_color: String,
    pub css: PathBuf,
    pub ui: PathBuf,
    pub hooks: PathBuf,
    pub lib: PathBuf,
    /// Cargo dependency specs that replace registry versions, e.g. a local
    /// `path` while a crate is unpublished.
    pub dependency_overrides: Table,
}

impl Config {
    pub fn new(root: &Path, registry: &str, style: &str, base_color: &str) -> Self {
        Self {
            root: root.to_path_buf(),
            registry: registry.to_string(),
            style: style.to_string(),
            base_color: base_color.to_string(),
            css: "tailwind.css".into(),
            ui: "src/components/ui".into(),
            hooks: "src/hooks".into(),
            lib: "src/utils".into(),
            dependency_overrides: Table::new(),
        }
    }

    pub fn load(root: &Path) -> Result<Self> {
        let path = root.join(FILE_NAME);
        let text = fs::read_to_string(&path)
            .with_context(|| format!("{} not found: run `oniko init` first", path.display()))?;
        let doc: DocumentMut = text
            .parse()
            .with_context(|| format!("parsing {}", path.display()))?;

        let string = |key: &str| -> Result<String> {
            doc.get(key)
                .and_then(Item::as_str)
                .map(str::to_string)
                .with_context(|| format!("`{key}` is missing in {FILE_NAME}"))
        };
        let mut config = Self::new(
            root,
            &string("registry")?,
            &string("style")?,
            &string("base_color")?,
        );
        if let Some(paths) = doc.get("paths").and_then(Item::as_table_like) {
            for (key, slot) in [
                ("css", &mut config.css),
                ("ui", &mut config.ui),
                ("hooks", &mut config.hooks),
                ("lib", &mut config.lib),
            ] {
                if let Some(path) = paths.get(key).and_then(Item::as_str) {
                    *slot = path.into();
                }
            }
        }
        if let Some(overrides) = doc.get("dependency_overrides").and_then(Item::as_table) {
            config.dependency_overrides = overrides.clone();
        }
        Ok(config)
    }

    pub fn save(&self) -> Result<()> {
        let mut doc = DocumentMut::new();
        doc.decor_mut()
            .set_prefix("# oniko-cn project configuration. See `oniko --help`.\n");
        doc["registry"] = value(&self.registry);
        doc["style"] = value(&self.style);
        doc["base_color"] = value(&self.base_color);
        let mut paths = Table::new();
        for (key, path) in [
            ("css", &self.css),
            ("ui", &self.ui),
            ("hooks", &self.hooks),
            ("lib", &self.lib),
        ] {
            paths[key] = value(path.to_string_lossy().as_ref());
        }
        doc["paths"] = Item::Table(paths);
        if !self.dependency_overrides.is_empty() {
            doc["dependency_overrides"] = Item::Table(self.dependency_overrides.clone());
        }
        let path = self.root.join(FILE_NAME);
        fs::write(&path, doc.to_string()).with_context(|| format!("writing {}", path.display()))
    }

    pub fn ensure_absent(root: &Path) -> Result<()> {
        if root.join(FILE_NAME).exists() {
            bail!(
                "{FILE_NAME} already exists in {} (use --force to overwrite)",
                root.display()
            );
        }
        Ok(())
    }
}
