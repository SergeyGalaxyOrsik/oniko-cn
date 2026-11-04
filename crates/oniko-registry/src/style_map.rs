//! Compiles a style sheet (`style-<name>.css`) into a map from `cn-*`
//! placeholder classes to Tailwind classes.
//!
//! Style sheets follow a deliberately small format:
//!
//! ```css
//! .style-vega {
//!   .cn-button { @apply rounded-md text-sm; }
//! }
//! ```
//!
//! Only rules whose selector is a single bare `cn-*` class are compiled; any
//! other rule is reported in [`StyleMap::skipped`] so nothing is dropped
//! silently.

use std::collections::BTreeMap;

use anyhow::{Result, bail};

pub const CN_PREFIX: &str = "cn-";

#[derive(Debug, Default)]
pub struct StyleMap {
    pub classes: BTreeMap<String, String>,
    /// Selectors that were not compiled because they are not a bare `cn-*` class.
    pub skipped: Vec<String>,
}

impl StyleMap {
    pub fn get(&self, class: &str) -> Option<&str> {
        self.classes.get(class).map(String::as_str)
    }
}

pub fn parse(css: &str) -> Result<StyleMap> {
    let css = strip_comments(css);
    let mut map = StyleMap::default();
    // Stack of open blocks: selector text + collected @apply classes.
    let mut stack: Vec<(String, Vec<String>)> = Vec::new();
    let mut buf = String::new();

    for ch in css.chars() {
        match ch {
            '{' => {
                stack.push((buf.trim().to_string(), Vec::new()));
                buf.clear();
            }
            ';' => {
                let stmt = buf.trim();
                if let Some(classes) = stmt.strip_prefix("@apply") {
                    match stack.last_mut() {
                        Some((_, applied)) => applied.push(classes.trim().to_string()),
                        None => bail!("`@apply` outside of a rule: {stmt}"),
                    }
                }
                buf.clear();
            }
            '}' => {
                let Some((selector, applied)) = stack.pop() else {
                    bail!("unbalanced `}}` in style sheet");
                };
                buf.clear();
                if applied.is_empty() {
                    continue;
                }
                let classes = applied.join(" ");
                match bare_cn_class(&selector) {
                    Some(name) => {
                        map.classes
                            .entry(name.to_string())
                            .and_modify(|existing| {
                                existing.push(' ');
                                existing.push_str(&classes);
                            })
                            .or_insert(classes);
                    }
                    None => map.skipped.push(selector),
                }
            }
            _ => buf.push(ch),
        }
    }
    if !stack.is_empty() {
        bail!("unclosed `{{` in style sheet");
    }
    Ok(map)
}

/// `.cn-button` -> `Some("cn-button")`; anything more complex -> `None`.
fn bare_cn_class(selector: &str) -> Option<&str> {
    let name = selector.strip_prefix('.')?;
    let valid =
        name.starts_with(CN_PREFIX) && name.chars().all(|c| c.is_ascii_alphanumeric() || c == '-');
    valid.then_some(name)
}

fn strip_comments(css: &str) -> String {
    let mut out = String::with_capacity(css.len());
    let mut rest = css;
    while let Some(start) = rest.find("/*") {
        out.push_str(&rest[..start]);
        rest = match rest[start + 2..].find("*/") {
            Some(end) => &rest[start + 2 + end + 2..],
            None => "",
        };
    }
    out.push_str(rest);
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_nested_rules() {
        let css = r#"
            .style-x {
              /* MARK: Button */
              .cn-button { @apply rounded-md text-sm; }
              .cn-button-variant-default {
                @apply bg-primary hover:bg-[color-mix(in_oklch,var(--secondary),var(--foreground)_5%)];
              }
              .cn-card:has(> [data-slot=x]) { @apply p-0; }
            }
        "#;
        let map = parse(css).unwrap();
        assert_eq!(map.get("cn-button"), Some("rounded-md text-sm"));
        assert_eq!(
            map.get("cn-button-variant-default"),
            Some("bg-primary hover:bg-[color-mix(in_oklch,var(--secondary),var(--foreground)_5%)]")
        );
        assert_eq!(map.skipped, vec![".cn-card:has(> [data-slot=x])"]);
    }

    #[test]
    fn compiles_every_shipped_style() {
        let dir = concat!(env!("CARGO_MANIFEST_DIR"), "/../../registry/styles");
        let mut count = 0;
        for entry in std::fs::read_dir(dir).unwrap() {
            let path = entry.unwrap().path();
            let map = parse(&std::fs::read_to_string(&path).unwrap()).unwrap();
            assert!(map.classes.len() > 300, "{path:?}: {}", map.classes.len());
            assert!(map.get("cn-button").is_some(), "{path:?}");
            count += 1;
        }
        assert_eq!(count, 8);
    }
}
