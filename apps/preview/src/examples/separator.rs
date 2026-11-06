//! Separator examples, ported from shadcn/ui `examples/base/separator-*.tsx`.

use dioxus::prelude::*;

use super::Example;
use crate::components::ui::separator::{Separator, SeparatorOrientation};

#[component]
pub fn Examples() -> Element {
    rsx! {
        div { class: "flex flex-col gap-10",
            SeparatorDemo {}
            SeparatorVertical {}
            SeparatorMenu {}
            SeparatorList {}
            SeparatorRtl {}
        }
    }
}

#[component]
fn SeparatorDemo() -> Element {
    rsx! {
        Example { title: "Demo",
            div { class: "flex max-w-sm flex-col gap-4 text-sm",
                div { class: "flex flex-col gap-1.5",
                    div { class: "leading-none font-medium", "shadcn/ui" }
                    div { class: "text-muted-foreground", "The Foundation for your Design System" }
                }
                Separator {}
                div {
                    "A set of beautifully designed components that you can customize, extend, and build on."
                }
            }
        }
    }
}

#[component]
fn SeparatorVertical() -> Element {
    rsx! {
        Example { title: "Vertical",
            div { class: "flex h-5 items-center gap-4 text-sm",
                div { "Blog" }
                Separator { orientation: SeparatorOrientation::Vertical }
                div { "Docs" }
                Separator { orientation: SeparatorOrientation::Vertical }
                div { "Source" }
            }
        }
    }
}

#[component]
fn SeparatorMenu() -> Element {
    rsx! {
        Example { title: "Menu",
            div { class: "flex items-center gap-2 text-sm md:gap-4",
                div { class: "flex flex-col gap-1",
                    span { class: "font-medium", "Settings" }
                    span { class: "text-xs text-muted-foreground", "Manage preferences" }
                }
                Separator { orientation: SeparatorOrientation::Vertical }
                div { class: "flex flex-col gap-1",
                    span { class: "font-medium", "Account" }
                    span { class: "text-xs text-muted-foreground", "Profile & security" }
                }
                Separator { orientation: SeparatorOrientation::Vertical, class: "hidden md:block" }
                div { class: "hidden flex-col gap-1 md:flex",
                    span { class: "font-medium", "Help" }
                    span { class: "text-xs text-muted-foreground", "Support & docs" }
                }
            }
        }
    }
}

#[component]
fn SeparatorList() -> Element {
    rsx! {
        Example { title: "List",
            div { class: "flex w-full max-w-sm flex-col gap-2 text-sm",
                dl { class: "flex items-center justify-between",
                    dt { "Item 1" }
                    dd { class: "text-muted-foreground", "Value 1" }
                }
                Separator {}
                dl { class: "flex items-center justify-between",
                    dt { "Item 2" }
                    dd { class: "text-muted-foreground", "Value 2" }
                }
                Separator {}
                dl { class: "flex items-center justify-between",
                    dt { "Item 3" }
                    dd { class: "text-muted-foreground", "Value 3" }
                }
            }
        }
    }
}

#[component]
fn SeparatorRtl() -> Element {
    rsx! {
        Example { title: "RTL",
            div { class: "flex max-w-sm flex-col gap-4 text-sm", dir: "rtl",
                div { class: "flex flex-col gap-1.5",
                    div { class: "leading-none font-medium", "shadcn/ui" }
                    div { class: "text-muted-foreground", "الأساس لنظام التصميم الخاص بك" }
                }
                Separator {}
                div { "مجموعة من المكونات المصممة بشكل جميل يمكنك تخصيصها وتوسيعها والبناء عليها." }
            }
        }
    }
}
