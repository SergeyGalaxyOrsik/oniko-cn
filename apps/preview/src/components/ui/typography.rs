//! Typography: styles for headings, paragraphs, lists and the like. shadcn/ui ships no typography
//! component, only utility classes on plain elements; each part here is one such element with
//! upstream's classes, so `TypographyH1` is an `h1` and `TypographyList` is a `ul`.
//!
//! Spacing between parts comes from the classes themselves (`[&:not(:first-child)]:mt-6` and the like), as
//! in the docs. Pass `class` to change them.
//!
//! Installed by oniko-cn. This file is yours: edit it freely.

use dioxus::prelude::*;
use oniko_cn::{cn, take_class};

#[component]
pub fn TypographyH1(
    #[props(extends = GlobalAttributes, extends = h1)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(
        "scroll-m-20 text-4xl font-extrabold tracking-tight text-balance",
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        h1 {
            "data-slot": "typography-h1",
            class: class,
            ..attributes,
            {children}
        }
    }
}

#[component]
pub fn TypographyH2(
    #[props(extends = GlobalAttributes, extends = h2)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(
        "scroll-m-20 border-b pb-2 text-3xl font-semibold tracking-tight first:mt-0",
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        h2 {
            "data-slot": "typography-h2",
            class: class,
            ..attributes,
            {children}
        }
    }
}

#[component]
pub fn TypographyH3(
    #[props(extends = GlobalAttributes, extends = h3)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(
        "scroll-m-20 text-2xl font-semibold tracking-tight",
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        h3 {
            "data-slot": "typography-h3",
            class: class,
            ..attributes,
            {children}
        }
    }
}

#[component]
pub fn TypographyH4(
    #[props(extends = GlobalAttributes, extends = h4)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(
        "scroll-m-20 text-xl font-semibold tracking-tight",
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        h4 {
            "data-slot": "typography-h4",
            class: class,
            ..attributes,
            {children}
        }
    }
}

#[component]
pub fn TypographyP(
    #[props(extends = GlobalAttributes, extends = p)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(
        "leading-7 [&:not(:first-child)]:mt-6",
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        p {
            "data-slot": "typography-p",
            class: class,
            ..attributes,
            {children}
        }
    }
}

#[component]
pub fn TypographyBlockquote(
    #[props(extends = GlobalAttributes, extends = blockquote)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(
        "mt-6 border-l-2 pl-6 italic",
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        blockquote {
            "data-slot": "typography-blockquote",
            class: class,
            ..attributes,
            {children}
        }
    }
}

#[component]
pub fn TypographyList(
    #[props(extends = GlobalAttributes, extends = ul)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(
        "my-6 ml-6 list-disc [&>li]:mt-2",
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        ul {
            "data-slot": "typography-list",
            class: class,
            ..attributes,
            {children}
        }
    }
}

#[component]
pub fn TypographyInlineCode(
    #[props(extends = GlobalAttributes, extends = code)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(
        "relative rounded bg-muted px-[0.3rem] py-[0.2rem] font-mono text-sm font-semibold",
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        code {
            "data-slot": "typography-inline-code",
            class: class,
            ..attributes,
            {children}
        }
    }
}

#[component]
pub fn TypographyLead(
    #[props(extends = GlobalAttributes, extends = p)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(
        "text-xl text-muted-foreground",
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        p {
            "data-slot": "typography-lead",
            class: class,
            ..attributes,
            {children}
        }
    }
}

#[component]
pub fn TypographyLarge(
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(
        "text-lg font-semibold",
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        div {
            "data-slot": "typography-large",
            class: class,
            ..attributes,
            {children}
        }
    }
}

#[component]
pub fn TypographySmall(
    #[props(extends = GlobalAttributes, extends = small)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(
        "text-sm leading-none font-medium",
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        small {
            "data-slot": "typography-small",
            class: class,
            ..attributes,
            {children}
        }
    }
}

#[component]
pub fn TypographyMuted(
    #[props(extends = GlobalAttributes, extends = p)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(
        "text-sm text-muted-foreground",
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        p {
            "data-slot": "typography-muted",
            class: class,
            ..attributes,
            {children}
        }
    }
}

#[component]
pub fn TypographyTableRow(
    #[props(extends = GlobalAttributes, extends = tr)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(
        "m-0 border-t p-0 even:bg-muted",
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        tr {
            "data-slot": "typography-table-row",
            class: class,
            ..attributes,
            {children}
        }
    }
}

#[component]
pub fn TypographyTableHead(
    #[props(extends = GlobalAttributes, extends = th)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(
        "border px-4 py-2 text-left font-bold [&[align=center]]:text-center [&[align=right]]:text-right",
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        th {
            "data-slot": "typography-table-head",
            class: class,
            ..attributes,
            {children}
        }
    }
}

#[component]
pub fn TypographyTableCell(
    #[props(extends = GlobalAttributes, extends = td)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(
        "border px-4 py-2 text-left [&[align=center]]:text-center [&[align=right]]:text-right",
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        td {
            "data-slot": "typography-table-cell",
            class: class,
            ..attributes,
            {children}
        }
    }
}

/// A `table` in the scrolling wrapper from the docs. Fill it with `thead` and `tbody` elements holding
/// `TypographyTableRow`, `TypographyTableHead` and `TypographyTableCell`.
#[component]
pub fn TypographyTable(
    #[props(extends = GlobalAttributes, extends = table)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!("w-full", take_class(&mut attributes).as_deref());

    rsx! {
        div { "data-slot": "typography-table-container", class: "my-6 w-full overflow-y-auto",
            table { "data-slot": "typography-table", class: class, ..attributes, {children} }
        }
    }
}
