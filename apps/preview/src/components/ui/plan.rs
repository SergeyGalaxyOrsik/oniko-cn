//! Plan: a collapsible card that shows an AI-generated plan, with a header, expandable content and a footer.
//!
//! While `is_streaming` is set, the title and description pulse, so the plan reads as work in progress.
//!
//! Installed by oniko-cn. This file is yours: edit it freely.

use dioxus::prelude::*;
use oniko_cn::{cn, take_class};

use super::button::{ButtonSize, ButtonVariant, button_variants};
use super::card::Card;
use super::collapsible::{Collapsible, CollapsibleContent, CollapsibleTrigger};

#[derive(Clone, Copy)]
struct PlanContext {
    is_streaming: Signal<bool>,
}

/// A signal that follows a prop, so context shared with children never goes stale.
fn use_prop<T: Clone + PartialEq + 'static>(value: T) -> Signal<T> {
    let mut signal = use_signal(|| value.clone());
    if *signal.peek() != value {
        signal.set(value);
    }
    signal
}

#[component]
pub fn Plan(
    /// Marks the plan as still being written. Title and description pulse while it is set.
    #[props(default)]
    is_streaming: bool,
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
    let class = cn!(take_class(&mut attributes));
    let is_streaming = use_prop(is_streaming);
    use_context_provider(|| PlanContext { is_streaming });

    rsx! {
        div {
            "data-slot": "plan",
            "data-streaming": is_streaming().then_some("true"),
            class: class,
            ..attributes,
            Collapsible {
                open: open,
                default_open: default_open,
                on_open_change: on_open_change,
                class: "flex flex-col",
                Card { {children} }
            }
        }
    }
}

#[component]
pub fn PlanHeader(
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(
        "grid auto-rows-min items-start gap-1 px-6 has-data-[slot=plan-action]:grid-cols-[1fr_auto]",
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        div {
            "data-slot": "plan-header",
            class: class,
            ..attributes,
            {children}
        }
    }
}

#[component]
pub fn PlanTitle(
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let is_streaming = use_context::<PlanContext>().is_streaming;
    let class = cn!(
        "font-semibold leading-none",
        is_streaming().then_some("animate-pulse"),
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        div {
            "data-slot": "plan-title",
            class: class,
            ..attributes,
            {children}
        }
    }
}

#[component]
pub fn PlanDescription(
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let is_streaming = use_context::<PlanContext>().is_streaming;
    let class = cn!(
        "text-muted-foreground text-sm",
        is_streaming().then_some("animate-pulse"),
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        div {
            "data-slot": "plan-description",
            class: class,
            ..attributes,
            {children}
        }
    }
}

/// Places its content in the top right corner of `PlanHeader`.
#[component]
pub fn PlanAction(
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(
        "col-start-2 row-span-2 row-start-1 self-start justify-self-end",
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        div {
            "data-slot": "plan-action",
            class: class,
            ..attributes,
            {children}
        }
    }
}

/// Button that expands and collapses the enclosing `Plan`.
#[component]
pub fn PlanTrigger() -> Element {
    rsx! {
        CollapsibleTrigger {
            "data-slot": "plan-trigger",
            class: button_variants(ButtonVariant::Ghost, ButtonSize::IconSm, None),
            svg {
                view_box: "0 0 24 24",
                fill: "none",
                stroke: "currentColor",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
                class: "size-4",
                "aria-hidden": "true",
                path { d: "m7 15 5 5 5-5" }
                path { d: "m7 9 5-5 5 5" }
            }
            span { class: "sr-only", "Toggle plan" }
        }
    }
}

/// Content that is shown only while the plan is open.
#[component]
pub fn PlanContent(
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(take_class(&mut attributes));
    rsx! {
        CollapsibleContent {
            class: "px-6",
            div {
                "data-slot": "plan-content",
                class: class,
                ..attributes,
                {children}
            }
        }
    }
}

#[component]
pub fn PlanFooter(
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(
        "flex items-center px-6",
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        div {
            "data-slot": "plan-footer",
            class: class,
            ..attributes,
            {children}
        }
    }
}
