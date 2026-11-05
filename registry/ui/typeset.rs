//! Typeset: a typography scale for prose-like content. `Typeset` sets the
//! flow between blocks, and each part (`H1`..`H4`, `P`, `List`, `ListItem`,
//! `Blockquote`, `InlineCode`) renders one element with its look.
//!
//! No typeset placeholder class has a rule in registry/styles, so the look is
//! plain Tailwind: the classic shadcn typography scale, not the `typeset.css`
//! engine from https://ui.shadcn.com/docs/typeset. Vertical spacing comes from
//! the `Typeset` wrapper, so the parts can also be used on their own.
//!
//! Installed by oniko-cn. This file is yours: edit it freely.

use dioxus::prelude::*;
use oniko_cn::{cn, take_class};

const TYPESET_BASE: &str = "text-base leading-7 text-foreground [&>*+*]:mt-6";

#[component]
pub fn Typeset(
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(TYPESET_BASE, take_class(&mut attributes).as_deref());

    rsx! {
        div {
            "data-slot": "typeset",
            class: class,
            ..attributes,
            {children}
        }
    }
}

const H1_BASE: &str = "scroll-m-20 text-4xl font-extrabold tracking-tight";

#[component]
pub fn H1(
    #[props(extends = GlobalAttributes, extends = h1)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(H1_BASE, take_class(&mut attributes).as_deref());

    rsx! {
        h1 {
            "data-slot": "typeset-h1",
            class: class,
            ..attributes,
            {children}
        }
    }
}

const H2_BASE: &str = "scroll-m-20 border-b pb-2 text-3xl font-semibold tracking-tight";

#[component]
pub fn H2(
    #[props(extends = GlobalAttributes, extends = h2)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(H2_BASE, take_class(&mut attributes).as_deref());

    rsx! {
        h2 {
            "data-slot": "typeset-h2",
            class: class,
            ..attributes,
            {children}
        }
    }
}

const H3_BASE: &str = "scroll-m-20 text-2xl font-semibold tracking-tight";

#[component]
pub fn H3(
    #[props(extends = GlobalAttributes, extends = h3)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(H3_BASE, take_class(&mut attributes).as_deref());

    rsx! {
        h3 {
            "data-slot": "typeset-h3",
            class: class,
            ..attributes,
            {children}
        }
    }
}

const H4_BASE: &str = "scroll-m-20 text-xl font-semibold tracking-tight";

#[component]
pub fn H4(
    #[props(extends = GlobalAttributes, extends = h4)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(H4_BASE, take_class(&mut attributes).as_deref());

    rsx! {
        h4 {
            "data-slot": "typeset-h4",
            class: class,
            ..attributes,
            {children}
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ParagraphVariant {
    #[default]
    Default,
    Lead,
    Large,
    Small,
    Muted,
}

impl ParagraphVariant {
    pub fn class(self) -> &'static str {
        match self {
            Self::Default => "leading-7",
            Self::Lead => "text-xl text-muted-foreground",
            Self::Large => "text-lg font-semibold",
            Self::Small => "text-sm leading-none font-medium",
            Self::Muted => "text-sm text-muted-foreground",
        }
    }
}

#[component]
pub fn P(
    #[props(default)] variant: ParagraphVariant,
    #[props(extends = GlobalAttributes, extends = p)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(variant.class(), take_class(&mut attributes).as_deref());

    rsx! {
        p {
            "data-slot": "typeset-p",
            class: class,
            ..attributes,
            {children}
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ListKind {
    #[default]
    Unordered,
    Ordered,
}

impl ListKind {
    pub fn class(self) -> &'static str {
        match self {
            Self::Unordered => "list-disc",
            Self::Ordered => "list-decimal",
        }
    }
}

const LIST_BASE: &str = "ml-6 [&>li]:mt-2";

#[component]
pub fn List(
    #[props(default)] kind: ListKind,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(
        LIST_BASE,
        kind.class(),
        take_class(&mut attributes).as_deref()
    );

    match kind {
        ListKind::Unordered => rsx! {
            ul {
                "data-slot": "typeset-list",
                class: class,
                ..attributes,
                {children}
            }
        },
        ListKind::Ordered => rsx! {
            ol {
                "data-slot": "typeset-list",
                class: class,
                ..attributes,
                {children}
            }
        },
    }
}

#[component]
pub fn ListItem(
    #[props(extends = GlobalAttributes, extends = li)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(take_class(&mut attributes).as_deref());

    rsx! {
        li {
            "data-slot": "typeset-list-item",
            class: class,
            ..attributes,
            {children}
        }
    }
}

const BLOCKQUOTE_BASE: &str = "border-l-2 pl-6 italic";

#[component]
pub fn Blockquote(
    #[props(extends = GlobalAttributes, extends = blockquote)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(BLOCKQUOTE_BASE, take_class(&mut attributes).as_deref());

    rsx! {
        blockquote {
            "data-slot": "typeset-blockquote",
            class: class,
            ..attributes,
            {children}
        }
    }
}

const INLINE_CODE_BASE: &str =
    "relative rounded bg-muted px-[0.3rem] py-[0.2rem] font-mono text-sm font-semibold";

#[component]
pub fn InlineCode(
    #[props(extends = GlobalAttributes, extends = code)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(INLINE_CODE_BASE, take_class(&mut attributes).as_deref());

    rsx! {
        code {
            "data-slot": "typeset-inline-code",
            class: class,
            ..attributes,
            {children}
        }
    }
}
