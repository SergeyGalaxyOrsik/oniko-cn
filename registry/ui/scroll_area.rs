//! ScrollArea: a scrollable region with a styled viewport.
//!
//! The viewport is a native scroll container, so it works the same in the browser and in the
//! desktop webview without JavaScript. Choose the scrolling axis with `orientation`.
//!
//! Installed by oniko-cn. This file is yours: edit it freely.

use dioxus::prelude::*;
use oniko_cn::{cn, take_class};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ScrollOrientation {
    #[default]
    Vertical,
    Horizontal,
    Both,
}

impl ScrollOrientation {
    pub fn class(self) -> &'static str {
        match self {
            Self::Vertical => "overflow-y-auto overflow-x-hidden",
            Self::Horizontal => "overflow-x-auto overflow-y-hidden",
            Self::Both => "overflow-auto",
        }
    }
}

const SCROLL_AREA_BASE: &str = "relative overflow-hidden";

const SCROLL_AREA_VIEWPORT_BASE: &str = "size-full rounded-[inherit] outline-none [scrollbar-width:thin] focus-visible:ring-[3px] focus-visible:outline-1";

#[component]
pub fn ScrollArea(
    #[props(default)] orientation: ScrollOrientation,
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(SCROLL_AREA_BASE, take_class(&mut attributes).as_deref());

    rsx! {
        div {
            "data-slot": "scroll-area",
            class: class,
            ..attributes,
            div {
                "data-slot": "scroll-area-viewport",
                class: cn!(SCROLL_AREA_VIEWPORT_BASE, orientation.class()),
                tabindex: "0",
                {children}
            }
        }
    }
}
