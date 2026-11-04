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
            Self::InlineStart => "pl-2 has-[>button]:-ml-1 has-[>kbd]:ml-[-0.15rem] order-first",
            Self::InlineEnd => "pr-2 has-[>button]:-mr-1 has-[>kbd]:mr-[-0.15rem] order-last",
            Self::BlockStart => {
                "px-2.5 pt-2 group-has-[>input]/input-group:pt-2 [.border-b]:pb-2 order-first w-full justify-start"
            }
            Self::BlockEnd => {
                "px-2.5 pb-2 group-has-[>input]/input-group:pb-2 [.border-t]:pt-2 order-last w-full justify-start"
            }
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
            Self::Xs => {
                "h-6 gap-1 rounded-[calc(var(--radius)-5px)] px-1.5 [&>svg:not([class*='size-'])]:size-3.5"
            }
            Self::IconXs => "size-6 rounded-[calc(var(--radius)-5px)] p-0 has-[>svg]:p-0",
            Self::IconSm => "size-8 p-0 has-[>svg]:p-0",
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
        "border-input dark:bg-input/30 has-[[data-slot=input-group-control]:focus-visible]:border-ring has-[[data-slot=input-group-control]:focus-visible]:ring-ring/50 has-[[data-slot][aria-invalid=true]]:ring-destructive/20 has-[[data-slot][aria-invalid=true]]:border-destructive dark:has-[[data-slot][aria-invalid=true]]:ring-destructive/40 h-9 rounded-md border shadow-xs transition-[color,box-shadow] in-data-[slot=combobox-content]:focus-within:border-inherit in-data-[slot=combobox-content]:focus-within:ring-0 has-[[data-slot=input-group-control]:focus-visible]:ring-3 has-[[data-slot][aria-invalid=true]]:ring-3 has-[>[data-align=block-end]]:h-auto has-[>[data-align=block-end]]:flex-col has-[>[data-align=block-start]]:h-auto has-[>[data-align=block-start]]:flex-col has-[>[data-align=block-end]]:[&>input]:pt-3 has-[>[data-align=block-start]]:[&>input]:pb-3 has-[>[data-align=inline-end]]:[&>input]:pr-1.5 has-[>[data-align=inline-start]]:[&>input]:pl-1.5 group/input-group relative flex w-full min-w-0 items-center",
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
        "text-muted-foreground h-auto gap-2 py-1.5 text-sm font-medium group-data-[disabled=true]/input-group:opacity-50 [&>kbd]:rounded-[calc(var(--radius)-5px)] [&>svg:not([class*='size-'])]:size-4 group/input-group-addon flex cursor-text items-center justify-center select-none",
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
        "text-muted-foreground gap-2 text-sm [&_svg:not([class*='size-'])]:size-4 flex items-center",
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
        "rounded-none border-0 bg-transparent shadow-none ring-0 focus-visible:ring-0 aria-invalid:ring-0 dark:bg-transparent flex-1 min-w-0 outline-none",
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
        "rounded-none border-0 bg-transparent py-2 shadow-none ring-0 focus-visible:ring-0 aria-invalid:ring-0 dark:bg-transparent flex-1 min-w-0 resize-none field-sizing-content outline-none",
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
        "gap-2 text-sm",
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
