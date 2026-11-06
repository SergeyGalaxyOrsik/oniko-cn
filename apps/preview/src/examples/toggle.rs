//! Toggle examples, ported from shadcn/ui `examples/base/toggle-*.tsx`.

use dioxus::prelude::*;
use lucide_dioxus::{Bold, Bookmark, Italic};

use super::Example;
use crate::components::ui::toggle::{Toggle, ToggleSize, ToggleVariant};

#[component]
pub fn Examples() -> Element {
    rsx! {
        div { class: "flex flex-col gap-10",
            ToggleDemo {}
            ToggleOutline {}
            ToggleText {}
            ToggleSizes {}
            ToggleDisabled {}
            ToggleRtl {}
        }
    }
}

#[component]
fn ToggleDemo() -> Element {
    rsx! {
        Example { title: "Demo",
            Toggle {
                "aria-label": "Toggle bookmark",
                size: ToggleSize::Sm,
                variant: ToggleVariant::Outline,
                Bookmark { class: "group-aria-pressed/toggle:fill-foreground" }
                "Bookmark"
            }
        }
    }
}

#[component]
fn ToggleOutline() -> Element {
    rsx! {
        Example { title: "Outline",
            div { class: "flex flex-wrap items-center gap-2",
                Toggle { variant: ToggleVariant::Outline, "aria-label": "Toggle italic",
                    Italic {}
                    "Italic"
                }
                Toggle { variant: ToggleVariant::Outline, "aria-label": "Toggle bold",
                    Bold {}
                    "Bold"
                }
            }
        }
    }
}

#[component]
fn ToggleText() -> Element {
    rsx! {
        Example { title: "With Text",
            Toggle { "aria-label": "Toggle italic",
                Italic {}
                "Italic"
            }
        }
    }
}

#[component]
fn ToggleSizes() -> Element {
    rsx! {
        Example { title: "Size",
            div { class: "flex flex-wrap items-center gap-2",
                Toggle {
                    variant: ToggleVariant::Outline,
                    "aria-label": "Toggle small",
                    size: ToggleSize::Sm,
                    "Small"
                }
                Toggle {
                    variant: ToggleVariant::Outline,
                    "aria-label": "Toggle default",
                    size: ToggleSize::Default,
                    "Default"
                }
                Toggle {
                    variant: ToggleVariant::Outline,
                    "aria-label": "Toggle large",
                    size: ToggleSize::Lg,
                    "Large"
                }
            }
        }
    }
}

#[component]
fn ToggleDisabled() -> Element {
    rsx! {
        Example { title: "Disabled",
            div { class: "flex flex-wrap items-center gap-2",
                Toggle { "aria-label": "Toggle disabled", disabled: true, "Disabled" }
                Toggle {
                    variant: ToggleVariant::Outline,
                    "aria-label": "Toggle disabled outline",
                    disabled: true,
                    "Disabled"
                }
            }
        }
    }
}

#[component]
fn ToggleRtl() -> Element {
    rsx! {
        Example { title: "RTL",
            Toggle {
                "aria-label": "Toggle bookmark",
                size: ToggleSize::Sm,
                variant: ToggleVariant::Outline,
                dir: "rtl",
                Bookmark { class: "group-aria-pressed/toggle:fill-foreground" }
                "إشارة مرجعية"
            }
        }
    }
}
