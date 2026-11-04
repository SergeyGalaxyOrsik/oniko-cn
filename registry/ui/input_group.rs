//! Input group: adds addons, buttons and helper text to an input or a textarea.
//!
//! Installed by oniko-cn. This file is yours: edit it freely.

use std::rc::Rc;

use dioxus::prelude::*;
use oniko_cn::{cn, take_class};

use super::button::{ButtonSize, ButtonVariant, button_variants};

#[derive(Clone, Copy)]
struct InputGroupContext {
    input: Signal<Option<Rc<MountedData>>>,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum InputGroupAlign {
    #[default]
    InlineStart,
    InlineEnd,
    BlockStart,
    BlockEnd,
}

impl InputGroupAlign {
    fn name(self) -> &'static str {
        match self {
            Self::InlineStart => "inline-start",
            Self::InlineEnd => "inline-end",
            Self::BlockStart => "block-start",
            Self::BlockEnd => "block-end",
        }
    }

    fn class(self) -> &'static str {
        match self {
            Self::InlineStart => "cn-input-group-addon-align-inline-start order-first",
            Self::InlineEnd => "cn-input-group-addon-align-inline-end order-last",
            Self::BlockStart => "cn-input-group-addon-align-block-start order-first w-full justify-start",
            Self::BlockEnd => "cn-input-group-addon-align-block-end order-last w-full justify-start",
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum InputGroupButtonSize {
    #[default]
    Xs,
    IconXs,
    IconSm,
}

impl InputGroupButtonSize {
    fn button_size(self) -> ButtonSize {
        match self {
            Self::Xs => ButtonSize::Xs,
            Self::IconXs => ButtonSize::IconXs,
            Self::IconSm => ButtonSize::IconSm,
        }
    }

    fn class(self) -> &'static str {
        match self {
            Self::Xs => "cn-input-group-button-size-xs",
            Self::IconXs => "cn-input-group-button-size-icon-xs",
            Self::IconSm => "cn-input-group-button-size-icon-sm",
        }
    }
}

#[component]
pub fn InputGroup(
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let input = use_signal(|| None::<Rc<MountedData>>);
    use_context_provider(|| InputGroupContext { input });

    let mut attributes = attributes;
    let class = cn!(
        "cn-input-group group/input-group relative flex w-full min-w-0 items-center",
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        div {
            "data-slot": "input-group",
            role: "group",
            class: class,
            ..attributes,
            {children}
        }
    }
}

#[component]
pub fn InputGroupAddon(
    #[props(default)] align: InputGroupAlign,
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let context = try_use_context::<InputGroupContext>();
    let mut attributes = attributes;
    let class = cn!(
        "cn-input-group-addon group/input-group-addon flex cursor-text items-center justify-center select-none",
        align.class(),
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        div {
            "data-slot": "input-group-addon",
            "data-align": align.name(),
            role: "group",
            class: class,
            onclick: move |_| {
                if let Some(context) = context
                    && let Some(input) = context.input.peek().clone()
                {
                    spawn(async move {
                        _ = input.set_focus(true).await;
                    });
                }
            },
            ..attributes,
            {children}
        }
    }
}

#[component]
pub fn InputGroupText(
    #[props(extends = GlobalAttributes, extends = span)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(
        "cn-input-group-text flex items-center",
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        span {
            "data-slot": "input-group-text",
            class: class,
            ..attributes,
            {children}
        }
    }
}

#[component]
pub fn InputGroupInput(
    oninput: Option<EventHandler<FormEvent>>,
    onkeydown: Option<EventHandler<KeyboardEvent>>,
    #[props(extends = GlobalAttributes, extends = input)] attributes: Vec<Attribute>,
) -> Element {
    let mut context = use_context::<InputGroupContext>();
    let mut attributes = attributes;
    let class = cn!(
        "cn-input-group-input flex-1 min-w-0 outline-none",
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        input {
            "data-slot": "input-group-control",
            class: class,
            onmounted: move |event| context.input.set(Some(event.data())),
            oninput: move |event| {
                if let Some(handler) = oninput {
                    handler.call(event);
                }
            },
            onkeydown: move |event| {
                if let Some(handler) = onkeydown {
                    handler.call(event);
                }
            },
            ..attributes,
        }
    }
}

#[component]
pub fn InputGroupTextarea(
    oninput: Option<EventHandler<FormEvent>>,
    onkeydown: Option<EventHandler<KeyboardEvent>>,
    #[props(extends = GlobalAttributes, extends = textarea)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut context = use_context::<InputGroupContext>();
    let mut attributes = attributes;
    let class = cn!(
        "cn-input-group-textarea flex-1 min-w-0 resize-none field-sizing-content outline-none",
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        textarea {
            "data-slot": "input-group-control",
            class: class,
            onmounted: move |event| context.input.set(Some(event.data())),
            oninput: move |event| {
                if let Some(handler) = oninput {
                    handler.call(event);
                }
            },
            onkeydown: move |event| {
                if let Some(handler) = onkeydown {
                    handler.call(event);
                }
            },
            ..attributes,
            {children}
        }
    }
}

#[component]
pub fn InputGroupButton(
    #[props(default = ButtonVariant::Ghost)] variant: ButtonVariant,
    #[props(default)] size: InputGroupButtonSize,
    onclick: Option<EventHandler<MouseEvent>>,
    #[props(extends = GlobalAttributes, extends = button)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let extra = cn!(
        "cn-input-group-button gap-2 text-sm",
        size.class(),
        take_class(&mut attributes).as_deref()
    );
    let class = button_variants(variant, size.button_size(), Some(extra.as_str()));

    rsx! {
        button {
            "data-slot": "input-group-button",
            r#type: "button",
            class: class,
            onclick: move |event: MouseEvent| {
                // Keeps the addon's click handler from moving focus to the input.
                event.stop_propagation();
                if let Some(handler) = onclick {
                    handler.call(event);
                }
            },
            ..attributes,
            {children}
        }
    }
}
