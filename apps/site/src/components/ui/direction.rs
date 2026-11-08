//! Direction: a provider that sets the text direction (`ltr` or `rtl`) for the components below it.
//!
//! Installed by oniko-cn. This file is yours: edit it freely.

use dioxus::prelude::*;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Direction {
    #[default]
    Ltr,
    Rtl,
}

impl Direction {
    /// Value of the HTML `dir` attribute.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Ltr => "ltr",
            Self::Rtl => "rtl",
        }
    }
}

/// Provides `direction` to `use_direction` in the subtree. It renders no element, so set `dir` on
/// the document or a wrapper yourself.
#[component]
pub fn DirectionProvider(#[props(default)] direction: Direction, children: Element) -> Element {
    let mut current = use_context_provider(|| Signal::new(direction));
    if current() != direction {
        current.set(direction);
    }

    rsx! { {children} }
}

/// The direction set by the closest `DirectionProvider`, or `Direction::Ltr` without one.
pub fn use_direction() -> Direction {
    try_use_context::<Signal<Direction>>()
        .map(|direction| direction())
        .unwrap_or_default()
}
