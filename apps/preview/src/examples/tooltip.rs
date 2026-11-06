//! Tooltip examples, ported from shadcn/ui `examples/base/tooltip-*.tsx`.

use dioxus::prelude::*;
use lucide_dioxus::Save;

use super::Example;
use crate::components::ui::button::{Button, ButtonSize, ButtonVariant, button_variants};
use crate::components::ui::kbd::Kbd;
use crate::components::ui::tooltip::{Tooltip, TooltipContent, TooltipSide, TooltipTrigger};

fn as_span((attributes, children): (Vec<Attribute>, Element)) -> Element {
    rsx! {
        span { class: "inline-block w-fit", ..attributes, {children} }
    }
}

#[component]
pub fn Examples() -> Element {
    rsx! {
        TooltipDemo {}
        TooltipSides {}
        TooltipKeyboard {}
        TooltipDisabled {}
        TooltipRtl {}
    }
}

#[component]
fn TooltipDemo() -> Element {
    rsx! {
        Example { title: "Demo",
            Tooltip {
                TooltipTrigger { class: button_variants(ButtonVariant::Outline, ButtonSize::Default, None),
                    "Hover"
                }
                TooltipContent {
                    p { "Add to library" }
                }
            }
        }
    }
}

#[component]
fn TooltipSides() -> Element {
    let sides = [
        (TooltipSide::Left, "left"),
        (TooltipSide::Top, "top"),
        (TooltipSide::Bottom, "bottom"),
        (TooltipSide::Right, "right"),
    ];

    rsx! {
        Example { title: "Side",
            div { class: "flex flex-wrap gap-2",
                for (side , name) in sides {
                    Tooltip { key: "{name}",
                        TooltipTrigger { class: button_variants(ButtonVariant::Outline, ButtonSize::Default, Some("w-fit capitalize")),
                            "{name}"
                        }
                        TooltipContent { side,
                            p { "Add to library" }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn TooltipKeyboard() -> Element {
    rsx! {
        Example { title: "With Keyboard Shortcut",
            Tooltip {
                TooltipTrigger {
                    aria_label: "Save",
                    class: button_variants(ButtonVariant::Outline, ButtonSize::IconSm, None),
                    Save {}
                }
                TooltipContent {
                    "Save Changes "
                    Kbd { "S" }
                }
            }
        }
    }
}

#[component]
fn TooltipDisabled() -> Element {
    rsx! {
        Example { title: "Disabled Button",
            Tooltip {
                TooltipTrigger { render: as_span,
                    Button { variant: ButtonVariant::Outline, disabled: true, "Disabled" }
                }
                TooltipContent {
                    p { "This feature is currently unavailable" }
                }
            }
        }
    }
}

#[component]
fn TooltipRtl() -> Element {
    let physical = [
        (TooltipSide::Left, "يسار"),
        (TooltipSide::Top, "أعلى"),
        (TooltipSide::Bottom, "أسفل"),
        (TooltipSide::Right, "يمين"),
    ];
    let logical = [
        (TooltipSide::InlineStart, "بداية السطر"),
        (TooltipSide::InlineEnd, "نهاية السطر"),
    ];

    rsx! {
        Example { title: "RTL",
            div { class: "grid gap-4", dir: "rtl",
                div { class: "flex flex-wrap justify-center gap-2",
                    for (side , label) in physical {
                        Tooltip { key: "{label}",
                            TooltipTrigger { class: button_variants(ButtonVariant::Outline, ButtonSize::Default, None),
                                "{label}"
                            }
                            TooltipContent { side, dir: "rtl", "إضافة إلى المكتبة" }
                        }
                    }
                }
                div { class: "flex flex-wrap justify-center gap-2",
                    for (side , label) in logical {
                        Tooltip { key: "{label}",
                            TooltipTrigger { class: button_variants(ButtonVariant::Outline, ButtonSize::Default, None),
                                "{label}"
                            }
                            TooltipContent { side, dir: "rtl", "إضافة إلى المكتبة" }
                        }
                    }
                }
            }
        }
    }
}
