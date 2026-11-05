//! Sources: a collapsible list of the sources an AI answer was based on, with a link for each source.
//!
//! `Sources` holds the open state, `SourcesTrigger` shows the source count and toggles the list,
//! and `SourcesContent` holds the `Source` links. Each `Source` opens its link in a new tab.
//!
//! Installed by oniko-cn. This file is yours: edit it freely.

use dioxus::prelude::*;
use oniko_cn::{cn, take_class};

use super::collapsible::{Collapsible, CollapsibleContent, CollapsibleTrigger};

/// Root of the sources list. Closed by default; the trigger opens and closes it.
#[component]
pub fn Sources(
    /// Controlled open state. When set, `on_open_change` is the only way to change it.
    open: Option<bool>,
    /// Open state at first render when `open` is not controlled.
    #[props(default)]
    default_open: bool,
    on_open_change: Option<EventHandler<bool>>,
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(
        "mb-4 text-xs text-primary",
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        div {
            "data-slot": "sources",
            class: class,
            ..attributes,
            Collapsible {
                open: open,
                default_open: default_open,
                on_open_change: on_open_change,
                {children}
            }
        }
    }
}

/// The toggle that shows the source count. Defaults to "Used {count} sources".
#[component]
pub fn SourcesTrigger(
    #[props(default)] count: usize,
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Option<Element>,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(take_class(&mut attributes).as_deref());
    let label = children.unwrap_or_else(|| rsx! { "Used {count} sources" });

    rsx! {
        div {
            "data-slot": "sources-trigger",
            class: class,
            ..attributes,
            CollapsibleTrigger {
                class: "group flex items-center gap-2 text-muted-foreground transition-colors hover:text-foreground",
                span { "data-slot": "sources-trigger-label", {label} }
                svg {
                    "data-slot": "sources-trigger-chevron",
                    class: "size-4 shrink-0 transition-transform group-data-[state=open]:rotate-180",
                    "aria-hidden": "true",
                    view_box: "0 0 24 24",
                    fill: "none",
                    stroke: "currentColor",
                    stroke_width: "2",
                    stroke_linecap: "round",
                    stroke_linejoin: "round",
                    path { d: "m6 9 6 6 6-6" }
                }
            }
        }
    }
}

/// The list of `Source` links. Shown only while `Sources` is open.
#[component]
pub fn SourcesContent(
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(take_class(&mut attributes).as_deref());

    rsx! {
        div {
            "data-slot": "sources-content",
            class: class,
            ..attributes,
            CollapsibleContent {
                class: "mt-3 flex w-fit flex-col gap-2",
                {children}
            }
        }
    }
}

/// One source link. Shows a book icon and `title` unless `children` are given.
#[component]
pub fn Source(
    href: String,
    title: Option<String>,
    #[props(extends = GlobalAttributes, extends = a)] attributes: Vec<Attribute>,
    children: Option<Element>,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(
        "flex items-center gap-2 transition-colors hover:text-foreground",
        take_class(&mut attributes).as_deref()
    );
    let content = children.unwrap_or_else(|| {
        rsx! {
            svg {
                "data-slot": "source-icon",
                class: "size-4 shrink-0",
                "aria-hidden": "true",
                view_box: "0 0 24 24",
                fill: "none",
                stroke: "currentColor",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
                path { d: "M12 7v14" }
                path { d: "M3 18a1 1 0 0 1-1-1V4a1 1 0 0 1 1-1h5a4 4 0 0 1 4 4 4 4 0 0 1 4-4h5a1 1 0 0 1 1 1v13a1 1 0 0 1-1 1h-6a3 3 0 0 0-3 3 3 3 0 0 0-3-3z" }
            }
            span { "data-slot": "source-title", class: "block w-40 truncate", {title.unwrap_or_default()} }
        }
    });

    rsx! {
        a {
            "data-slot": "source",
            class: class,
            href: href,
            target: "_blank",
            rel: "noreferrer",
            ..attributes,
            {content}
        }
    }
}
