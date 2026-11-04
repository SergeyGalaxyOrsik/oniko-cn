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
    let class = cn!(
        "w-full caption-bottom text-sm",
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        div { "data-slot": "table-container", class: "relative w-full overflow-x-auto",
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
    let class = cn!("[&_tr]:border-b", take_class(&mut attributes).as_deref());

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
    let class = cn!(
        "[&_tr:last-child]:border-0",
        take_class(&mut attributes).as_deref()
    );

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
    let class = cn!(
        "bg-muted/50 border-t font-medium [&>tr]:last:border-b-0",
        take_class(&mut attributes).as_deref()
    );

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
        "hover:bg-muted/50 data-[state=selected]:bg-muted border-b transition-colors",
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
    let class = cn!(
        "text-foreground h-10 px-2 text-left align-middle font-medium whitespace-nowrap [&:has([role=checkbox])]:pr-0",
        take_class(&mut attributes).as_deref()
    );

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
    let class = cn!(
        "p-2 align-middle whitespace-nowrap [&:has([role=checkbox])]:pr-0",
        take_class(&mut attributes).as_deref()
    );

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
    let class = cn!(
        "text-muted-foreground mt-4 text-sm",
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        caption {
            "data-slot": "table-caption",
            class: class,
            ..attributes,
            {children}
        }
    }
}
