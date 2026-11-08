//! AspectRatio: displays content within a desired ratio.
//!
//! Installed by oniko-cn. This file is yours: edit it freely.

use dioxus::prelude::*;
use oniko_cn::{cn, take_class};

#[component]
pub fn AspectRatio(
    /// Width divided by height, e.g. `16.0 / 9.0`.
    ratio: f64,
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(
        "relative aspect-(--ratio)",
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        div {
            "data-slot": "aspect-ratio",
            style: "--ratio: {ratio}",
            class: class,
            ..attributes,
            {children}
        }
    }
}
