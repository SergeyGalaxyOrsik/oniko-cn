//! Breadcrumb: displays the path to the current resource using a hierarchy of links.
//!
//! Installed by oniko-cn. This file is yours: edit it freely.

use dioxus::prelude::*;
use oniko_cn::{cn, take_class};

#[component]
pub fn Breadcrumb(
    #[props(extends = GlobalAttributes, extends = nav)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(take_class(&mut attributes).as_deref());

    rsx! {
        nav {
            "data-slot": "breadcrumb",
            aria_label: "breadcrumb",
            class: class,
            ..attributes,
            {children}
        }
    }
}

#[component]
pub fn BreadcrumbList(
    #[props(extends = GlobalAttributes, extends = ol)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(
        "cn-breadcrumb-list flex flex-wrap items-center wrap-break-word",
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        ol {
            "data-slot": "breadcrumb-list",
            class: class,
            ..attributes,
            {children}
        }
    }
}

#[component]
pub fn BreadcrumbItem(
    #[props(extends = GlobalAttributes, extends = li)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(
        "cn-breadcrumb-item inline-flex items-center",
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        li {
            "data-slot": "breadcrumb-item",
            class: class,
            ..attributes,
            {children}
        }
    }
}

#[component]
pub fn BreadcrumbLink(
    #[props(extends = GlobalAttributes, extends = a)] attributes: Vec<Attribute>,
    onclick: Option<EventHandler<MouseEvent>>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!("cn-breadcrumb-link", take_class(&mut attributes).as_deref());

    rsx! {
        a {
            "data-slot": "breadcrumb-link",
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
pub fn BreadcrumbPage(
    #[props(extends = GlobalAttributes, extends = span)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!("cn-breadcrumb-page", take_class(&mut attributes).as_deref());

    rsx! {
        span {
            "data-slot": "breadcrumb-page",
            role: "link",
            aria_disabled: "true",
            aria_current: "page",
            class: class,
            ..attributes,
            {children}
        }
    }
}

/// Renders a chevron unless `children` is given.
#[component]
pub fn BreadcrumbSeparator(
    #[props(extends = GlobalAttributes, extends = li)] attributes: Vec<Attribute>,
    children: Option<Element>,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(
        "cn-breadcrumb-separator",
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        li {
            "data-slot": "breadcrumb-separator",
            role: "presentation",
            aria_hidden: "true",
            class: class,
            ..attributes,
            if let Some(children) = children {
                {children}
            } else {
                svg {
                    class: "rtl:rotate-180",
                    view_box: "0 0 24 24",
                    fill: "none",
                    stroke: "currentColor",
                    stroke_width: "2",
                    stroke_linecap: "round",
                    stroke_linejoin: "round",
                    path { d: "m9 18 6-6-6-6" }
                }
            }
        }
    }
}

#[component]
pub fn BreadcrumbEllipsis(
    #[props(extends = GlobalAttributes, extends = span)] attributes: Vec<Attribute>,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(
        "cn-breadcrumb-ellipsis flex items-center justify-center",
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        span {
            "data-slot": "breadcrumb-ellipsis",
            role: "presentation",
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
            span { class: "sr-only", "More" }
        }
    }
}
