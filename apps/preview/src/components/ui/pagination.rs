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
        "gap-1 flex flex-row items-center",
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
    let mut attributes = attributes;
    let variant = if is_active {
        ButtonVariant::Outline
    } else {
        ButtonVariant::Ghost
    };
    let class = button_variants(variant, size, take_class(&mut attributes).as_deref());

    rsx! {
        a {
            "data-slot": "pagination-link",
            aria_current: if is_active { "page" },
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
pub fn PaginationPrevious(
    #[props(extends = GlobalAttributes, extends = a)] attributes: Vec<Attribute>,
    onclick: Option<EventHandler<MouseEvent>>,
) -> Element {
    let mut attributes = attributes;
    let extra = cn!("pl-2!", take_class(&mut attributes).as_deref());
    let class = button_variants(
        ButtonVariant::Ghost,
        ButtonSize::Default,
        Some(extra.as_str()),
    );

    rsx! {
        a {
            "data-slot": "pagination-previous",
            aria_label: "Go to previous page",
            class: class,
            onclick: move |event| {
                if let Some(handler) = onclick {
                    handler.call(event);
                }
            },
            ..attributes,
            ChevronIcon { d: "m15 18-6-6 6-6" }
            span { class: "hidden sm:block", "Previous" }
        }
    }
}

#[component]
pub fn PaginationNext(
    #[props(extends = GlobalAttributes, extends = a)] attributes: Vec<Attribute>,
    onclick: Option<EventHandler<MouseEvent>>,
) -> Element {
    let mut attributes = attributes;
    let extra = cn!("pr-2!", take_class(&mut attributes).as_deref());
    let class = button_variants(
        ButtonVariant::Ghost,
        ButtonSize::Default,
        Some(extra.as_str()),
    );

    rsx! {
        a {
            "data-slot": "pagination-next",
            aria_label: "Go to next page",
            class: class,
            onclick: move |event| {
                if let Some(handler) = onclick {
                    handler.call(event);
                }
            },
            ..attributes,
            span { class: "hidden sm:block", "Next" }
            ChevronIcon { d: "m9 18 6-6-6-6" }
        }
    }
}

#[component]
pub fn PaginationEllipsis(
    #[props(extends = GlobalAttributes, extends = span)] attributes: Vec<Attribute>,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(
        "size-9 items-center justify-center [&_svg:not([class*='size-'])]:size-4 flex",
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        span {
            "data-slot": "pagination-ellipsis",
            aria_hidden: "true",
            class: class,
            ..attributes,
            svg {
                class: "pointer-events-none",
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

#[component]
fn ChevronIcon(d: &'static str) -> Element {
    rsx! {
        svg {
            class: "pointer-events-none size-4 shrink-0",
            view_box: "0 0 24 24",
            fill: "none",
            stroke: "currentColor",
            stroke_width: "2",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            path { d: d }
        }
    }
}
