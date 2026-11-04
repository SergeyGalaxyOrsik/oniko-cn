//! Inlines a [`StyleMap`] into Rust source code.
//!
//! Every string literal that contains `cn-*` placeholder classes is rewritten:
//! each placeholder is replaced by the Tailwind classes its style maps it to
//! (placed first, so the literal's own classes still win in `cn!`), and
//! unmapped placeholders are removed. Comments, char literals and lifetimes
//! are skipped by a small lexer so only real string literals are touched.

use crate::style_map::{CN_PREFIX, StyleMap};

/// Placeholders kept verbatim: they are CSS hooks, not style slots.
const ALLOWLIST: &[&str] = &[];

pub fn apply_style_map(source: &str, map: &StyleMap) -> String {
    let bytes = source.as_bytes();
    let mut out = String::with_capacity(source.len());
    let mut i = 0;
    let mut copied = 0;

    while i < bytes.len() {
        match bytes[i] {
            b'/' if bytes.get(i + 1) == Some(&b'/') => {
                i = source[i..].find('\n').map_or(bytes.len(), |n| i + n);
            }
            b'/' if bytes.get(i + 1) == Some(&b'*') => {
                i = skip_block_comment(bytes, i);
            }
            b'\'' => i = skip_char_or_lifetime(bytes, i),
            b'r' if is_raw_string_start(bytes, i) => {
                let hashes = bytes[i + 1..].iter().take_while(|&&b| b == b'#').count();
                let start = i + 1 + hashes + 1;
                let terminator = format!("\"{}", "#".repeat(hashes));
                let end = source[start..]
                    .find(&terminator)
                    .map_or(bytes.len(), |n| start + n);
                let literal = &source[start..end];
                if literal.contains(CN_PREFIX) {
                    out.push_str(&source[copied..start]);
                    out.push_str(&rewrite_classes(literal, map));
                    copied = end;
                }
                i = (end + terminator.len()).min(bytes.len());
            }
            b'"' => {
                let start = i + 1;
                let end = find_string_end(bytes, start);
                let literal = &source[start..end];
                if literal.contains(CN_PREFIX) && !literal.contains('\\') {
                    out.push_str(&source[copied..start]);
                    out.push_str(&escape(&rewrite_classes(literal, map)));
                    copied = end;
                }
                i = (end + 1).min(bytes.len());
            }
            _ => i += 1,
        }
    }
    out.push_str(&source[copied..]);
    out
}

fn rewrite_classes(literal: &str, map: &StyleMap) -> String {
    let mut styled: Vec<&str> = Vec::new();
    let mut own: Vec<&str> = Vec::new();
    for class in literal.split_whitespace() {
        if !class.starts_with(CN_PREFIX) || ALLOWLIST.contains(&class) {
            own.push(class);
        } else if let Some(mapped) = map.get(class) {
            styled.extend(mapped.split_whitespace());
        }
    }
    let mut seen = std::collections::HashSet::new();
    styled
        .into_iter()
        .chain(own)
        .filter(|class| seen.insert(*class))
        .collect::<Vec<_>>()
        .join(" ")
}

fn escape(s: &str) -> String {
    s.replace('\\', "\\\\").replace('"', "\\\"")
}

fn find_string_end(bytes: &[u8], mut i: usize) -> usize {
    while i < bytes.len() {
        match bytes[i] {
            b'\\' => i += 2,
            b'"' => return i,
            _ => i += 1,
        }
    }
    bytes.len()
}

fn skip_block_comment(bytes: &[u8], mut i: usize) -> usize {
    let mut depth = 0;
    while i < bytes.len() {
        if bytes[i] == b'/' && bytes.get(i + 1) == Some(&b'*') {
            depth += 1;
            i += 2;
        } else if bytes[i] == b'*' && bytes.get(i + 1) == Some(&b'/') {
            depth -= 1;
            i += 2;
            if depth == 0 {
                return i;
            }
        } else {
            i += 1;
        }
    }
    i
}

/// `'x'`, `'\n'`, `'\u{1F600}'` are char literals; `'a` is a lifetime.
fn skip_char_or_lifetime(bytes: &[u8], i: usize) -> usize {
    if bytes.get(i + 1) == Some(&b'\\') {
        let mut j = i + 2;
        while j < bytes.len() && bytes[j] != b'\'' {
            j += 1;
        }
        return j + 1;
    }
    // A (possibly multi-byte) char followed by a closing quote.
    let rest = std::str::from_utf8(&bytes[i + 1..]).unwrap_or("");
    if let Some(c) = rest.chars().next()
        && rest[c.len_utf8()..].starts_with('\'')
    {
        return i + 1 + c.len_utf8() + 1;
    }
    i + 1
}

fn is_raw_string_start(bytes: &[u8], i: usize) -> bool {
    let prev_is_ident = i > 0 && (bytes[i - 1].is_ascii_alphanumeric() || bytes[i - 1] == b'_');
    if prev_is_ident {
        return false;
    }
    let hashes = bytes[i + 1..].iter().take_while(|&&b| b == b'#').count();
    bytes.get(i + 1 + hashes) == Some(&b'"')
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::style_map;

    fn map() -> StyleMap {
        style_map::parse(
            ".s { .cn-button { @apply rounded-md text-sm; } .cn-button-variant-default { @apply bg-primary [&_svg:not([class*='size-'])]:size-4; } }",
        )
        .unwrap()
    }

    #[test]
    fn inlines_and_removes_placeholders() {
        let src = r#"const BASE: &str = "cn-button inline-flex cn-unknown";"#;
        assert_eq!(
            apply_style_map(src, &map()),
            r#"const BASE: &str = "rounded-md text-sm inline-flex";"#
        );
    }

    #[test]
    fn leaves_comments_chars_and_lifetimes_alone() {
        let src = r#"
// "cn-button" in a comment
/* "cn-button" */
fn f<'a>(x: &'a str) -> char { let _ = "cn-button-variant-default"; '"' }
"#;
        let out = apply_style_map(src, &map());
        assert!(out.contains(r#"// "cn-button" in a comment"#));
        assert!(out.contains(r#"/* "cn-button" */"#));
        assert!(out.contains(r#"let _ = "bg-primary [&_svg:not([class*='size-'])]:size-4";"#));
        assert!(out.contains(r#"'"' }"#));
    }

    #[test]
    fn handles_raw_strings() {
        let src = r##"let s = r#"cn-button "quoted""#;"##;
        assert_eq!(
            apply_style_map(src, &map()),
            r##"let s = r#"rounded-md text-sm "quoted""#;"##
        );
    }
}
