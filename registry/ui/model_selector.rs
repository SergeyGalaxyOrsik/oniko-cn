//! Model selector: a trigger button that opens a dialog with a searchable list of AI models.
//!
//! Built on `Dialog` for the window and `Command` for the search and keyboard navigation.
//! Choosing an item closes the dialog and calls its `on_select`.
//!
//! Installed by oniko-cn. This file is yours: edit it freely.

use dioxus::prelude::*;
use oniko_cn::{cn, take_class};

use super::button::{ButtonSize, ButtonVariant};
use super::command::{Command, CommandEmpty, CommandGroup, CommandInput, CommandItem, CommandList};
use super::dialog::{Dialog, DialogContent, DialogTitle, DialogTrigger};

/// A signal that follows a prop, so context shared with children never goes stale.
fn use_prop<T: Clone + PartialEq + 'static>(value: T) -> Signal<T> {
    let mut signal = use_signal(|| value.clone());
    if *signal.peek() != value {
        signal.set(value);
    }
    signal
}

#[derive(Clone, Copy)]
struct ModelSelectorContext {
    internal: Signal<bool>,
    controlled: Signal<Option<bool>>,
    on_open_change: Option<EventHandler<bool>>,
}

impl ModelSelectorContext {
    fn is_open(self) -> bool {
        (self.controlled)().unwrap_or_else(|| (self.internal)())
    }

    fn change(self, next: bool) {
        if self.controlled.peek().is_none() {
            let mut internal = self.internal;
            internal.set(next);
        }
        if let Some(handler) = self.on_open_change {
            handler.call(next);
        }
    }
}

/// Owns the open state. Pass `open` to control it, or let it manage itself.
#[component]
pub fn ModelSelector(
    open: Option<bool>,
    #[props(default)] default_open: bool,
    on_open_change: Option<EventHandler<bool>>,
    children: Element,
) -> Element {
    let internal = use_signal(|| default_open);
    let controlled = use_prop(open);
    let context = use_context_provider(|| ModelSelectorContext {
        internal,
        controlled,
        on_open_change,
    });

    let is_open = context.is_open();
    rsx! {
        Dialog {
            open: Some(is_open),
            on_open_change: move |next: bool| context.change(next),
            {children}
        }
    }
}

/// The button that opens the model selector.
#[component]
pub fn ModelSelectorTrigger(
    #[props(default = ButtonVariant::Outline)] variant: ButtonVariant,
    #[props(default)] size: ButtonSize,
    onclick: Option<EventHandler<MouseEvent>>,
    children: Element,
) -> Element {
    rsx! {
        DialogTrigger {
            variant: variant,
            size: size,
            onclick: onclick,
            {children}
        }
    }
}

/// The dialog window that holds the search field and the model list.
///
/// `title` is read by screen readers only.
#[component]
pub fn ModelSelectorContent(
    #[props(default = "Model Selector".to_string())] title: String,
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(
        "flex w-[min(32rem,calc(100vw-2rem))] flex-col overflow-hidden p-0",
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        DialogContent {
            show_close_button: false,
            class: "p-0",
            DialogTitle {
                class: "sr-only",
                {title}
            }
            div {
                "data-slot": "model-selector-content",
                class: class,
                ..attributes,
                Command {
                    {children}
                }
            }
        }
    }
}

/// The search field. Typing filters the items by their `value` and `keywords`.
#[component]
pub fn ModelSelectorInput(
    #[props(default = "Search models...".to_string())] placeholder: String,
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(take_class(&mut attributes));

    rsx! {
        div {
            "data-slot": "model-selector-input",
            class: class,
            ..attributes,
            CommandInput {
                placeholder: placeholder,
            }
        }
    }
}

#[component]
pub fn ModelSelectorList(
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(
        "max-h-80 overflow-y-auto",
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        div {
            "data-slot": "model-selector-list",
            class: class,
            ..attributes,
            CommandList {
                {children}
            }
        }
    }
}

/// Shown when no item matches the search.
#[component]
pub fn ModelSelectorEmpty(
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(take_class(&mut attributes));

    rsx! {
        div {
            "data-slot": "model-selector-empty",
            class: class,
            ..attributes,
            CommandEmpty {
                {children}
            }
        }
    }
}

/// A labelled set of models. Hidden while none of its items match the search.
#[component]
pub fn ModelSelectorGroup(
    heading: Option<String>,
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(take_class(&mut attributes));

    rsx! {
        div {
            "data-slot": "model-selector-group",
            class: class,
            ..attributes,
            CommandGroup {
                heading: heading,
                {children}
            }
        }
    }
}

/// One model. Selecting it closes the dialog and calls `on_select` with `value`.
#[component]
pub fn ModelSelectorItem(
    /// Identifies the model. Matched against the search and passed to `on_select`.
    value: String,
    /// Extra text the search also matches against, such as the provider name.
    #[props(default)]
    keywords: Vec<String>,
    #[props(default)] disabled: bool,
    /// Marks the model the app currently uses. Shows a check mark.
    #[props(default)]
    selected: bool,
    on_select: Option<EventHandler<String>>,
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let context = use_context::<ModelSelectorContext>();
    let mut attributes = attributes;
    let class = cn!(
        "flex min-w-0 flex-1 items-center gap-2",
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        CommandItem {
            value: value,
            keywords: keywords,
            disabled: disabled,
            class: "relative flex cursor-default select-none items-center gap-2 rounded-md px-2 py-1.5 text-sm outline-none data-[disabled=true]:pointer-events-none data-[disabled=true]:opacity-50 data-[selected=true]:bg-accent data-[selected=true]:text-accent-foreground",
            on_select: move |value: String| {
                context.change(false);
                if let Some(handler) = on_select {
                    handler.call(value);
                }
            },
            div {
                "data-slot": "model-selector-item",
                "data-selected": selected.then_some("true"),
                class: class,
                ..attributes,
                {children}
                if selected {
                    svg {
                        "data-slot": "model-selector-item-check",
                        class: "ml-auto size-4 shrink-0",
                        view_box: "0 0 24 24",
                        fill: "none",
                        stroke: "currentColor",
                        stroke_width: "2",
                        stroke_linecap: "round",
                        stroke_linejoin: "round",
                        path { d: "M20 6 9 17l-5-5" }
                    }
                }
            }
        }
    }
}

#[component]
pub fn ModelSelectorSeparator(
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(
        "-mx-1 my-1 h-px bg-border",
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        div {
            "data-slot": "model-selector-separator",
            role: "separator",
            class: class,
            ..attributes,
        }
    }
}

/// Keyboard shortcut hint, aligned to the end of an item.
#[component]
pub fn ModelSelectorShortcut(
    #[props(extends = GlobalAttributes, extends = span)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(
        "ml-auto text-xs tracking-widest text-muted-foreground",
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        span {
            "data-slot": "model-selector-shortcut",
            class: class,
            ..attributes,
            {children}
        }
    }
}

/// A provider logo. Pass the image URL in `src`; `alt` is empty when the name is shown next to it.
#[component]
pub fn ModelSelectorLogo(
    src: String,
    #[props(default)] alt: String,
    #[props(extends = GlobalAttributes, extends = img)] attributes: Vec<Attribute>,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(
        "size-3 shrink-0 rounded-sm object-contain",
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        img {
            "data-slot": "model-selector-logo",
            src: "{src}",
            alt: "{alt}",
            class: class,
            ..attributes,
        }
    }
}

/// Overlapping row of provider logos.
#[component]
pub fn ModelSelectorLogoGroup(
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(
        "flex shrink-0 items-center -space-x-1 [&>img]:rounded-full [&>img]:bg-background [&>img]:ring-1 [&>img]:ring-background",
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        div {
            "data-slot": "model-selector-logo-group",
            class: class,
            ..attributes,
            {children}
        }
    }
}

/// The model or provider name. Truncates with an ellipsis and takes the free space.
#[component]
pub fn ModelSelectorName(
    #[props(extends = GlobalAttributes, extends = span)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(
        "flex-1 truncate text-left",
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        span {
            "data-slot": "model-selector-name",
            class: class,
            ..attributes,
            {children}
        }
    }
}
