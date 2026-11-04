//! Class-name helpers for oniko-cn components.
//!
//! [`cn!`] is the Rust counterpart of shadcn's `cn()` (`twMerge(clsx(...))`):
//! it joins class fragments, skips empty / `None` parts and resolves Tailwind
//! conflicts so that later fragments win (`cn!("h-9 px-2", "h-12")` gives
//! `"px-2 h-12"`).

use tailwind_fuse::merge::tw_merge_slice;

/// Anything that can contribute a class fragment to [`cn!`].
pub trait ClassPart {
    fn class_part(&self) -> Option<&str>;
}

impl ClassPart for str {
    fn class_part(&self) -> Option<&str> {
        Some(self)
    }
}

impl ClassPart for String {
    fn class_part(&self) -> Option<&str> {
        Some(self.as_str())
    }
}

impl<T: ClassPart + ?Sized> ClassPart for &T {
    fn class_part(&self) -> Option<&str> {
        (**self).class_part()
    }
}

impl<T: ClassPart> ClassPart for Option<T> {
    fn class_part(&self) -> Option<&str> {
        self.as_ref().and_then(ClassPart::class_part)
    }
}

/// Merges class fragments; later fragments win on Tailwind conflicts.
pub fn merge(parts: &[Option<&str>]) -> String {
    let parts: Vec<&str> = parts
        .iter()
        .flatten()
        .copied()
        .filter(|p| !p.trim().is_empty())
        .collect();
    tw_merge_slice(&parts)
}

/// `cn!(a, b, ...)`: join and merge class fragments (`&str`, `String`,
/// `Option<...>` or references to them).
#[macro_export]
macro_rules! cn {
    ($($part:expr),* $(,)?) => {
        $crate::merge(&[$($crate::ClassPart::class_part(&$part)),*])
    };
}

#[cfg(feature = "dioxus")]
mod attrs {
    use dioxus_core::{Attribute, AttributeValue};

    /// Removes every plain `class` attribute from a spread attribute list and
    /// returns their joined value, so a component can feed the caller's
    /// classes through [`cn!`](crate::cn) instead of letting them be
    /// concatenated blindly.
    pub fn take_class(attributes: &mut Vec<Attribute>) -> Option<String> {
        let mut classes: Vec<String> = Vec::new();
        attributes.retain(|attr| {
            if attr.name != "class" || attr.namespace.is_some() {
                return true;
            }
            if let AttributeValue::Text(text) = &attr.value {
                classes.push(text.clone());
            }
            false
        });
        (!classes.is_empty()).then(|| classes.join(" "))
    }
}

#[cfg(feature = "dioxus")]
pub use attrs::take_class;

#[cfg(test)]
mod tests {
    #[test]
    fn later_fragment_wins() {
        assert_eq!(cn!("h-9 px-2.5", "h-12"), "px-2.5 h-12");
        assert_eq!(
            cn!("bg-primary hover:bg-primary/80", "bg-red-500"),
            "hover:bg-primary/80 bg-red-500"
        );
    }

    #[test]
    fn skips_empty_and_none() {
        let none: Option<String> = None;
        assert_eq!(cn!("a", none, "", Some("b")), "a b");
    }

    #[test]
    fn font_size_and_text_color_do_not_collide() {
        let out = cn!("text-sm text-primary-foreground");
        assert!(
            out.contains("text-sm") && out.contains("text-primary-foreground"),
            "{out}"
        );
    }

    #[test]
    fn tailwind_v4_classes_survive() {
        // Real class string of the vega button (default variant, default size).
        let input = "group/button inline-flex shrink-0 items-center rounded-md border border-transparent bg-clip-padding text-sm font-medium focus-visible:ring-3 focus-visible:ring-ring/50 aria-invalid:ring-3 active:not-aria-[haspopup]:translate-y-px [&_svg:not([class*='size-'])]:size-4 bg-primary text-primary-foreground hover:bg-primary/80 h-9 gap-1.5 px-2.5 in-data-[slot=button-group]:rounded-md has-data-[icon=inline-end]:pr-2 has-data-[icon=inline-start]:pl-2";
        let out = cn!(input);
        for class in input.split_whitespace() {
            assert!(
                out.split_whitespace().any(|c| c == class),
                "lost `{class}` in `{out}`"
            );
        }
    }

    #[test]
    fn ring_width_vs_ring_color() {
        let out = cn!("focus-visible:ring-3 focus-visible:ring-ring/50");
        assert_eq!(out, "focus-visible:ring-3 focus-visible:ring-ring/50");
    }

    #[test]
    fn caller_overrides_size() {
        let out = cn!("size-9 rounded-md", "size-12");
        assert_eq!(out, "rounded-md size-12");
    }
}
