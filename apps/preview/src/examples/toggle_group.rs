//! ToggleGroup examples, ported from shadcn/ui `examples/base/toggle-group-*.tsx`.

use dioxus::prelude::*;
use lucide_dioxus::{Bold, Italic, Underline};

use super::Example;
use crate::components::ui::field::{Field, FieldDescription, FieldLabel};
use crate::components::ui::toggle::{ToggleSize, ToggleVariant};
use crate::components::ui::toggle_group::{ToggleGroup, ToggleGroupItem, ToggleGroupOrientation};

#[component]
pub fn Examples() -> Element {
    rsx! {
        div { class: "flex flex-col gap-10",
            ToggleGroupDemo {}
            ToggleGroupOutline {}
            ToggleGroupSizes {}
            ToggleGroupSpacing {}
            ToggleGroupVertical {}
            ToggleGroupDisabled {}
            ToggleGroupFontWeightSelector {}
            ToggleGroupRtl {}
        }
    }
}

#[component]
fn ToggleGroupDemo() -> Element {
    rsx! {
        Example { title: "Demo",
            ToggleGroup { variant: ToggleVariant::Outline, multiple: true,
                ToggleGroupItem { value: "bold", "aria-label": "Toggle bold", Bold {} }
                ToggleGroupItem { value: "italic", "aria-label": "Toggle italic", Italic {} }
                ToggleGroupItem { value: "strikethrough", "aria-label": "Toggle strikethrough", Underline {} }
            }
        }
    }
}

#[component]
fn ToggleGroupOutline() -> Element {
    rsx! {
        Example { title: "Outline",
            ToggleGroup { variant: ToggleVariant::Outline, default_value: vec!["all".to_string()],
                ToggleGroupItem { value: "all", "aria-label": "Toggle all", "All" }
                ToggleGroupItem { value: "missed", "aria-label": "Toggle missed", "Missed" }
            }
        }
    }
}

#[component]
fn ToggleGroupSizes() -> Element {
    rsx! {
        Example { title: "Size",
            div { class: "flex flex-col gap-4",
                ToggleGroup {
                    size: ToggleSize::Sm,
                    default_value: vec!["top".to_string()],
                    variant: ToggleVariant::Outline,
                    ToggleGroupItem { value: "top", "aria-label": "Toggle top", "Top" }
                    ToggleGroupItem { value: "bottom", "aria-label": "Toggle bottom", "Bottom" }
                    ToggleGroupItem { value: "left", "aria-label": "Toggle left", "Left" }
                    ToggleGroupItem { value: "right", "aria-label": "Toggle right", "Right" }
                }
                ToggleGroup { default_value: vec!["top".to_string()], variant: ToggleVariant::Outline,
                    ToggleGroupItem { value: "top", "aria-label": "Toggle top", "Top" }
                    ToggleGroupItem { value: "bottom", "aria-label": "Toggle bottom", "Bottom" }
                    ToggleGroupItem { value: "left", "aria-label": "Toggle left", "Left" }
                    ToggleGroupItem { value: "right", "aria-label": "Toggle right", "Right" }
                }
            }
        }
    }
}

#[component]
fn ToggleGroupSpacing() -> Element {
    rsx! {
        Example { title: "Spacing",
            ToggleGroup {
                size: ToggleSize::Sm,
                default_value: vec!["top".to_string()],
                variant: ToggleVariant::Outline,
                spacing: 2,
                ToggleGroupItem { value: "top", "aria-label": "Toggle top", "Top" }
                ToggleGroupItem { value: "bottom", "aria-label": "Toggle bottom", "Bottom" }
                ToggleGroupItem { value: "left", "aria-label": "Toggle left", "Left" }
                ToggleGroupItem { value: "right", "aria-label": "Toggle right", "Right" }
            }
        }
    }
}

#[component]
fn ToggleGroupVertical() -> Element {
    rsx! {
        Example { title: "Vertical",
            ToggleGroup {
                multiple: true,
                orientation: ToggleGroupOrientation::Vertical,
                spacing: 1,
                default_value: vec!["bold".to_string(), "italic".to_string()],
                ToggleGroupItem { value: "bold", "aria-label": "Toggle bold", Bold {} }
                ToggleGroupItem { value: "italic", "aria-label": "Toggle italic", Italic {} }
                ToggleGroupItem { value: "underline", "aria-label": "Toggle underline", Underline {} }
            }
        }
    }
}

#[component]
fn ToggleGroupDisabled() -> Element {
    rsx! {
        Example { title: "Disabled",
            ToggleGroup { disabled: true,
                ToggleGroupItem { value: "bold", "aria-label": "Toggle bold", Bold {} }
                ToggleGroupItem { value: "italic", "aria-label": "Toggle italic", Italic {} }
                ToggleGroupItem { value: "strikethrough", "aria-label": "Toggle strikethrough", Underline {} }
            }
        }
    }
}

#[component]
fn ToggleGroupFontWeightSelector() -> Element {
    let mut font_weight = use_signal(|| "normal".to_string());
    let weights = [
        ("light", "Light", "font-light"),
        ("normal", "Normal", "font-normal"),
        ("medium", "Medium", "font-medium"),
        ("bold", "Bold", "font-bold"),
    ];

    rsx! {
        Example { title: "Custom", class: "*:data-[slot=field]:max-w-xs",
            Field {
                FieldLabel { "Font Weight" }
                ToggleGroup {
                    value: vec![font_weight()],
                    on_value_change: move |value: Vec<String>| {
                        if let Some(first) = value.into_iter().next() {
                            font_weight.set(first);
                        }
                    },
                    variant: ToggleVariant::Outline,
                    spacing: 2,
                    size: ToggleSize::Lg,
                    for (value, label, weight_class) in weights {
                        ToggleGroupItem {
                            key: "{value}",
                            value: value,
                            "aria-label": label,
                            class: "flex size-16 flex-col items-center justify-center rounded-xl",
                            span { class: "text-2xl leading-none {weight_class}", "Aa" }
                            span { class: "text-xs text-muted-foreground", "{label}" }
                        }
                    }
                }
                FieldDescription {
                    "Use "
                    code { class: "rounded-md bg-muted px-1 py-0.5 font-mono", "font-{font_weight}" }
                    " to set the font weight."
                }
            }
        }
    }
}

#[component]
fn ToggleGroupRtl() -> Element {
    rsx! {
        Example { title: "RTL",
            ToggleGroup {
                variant: ToggleVariant::Outline,
                default_value: vec!["list".to_string()],
                dir: "rtl",
                ToggleGroupItem { value: "list", "aria-label": "قائمة", "قائمة" }
                ToggleGroupItem { value: "grid", "aria-label": "شبكة", "شبكة" }
                ToggleGroupItem { value: "cards", "aria-label": "بطاقات", "بطاقات" }
            }
        }
    }
}
