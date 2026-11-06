//! Attachment: a file or image chip, with a status, actions and an optional trigger that makes
//! the whole card clickable. Parts are `Attachment`, `AttachmentGroup`, `AttachmentMedia`,
//! `AttachmentContent`, `AttachmentTitle`, `AttachmentDescription`, `AttachmentActions`,
//! `AttachmentAction` and `AttachmentTrigger`.
//!
//! Installed by oniko-cn. This file is yours: edit it freely.

use dioxus::prelude::*;
use oniko_cn::{cn, take_class};

use super::button::{ButtonSize, ButtonVariant, button_variants};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum AttachmentState {
    Idle,
    Uploading,
    Processing,
    Error,
    #[default]
    Done,
}

impl AttachmentState {
    /// Value of the `data-state` attribute that the styles key off.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Idle => "idle",
            Self::Uploading => "uploading",
            Self::Processing => "processing",
            Self::Error => "error",
            Self::Done => "done",
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum AttachmentSize {
    #[default]
    Default,
    Sm,
    Xs,
}

impl AttachmentSize {
    pub fn class(self) -> &'static str {
        match self {
            Self::Default => {
                "gap-2 has-data-[slot=attachment-content]:px-2.5 has-data-[slot=attachment-content]:py-2 has-data-[slot=attachment-media]:p-2 text-sm"
            }
            Self::Sm => {
                "gap-2.5 has-data-[slot=attachment-content]:px-2 has-data-[slot=attachment-content]:py-1.5 has-data-[slot=attachment-media]:p-1.5 text-xs"
            }
            Self::Xs => {
                "gap-1.5 has-data-[slot=attachment-content]:px-1.5 has-data-[slot=attachment-content]:py-1 has-data-[slot=attachment-media]:p-1 text-xs rounded-lg"
            }
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::Default => "default",
            Self::Sm => "sm",
            Self::Xs => "xs",
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum AttachmentOrientation {
    #[default]
    Horizontal,
    Vertical,
}

impl AttachmentOrientation {
    pub fn class(self) -> &'static str {
        match self {
            Self::Horizontal => "min-w-40 items-center",
            Self::Vertical => "w-24 has-data-[slot=attachment-content]:w-30 flex-col",
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::Horizontal => "horizontal",
            Self::Vertical => "vertical",
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum AttachmentMediaVariant {
    #[default]
    Icon,
    Image,
}

impl AttachmentMediaVariant {
    pub fn class(self) -> &'static str {
        match self {
            Self::Icon => "",
            Self::Image => {
                "opacity-60 group-data-[state=idle]/attachment:opacity-100 group-data-[state=done]/attachment:opacity-100 *:[img]:aspect-square *:[img]:w-full *:[img]:object-cover"
            }
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::Icon => "icon",
            Self::Image => "image",
        }
    }
}

const ATTACHMENT_BASE: &str = "rounded-xl w-fit focus-within:ring-1 focus-within:ring-ring/50 group/attachment relative flex max-w-full min-w-0 shrink-0 flex-wrap border bg-card text-card-foreground transition-colors has-[>a,>button]:hover:bg-muted/50 data-[state=error]:border-destructive/30 data-[state=idle]:border-dashed";

#[component]
pub fn Attachment(
    #[props(default)] state: AttachmentState,
    #[props(default)] size: AttachmentSize,
    #[props(default)] orientation: AttachmentOrientation,
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(
        ATTACHMENT_BASE,
        size.class(),
        orientation.class(),
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        div {
            "data-slot": "attachment",
            "data-state": state.as_str(),
            "data-size": size.as_str(),
            "data-orientation": orientation.as_str(),
            class: class,
            ..attributes,
            {children}
        }
    }
}

#[component]
pub fn AttachmentMedia(
    #[props(default)] variant: AttachmentMediaVariant,
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(
        "bg-muted text-foreground w-10 rounded-lg group-data-[size=sm]/attachment:w-8 group-data-[size=xs]/attachment:w-7 group-data-[size=xs]/attachment:rounded-md [&_svg:not([class*='size-'])]:size-4 group-data-[size=xs]/attachment:[&_svg:not([class*='size-'])]:size-3.5 group-data-[orientation=vertical]/attachment:w-full group-data-[orientation=vertical]/attachment:[&_svg:not([class*='size-'])]:size-6 group-data-[orientation=vertical]/attachment:*:data-[slot=spinner]:size-6! relative flex aspect-square shrink-0 items-center justify-center overflow-hidden group-data-[state=error]/attachment:bg-destructive/10 group-data-[state=error]/attachment:text-destructive [&_svg]:pointer-events-none",
        variant.class(),
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        div {
            "data-slot": "attachment-media",
            "data-variant": variant.as_str(),
            class: class,
            ..attributes,
            {children}
        }
    }
}

#[component]
pub fn AttachmentContent(
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(
        "leading-tight group-data-[orientation=vertical]/attachment:px-1 max-w-full min-w-0 flex-1",
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        div { "data-slot": "attachment-content", class: class, ..attributes, {children} }
    }
}

#[component]
pub fn AttachmentTitle(
    #[props(extends = GlobalAttributes, extends = span)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(
        "font-medium block max-w-full min-w-0 truncate group-data-[state=processing]/attachment:shimmer group-data-[state=uploading]/attachment:shimmer",
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        span { "data-slot": "attachment-title", class: class, ..attributes, {children} }
    }
}

#[component]
pub fn AttachmentDescription(
    #[props(extends = GlobalAttributes, extends = span)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(
        "mt-0.5 text-xs block min-w-0 truncate text-muted-foreground group-data-[state=error]/attachment:text-destructive/80",
        "max-w-full",
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        span { "data-slot": "attachment-description", class: class, ..attributes, {children} }
    }
}

#[component]
pub fn AttachmentActions(
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(
        "group-data-[orientation=vertical]/attachment:absolute group-data-[orientation=vertical]/attachment:top-3 group-data-[orientation=vertical]/attachment:right-3 relative z-20 group-data-[orientation=vertical]/attachment:gap-1 flex shrink-0 items-center",
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        div { "data-slot": "attachment-actions", class: class, ..attributes, {children} }
    }
}

/// A button for one action, such as removing the file. Ghost and `IconXs` unless told otherwise.
#[component]
pub fn AttachmentAction(
    #[props(default = ButtonVariant::Ghost)] variant: ButtonVariant,
    #[props(default = ButtonSize::IconXs)] size: ButtonSize,
    onclick: Option<EventHandler<MouseEvent>>,
    #[props(extends = GlobalAttributes, extends = button)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = button_variants(
        variant,
        size,
        Some(&cn!("", take_class(&mut attributes).as_deref())),
    );

    rsx! {
        button {
            "data-slot": "attachment-action",
            "type": "button",
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

/// Makes the whole card clickable. Renders a link when `href` is set, otherwise a button.
/// It fills the card behind the actions, so the actions stay clickable.
#[component]
pub fn AttachmentTrigger(
    href: Option<String>,
    onclick: Option<EventHandler<MouseEvent>>,
    #[props(extends = GlobalAttributes, extends = button, extends = a)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(
        "absolute inset-0 z-10 outline-none",
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        if let Some(href) = href {
            a {
                "data-slot": "attachment-trigger",
                href: href,
                class: class,
                onclick: move |event| {
                    if let Some(handler) = onclick {
                        handler.call(event);
                    }
                },
                ..attributes,
                {children}
            }
        } else {
            button {
                "data-slot": "attachment-trigger",
                "type": "button",
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
}

#[component]
pub fn AttachmentGroup(
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(
        "gap-3 scroll-px-1 py-1 flex min-w-0 scroll-fade-x snap-x snap-mandatory scrollbar-none overflow-x-auto overscroll-x-contain *:data-[slot=attachment]:flex-none *:data-[slot=attachment]:snap-start",
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        div { "data-slot": "attachment-group", class: class, ..attributes, {children} }
    }
}
