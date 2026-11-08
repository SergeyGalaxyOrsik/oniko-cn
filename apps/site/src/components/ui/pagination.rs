//! Pagination: a list of page links with previous and next controls and an ellipsis for skipped pages.
//!
//! Installed by oniko-cn. This file is yours: edit it freely.

use dioxus::prelude::*;
use oniko_cn::{cn, take_class};

use super::button::{ButtonSize, ButtonVariant, button_variants};

#[component]
pub fn Pagination(
    #[props(extends = GlobalAttributes, extends = nav)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(
        "mx-auto flex w-full justify-center",
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        nav {
            "data-slot": "pagination",
            role: "navigation",
            aria_label: "pagination",
            class: class,
            ..attributes,
            {children}
        }
    }
}

#[component]
pub fn PaginationContent(
    #[props(extends = GlobalAttributes, extends = ul)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(
        "gap-0.5 flex items-center",
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        ul {
            "data-slot": "pagination-content",
            class: class,
            ..attributes,
            {children}
        }
    }
}

#[component]
pub fn PaginationItem(
    #[props(extends = GlobalAttributes, extends = li)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(take_class(&mut attributes).as_deref());

    rsx! {
        li {
            "data-slot": "pagination-item",
            class: class,
            ..attributes,
            {children}
        }
    }
}

/// A page link. The current page is marked with `is_active` and gets the outline look.
#[component]
pub fn PaginationLink(
    #[props(default)] is_active: bool,
    #[props(default = ButtonSize::Icon)] size: ButtonSize,
    #[props(extends = GlobalAttributes, extends = a)] attributes: Vec<Attribute>,
    onclick: Option<EventHandler<MouseEvent>>,
    children: Element,
) -> Element {
    link(is_active, size, None, None, attributes, onclick, children)
}

#[component]
pub fn PaginationPrevious(
    #[props(default)] is_active: bool,
    #[props(into, default = "Previous".to_string())] text: String,
    #[props(extends = GlobalAttributes, extends = a)] attributes: Vec<Attribute>,
    onclick: Option<EventHandler<MouseEvent>>,
) -> Element {
    link(
        is_active,
        ButtonSize::Default,
        Some("pl-2!"),
        Some("Go to previous page"),
        attributes,
        onclick,
        rsx! {
            svg {
                "data-icon": "inline-start",
                class: "rtl:rotate-180",
                view_box: "0 0 24 24",
                fill: "none",
                stroke: "currentColor",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
                path { d: "m15 18-6-6 6-6" }
            }
            span { class: "hidden sm:block", "{text}" }
        },
    )
}

#[component]
pub fn PaginationNext(
    #[props(default)] is_active: bool,
    #[props(into, default = "Next".to_string())] text: String,
    #[props(extends = GlobalAttributes, extends = a)] attributes: Vec<Attribute>,
    onclick: Option<EventHandler<MouseEvent>>,
) -> Element {
    link(
        is_active,
        ButtonSize::Default,
        Some("pr-2!"),
        Some("Go to next page"),
        attributes,
        onclick,
        rsx! {
            span { class: "hidden sm:block", "{text}" }
            svg {
                "data-icon": "inline-end",
                class: "rtl:rotate-180",
                view_box: "0 0 24 24",
                fill: "none",
                stroke: "currentColor",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
                path { d: "m9 18 6-6-6-6" }
            }
        },
    )
}

fn link(
    is_active: bool,
    size: ButtonSize,
    part_class: Option<&str>,
    label: Option<&'static str>,
    mut attributes: Vec<Attribute>,
    onclick: Option<EventHandler<MouseEvent>>,
    children: Element,
) -> Element {
    let variant = if is_active {
        ButtonVariant::Outline
    } else {
        ButtonVariant::Ghost
    };
    let extra = cn!("", part_class, take_class(&mut attributes).as_deref());
    let class = button_variants(variant, size, Some(extra.as_str()));

    rsx! {
        a {
            "data-slot": "pagination-link",
            "data-active": is_active.then_some("true"),
            aria_current: if is_active { "page" },
            aria_label: label,
            class: class,
            onclick: move |event| {
                if let Some(handler) = onclick {
                    handler.call(event);
                }
            },
            ..attributes,
            {children}
        }
    }
}

#[component]
pub fn PaginationEllipsis(
    #[props(extends = GlobalAttributes, extends = span)] attributes: Vec<Attribute>,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(
        "size-7 items-center justify-center [&_svg:not([class*='size-'])]:size-3.5 flex",
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        span {
            "data-slot": "pagination-ellipsis",
            aria_hidden: "true",
            class: class,
            ..attributes,
            svg {
                view_box: "0 0 24 24",
                fill: "none",
                stroke: "currentColor",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
                circle { cx: "12", cy: "12", r: "1" }
                circle { cx: "19", cy: "12", r: "1" }
                circle { cx: "5", cy: "12", r: "1" }
            }
            span { class: "sr-only", "More pages" }
        }
    }
}
