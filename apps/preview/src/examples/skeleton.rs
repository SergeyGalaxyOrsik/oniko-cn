//! Skeleton examples, ported from shadcn/ui `examples/base/skeleton-*.tsx`.

use dioxus::prelude::*;

use super::Example;
use crate::components::ui::card::{Card, CardContent, CardHeader};
use crate::components::ui::skeleton::Skeleton;

#[component]
pub fn Examples() -> Element {
    rsx! {
        div { class: "flex flex-col gap-10",
            SkeletonDemo {}
            SkeletonAvatar {}
            SkeletonCard {}
            SkeletonText {}
            SkeletonForm {}
            SkeletonTable {}
            SkeletonRtl {}
        }
    }
}

#[component]
fn SkeletonDemo() -> Element {
    rsx! {
        Example { title: "Demo",
            div { class: "flex items-center gap-4",
                Skeleton { class: "h-12 w-12 rounded-full" }
                div { class: "space-y-2",
                    Skeleton { class: "h-4 w-[250px]" }
                    Skeleton { class: "h-4 w-[200px]" }
                }
            }
        }
    }
}

#[component]
fn SkeletonAvatar() -> Element {
    rsx! {
        Example { title: "Avatar",
            div { class: "flex w-fit items-center gap-4",
                Skeleton { class: "size-10 shrink-0 rounded-full" }
                div { class: "grid gap-2",
                    Skeleton { class: "h-4 w-[150px]" }
                    Skeleton { class: "h-4 w-[100px]" }
                }
            }
        }
    }
}

#[component]
fn SkeletonCard() -> Element {
    rsx! {
        Example { title: "Card", class: "h-80",
            Card { class: "w-full max-w-xs",
                CardHeader {
                    Skeleton { class: "h-4 w-2/3" }
                    Skeleton { class: "h-4 w-1/2" }
                }
                CardContent {
                    Skeleton { class: "aspect-video w-full" }
                }
            }
        }
    }
}

#[component]
fn SkeletonText() -> Element {
    rsx! {
        Example { title: "Text",
            div { class: "flex w-full max-w-xs flex-col gap-2",
                Skeleton { class: "h-4 w-full" }
                Skeleton { class: "h-4 w-full" }
                Skeleton { class: "h-4 w-3/4" }
            }
        }
    }
}

#[component]
fn SkeletonForm() -> Element {
    rsx! {
        Example { title: "Form",
            div { class: "flex w-full max-w-xs flex-col gap-7",
                div { class: "flex flex-col gap-3",
                    Skeleton { class: "h-4 w-20" }
                    Skeleton { class: "h-8 w-full" }
                }
                div { class: "flex flex-col gap-3",
                    Skeleton { class: "h-4 w-24" }
                    Skeleton { class: "h-8 w-full" }
                }
                Skeleton { class: "h-8 w-24" }
            }
        }
    }
}

#[component]
fn SkeletonTable() -> Element {
    rsx! {
        Example { title: "Table",
            div { class: "flex w-full max-w-sm flex-col gap-2",
                for index in 0..5 {
                    div { key: "{index}", class: "flex gap-4",
                        Skeleton { class: "h-4 flex-1" }
                        Skeleton { class: "h-4 w-24" }
                        Skeleton { class: "h-4 w-20" }
                    }
                }
            }
        }
    }
}

#[component]
fn SkeletonRtl() -> Element {
    rsx! {
        Example { title: "RTL",
            div { class: "flex items-center gap-4", dir: "rtl",
                Skeleton { class: "h-12 w-12 rounded-full" }
                div { class: "space-y-2",
                    Skeleton { class: "h-4 w-[250px]" }
                    Skeleton { class: "h-4 w-[200px]" }
                }
            }
        }
    }
}
