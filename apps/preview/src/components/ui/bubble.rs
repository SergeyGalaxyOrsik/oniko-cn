//! Bubble: a chat message bubble with variants, alignment and reactions.
//!
//! Installed by oniko-cn. This file is yours: edit it freely.

use dioxus::prelude::*;
use oniko_cn::{cn, take_class};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum BubbleVariant {
    #[default]
    Default,
    Secondary,
    Muted,
    Tinted,
    Outline,
    Ghost,
    Destructive,
}

impl BubbleVariant {
    pub fn class(self) -> &'static str {
        match self {
            Self::Default => {
                "*:data-[slot=bubble-content]:bg-primary *:data-[slot=bubble-content]:text-primary-foreground [&>[data-slot=bubble-content]:is(button,a):hover]:bg-primary/80"
            }
            Self::Secondary => {
                "*:data-[slot=bubble-content]:bg-secondary *:data-[slot=bubble-content]:text-secondary-foreground [&>[data-slot=bubble-content]:is(button,a):hover]:bg-[color-mix(in_oklch,var(--secondary),var(--foreground)_5%)]"
            }
            Self::Muted => {
                "*:data-[slot=bubble-content]:bg-muted [&>[data-slot=bubble-content]:is(button,a):hover]:bg-[color-mix(in_oklch,var(--muted),var(--foreground)_5%)]"
            }
            Self::Tinted => {
                "*:data-[slot=bubble-content]:bg-[oklch(from_var(--primary)_0.93_calc(c*0.4)_h)] dark:*:data-[slot=bubble-content]:bg-[oklch(from_var(--primary)_0.3_calc(c*0.4)_h)] *:data-[slot=bubble-content]:text-foreground [&>[data-slot=bubble-content]:is(button,a):hover]:bg-[oklch(from_var(--primary)_0.88_calc(c*0.5)_h)] dark:[&>[data-slot=bubble-content]:is(button,a):hover]:bg-[oklch(from_var(--primary)_0.35_calc(c*0.5)_h)]"
            }
            Self::Outline => {
                "*:data-[slot=bubble-content]:bg-background *:data-[slot=bubble-content]:border-border [&>[data-slot=bubble-content]:is(button,a):hover]:bg-muted [&>[data-slot=bubble-content]:is(button,a):hover]:text-foreground dark:[&>[data-slot=bubble-content]:is(button,a):hover]:bg-input/30"
            }
            Self::Ghost => {
                "*:data-[slot=bubble-content]:rounded-none *:data-[slot=bubble-content]:bg-transparent *:data-[slot=bubble-content]:p-0 [&>[data-slot=bubble-content]:is(button,a):hover]:bg-muted [&>[data-slot=bubble-content]:is(button,a):hover]:text-foreground dark:[&>[data-slot=bubble-content]:is(button,a):hover]:bg-muted/50 border-none"
            }
            Self::Destructive => {
                "*:data-[slot=bubble-content]:bg-destructive/10 dark:*:data-[slot=bubble-content]:bg-destructive/20 *:data-[slot=bubble-content]:text-destructive [&>[data-slot=bubble-content]:is(button,a):hover]:bg-destructive/20 dark:[&>[data-slot=bubble-content]:is(button,a):hover]:bg-destructive/30"
            }
        }
    }

    /// Value of the `data-variant` attribute that the styles key off.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Default => "default",
            Self::Secondary => "secondary",
            Self::Muted => "muted",
            Self::Tinted => "tinted",
            Self::Outline => "outline",
            Self::Ghost => "ghost",
            Self::Destructive => "destructive",
        }
    }
}

/// Which side of the conversation a bubble sits on.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum BubbleAlign {
    #[default]
    Start,
    End,
}

impl BubbleAlign {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Start => "start",
            Self::End => "end",
        }
    }

    fn reactions_class(self) -> &'static str {
        match self {
            Self::Start => "left-3",
            Self::End => "right-3",
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum BubbleReactionsSide {
    Top,
    #[default]
    Bottom,
}

impl BubbleReactionsSide {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Top => "top",
            Self::Bottom => "bottom",
        }
    }

    fn class(self) -> &'static str {
        match self {
            Self::Top => "top-0 -translate-y-3/4",
            Self::Bottom => "bottom-0 translate-y-3/4",
        }
    }
}

#[component]
pub fn BubbleGroup(
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(
        "gap-2 flex min-w-0 flex-col",
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        div { "data-slot": "bubble-group", class: class, ..attributes, {children} }
    }
}

#[component]
pub fn Bubble(
    #[props(default)] variant: BubbleVariant,
    #[props(default)] align: BubbleAlign,
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(
        "gap-1 data-[align=end]:self-end max-w-[80%] data-[variant=ghost]:max-w-full group-data-[align=end]/message:self-end group/bubble relative flex w-fit min-w-0 flex-col",
        variant.class(),
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        div {
            "data-slot": "bubble",
            "data-variant": variant.as_str(),
            "data-align": align.as_str(),
            class: class,
            ..attributes,
            {children}
        }
    }
}

const BUBBLE_CONTENT_BASE: &str = "rounded-xl border border-transparent px-3 py-2 text-sm leading-relaxed [button,a]:outline-none [button,a]:focus-visible:border-ring [button,a]:focus-visible:ring-3 [button,a]:focus-visible:ring-ring/50 group-data-[align=end]/bubble:self-end w-fit max-w-full min-w-0 overflow-hidden wrap-break-word [button]:text-left [button,a]:transition-colors";

/// Bubble content classes for any element, e.g. a custom clickable container.
pub fn bubble_content_variants(class: Option<&str>) -> String {
    cn!(BUBBLE_CONTENT_BASE, class)
}

#[component]
pub fn BubbleContent(
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = bubble_content_variants(take_class(&mut attributes).as_deref());

    rsx! {
        div { "data-slot": "bubble-content", class: class, ..attributes, {children} }
    }
}

/// A bubble body that is a `<button>`, for clickable messages.
#[component]
pub fn BubbleContentButton(
    #[props(extends = GlobalAttributes, extends = button)] attributes: Vec<Attribute>,
    onclick: Option<EventHandler<MouseEvent>>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = bubble_content_variants(take_class(&mut attributes).as_deref());

    rsx! {
        button {
            "data-slot": "bubble-content",
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

/// A bubble body that is an `<a>`, for messages that link somewhere.
#[component]
pub fn BubbleContentLink(
    #[props(extends = GlobalAttributes, extends = a)] attributes: Vec<Attribute>,
    onclick: Option<EventHandler<MouseEvent>>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = bubble_content_variants(take_class(&mut attributes).as_deref());

    rsx! {
        a {
            "data-slot": "bubble-content",
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
pub fn BubbleReactions(
    #[props(default)] side: BubbleReactionsSide,
    #[props(default = BubbleAlign::End)] align: BubbleAlign,
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(
        "rounded-full ring-3 ring-card bg-muted shrink-0 gap-1 px-1.5 py-0.5 has-[button]:p-0 text-sm absolute z-10 flex w-fit items-center justify-center",
        side.class(),
        align.reactions_class(),
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        div {
            "data-slot": "bubble-reactions",
            "data-align": align.as_str(),
            "data-side": side.as_str(),
            class: class,
            ..attributes,
            {children}
        }
    }
}
