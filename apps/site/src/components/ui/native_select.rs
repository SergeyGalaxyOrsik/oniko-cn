//! NativeSelect: a styled wrapper around the browser's native `<select>`.
//! NativeSelectOption and NativeSelectOptGroup: the options and option groups it contains.
//!
//! Use it where the platform picker is what you want, for example in long forms or on mobile.
//! Pass `value`, `onchange`, `disabled` and `aria-invalid` directly on the component.
//! As in shadcn/ui, a `class` you pass lands on the wrapper, not on the `select`.
//!
//! Installed by oniko-cn. This file is yours: edit it freely.

use dioxus::prelude::*;
use oniko_cn::{cn, take_class};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum NativeSelectSize {
    #[default]
    Default,
    Sm,
}

impl NativeSelectSize {
    pub fn data_size(self) -> &'static str {
        match self {
            Self::Default => "default",
            Self::Sm => "sm",
        }
    }
}

const NATIVE_SELECT_WRAPPER: &str =
    "group/native-select relative w-fit has-[select:disabled]:opacity-50";

const NATIVE_SELECT_BASE: &str = "border-input bg-input/20 placeholder:text-muted-foreground selection:bg-primary selection:text-primary-foreground dark:bg-input/30 dark:hover:bg-input/50 focus-visible:border-ring focus-visible:ring-ring/30 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 h-7 w-full min-w-0 appearance-none rounded-md border py-0.5 pr-6 pl-2 text-xs/relaxed transition-colors select-none focus-visible:ring-2 aria-invalid:ring-2 data-[size=sm]:h-6 data-[size=sm]:text-[0.625rem] outline-none disabled:pointer-events-none disabled:cursor-not-allowed";

#[component]
pub fn NativeSelect(
    #[props(default)] size: NativeSelectSize,
    onchange: Option<EventHandler<FormEvent>>,
    oninput: Option<EventHandler<FormEvent>>,
    onfocus: Option<EventHandler<FocusEvent>>,
    onblur: Option<EventHandler<FocusEvent>>,
    #[props(extends = GlobalAttributes, extends = select)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let wrapper_class = cn!(
        NATIVE_SELECT_WRAPPER,
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        div {
            "data-slot": "native-select-wrapper",
            "data-size": size.data_size(),
            class: wrapper_class,
            select {
                "data-slot": "native-select",
                "data-size": size.data_size(),
                class: NATIVE_SELECT_BASE,
                onchange: move |event| {
                    if let Some(handler) = onchange {
                        handler.call(event);
                    }
                },
                oninput: move |event| {
                    if let Some(handler) = oninput {
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
                ..attributes,
                {children}
            }
            svg {
                "data-slot": "native-select-icon",
                class: "text-muted-foreground top-1/2 right-1.5 size-3.5 -translate-y-1/2 group-data-[size=sm]/native-select:size-3 group-data-[size=sm]/native-select:-translate-y-[calc(--spacing(1.25))] pointer-events-none absolute select-none",
                "aria-hidden": "true",
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

#[component]
pub fn NativeSelectOption(
    #[props(extends = GlobalAttributes, extends = option)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(
        "bg-[Canvas] text-[CanvasText]",
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        option {
            "data-slot": "native-select-option",
            class: class,
            ..attributes,
            {children}
        }
    }
}

#[component]
pub fn NativeSelectOptGroup(
    #[props(extends = GlobalAttributes, extends = optgroup)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(
        "bg-[Canvas] text-[CanvasText]",
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        optgroup {
            "data-slot": "native-select-optgroup",
            class: class,
            ..attributes,
            {children}
        }
    }
}
