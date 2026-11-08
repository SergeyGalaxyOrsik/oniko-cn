//! Kbd: displays textual keyboard input, such as a key or a shortcut.
//! KbdGroup: groups several keys into one shortcut.
//!
//! Installed by oniko-cn. This file is yours: edit it freely.

use dioxus::prelude::*;
use oniko_cn::{cn, take_class};

const KBD_BASE: &str = "bg-muted text-muted-foreground in-data-[slot=tooltip-content]:bg-background/20 in-data-[slot=tooltip-content]:text-background dark:in-data-[slot=tooltip-content]:bg-background/10 h-5 w-fit min-w-5 gap-1 rounded-xs px-1 font-sans text-[0.625rem] font-medium [&_svg:not([class*='size-'])]:size-3 pointer-events-none inline-flex items-center justify-center select-none";

const KBD_GROUP_BASE: &str = "gap-1 inline-flex items-center";

#[component]
pub fn Kbd(
    #[props(extends = GlobalAttributes, extends = kbd)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(KBD_BASE, take_class(&mut attributes).as_deref());

    rsx! {
        kbd {
            "data-slot": "kbd",
            class: class,
            ..attributes,
            {children}
        }
    }
}

#[component]
pub fn KbdGroup(
    #[props(extends = GlobalAttributes, extends = kbd)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(KBD_GROUP_BASE, take_class(&mut attributes).as_deref());

    rsx! {
        kbd {
            "data-slot": "kbd-group",
            class: class,
            ..attributes,
            {children}
        }
    }
}
