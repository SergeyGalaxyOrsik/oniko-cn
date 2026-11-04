//! Registry JSON schema. Field names follow shadcn's `registry.json` /
//! `registry-item.json` so the formats stay recognisable; `dependencies`
//! name Cargo crates (`name` or `name@version`) instead of npm packages.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

pub const ITEM_SCHEMA: &str = "https://oniko.dev/schema/registry-item.json";

/// `registry/registry.json`: the source list of items.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Registry {
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub homepage: Option<String>,
    pub items: Vec<RegistryItem>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ItemType {
    #[serde(rename = "registry:ui")]
    Ui,
    #[serde(rename = "registry:lib")]
    Lib,
    #[serde(rename = "registry:hook")]
    Hook,
    #[serde(rename = "registry:block")]
    Block,
    #[serde(rename = "registry:style")]
    Style,
    #[serde(rename = "registry:theme")]
    Theme,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RegistryItem {
    #[serde(rename = "$schema", default, skip_serializing_if = "Option::is_none")]
    pub schema: Option<String>,
    pub name: String,
    #[serde(rename = "type")]
    pub kind: ItemType,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Cargo crates the item needs, as `name` or `name@version`.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub dependencies: Vec<String>,
    /// Other registry items this item needs.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub registry_dependencies: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub files: Vec<RegistryFile>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub css_vars: Option<CssVars>,
    /// Raw CSS appended to the project's stylesheet (used by `registry:style`).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub css: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegistryFile {
    /// Path relative to the registry root, e.g. `ui/button.rs`.
    pub path: String,
    #[serde(rename = "type")]
    pub kind: ItemType,
    /// Compiled source; filled in by [`crate::build`].
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CssVars {
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub theme: BTreeMap<String, String>,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub light: BTreeMap<String, String>,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub dark: BTreeMap<String, String>,
}

/// Summary entry of the published `registry.json` index.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndexEntry {
    pub name: String,
    #[serde(rename = "type")]
    pub kind: ItemType,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// Published `registry.json`: what `oniko list` and `oniko init` read.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublishedIndex {
    pub name: String,
    pub styles: Vec<String>,
    pub themes: Vec<String>,
    pub items: Vec<IndexEntry>,
}

/// Splits `name@version` into its parts.
pub fn parse_dependency(spec: &str) -> (&str, Option<&str>) {
    match spec.split_once('@') {
        Some((name, version)) => (name, Some(version)),
        None => (spec, None),
    }
}
