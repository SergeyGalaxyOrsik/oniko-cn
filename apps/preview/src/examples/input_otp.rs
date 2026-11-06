//! Input OTP examples, ported from shadcn/ui `examples/base/input-otp-*.tsx`.

use dioxus::prelude::*;
use lucide_dioxus::RefreshCw;

use super::Example;
use crate::components::ui::button::{Button, ButtonSize, ButtonVariant};
use crate::components::ui::card::{
    Card, CardContent, CardDescription, CardFooter, CardHeader, CardTitle,
};
use crate::components::ui::field::{Field, FieldDescription, FieldLabel};
use crate::components::ui::input_otp::{
    InputOTP, InputOTPGroup, InputOTPSeparator, InputOTPSlot, OtpPattern,
};

#[component]
pub fn Examples() -> Element {
    rsx! {
        InputOtpDemo {}
        InputOtpPattern {}
        InputOtpWithSeparator {}
        InputOtpDisabled {}
        InputOtpControlled {}
        InputOtpInvalid {}
        InputOtpFourDigits {}
        InputOtpAlphanumeric {}
        InputOtpForm {}
        InputOtpRtl {}
    }
}

#[component]
fn InputOtpDemo() -> Element {
    rsx! {
        Example { title: "Demo",
            InputOTP { max_length: 6, default_value: "123456",
                InputOTPGroup {
                    InputOTPSlot { index: 0 }
                    InputOTPSlot { index: 1 }
                    InputOTPSlot { index: 2 }
                    InputOTPSlot { index: 3 }
                    InputOTPSlot { index: 4 }
                    InputOTPSlot { index: 5 }
                }
            }
        }
    }
}

#[component]
fn InputOtpPattern() -> Element {
    rsx! {
        Example { title: "Pattern",
            Field { class: "w-fit",
                FieldLabel { r#for: "digits-only", "Digits Only" }
                InputOTP { id: "digits-only", max_length: 6, pattern: OtpPattern::Digits,
                    InputOTPGroup {
                        InputOTPSlot { index: 0 }
                        InputOTPSlot { index: 1 }
                        InputOTPSlot { index: 2 }
                        InputOTPSlot { index: 3 }
                        InputOTPSlot { index: 4 }
                        InputOTPSlot { index: 5 }
                    }
                }
            }
        }
    }
}

#[component]
fn InputOtpWithSeparator() -> Element {
    rsx! {
        Example { title: "Separator",
            InputOTP { max_length: 6,
                InputOTPGroup {
                    InputOTPSlot { index: 0 }
                    InputOTPSlot { index: 1 }
                }
                InputOTPSeparator {}
                InputOTPGroup {
                    InputOTPSlot { index: 2 }
                    InputOTPSlot { index: 3 }
                }
                InputOTPSeparator {}
                InputOTPGroup {
                    InputOTPSlot { index: 4 }
                    InputOTPSlot { index: 5 }
                }
            }
        }
    }
}

#[component]
fn InputOtpDisabled() -> Element {
    rsx! {
        Example { title: "Disabled",
            InputOTP {
                id: "disabled",
                max_length: 6,
                disabled: true,
                value: "123456".to_string(),
                InputOTPGroup {
                    InputOTPSlot { index: 0 }
                    InputOTPSlot { index: 1 }
                    InputOTPSlot { index: 2 }
                }
                InputOTPSeparator {}
                InputOTPGroup {
                    InputOTPSlot { index: 3 }
                    InputOTPSlot { index: 4 }
                    InputOTPSlot { index: 5 }
                }
            }
        }
    }
}

#[component]
fn InputOtpControlled() -> Element {
    let mut value = use_signal(String::new);

    rsx! {
        Example { title: "Controlled",
            div { class: "space-y-2",
                InputOTP {
                    max_length: 6,
                    value: value(),
                    onchange: move |next| value.set(next),
                    InputOTPGroup {
                        InputOTPSlot { index: 0 }
                        InputOTPSlot { index: 1 }
                        InputOTPSlot { index: 2 }
                        InputOTPSlot { index: 3 }
                        InputOTPSlot { index: 4 }
                        InputOTPSlot { index: 5 }
                    }
                }
                div { class: "text-center text-sm",
                    if value().is_empty() {
                        "Enter your one-time password."
                    } else {
                        "You entered: {value}"
                    }
                }
            }
        }
    }
}

#[component]
fn InputOtpInvalid() -> Element {
    let mut value = use_signal(|| "000000".to_string());

    rsx! {
        Example { title: "Invalid",
            InputOTP {
                max_length: 6,
                value: value(),
                onchange: move |next| value.set(next),
                InputOTPGroup {
                    InputOTPSlot { index: 0, "aria-invalid": "true" }
                    InputOTPSlot { index: 1, "aria-invalid": "true" }
                }
                InputOTPSeparator {}
                InputOTPGroup {
                    InputOTPSlot { index: 2, "aria-invalid": "true" }
                    InputOTPSlot { index: 3, "aria-invalid": "true" }
                }
                InputOTPSeparator {}
                InputOTPGroup {
                    InputOTPSlot { index: 4, "aria-invalid": "true" }
                    InputOTPSlot { index: 5, "aria-invalid": "true" }
                }
            }
        }
    }
}

#[component]
fn InputOtpFourDigits() -> Element {
    rsx! {
        Example { title: "Four Digits",
            InputOTP { max_length: 4, pattern: OtpPattern::Digits,
                InputOTPGroup {
                    InputOTPSlot { index: 0 }
                    InputOTPSlot { index: 1 }
                    InputOTPSlot { index: 2 }
                    InputOTPSlot { index: 3 }
                }
            }
        }
    }
}

#[component]
fn InputOtpAlphanumeric() -> Element {
    rsx! {
        Example { title: "Alphanumeric",
            InputOTP { max_length: 6, pattern: OtpPattern::DigitsAndChars,
                InputOTPGroup {
                    InputOTPSlot { index: 0 }
                    InputOTPSlot { index: 1 }
                    InputOTPSlot { index: 2 }
                }
                InputOTPSeparator {}
                InputOTPGroup {
                    InputOTPSlot { index: 3 }
                    InputOTPSlot { index: 4 }
                    InputOTPSlot { index: 5 }
                }
            }
        }
    }
}

#[component]
fn InputOtpForm() -> Element {
    const SLOT_CLASS: &str = "*:data-[slot=input-otp-slot]:h-12 *:data-[slot=input-otp-slot]:w-11 *:data-[slot=input-otp-slot]:text-xl";

    rsx! {
        Example { title: "Form", class: "h-[30rem]",
            Card { class: "mx-auto max-w-md",
                CardHeader {
                    CardTitle { "Verify your login" }
                    CardDescription {
                        "Enter the verification code we sent to your email address: "
                        span { class: "font-medium", "m@example.com" }
                        "."
                    }
                }
                CardContent {
                    Field {
                        div { class: "flex items-center justify-between",
                            FieldLabel { r#for: "otp-verification", "Verification code" }
                            Button { variant: ButtonVariant::Outline, size: ButtonSize::Xs,
                                RefreshCw { size: 16 }
                                "Resend Code"
                            }
                        }
                        InputOTP { max_length: 6, id: "otp-verification", required: true,
                            InputOTPGroup { class: SLOT_CLASS,
                                InputOTPSlot { index: 0 }
                                InputOTPSlot { index: 1 }
                                InputOTPSlot { index: 2 }
                            }
                            InputOTPSeparator { class: "mx-2" }
                            InputOTPGroup { class: SLOT_CLASS,
                                InputOTPSlot { index: 3 }
                                InputOTPSlot { index: 4 }
                                InputOTPSlot { index: 5 }
                            }
                        }
                        FieldDescription {
                            a { href: "#", "I no longer have access to this email address." }
                        }
                    }
                }
                CardFooter {
                    Field {
                        Button { r#type: "submit", class: "w-full", "Verify" }
                        div { class: "text-sm text-muted-foreground",
                            "Having trouble signing in? "
                            a {
                                href: "#",
                                class: "underline underline-offset-4 transition-colors hover:text-primary",
                                "Contact support"
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn InputOtpRtl() -> Element {
    rsx! {
        Example { title: "RTL",
            Field { class: "mx-auto max-w-xs",
                FieldLabel { r#for: "input-otp-rtl", "رمز التحقق" }
                InputOTP {
                    max_length: 6,
                    default_value: "123456",
                    dir: "rtl",
                    id: "input-otp-rtl",
                    InputOTPGroup {
                        InputOTPSlot { index: 0 }
                        InputOTPSlot { index: 1 }
                        InputOTPSlot { index: 2 }
                        InputOTPSlot { index: 3 }
                        InputOTPSlot { index: 4 }
                        InputOTPSlot { index: 5 }
                    }
                }
            }
        }
    }
}
