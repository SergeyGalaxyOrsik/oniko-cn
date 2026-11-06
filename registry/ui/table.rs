//! Table: a styled HTML table, split into parts that match the native elements.
//!
//! Installed by oniko-cn. This file is yours: edit it freely.

use dioxus::prelude::*;
use oniko_cn::{cn, take_class};

#[component]
pub fn Table(
    #[props(extends = GlobalAttributes, extends = table)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!("cn-table", take_class(&mut attributes).as_deref());

    rsx! {
        div { "data-slot": "table-container", class: "cn-table-container",
            table {
                "data-slot": "table",
                class: class,
                ..attributes,
                {children}
            }
        }
    }
}

#[component]
pub fn TableHeader(
    #[props(extends = GlobalAttributes, extends = thead)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!("cn-table-header", take_class(&mut attributes).as_deref());

    rsx! {
        thead {
            "data-slot": "table-header",
            class: class,
            ..attributes,
            {children}
        }
    }
}

#[component]
pub fn TableBody(
    #[props(extends = GlobalAttributes, extends = tbody)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!("cn-table-body", take_class(&mut attributes).as_deref());

    rsx! {
        tbody {
            "data-slot": "table-body",
            class: class,
            ..attributes,
            {children}
        }
    }
}

#[component]
pub fn TableFooter(
    #[props(extends = GlobalAttributes, extends = tfoot)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!("cn-table-footer", take_class(&mut attributes).as_deref());

    rsx! {
        tfoot {
            "data-slot": "table-footer",
            class: class,
            ..attributes,
            {children}
        }
    }
}

#[component]
pub fn TableRow(
    #[props(extends = GlobalAttributes, extends = tr)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(
        "cn-table-row has-aria-expanded:bg-muted/50",
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        tr {
            "data-slot": "table-row",
            class: class,
            ..attributes,
            {children}
        }
    }
}

#[component]
pub fn TableHead(
    #[props(extends = GlobalAttributes, extends = th)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!("cn-table-head", take_class(&mut attributes).as_deref());

    rsx! {
        th {
            "data-slot": "table-head",
            class: class,
            ..attributes,
            {children}
        }
    }
}

#[component]
pub fn TableCell(
    #[props(extends = GlobalAttributes, extends = td)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!("cn-table-cell", take_class(&mut attributes).as_deref());

    rsx! {
        td {
            "data-slot": "table-cell",
            class: class,
            ..attributes,
            {children}
        }
    }
}

#[component]
pub fn TableCaption(
    #[props(extends = GlobalAttributes, extends = caption)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!("cn-table-caption", take_class(&mut attributes).as_deref());

    rsx! {
        caption {
            "data-slot": "table-caption",
            class: class,
            ..attributes,
            {children}
        }
    }
}
