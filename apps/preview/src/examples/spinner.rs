//! Spinner examples, ported from shadcn/ui `examples/base/spinner-*.tsx`.

use dioxus::prelude::*;
use lucide_dioxus::{ArrowUp, Loader};

use super::Example;
use crate::components::ui::badge::{Badge, BadgeVariant};
use crate::components::ui::button::{Button, ButtonSize, ButtonVariant};
use crate::components::ui::empty::{
    Empty, EmptyContent, EmptyDescription, EmptyHeader, EmptyMedia, EmptyMediaVariant, EmptyTitle,
};
use crate::components::ui::input_group::{
    InputGroup, InputGroupAddon, InputGroupAlign, InputGroupButton, InputGroupInput,
    InputGroupTextarea,
};
use crate::components::ui::item::{Item, ItemContent, ItemMedia, ItemTitle, ItemVariant};
use crate::components::ui::spinner::Spinner;

#[component]
pub fn Examples() -> Element {
    rsx! {
        div { class: "flex flex-col gap-10",
            SpinnerDemo {}
            SpinnerCustom {}
            SpinnerSize {}
            SpinnerButton {}
            SpinnerBadge {}
            SpinnerInputGroup {}
            SpinnerEmpty {}
            SpinnerRtl {}
        }
    }
}

#[component]
fn SpinnerDemo() -> Element {
    rsx! {
        Example { title: "Demo",
            div { class: "flex w-full max-w-xs flex-col gap-4 [--radius:1rem]",
                Item { variant: ItemVariant::Muted,
                    ItemMedia {
                        Spinner {}
                    }
                    ItemContent {
                        ItemTitle { class: "line-clamp-1", "Processing payment..." }
                    }
                    ItemContent { class: "flex-none justify-end",
                        span { class: "text-sm tabular-nums", "$100.00" }
                    }
                }
            }
        }
    }
}

#[component]
fn SpinnerCustom() -> Element {
    rsx! {
        Example { title: "Custom",
            div { class: "flex items-center gap-4",
                span { role: "status", aria_label: "Loading", class: "contents",
                    Loader { class: "size-4 animate-spin" }
                }
            }
        }
    }
}

#[component]
fn SpinnerSize() -> Element {
    rsx! {
        Example { title: "Size",
            div { class: "flex items-center gap-6",
                Spinner { class: "size-3" }
                Spinner { class: "size-4" }
                Spinner { class: "size-6" }
                Spinner { class: "size-8" }
            }
        }
    }
}

#[component]
fn SpinnerButton() -> Element {
    rsx! {
        Example { title: "Button",
            div { class: "flex flex-col items-center gap-4",
                Button { disabled: true, size: ButtonSize::Sm,
                    span { "data-icon": "inline-start", class: "contents",
                        Spinner {}
                    }
                    "Loading..."
                }
                Button { variant: ButtonVariant::Outline, disabled: true, size: ButtonSize::Sm,
                    span { "data-icon": "inline-start", class: "contents",
                        Spinner {}
                    }
                    "Please wait"
                }
                Button { variant: ButtonVariant::Secondary, disabled: true, size: ButtonSize::Sm,
                    span { "data-icon": "inline-start", class: "contents",
                        Spinner {}
                    }
                    "Processing"
                }
            }
        }
    }
}

#[component]
fn SpinnerBadge() -> Element {
    rsx! {
        Example { title: "Badge",
            div { class: "flex items-center gap-4 [--radius:1.2rem]",
                Badge {
                    span { "data-icon": "inline-start", class: "contents",
                        Spinner {}
                    }
                    "Syncing"
                }
                Badge { variant: BadgeVariant::Secondary,
                    span { "data-icon": "inline-start", class: "contents",
                        Spinner {}
                    }
                    "Updating"
                }
                Badge { variant: BadgeVariant::Outline,
                    span { "data-icon": "inline-start", class: "contents",
                        Spinner {}
                    }
                    "Processing"
                }
            }
        }
    }
}

#[component]
fn SpinnerInputGroup() -> Element {
    rsx! {
        Example { title: "Input Group",
            div { class: "flex w-full max-w-md flex-col gap-4",
                InputGroup {
                    InputGroupInput { placeholder: "Send a message...", disabled: true }
                    InputGroupAddon { align: InputGroupAlign::InlineEnd,
                        Spinner {}
                    }
                }
                InputGroup {
                    InputGroupTextarea { placeholder: "Send a message...", disabled: true }
                    InputGroupAddon { align: InputGroupAlign::BlockEnd,
                        Spinner {}
                        " Validating..."
                        InputGroupButton { class: "ml-auto", variant: ButtonVariant::Default,
                            ArrowUp {}
                            span { class: "sr-only", "Send" }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn SpinnerEmpty() -> Element {
    rsx! {
        Example { title: "Empty",
            Empty { class: "w-full",
                EmptyHeader {
                    EmptyMedia { variant: EmptyMediaVariant::Icon,
                        Spinner {}
                    }
                    EmptyTitle { "Processing your request" }
                    EmptyDescription {
                        "Please wait while we process your request. Do not refresh the page."
                    }
                }
                EmptyContent {
                    Button { variant: ButtonVariant::Outline, size: ButtonSize::Sm, "Cancel" }
                }
            }
        }
    }
}

#[component]
fn SpinnerRtl() -> Element {
    rsx! {
        Example { title: "RTL",
            div { class: "flex w-full max-w-xs flex-col gap-4 [--radius:1rem]", dir: "rtl",
                Item { variant: ItemVariant::Muted, dir: "rtl",
                    ItemMedia {
                        Spinner {}
                    }
                    ItemContent {
                        ItemTitle { class: "line-clamp-1", "جاري معالجة الدفع..." }
                    }
                    ItemContent { class: "flex-none justify-end",
                        span { class: "text-sm tabular-nums", "١٠٠.٠٠ دولار" }
                    }
                }
            }
        }
    }
}
