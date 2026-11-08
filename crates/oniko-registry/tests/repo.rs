//! Repository invariants from AGENTS.md. Every failure message says how to fix it.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};

use oniko_registry::{ItemType, Registry, RegistryItem, build::build, style_map};

const WEB_ONLY_CRATES: &[&str] = &["web_sys", "wasm_bindgen", "js_sys", "gloo"];

fn root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn registry() -> Registry {
    let text = fs::read_to_string(root().join("registry/registry.json")).unwrap();
    serde_json::from_str(&text).unwrap()
}

/// Rust sources of `registry:ui` components and of `registry:block` patterns.
fn component_sources() -> Vec<PathBuf> {
    let mut files: Vec<PathBuf> = ["registry/ui", "registry/blocks"]
        .iter()
        .map(|dir| root().join(dir))
        .filter(|dir| dir.exists())
        .flat_map(|dir| fs::read_dir(dir).unwrap())
        .map(|e| e.unwrap().path())
        .filter(|p| p.extension().is_some_and(|e| e == "rs"))
        .collect();
    files.sort();
    files
}

fn is_installable(item: &RegistryItem) -> bool {
    matches!(item.kind, ItemType::Ui | ItemType::Block)
}

fn files_under(dir: &Path) -> BTreeMap<PathBuf, String> {
    fn walk(base: &Path, dir: &Path, out: &mut BTreeMap<PathBuf, String>) {
        for entry in fs::read_dir(dir).unwrap() {
            let path = entry.unwrap().path();
            if path.is_dir() {
                walk(base, &path, out);
            } else {
                let rel = path.strip_prefix(base).unwrap().to_path_buf();
                out.insert(rel, fs::read_to_string(&path).unwrap());
            }
        }
    }
    let mut out = BTreeMap::new();
    if dir.exists() {
        walk(dir, dir, &mut out);
    }
    out
}

fn assert_no_violations(violations: Vec<String>, fix: &str) {
    assert!(
        violations.is_empty(),
        "\n{}\n\nFix: {fix}\n",
        violations.join("\n")
    );
}

#[test]
fn published_registry_matches_sources() {
    let out = tempfile::tempdir().unwrap();
    build(&root().join("registry"), out.path()).unwrap();
    let expected = files_under(out.path());
    let actual = files_under(&root().join("public/r"));

    let mut violations = Vec::new();
    for (path, content) in &expected {
        match actual.get(path) {
            None => violations.push(format!("public/r/{} is missing", path.display())),
            Some(found) if found != content => {
                violations.push(format!("public/r/{} is stale", path.display()))
            }
            Some(_) => {}
        }
    }
    for path in actual.keys().filter(|p| !expected.contains_key(*p)) {
        violations.push(format!(
            "public/r/{} is not produced by the build",
            path.display()
        ));
    }
    assert_no_violations(
        violations,
        "run `cargo run -p oniko-cli -- build` (delete public/r first if files are left over)",
    );
}

/// The apps that install every component with `oniko add`.
const APPS: [&str; 2] = ["apps/preview", "apps/site"];

#[test]
fn apps_install_every_component_unmodified() {
    let mut violations = Vec::new();
    for app in APPS {
        let config = fs::read_to_string(root().join(app).join("oniko.toml")).unwrap();
        let style = config
            .lines()
            .find_map(|l| l.strip_prefix("style = \""))
            .and_then(|l| l.strip_suffix('"'))
            .unwrap_or_else(|| panic!("{app}/oniko.toml has no style"));
        let ui_dir = format!("{app}/src/components/ui");

        let mut expected_files = BTreeSet::from(["mod.rs".to_string()]);
        for item in registry().items.iter().filter(|i| is_installable(i)) {
            let path = root().join(format!("public/r/styles/{style}/{}.json", item.name));
            let compiled: RegistryItem =
                serde_json::from_str(&fs::read_to_string(&path).unwrap()).unwrap();
            for file in &compiled.files {
                let name = Path::new(&file.path)
                    .file_name()
                    .unwrap()
                    .to_string_lossy()
                    .to_string();
                match fs::read_to_string(root().join(&ui_dir).join(&name)) {
                    Err(_) => violations.push(format!("`{}` is not installed in {app}", item.name)),
                    Ok(installed) if Some(installed.as_str()) != file.content.as_deref() => {
                        violations.push(format!("{ui_dir}/{name} differs from the registry"))
                    }
                    Ok(_) => {}
                }
                expected_files.insert(name);
            }
        }
        for entry in fs::read_dir(root().join(&ui_dir)).unwrap() {
            let name = entry.unwrap().file_name().to_string_lossy().to_string();
            if !expected_files.contains(&name) {
                violations.push(format!("{ui_dir}/{name} is not a registry component"));
            }
        }
    }
    assert_no_violations(
        violations,
        "edit registry/ui/ or registry/blocks/, then `cargo run -p oniko-cli -- build` and \
         `cargo run -p oniko-cli -- add <name> --overwrite --cwd <app>` for apps/preview and apps/site",
    );
}

#[test]
fn components_do_not_use_web_only_crates() {
    let mut violations = Vec::new();
    for path in component_sources() {
        let source = fs::read_to_string(&path).unwrap();
        for (n, line) in source.lines().enumerate() {
            for name in WEB_ONLY_CRATES.iter().filter(|c| line.contains(*c)) {
                violations.push(format!("{}:{} uses `{name}`", path.display(), n + 1));
            }
        }
    }
    assert_no_violations(
        violations,
        "components must run on web and desktop: use MountedData or document::eval instead",
    );
}

#[test]
fn every_placeholder_is_defined_by_every_style() {
    let mut placeholders = BTreeSet::new();
    for path in component_sources() {
        let source = fs::read_to_string(&path).unwrap();
        let tokens = source.split(|c: char| !(c.is_ascii_alphanumeric() || c == '-'));
        placeholders.extend(
            tokens
                .filter(|t| t.starts_with(style_map::CN_PREFIX))
                .map(str::to_string),
        );
    }

    let mut violations = Vec::new();
    for entry in fs::read_dir(root().join("registry/styles")).unwrap() {
        let path = entry.unwrap().path();
        let map = style_map::parse(&fs::read_to_string(&path).unwrap()).unwrap();
        for class in placeholders.iter().filter(|c| map.get(c).is_none()) {
            let style = path.file_name().unwrap().to_string_lossy();
            violations.push(format!("`{class}` has no rule in registry/styles/{style}"));
        }
    }
    assert_no_violations(
        violations,
        "add a `.cn-...` rule to every style, or use an existing placeholder",
    );
}

#[test]
fn every_component_is_registered_and_documented() {
    let registry = registry();
    let registered: BTreeSet<PathBuf> = registry
        .items
        .iter()
        .flat_map(|i| {
            i.files
                .iter()
                .map(|f| root().join("registry").join(&f.path))
        })
        .collect();
    let catalog = fs::read_to_string(root().join("COMPONENTS.md")).unwrap();

    let mut violations = Vec::new();
    for path in component_sources() {
        if !registered.contains(&path) {
            violations.push(format!(
                "{} is not listed in registry/registry.json",
                path.display()
            ));
        }
    }
    for item in registry.items.iter().filter(|i| is_installable(i)) {
        let title = item.title.as_deref().unwrap_or(&item.name);
        if !catalog.contains(&format!("| {title} |")) {
            violations.push(format!("`{title}` has no row in COMPONENTS.md"));
        }
    }
    assert_no_violations(violations, "register the file and add a COMPONENTS.md row");
}
