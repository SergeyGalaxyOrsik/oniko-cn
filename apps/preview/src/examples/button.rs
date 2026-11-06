//! Button examples, ported from shadcn/ui `examples/base/button-*.tsx`.

use dioxus::prelude::*;
use lucide_dioxus::{
    ArrowRight, ArrowUp, ArrowUpRight, CircleFadingArrowUp, GitBranch, GitFork, Plus,
};

use super::Example;
use super::button_group::ButtonGroupDemo;
use crate::components::ui::button::{Button, ButtonSize, ButtonVariant, button_variants};
use crate::components::ui::spinner::Spinner;

#[component]
pub fn Examples() -> Element {
    rsx! {
        div { class: "flex flex-col gap-10",
            ButtonDemo {}
            ButtonSizeExample {}
            ButtonDefault {}
            ButtonOutline {}
            ButtonSecondary {}
            ButtonGhost {}
            ButtonDestructive {}
            ButtonLink {}
            ButtonIcon {}
            ButtonWithIcon {}
            ButtonRounded {}
            ButtonSpinner {}
            Example { title: "Button Group", ButtonGroupDemo {} }
            ButtonRender {}
            ButtonRtl {}
        }
    }
}

#[component]
fn ButtonDemo() -> Element {
    rsx! {
        Example { title: "Demo",
            div { class: "flex flex-wrap items-center gap-2 md:flex-row",
                Button { variant: ButtonVariant::Outline, "Button" }
                Button { variant: ButtonVariant::Outline, size: ButtonSize::Icon, aria_label: "Submit",
                    ArrowUp {}
                }
            }
        }
    }
}

#[component]
fn ButtonSizeExample() -> Element {
    rsx! {
        Example { title: "Size",
            div { class: "flex flex-col items-start gap-8 sm:flex-row",
                div { class: "flex items-start gap-2",
                    Button { size: ButtonSize::Xs, variant: ButtonVariant::Outline, "Extra Small" }
                    Button { size: ButtonSize::IconXs, aria_label: "Submit", variant: ButtonVariant::Outline,
                        ArrowUpRight {}
                    }
                }
                div { class: "flex items-start gap-2",
                    Button { size: ButtonSize::Sm, variant: ButtonVariant::Outline, "Small" }
                    Button { size: ButtonSize::IconSm, aria_label: "Submit", variant: ButtonVariant::Outline,
                        ArrowUpRight {}
                    }
                }
                div { class: "flex items-start gap-2",
                    Button { variant: ButtonVariant::Outline, "Default" }
                    Button { size: ButtonSize::Icon, aria_label: "Submit", variant: ButtonVariant::Outline,
                        ArrowUpRight {}
                    }
                }
                div { class: "flex items-start gap-2",
                    Button { variant: ButtonVariant::Outline, size: ButtonSize::Lg, "Large" }
                    Button { size: ButtonSize::IconLg, aria_label: "Submit", variant: ButtonVariant::Outline,
                        ArrowUpRight {}
                    }
                }
            }
        }
    }
}

#[component]
fn ButtonDefault() -> Element {
    rsx! {
        Example { title: "Default", Button { "Button" } }
    }
}

#[component]
fn ButtonOutline() -> Element {
    rsx! {
        Example { title: "Outline",
            Button { variant: ButtonVariant::Outline, "Outline" }
        }
    }
}

#[component]
fn ButtonSecondary() -> Element {
    rsx! {
        Example { title: "Secondary",
            Button { variant: ButtonVariant::Secondary, "Secondary" }
        }
    }
}

#[component]
fn ButtonGhost() -> Element {
    rsx! {
        Example { title: "Ghost",
            Button { variant: ButtonVariant::Ghost, "Ghost" }
        }
    }
}

#[component]
fn ButtonDestructive() -> Element {
    rsx! {
        Example { title: "Destructive",
            Button { variant: ButtonVariant::Destructive, "Destructive" }
        }
    }
}

#[component]
fn ButtonLink() -> Element {
    rsx! {
        Example { title: "Link",
            Button { variant: ButtonVariant::Link, "Link" }
        }
    }
}

#[component]
fn ButtonIcon() -> Element {
    rsx! {
        Example { title: "Icon",
            Button { variant: ButtonVariant::Outline, size: ButtonSize::Icon,
                CircleFadingArrowUp {}
            }
        }
    }
}

#[component]
fn ButtonWithIcon() -> Element {
    rsx! {
        Example { title: "With Icon",
            div { class: "flex gap-2",
                Button { variant: ButtonVariant::Outline,
                    span { "data-icon": "inline-start", class: "contents",
                        GitBranch {}
                    }
                    " New Branch"
                }
                Button { variant: ButtonVariant::Outline,
                    "Fork"
                    span { "data-icon": "inline-end", class: "contents",
                        GitFork {}
                    }
                }
            }
        }
    }
}

#[component]
fn ButtonRounded() -> Element {
    rsx! {
        Example { title: "Rounded",
            div { class: "flex gap-2",
                Button { class: "rounded-full", "Get Started" }
                Button { variant: ButtonVariant::Outline, size: ButtonSize::Icon, class: "rounded-full",
                    ArrowUp {}
                }
            }
        }
    }
}

#[component]
fn ButtonSpinner() -> Element {
    rsx! {
        Example { title: "Spinner",
            div { class: "flex gap-2",
                Button { variant: ButtonVariant::Outline, disabled: true,
                    span { "data-icon": "inline-start", class: "contents",
                        Spinner {}
                    }
                    "Generating"
                }
                Button { variant: ButtonVariant::Secondary, disabled: true,
                    "Downloading"
                    span { "data-icon": "inline-start", class: "contents",
                        Spinner {}
                    }
                }
            }
        }
    }
}

#[component]
fn ButtonRender() -> Element {
    rsx! {
        Example { title: "Link",
            a {
                href: "#",
                class: button_variants(ButtonVariant::Secondary, ButtonSize::Sm, None),
                "Login"
            }
        }
    }
}

#[component]
fn ButtonRtl() -> Element {
    rsx! {
        Example { title: "RTL",
            div { class: "flex flex-wrap items-center gap-2 md:flex-row", dir: "rtl",
                Button { variant: ButtonVariant::Outline, "زر" }
                Button { variant: ButtonVariant::Destructive, "حذف" }
                Button { variant: ButtonVariant::Outline,
                    "إرسال "
                    span { "data-icon": "inline-end", class: "contents",
                        ArrowRight { class: "rtl:rotate-180" }
                    }
                }
                Button { variant: ButtonVariant::Outline, size: ButtonSize::Icon, aria_label: "Add",
                    Plus {}
                }
                Button { variant: ButtonVariant::Secondary, disabled: true,
                    span { "data-icon": "inline-start", class: "contents",
                        Spinner {}
                    }
                    " جاري التحميل"
                }
            }
        }
    }
}
