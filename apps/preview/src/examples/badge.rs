//! Badge examples, ported from shadcn/ui `examples/base/badge-*.tsx`.

use dioxus::prelude::*;
use lucide_dioxus::{ArrowUpRight, BadgeCheck, Bookmark};

use super::Example;
use crate::components::ui::badge::{Badge, BadgeVariant, badge_variants};
use crate::components::ui::spinner::Spinner;

#[component]
pub fn Examples() -> Element {
    rsx! {
        div { class: "flex flex-col gap-10",
            BadgeDemo {}
            BadgeVariants {}
            BadgeWithIconLeft {}
            BadgeWithSpinner {}
            BadgeAsLink {}
            BadgeCustomColors {}
            BadgeRtl {}
        }
    }
}

#[component]
fn BadgeDemo() -> Element {
    rsx! {
        Example { title: "Demo",
            div { class: "flex w-full flex-wrap justify-center gap-2",
                Badge { "Badge" }
                Badge { variant: BadgeVariant::Secondary, "Secondary" }
                Badge { variant: BadgeVariant::Destructive, "Destructive" }
                Badge { variant: BadgeVariant::Outline, "Outline" }
            }
        }
    }
}

#[component]
fn BadgeVariants() -> Element {
    rsx! {
        Example { title: "Variants",
            div { class: "flex flex-wrap gap-2",
                Badge { "Default" }
                Badge { variant: BadgeVariant::Secondary, "Secondary" }
                Badge { variant: BadgeVariant::Destructive, "Destructive" }
                Badge { variant: BadgeVariant::Outline, "Outline" }
                Badge { variant: BadgeVariant::Ghost, "Ghost" }
            }
        }
    }
}

#[component]
fn BadgeWithIconLeft() -> Element {
    rsx! {
        Example { title: "With Icon",
            div { class: "flex flex-wrap gap-2",
                Badge { variant: BadgeVariant::Secondary,
                    span { "data-icon": "inline-start", class: "contents",
                        BadgeCheck {}
                    }
                    "Verified"
                }
                Badge { variant: BadgeVariant::Outline,
                    "Bookmark"
                    span { "data-icon": "inline-end", class: "contents",
                        Bookmark {}
                    }
                }
            }
        }
    }
}

#[component]
fn BadgeWithSpinner() -> Element {
    rsx! {
        Example { title: "With Spinner",
            div { class: "flex flex-wrap gap-2",
                Badge { variant: BadgeVariant::Destructive,
                    span { "data-icon": "inline-start", class: "contents",
                        Spinner {}
                    }
                    "Deleting"
                }
                Badge { variant: BadgeVariant::Secondary,
                    "Generating"
                    span { "data-icon": "inline-end", class: "contents",
                        Spinner {}
                    }
                }
            }
        }
    }
}

#[component]
fn BadgeAsLink() -> Element {
    rsx! {
        Example { title: "Link",
            a { href: "#link", class: badge_variants(BadgeVariant::Default, None),
                "data-slot": "badge",
                "Open Link "
                span { "data-icon": "inline-end", class: "contents",
                    ArrowUpRight {}
                }
            }
        }
    }
}

#[component]
fn BadgeCustomColors() -> Element {
    rsx! {
        Example { title: "Custom Colors",
            div { class: "flex flex-wrap gap-2",
                Badge { class: "bg-blue-50 text-blue-700 dark:bg-blue-950 dark:text-blue-300", "Blue" }
                Badge { class: "bg-green-50 text-green-700 dark:bg-green-950 dark:text-green-300", "Green" }
                Badge { class: "bg-sky-50 text-sky-700 dark:bg-sky-950 dark:text-sky-300", "Sky" }
                Badge { class: "bg-purple-50 text-purple-700 dark:bg-purple-950 dark:text-purple-300", "Purple" }
                Badge { class: "bg-red-50 text-red-700 dark:bg-red-950 dark:text-red-300", "Red" }
            }
        }
    }
}

#[component]
fn BadgeRtl() -> Element {
    rsx! {
        Example { title: "RTL",
            div { class: "flex w-full flex-wrap justify-center gap-2", dir: "rtl",
                Badge { "شارة" }
                Badge { variant: BadgeVariant::Secondary, "ثانوي" }
                Badge { variant: BadgeVariant::Destructive, "مدمر" }
                Badge { variant: BadgeVariant::Outline, "مخطط" }
                Badge { variant: BadgeVariant::Secondary,
                    span { "data-icon": "inline-start", class: "contents",
                        BadgeCheck {}
                    }
                    "متحقق"
                }
                Badge { variant: BadgeVariant::Outline,
                    "إشارة مرجعية"
                    span { "data-icon": "inline-end", class: "contents",
                        Bookmark {}
                    }
                }
            }
        }
    }
}
