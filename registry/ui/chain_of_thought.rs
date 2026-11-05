//! Chain of Thought: a collapsible list of reasoning steps, with optional search results and images.
//!
//! Installed by oniko-cn. This file is yours: edit it freely.

use dioxus::prelude::*;
use oniko_cn::{cn, take_class};

use super::badge::{Badge, BadgeVariant};
use super::collapsible::{Collapsible, CollapsibleContent, CollapsibleTrigger};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ChainOfThoughtStepStatus {
    #[default]
    Complete,
    Active,
    Pending,
}

impl ChainOfThoughtStepStatus {
    /// Value of the `data-status` attribute.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Complete => "complete",
            Self::Active => "active",
            Self::Pending => "pending",
        }
    }

    pub fn class(self) -> &'static str {
        match self {
            Self::Complete => "text-muted-foreground",
            Self::Active => "text-foreground",
            Self::Pending => "text-muted-foreground/50",
        }
    }
}

fn dot_icon() -> Element {
    rsx! {
        svg {
            "data-slot": "chain-of-thought-dot-icon",
            class: "size-4 shrink-0",
            view_box: "0 0 24 24",
            fill: "currentColor",
            circle { cx: "12", cy: "12", r: "3" }
        }
    }
}

/// Root of the chain. Shows and hides its steps, and is open by default.
#[component]
pub fn ChainOfThought(
    /// Controlled open state. When set, `on_open_change` is the only way to change it.
    open: Option<bool>,
    /// Open state at first render when `open` is not controlled.
    #[props(default = true)]
    default_open: bool,
    on_open_change: Option<EventHandler<bool>>,
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(
        "flex w-full flex-col gap-4",
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        div {
            "data-slot": "chain-of-thought",
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

/// The clickable title of the chain. Defaults to "Chain of Thought".
#[component]
pub fn ChainOfThoughtHeader(
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Option<Element>,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(take_class(&mut attributes).as_deref());
    let label = children.unwrap_or_else(|| rsx! { "Chain of Thought" });

    rsx! {
        div {
            "data-slot": "chain-of-thought-header",
            class: class,
            ..attributes,
            CollapsibleTrigger {
                class: "group flex w-full items-center gap-2 text-left text-sm text-muted-foreground transition-colors hover:text-foreground",
                svg {
                "data-slot": "chain-of-thought-header-icon",
                class: "size-4 shrink-0",
                view_box: "0 0 24 24",
                fill: "none",
                stroke: "currentColor",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
                path { d: "M12 18V5" }
                path { d: "M15 13a4.17 4.17 0 0 1-3-4 4.17 4.17 0 0 1-3 4" }
                path { d: "M17.598 6.5A3 3 0 1 0 12 5a3 3 0 1 0-5.598 1.5" }
                path { d: "M17.997 5.125a4 4 0 0 1 2.526 5.77" }
                path { d: "M18 18a4 4 0 0 0 2-7.464" }
                path { d: "M19.967 17.483A4 4 0 1 1 12 18a4 4 0 1 1-7.967-.517" }
                path { d: "M6 18a4 4 0 0 1-2-7.464" }
                path { d: "M6.003 5.125a4 4 0 0 0-2.526 5.77" }
            }
            span { "data-slot": "chain-of-thought-header-label", class: "flex-1", {label} }
            svg {
                "data-slot": "chain-of-thought-header-chevron",
                class: "size-4 shrink-0 transition-transform group-data-[state=open]:rotate-180",
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

/// One reasoning step. `icon` defaults to a dot; `description` is optional.
#[component]
pub fn ChainOfThoughtStep(
    #[props(default)] status: ChainOfThoughtStepStatus,
    icon: Option<Element>,
    label: Element,
    description: Option<Element>,
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(
        "flex gap-2 text-sm",
        status.class(),
        take_class(&mut attributes).as_deref()
    );
    let icon = icon.unwrap_or_else(dot_icon);

    rsx! {
        div {
            "data-slot": "chain-of-thought-step",
            "data-status": status.as_str(),
            class: class,
            ..attributes,
            div {
                "data-slot": "chain-of-thought-step-icon",
                class: "mt-0.5 flex size-4 shrink-0 items-center justify-center",
                {icon}
            }
            div { class: "flex min-w-0 flex-col gap-1",
                div { "data-slot": "chain-of-thought-step-label", {label} }
                if let Some(description) = description {
                    div {
                        "data-slot": "chain-of-thought-step-description",
                        class: "text-xs text-muted-foreground",
                        {description}
                    }
                }
            }
        }
    }
}

#[component]
pub fn ChainOfThoughtSearchResults(
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(
        "flex flex-wrap items-center gap-2",
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        div {
            "data-slot": "chain-of-thought-search-results",
            class: class,
            ..attributes,
            {children}
        }
    }
}

/// A single search result, shown as a secondary badge.
#[component]
pub fn ChainOfThoughtSearchResult(
    #[props(extends = GlobalAttributes, extends = span)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(take_class(&mut attributes).as_deref());

    rsx! {
        span {
            "data-slot": "chain-of-thought-search-result",
            class: class,
            ..attributes,
            Badge { variant: BadgeVariant::Secondary, class: "text-xs font-normal", {children} }
        }
    }
}

/// The part of a step that is hidden while the chain is closed.
#[component]
pub fn ChainOfThoughtContent(
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(take_class(&mut attributes).as_deref());

    rsx! {
        div {
            "data-slot": "chain-of-thought-content-wrapper",
            class: class,
            ..attributes,
            CollapsibleContent {
                class: "mt-2 flex flex-col gap-3 text-popover-foreground outline-none",
                {children}
            }
        }
    }
}

/// An image inside a step, with an optional caption below it.
#[component]
pub fn ChainOfThoughtImage(
    caption: Option<Element>,
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(
        "mt-2 flex flex-col gap-2",
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        div {
            "data-slot": "chain-of-thought-image",
            class: class,
            ..attributes,
            div {
                "data-slot": "chain-of-thought-image-frame",
                class: "flex w-full items-center justify-center overflow-hidden rounded-md border bg-muted p-3",
                {children}
            }
            if let Some(caption) = caption {
                p {
                    "data-slot": "chain-of-thought-image-caption",
                    class: "text-xs text-muted-foreground",
                    {caption}
                }
            }
        }
    }
}
