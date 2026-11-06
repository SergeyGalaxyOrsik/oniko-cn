//! Direction examples, ported from shadcn/ui `examples/base/card-rtl.tsx`, the only preview on the docs page.

use dioxus::prelude::*;

use super::Example;
use crate::components::ui::button::{Button, ButtonVariant};
use crate::components::ui::card::{
    Card, CardAction, CardContent, CardDescription, CardFooter, CardHeader, CardTitle,
};
use crate::components::ui::direction::{Direction, DirectionProvider};
use crate::components::ui::input::Input;
use crate::components::ui::label::Label;

#[component]
pub fn Examples() -> Element {
    rsx! {
        div { class: "flex flex-col gap-10", CardRtl {} }
    }
}

#[component]
fn CardRtl() -> Element {
    rsx! {
        Example { title: "Card RTL", class: "h-auto",
            DirectionProvider { direction: Direction::Rtl,
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
                                    Label { r#for: "email-dir", "البريد الإلكتروني" }
                                    Input {
                                        id: "email-dir",
                                        r#type: "email",
                                        placeholder: "m@example.com",
                                        required: true,
                                    }
                                }
                                div { class: "grid gap-2",
                                    div { class: "flex items-center",
                                        Label { r#for: "password-dir", "كلمة المرور" }
                                        a {
                                            href: "#",
                                            class: "ms-auto inline-block text-sm underline-offset-4 hover:underline",
                                            "نسيت كلمة المرور؟"
                                        }
                                    }
                                    Input { id: "password-dir", r#type: "password", required: true }
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
}
