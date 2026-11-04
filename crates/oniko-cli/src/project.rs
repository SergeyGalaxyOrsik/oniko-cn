//! Edits to the user's crate: Cargo dependencies, module declarations and
//! component files.

use std::fs;
use std::path::{Component, Path, PathBuf};

use anyhow::{Context, Result, bail};
use oniko_registry::parse_dependency;
use toml_edit::{DocumentMut, Item, Table, value};

/// Adds missing crates to `[dependencies]`; returns the names that were added.
/// Crates already present (in any form) are left untouched.
pub fn add_dependencies(root: &Path, specs: &[String], overrides: &Table) -> Result<Vec<String>> {
    let path = root.join("Cargo.toml");
    let text = fs::read_to_string(&path).with_context(|| format!("reading {}", path.display()))?;
    let mut doc: DocumentMut = text
        .parse()
        .with_context(|| format!("parsing {}", path.display()))?;
    let deps = doc
        .entry("dependencies")
        .or_insert_with(|| Item::Table(Table::new()))
        .as_table_like_mut()
        .context("[dependencies] is not a table")?;

    let mut added = Vec::new();
    for spec in specs {
        let (name, version) = parse_dependency(spec);
        if deps.contains_key(name) {
            continue;
        }
        let item = match overrides.get(name) {
            Some(item) => item.clone(),
            None => value(version.unwrap_or("*")),
        };
        deps.insert(name, item);
        added.push(name.to_string());
    }
    if !added.is_empty() {
        fs::write(&path, doc.to_string()).with_context(|| format!("writing {}", path.display()))?;
    }
    Ok(added)
}

/// Makes `dir` (relative to the crate root, under `src/`) a declared module
/// chain, creating `mod.rs` files and `mod` lines as needed.
pub fn ensure_module_dir(root: &Path, dir: &Path) -> Result<()> {
    let names = module_names(dir)?;
    let src = root.join("src");
    let mut parent_file = crate_root_file(&src)?;
    let mut current = src;
    for (depth, name) in names.iter().enumerate() {
        current = current.join(name);
        fs::create_dir_all(&current)?;
        declare_module(&parent_file, name, depth == 0)?;
        parent_file = module_file(&current);
        if !parent_file.exists() {
            fs::write(&parent_file, "")?;
        }
    }
    Ok(())
}

/// Declares `file`'s stem in the `mod.rs` of its directory.
pub fn declare_file_module(root: &Path, file: &Path) -> Result<()> {
    let dir = file
        .parent()
        .context("component file has no parent directory")?;
    ensure_module_dir(root, dir)?;
    let stem = file
        .file_stem()
        .and_then(|s| s.to_str())
        .context("invalid file name")?;
    declare_module(&module_file(&root.join(dir)), stem, false)
}

pub enum WriteOutcome {
    Created,
    Unchanged,
    Overwritten,
    SkippedExisting,
}

pub fn write_file(path: &Path, content: &str, overwrite: bool) -> Result<WriteOutcome> {
    if let Ok(existing) = fs::read_to_string(path) {
        if existing == content {
            return Ok(WriteOutcome::Unchanged);
        }
        if !overwrite {
            return Ok(WriteOutcome::SkippedExisting);
        }
        fs::write(path, content)?;
        return Ok(WriteOutcome::Overwritten);
    }
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(path, content).with_context(|| format!("writing {}", path.display()))?;
    Ok(WriteOutcome::Created)
}

fn module_names(dir: &Path) -> Result<Vec<String>> {
    let mut components = dir.components();
    if components.next() != Some(Component::Normal("src".as_ref())) {
        bail!("{} must be inside src/", dir.display());
    }
    components
        .map(|c| match c {
            Component::Normal(name) => name
                .to_str()
                .filter(|n| n.chars().all(|c| c.is_ascii_alphanumeric() || c == '_'))
                .map(str::to_string)
                .with_context(|| {
                    format!("`{}` is not a valid module name", name.to_string_lossy())
                }),
            _ => bail!("unsupported path {}", dir.display()),
        })
        .collect()
}

fn crate_root_file(src: &Path) -> Result<PathBuf> {
    ["main.rs", "lib.rs"]
        .iter()
        .map(|f| src.join(f))
        .find(|p| p.exists())
        .with_context(|| format!("no main.rs or lib.rs in {}", src.display()))
}

/// `foo.rs` next to the directory wins over `foo/mod.rs`, as in rustc.
fn module_file(dir: &Path) -> PathBuf {
    let sibling = dir.with_extension("rs");
    if sibling.exists() {
        sibling
    } else {
        dir.join("mod.rs")
    }
}

fn declare_module(file: &Path, name: &str, private: bool) -> Result<()> {
    let text = fs::read_to_string(file).unwrap_or_default();
    let declared = text.lines().any(|line| {
        let line = line.trim_start();
        let line = line
            .strip_prefix("pub(crate) ")
            .or_else(|| line.strip_prefix("pub "))
            .unwrap_or(line);
        line.strip_prefix("mod ")
            .is_some_and(|rest| rest.trim_end().trim_end_matches(';').trim() == name)
    });
    if declared {
        return Ok(());
    }
    let decl = if private {
        format!("mod {name};")
    } else {
        format!("pub mod {name};")
    };
    // Insert after the leading block of inner docs / attributes / mod lines.
    let mut lines: Vec<&str> = text.lines().collect();
    let insert_at = lines
        .iter()
        .take_while(|l| {
            let l = l.trim_start();
            l.starts_with("//!") || l.starts_with("#![") || l.contains("mod ") && l.ends_with(';')
        })
        .count();
    lines.insert(insert_at, &decl);
    let mut out = lines.join("\n");
    out.push('\n');
    fs::write(file, out).with_context(|| format!("writing {}", file.display()))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn crate_dir() -> tempfile::TempDir {
        let dir = tempfile::tempdir().unwrap();
        fs::create_dir_all(dir.path().join("src")).unwrap();
        fs::write(
            dir.path().join("src/main.rs"),
            "//! App.\nuse std::fmt;\n\nfn main() {}\n",
        )
        .unwrap();
        fs::write(
            dir.path().join("Cargo.toml"),
            "[package]\nname = \"app\"\n\n[dependencies]\ndioxus = \"0.7\" # keep me\n",
        )
        .unwrap();
        dir
    }

    #[test]
    fn declares_module_chain_once() {
        let dir = crate_dir();
        let file = Path::new("src/components/ui/button.rs");
        declare_file_module(dir.path(), file).unwrap();
        declare_file_module(dir.path(), file).unwrap();
        let main = fs::read_to_string(dir.path().join("src/main.rs")).unwrap();
        assert_eq!(
            main,
            "//! App.\nmod components;\nuse std::fmt;\n\nfn main() {}\n"
        );
        let components = fs::read_to_string(dir.path().join("src/components/mod.rs")).unwrap();
        assert_eq!(components, "pub mod ui;\n");
        let ui = fs::read_to_string(dir.path().join("src/components/ui/mod.rs")).unwrap();
        assert_eq!(ui, "pub mod button;\n");
    }

    #[test]
    fn adds_only_missing_dependencies_and_keeps_formatting() {
        let dir = crate_dir();
        let mut overrides = Table::new();
        overrides["oniko-cn"] = toml_edit::Item::Value(
            "{ path = \"../oniko-cn\" }"
                .parse::<toml_edit::Value>()
                .unwrap(),
        );
        let added = add_dependencies(
            dir.path(),
            &["dioxus@0.8".into(), "oniko-cn@0.1".into(), "serde@1".into()],
            &overrides,
        )
        .unwrap();
        assert_eq!(added, vec!["oniko-cn", "serde"]);
        let cargo = fs::read_to_string(dir.path().join("Cargo.toml")).unwrap();
        assert!(cargo.contains("dioxus = \"0.7\" # keep me"));
        assert!(cargo.contains("oniko-cn = { path = \"../oniko-cn\" }"));
        assert!(cargo.contains("serde = \"1\""));
    }
}
