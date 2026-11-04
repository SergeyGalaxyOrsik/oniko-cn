//! `oniko`: the oniko-cn command-line tool.

mod config;
mod project;
mod source;

use std::collections::{BTreeSet, VecDeque};
use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result, bail};
use clap::{Parser, Subcommand};
use oniko_registry::{ItemType, RegistryItem, build, theme};
use toml_edit::{InlineTable, Item, Value};

use config::Config;
use project::WriteOutcome;
use source::Source;

#[derive(Parser)]
#[command(
    name = "oniko",
    version,
    about = "Add oniko-cn components to your Dioxus app."
)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Set up oniko-cn in a Dioxus crate: oniko.toml, theme CSS, modules.
    Init {
        /// Registry URL or local path (relative to the crate).
        #[arg(long)]
        registry: String,
        #[arg(long, default_value = "vega")]
        style: String,
        #[arg(long, default_value = "neutral")]
        base_color: String,
        /// Use a local checkout of the oniko-cn crate instead of crates.io.
        #[arg(long)]
        cn_path: Option<PathBuf>,
        #[arg(long)]
        force: bool,
        #[arg(long, default_value = ".")]
        cwd: PathBuf,
    },
    /// Copy components (and their dependencies) into the crate.
    Add {
        #[arg(required = true)]
        components: Vec<String>,
        /// Replace files that differ from the registry version.
        #[arg(long)]
        overwrite: bool,
        #[arg(long, default_value = ".")]
        cwd: PathBuf,
    },
    /// List the components available in the configured registry.
    List {
        #[arg(long, default_value = ".")]
        cwd: PathBuf,
    },
    /// Compile a registry source directory into publishable JSON.
    Build {
        #[arg(long, default_value = "registry")]
        registry: PathBuf,
        #[arg(long, default_value = "public/r")]
        out: PathBuf,
    },
}

fn main() {
    if let Err(err) = run(Cli::parse()) {
        eprintln!("error: {err:#}");
        std::process::exit(1);
    }
}

fn run(cli: Cli) -> Result<()> {
    match cli.command {
        Command::Init {
            registry,
            style,
            base_color,
            cn_path,
            force,
            cwd,
        } => init(&cwd, &registry, &style, &base_color, cn_path, force),
        Command::Add {
            components,
            overwrite,
            cwd,
        } => add(&cwd, &components, overwrite),
        Command::List { cwd } => list(&cwd),
        Command::Build { registry, out } => {
            let report = build::build(&registry, &out)?;
            for (style, selector) in &report.skipped_selectors {
                println!(
                    "note: style `{style}`: selector `{selector}` is not a bare cn-* class, skipped"
                );
            }
            println!(
                "Built {} item(s) x {} style(s) and {} theme(s) into {}",
                report.items,
                report.styles.len(),
                report.themes.len(),
                out.display()
            );
            Ok(())
        }
    }
}

fn init(
    root: &Path,
    registry: &str,
    style: &str,
    base_color: &str,
    cn_path: Option<PathBuf>,
    force: bool,
) -> Result<()> {
    if !root.join("Cargo.toml").exists() {
        bail!(
            "no Cargo.toml in {}: run init inside your Dioxus crate",
            root.display()
        );
    }
    if !force {
        Config::ensure_absent(root)?;
    }
    let mut config = Config::new(root, registry, style, base_color);
    if let Some(path) = cn_path {
        let mut spec = InlineTable::new();
        spec.insert("path", Value::from(path.to_string_lossy().as_ref()));
        config.dependency_overrides["oniko-cn"] = Item::Value(Value::InlineTable(spec));
    }

    let source = Source::new(&config);
    let index = source.index()?;
    if !index.styles.iter().any(|s| s == style) {
        bail!(
            "unknown style `{style}`; available: {}",
            index.styles.join(", ")
        );
    }
    if !index.themes.iter().any(|t| t == base_color) {
        bail!(
            "unknown base color `{base_color}`; available: {}",
            index.themes.join(", ")
        );
    }
    let style_item = source.style()?;
    let theme_item = source.theme(base_color)?;
    let vars = theme_item.css_vars.context("theme has no cssVars")?;

    config.save()?;
    let css_path = root.join(&config.css);
    let existing = fs::read_to_string(&css_path).unwrap_or_default();
    let block = theme::render_block(style_item.css.as_deref().unwrap_or_default(), &vars);
    fs::write(&css_path, theme::upsert_block(&existing, &block))
        .with_context(|| format!("writing {}", css_path.display()))?;
    project::ensure_module_dir(root, &config.ui)?;
    let added =
        project::add_dependencies(root, &style_item.dependencies, &config.dependency_overrides)?;

    println!(
        "Initialized oniko-cn ({style}, {base_color}) in {}",
        root.display()
    );
    println!("  wrote {} and {}", config::FILE_NAME, config.css.display());
    if !added.is_empty() {
        println!("  added crates: {}", added.join(", "));
    }
    println!("Load the compiled stylesheet in your root component:");
    println!("  document::Stylesheet {{ href: asset!(\"/assets/tailwind.css\") }}");
    Ok(())
}

fn add(root: &Path, names: &[String], overwrite: bool) -> Result<()> {
    let config = Config::load(root)?;
    let source = Source::new(&config);
    let items = resolve(&source, names)?;

    let mut crates: Vec<String> = Vec::new();
    for item in &items {
        for file in &item.files {
            let target = target_path(&config, item, &file.path)?;
            let content = file
                .content
                .as_deref()
                .context("registry file has no content")?;
            let outcome = project::write_file(&root.join(&target), content, overwrite)?;
            project::declare_file_module(root, &target)?;
            let status = match outcome {
                WriteOutcome::Created => "created",
                WriteOutcome::Overwritten => "overwritten",
                WriteOutcome::Unchanged => "up to date",
                WriteOutcome::SkippedExisting => "exists, differs (use --overwrite)",
            };
            println!("  {:<40} {status}", target.display());
        }
        crates.extend(item.dependencies.iter().cloned());
    }
    let added = project::add_dependencies(root, &crates, &config.dependency_overrides)?;
    if !added.is_empty() {
        println!("  added crates: {}", added.join(", "));
    }
    Ok(())
}

/// Fetches the requested items and their registry dependencies, dependencies first.
fn resolve(source: &Source, names: &[String]) -> Result<Vec<RegistryItem>> {
    let mut queue: VecDeque<String> = names.iter().cloned().collect();
    let mut seen = BTreeSet::new();
    let mut items = Vec::new();
    while let Some(name) = queue.pop_front() {
        if !seen.insert(name.clone()) {
            continue;
        }
        let item = source
            .item(&name)
            .with_context(|| format!("component `{name}`"))?;
        queue.extend(item.registry_dependencies.iter().cloned());
        items.push(item);
    }
    items.reverse();
    Ok(items)
}

fn target_path(config: &Config, item: &RegistryItem, file: &str) -> Result<PathBuf> {
    let name = Path::new(file)
        .file_name()
        .context("registry file without a name")?;
    let dir = match item.kind {
        ItemType::Ui | ItemType::Block => &config.ui,
        ItemType::Hook => &config.hooks,
        ItemType::Lib => &config.lib,
        ItemType::Style | ItemType::Theme => bail!("`{}` is not installable with add", item.name),
    };
    Ok(dir.join(name))
}

fn list(root: &Path) -> Result<()> {
    let config = Config::load(root)?;
    let index = Source::new(&config).index()?;
    println!(
        "{} (style: {}, styles: {})",
        index.name,
        config.style,
        index.styles.join(", ")
    );
    for item in index.items {
        println!(
            "  {:<20} {}",
            item.name,
            item.description.unwrap_or_default()
        );
    }
    Ok(())
}
