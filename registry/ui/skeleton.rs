//! Skeleton: a pulsing placeholder that stands in for content while it loads.
//!
//! Installed by oniko-cn. This file is yours: edit it freely.

use dioxus::prelude::*;
use oniko_cn::{cn, take_class};

const SKELETON_BASE: &str = "cn-skeleton animate-pulse motion-reduce:animate-none";

#[component]
pub fn Skeleton(
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(SKELETON_BASE, take_class(&mut attributes).as_deref());

    rsx! {
        div {
            "data-slot": "skeleton",
            class: class,
            ..attributes,
        }
    }
}
