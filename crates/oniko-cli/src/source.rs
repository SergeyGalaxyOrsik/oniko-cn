//! Reads registry JSON from an HTTP(S) URL or a local directory.

use std::fs;
use std::path::PathBuf;

use anyhow::{Context, Result};
use oniko_registry::{PublishedIndex, RegistryItem};
use serde::de::DeserializeOwned;

use crate::config::Config;

pub struct Source {
    base: Base,
    style: String,
}

enum Base {
    Url(String),
    Dir(PathBuf),
}

impl Source {
    pub fn new(config: &Config) -> Self {
        let registry = config.registry.trim_end_matches('/');
        let base = if is_url(registry) {
            Base::Url(registry.to_string())
        } else {
            Base::Dir(config.root.join(registry))
        };
        Self {
            base,
            style: config.style.clone(),
        }
    }

    pub fn index(&self) -> Result<PublishedIndex> {
        self.get("registry.json")
    }

    pub fn style(&self) -> Result<RegistryItem> {
        self.get(&format!("styles/{}/index.json", self.style))
    }

    pub fn theme(&self, name: &str) -> Result<RegistryItem> {
        self.get(&format!("themes/{name}.json"))
    }

    /// `name` is either an item of this registry or a full item URL.
    pub fn item(&self, name: &str) -> Result<RegistryItem> {
        if is_url(name) {
            return parse(name, &http_get(name)?);
        }
        self.get(&format!("styles/{}/{name}.json", self.style))
    }

    fn get<T: DeserializeOwned>(&self, rel: &str) -> Result<T> {
        match &self.base {
            Base::Url(base) => {
                let url = format!("{base}/{rel}");
                parse(&url, &http_get(&url)?)
            }
            Base::Dir(dir) => {
                let path = dir.join(rel);
                let text = fs::read_to_string(&path)
                    .with_context(|| format!("reading {}", path.display()))?;
                parse(&path.display().to_string(), &text)
            }
        }
    }
}

fn is_url(s: &str) -> bool {
    s.starts_with("http://") || s.starts_with("https://")
}

fn http_get(url: &str) -> Result<String> {
    ureq::get(url)
        .call()
        .with_context(|| format!("GET {url}"))?
        .body_mut()
        .read_to_string()
        .with_context(|| format!("reading body of {url}"))
}

fn parse<T: DeserializeOwned>(origin: &str, text: &str) -> Result<T> {
    serde_json::from_str(text).with_context(|| format!("parsing {origin}"))
}
