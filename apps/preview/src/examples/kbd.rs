//! Kbd examples, ported from shadcn/ui `examples/base/kbd-*.tsx`.

use dioxus::prelude::*;
use lucide_dioxus::Search;

use super::Example;
use crate::components::ui::button::{Button, ButtonVariant};
use crate::components::ui::button_group::ButtonGroup;
use crate::components::ui::input_group::{
    InputGroup, InputGroupAddon, InputGroupAlign, InputGroupInput,
};
use crate::components::ui::kbd::{Kbd, KbdGroup};
use crate::components::ui::tooltip::{Tooltip, TooltipContent, TooltipTrigger};

#[component]
pub fn Examples() -> Element {
    rsx! {
        div { class: "flex flex-col gap-10",
            KbdDemo {}
            KbdGroupExample {}
            KbdButton {}
            KbdTooltip {}
            KbdInputGroup {}
            KbdRtl {}
        }
    }
}

#[component]
fn KbdDemo() -> Element {
    rsx! {
        Example { title: "Demo",
            div { class: "flex flex-col items-center gap-4",
                KbdGroup {
                    Kbd { "⌘" }
                    Kbd { "⇧" }
                    Kbd { "⌥" }
                    Kbd { "⌃" }
                }
                KbdGroup {
                    Kbd { "Ctrl" }
                    span { "+" }
                    Kbd { "B" }
                }
            }
        }
    }
}

#[component]
fn KbdGroupExample() -> Element {
    rsx! {
        Example { title: "Group",
            div { class: "flex flex-col items-center gap-4",
                p { class: "text-sm text-muted-foreground",
                    "Use "
                    KbdGroup {
                        Kbd { "Ctrl + B" }
                        Kbd { "Ctrl + K" }
                    }
                    " to open the command palette"
                }
            }
        }
    }
}

#[component]
fn KbdButton() -> Element {
    rsx! {
        Example { title: "Button",
            Button { variant: ButtonVariant::Outline,
                "Accept "
                Kbd { "data-icon": "inline-end", class: "translate-x-0.5", "⏎" }
            }
        }
    }
}

#[component]
fn KbdTooltip() -> Element {
    rsx! {
        Example { title: "Tooltip",
            div { class: "flex flex-wrap gap-4",
                ButtonGroup {
                    Tooltip {
                        TooltipTrigger {
                            Button { variant: ButtonVariant::Outline, "Save" }
                        }
                        TooltipContent {
                            "Save Changes "
                            Kbd { "S" }
                        }
                    }
                    Tooltip {
                        TooltipTrigger {
                            Button { variant: ButtonVariant::Outline, "Print" }
                        }
                        TooltipContent {
                            "Print Document "
                            KbdGroup {
                                Kbd { "Ctrl" }
                                Kbd { "P" }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn KbdInputGroup() -> Element {
    rsx! {
        Example { title: "Input Group",
            div { class: "flex w-full max-w-xs flex-col gap-6",
                InputGroup {
                    InputGroupInput { placeholder: "Search..." }
                    InputGroupAddon {
                        Search {}
                    }
                    InputGroupAddon { align: InputGroupAlign::InlineEnd,
                        Kbd { "⌘" }
                        Kbd { "K" }
                    }
                }
            }
        }
    }
}

#[component]
fn KbdRtl() -> Element {
    rsx! {
        Example { title: "RTL",
            div { class: "flex flex-col items-center gap-4", dir: "rtl",
                KbdGroup {
                    Kbd { "⌘" }
                    Kbd { "⇧" }
                    Kbd { "⌥" }
                    Kbd { "⌃" }
                }
                KbdGroup {
                    Kbd { "Ctrl" }
                    span { "+" }
                    Kbd { "B" }
                }
            }
        }
    }
}
