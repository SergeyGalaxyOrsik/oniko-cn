//! NativeSelect: a styled wrapper around the browser's native `<select>`.
//! NativeSelectOption and NativeSelectOptGroup: the options and option groups it contains.
//!
//! Use it where the platform picker is what you want, for example in long forms or on mobile.
//! Pass `value`, `onchange`, `disabled` and `aria-invalid` directly on the component.
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

const NATIVE_SELECT_WRAPPER: &str = "relative w-full has-[select:disabled]:opacity-50";

const NATIVE_SELECT_BASE: &str =
    "cn-native-select outline-none disabled:pointer-events-none disabled:cursor-not-allowed";

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
    let class = cn!(NATIVE_SELECT_BASE, take_class(&mut attributes).as_deref());

    rsx! {
        div {
            "data-slot": "native-select-wrapper",
            class: NATIVE_SELECT_WRAPPER,
            select {
                "data-slot": "native-select",
                "data-size": size.data_size(),
                class: class,
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
                class: "cn-native-select-icon pointer-events-none absolute",
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
    value: Option<String>,
    #[props(default)] disabled: bool,
    #[props(extends = GlobalAttributes, extends = option)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    rsx! {
        option {
            "data-slot": "native-select-option",
            value: value,
            disabled: disabled,
            ..attributes,
            {children}
        }
    }
}

#[component]
pub fn NativeSelectOptGroup(
    label: String,
    #[props(default)] disabled: bool,
    #[props(extends = GlobalAttributes, extends = optgroup)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    rsx! {
        optgroup {
            "data-slot": "native-select-optgroup",
            label: label,
            disabled: disabled,
            ..attributes,
            {children}
        }
    }
}
