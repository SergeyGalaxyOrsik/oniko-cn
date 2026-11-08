//! Spinner: a loading indicator, an icon that spins while work is pending.
//!
//! Installed by oniko-cn. This file is yours: edit it freely.

use dioxus::prelude::*;
use oniko_cn::{cn, take_class};

const SPINNER_BASE: &str = "size-4 animate-spin";

#[component]
pub fn Spinner(
    #[props(extends = GlobalAttributes, extends = svg)] attributes: Vec<Attribute>,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(SPINNER_BASE, take_class(&mut attributes).as_deref());

    rsx! {
        svg {
            "data-slot": "spinner",
            class: class,
            role: "status",
            "aria-label": "Loading",
            xmlns: "http://www.w3.org/2000/svg",
            view_box: "0 0 24 24",
            fill: "none",
            stroke: "currentColor",
            stroke_width: "2",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            ..attributes,
            path { d: "M21 12a9 9 0 1 1-6.219-8.56" }
        }
    }
}
