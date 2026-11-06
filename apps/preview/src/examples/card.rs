//! Card examples, ported from shadcn/ui `examples/base/card-*.tsx`.

use dioxus::prelude::*;
use lucide_dioxus::ChevronRight;

use super::Example;
use crate::components::ui::badge::{Badge, BadgeVariant};
use crate::components::ui::button::{Button, ButtonSize, ButtonVariant};
use crate::components::ui::card::{
    Card, CardAction, CardContent, CardDescription, CardFooter, CardHeader, CardSize, CardTitle,
};
use crate::components::ui::input::Input;
use crate::components::ui::label::Label;
use crate::components::ui::toggle::{ToggleSize, ToggleVariant};
use crate::components::ui::toggle_group::{ToggleGroup, ToggleGroupItem};

#[component]
pub fn Examples() -> Element {
    rsx! {
        div { class: "flex flex-col gap-10",
            CardDemo {}
            CardSmall {}
            CardSpacing {}
            CardEdgeToEdge {}
            CardImage {}
            CardRtl {}
        }
    }
}

#[component]
fn CardDemo() -> Element {
    rsx! {
        Example { title: "Demo", class: "h-[30rem]",
            Card { class: "w-full max-w-sm",
                CardHeader {
                    CardTitle { "Login to your account" }
                    CardDescription { "Enter your email below to login to your account" }
                    CardAction {
                        Button { variant: ButtonVariant::Link, "Sign Up" }
                    }
                }
                CardContent {
                    form {
                        div { class: "flex flex-col gap-6",
                            div { class: "grid gap-2",
                                Label { r#for: "email", "Email" }
                                Input {
                                    id: "email",
                                    r#type: "email",
                                    placeholder: "m@example.com",
                                    required: true,
                                }
                            }
                            div { class: "grid gap-2",
                                div { class: "flex items-center",
                                    Label { r#for: "password", "Password" }
                                    a {
                                        href: "#",
                                        class: "ml-auto inline-block text-sm underline-offset-4 hover:underline",
                                        "Forgot your password?"
                                    }
                                }
                                Input { id: "password", r#type: "password", required: true }
                            }
                        }
                    }
                }
                CardFooter { class: "flex-col gap-2",
                    Button { class: "w-full", "Login" }
                    Button { variant: ButtonVariant::Outline, class: "w-full", "Login with Google" }
                }
            }
        }
    }
}

#[component]
fn CardSmall() -> Element {
    let feature_name = "Scheduled reports";
    let items = [
        "Choose a schedule (daily, or weekly).",
        "Send to channels or specific teammates.",
        "Include charts, tables, and key metrics.",
    ];
    rsx! {
        Example { title: "Small", class: "h-96",
            Card { size: CardSize::Sm, class: "mx-auto w-full max-w-xs",
                CardHeader {
                    CardTitle { "{feature_name}" }
                    CardDescription { "Weekly snapshots. No more manual exports." }
                }
                CardContent {
                    ul { class: "grid gap-2 py-2 text-sm",
                        for item in items {
                            li { class: "flex gap-2",
                                ChevronRight { class: "mt-0.5 size-4 shrink-0 text-muted-foreground" }
                                span { "{item}" }
                            }
                        }
                    }
                }
                CardFooter { class: "flex-col gap-2",
                    Button { size: ButtonSize::Sm, class: "w-full", "Set up scheduled reports" }
                    Button {
                        variant: ButtonVariant::Outline,
                        size: ButtonSize::Sm,
                        class: "w-full",
                        "See what's new"
                    }
                }
            }
        }
    }
}

#[component]
fn CardSpacing() -> Element {
    let options = [
        ("[--card-spacing:--spacing(4)]", "16px", "4"),
        ("[--card-spacing:--spacing(5)]", "20px", "5"),
        ("[--card-spacing:--spacing(6)]", "24px", "6"),
        ("[--card-spacing:--spacing(8)]", "32px", "8"),
    ];
    let mut spacing = use_signal(|| "4".to_string());
    let selected = options
        .iter()
        .find(|(_, _, value)| *value == spacing())
        .map(|(class, _, _)| *class)
        .unwrap_or_default();

    rsx! {
        Example { title: "Spacing", class: "h-[34rem]",
            div { class: "mx-auto grid w-full max-w-sm gap-4",
                ToggleGroup {
                    value: vec![spacing()],
                    on_value_change: move |value: Vec<String>| {
                        if let Some(first) = value.into_iter().next() {
                            spacing.set(first);
                        }
                    },
                    variant: ToggleVariant::Outline,
                    size: ToggleSize::Sm,
                    class: "justify-center",
                    for (_ , label , value) in options {
                        ToggleGroupItem { key: "{value}", value: value.to_string(), "{label}" }
                    }
                }
                Card { class: selected,
                    CardHeader {
                        CardTitle { "Login to your account" }
                        CardDescription { "Enter your email below to login to your account" }
                        CardAction {
                            Button { variant: ButtonVariant::Link, "Sign Up" }
                        }
                    }
                    CardContent {
                        form {
                            div { class: "flex flex-col gap-6",
                                div { class: "grid gap-2",
                                    Label { r#for: "email-spacing", "Email" }
                                    Input {
                                        id: "email-spacing",
                                        r#type: "email",
                                        placeholder: "m@example.com",
                                        required: true,
                                    }
                                }
                                div { class: "grid gap-2",
                                    div { class: "flex items-center",
                                        Label { r#for: "password-spacing", "Password" }
                                        a {
                                            href: "#",
                                            class: "ml-auto inline-block text-sm underline-offset-4 hover:underline",
                                            "Forgot your password?"
                                        }
                                    }
                                    Input { id: "password-spacing", r#type: "password", required: true }
                                }
                            }
                        }
                    }
                    CardFooter { class: "flex-col gap-2",
                        Button { class: "w-full", "Login" }
                        Button { variant: ButtonVariant::Outline, class: "w-full", "Login with Google" }
                    }
                }
            }
        }
    }
}

#[component]
fn CardEdgeToEdge() -> Element {
    rsx! {
        Example { title: "Edge to Edge", class: "h-[24rem]",
            Card { class: "mx-auto w-full max-w-sm",
                CardHeader {
                    CardTitle { "Terms of Service" }
                    CardDescription { "Review the terms before accepting the agreement." }
                }
                CardContent { class: "-mb-(--card-spacing)",
                    div { class: "-mx-(--card-spacing) max-h-48 space-y-4 overflow-y-scroll border-t bg-muted/50 px-(--card-spacing) py-4 text-sm leading-relaxed",
                        p {
                            "These terms govern your use of the workspace, including access to shared documents, project files, and collaboration tools."
                        }
                        p {
                            "You are responsible for the content you upload and for ensuring that your team has the appropriate permissions to view or edit it."
                        }
                        p {
                            "We may update features or limits as the service evolves. When those changes materially affect your workflow, we will notify your workspace administrators."
                        }
                        p {
                            "By continuing, you agree to keep your account credentials secure and to follow your organization's acceptable use policies."
                        }
                    }
                }
                CardFooter { class: "justify-end gap-2",
                    Button { variant: ButtonVariant::Outline, "Decline" }
                    Button { "Accept" }
                }
            }
        }
    }
}

#[component]
fn CardImage() -> Element {
    rsx! {
        Example { title: "Image", class: "h-[32rem]",
            Card { class: "relative mx-auto w-full max-w-sm pt-0",
                div { class: "absolute inset-0 z-30 aspect-video bg-black/35" }
                img {
                    src: "https://avatar.vercel.sh/shadcn1",
                    alt: "Event cover",
                    class: "relative z-20 aspect-video w-full object-cover brightness-60 grayscale dark:brightness-40",
                }
                CardHeader {
                    CardAction {
                        Badge { variant: BadgeVariant::Secondary, "Featured" }
                    }
                    CardTitle { "Design systems meetup" }
                    CardDescription {
                        "A practical talk on component APIs, accessibility, and shipping faster."
                    }
                }
                CardFooter {
                    Button { class: "w-full", "View Event" }
                }
            }
        }
    }
}

#[component]
fn CardRtl() -> Element {
    rsx! {
        Example { title: "RTL", class: "h-[30rem]",
            Card { class: "w-full max-w-sm", dir: "rtl",
                CardHeader {
                    CardTitle { "تسجيل الدخول إلى حسابك" }
                    CardDescription { "أدخل بريدك الإلكتروني أدناه لتسجيل الدخول إلى حسابك" }
                    CardAction {
                        Button { variant: ButtonVariant::Link, "إنشاء حساب" }
                    }
                }
                CardContent {
                    form {
                        div { class: "flex flex-col gap-6",
                            div { class: "grid gap-2",
                                Label { r#for: "email-rtl", "البريد الإلكتروني" }
                                Input {
                                    id: "email-rtl",
                                    r#type: "email",
                                    placeholder: "m@example.com",
                                    required: true,
                                }
                            }
                            div { class: "grid gap-2",
                                div { class: "flex items-center",
                                    Label { r#for: "password-rtl", "كلمة المرور" }
                                    a {
                                        href: "#",
                                        class: "ms-auto inline-block text-sm underline-offset-4 hover:underline",
                                        "نسيت كلمة المرور؟"
                                    }
                                }
                                Input { id: "password-rtl", r#type: "password", required: true }
                            }
                        }
                    }
                }
                CardFooter { class: "flex-col gap-2",
                    Button { class: "w-full", "تسجيل الدخول" }
                    Button { variant: ButtonVariant::Outline, class: "w-full", "تسجيل الدخول باستخدام Google" }
                }
            }
        }
    }
}
