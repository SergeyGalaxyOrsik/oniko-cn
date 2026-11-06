//! Input group: adds addons, buttons and helper text to an input or a textarea.
//!
//! Installed by oniko-cn. This file is yours: edit it freely.

use std::rc::Rc;

use dioxus::prelude::*;
use oniko_cn::{cn, take_class};

use super::button::{Button, ButtonSize, ButtonVariant, button_variants};
use super::input::input_variants;
use super::textarea::textarea_variants;

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
            Self::BlockStart => {
                "cn-input-group-addon-align-block-start order-first w-full justify-start"
            }
            Self::BlockEnd => {
                "cn-input-group-addon-align-block-end order-last w-full justify-start"
            }
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum InputGroupButtonSize {
    #[default]
    Xs,
    Sm,
    IconXs,
    IconSm,
}

impl InputGroupButtonSize {
    fn name(self) -> &'static str {
        match self {
            Self::Xs => "xs",
            Self::Sm => "sm",
            Self::IconXs => "icon-xs",
            Self::IconSm => "icon-sm",
        }
    }

    fn class(self) -> &'static str {
        match self {
            Self::Xs => "cn-input-group-button-size-xs",
            Self::Sm => "cn-input-group-button-size-sm",
            Self::IconXs => "cn-input-group-button-size-icon-xs",
            Self::IconSm => "cn-input-group-button-size-icon-sm",
        }
    }
}

/// Classes of an `InputGroupButton`, for another element that should look like one,
/// e.g. a dropdown or popover trigger.
pub fn input_group_button_variants(
    variant: ButtonVariant,
    size: InputGroupButtonSize,
    class: Option<&str>,
) -> String {
    let extra = cn!(
        "cn-input-group-button flex items-center shadow-none",
        size.class(),
        class
    );
    button_variants(variant, ButtonSize::Default, Some(extra.as_str()))
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
        "group/input-group cn-input-group relative flex w-full min-w-0 items-center outline-none has-[>textarea]:h-auto",
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
        "cn-input-group-addon flex cursor-text items-center justify-center select-none",
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
pub fn InputGroupButton(
    #[props(default = ButtonVariant::Ghost)] variant: ButtonVariant,
    #[props(default)] size: InputGroupButtonSize,
    onclick: Option<EventHandler<MouseEvent>>,
    #[props(extends = GlobalAttributes, extends = button)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(
        "cn-input-group-button flex items-center shadow-none",
        size.class(),
        take_class(&mut attributes).as_deref()
    );
    attributes.push(Attribute::new("class", class, None, false));
    attributes.push(Attribute::new("data-size", size.name(), None, false));
    if !attributes.iter().any(|attr| attr.name == "type") {
        attributes.push(Attribute::new("type", "button", None, false));
    }

    rsx! {
        Button {
            variant,
            attributes,
            onclick: move |event: MouseEvent| {
                // The addon focuses the input on click, except when the click lands on a button.
                event.stop_propagation();
                if let Some(handler) = onclick {
                    handler.call(event);
                }
            },
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
        "cn-input-group-text flex items-center [&_svg]:pointer-events-none",
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        span { class: class, ..attributes, {children} }
    }
}

#[component]
pub fn InputGroupInput(
    onclick: Option<EventHandler<MouseEvent>>,
    oninput: Option<EventHandler<FormEvent>>,
    onchange: Option<EventHandler<FormEvent>>,
    onfocus: Option<EventHandler<FocusEvent>>,
    onblur: Option<EventHandler<FocusEvent>>,
    onkeydown: Option<EventHandler<KeyboardEvent>>,
    #[props(extends = GlobalAttributes, extends = input)] attributes: Vec<Attribute>,
) -> Element {
    let mut context = use_context::<InputGroupContext>();
    let mut attributes = attributes;
    let extra = cn!(
        "cn-input-group-input flex-1",
        take_class(&mut attributes).as_deref()
    );
    let class = input_variants(Some(extra.as_str()));

    rsx! {
        input {
            "data-slot": "input-group-control",
            class: class,
            onmounted: move |event| context.input.set(Some(event.data())),
            onclick: move |event| {
                if let Some(handler) = onclick {
                    handler.call(event);
                }
            },
            oninput: move |event| {
                if let Some(handler) = oninput {
                    handler.call(event);
                }
            },
            onchange: move |event| {
                if let Some(handler) = onchange {
                    handler.call(event);
                }
            },
            onfocus: move |event| {
                if let Some(handler) = onfocus {
                    handler.call(event);
                }
            },
            onblur: move |event| {
                if let Some(handler) = onblur {
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
    onclick: Option<EventHandler<MouseEvent>>,
    oninput: Option<EventHandler<FormEvent>>,
    onchange: Option<EventHandler<FormEvent>>,
    onfocus: Option<EventHandler<FocusEvent>>,
    onblur: Option<EventHandler<FocusEvent>>,
    onkeydown: Option<EventHandler<KeyboardEvent>>,
    #[props(extends = GlobalAttributes, extends = textarea)] attributes: Vec<Attribute>,
) -> Element {
    let mut attributes = attributes;
    let extra = cn!(
        "cn-input-group-textarea flex-1 resize-none",
        take_class(&mut attributes).as_deref()
    );
    let class = textarea_variants(Some(extra.as_str()));

    rsx! {
        textarea {
            "data-slot": "input-group-control",
            class: class,
            onclick: move |event| {
                if let Some(handler) = onclick {
                    handler.call(event);
                }
            },
            oninput: move |event| {
                if let Some(handler) = oninput {
                    handler.call(event);
                }
            },
            onchange: move |event| {
                if let Some(handler) = onchange {
                    handler.call(event);
                }
            },
            onfocus: move |event| {
                if let Some(handler) = onfocus {
                    handler.call(event);
                }
            },
            onblur: move |event| {
                if let Some(handler) = onblur {
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
