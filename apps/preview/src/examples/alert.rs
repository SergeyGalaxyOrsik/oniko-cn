//! Alert examples, ported from shadcn/ui `examples/base/alert-*.tsx`.

use dioxus::prelude::*;
use lucide_dioxus::{CircleAlert, CircleCheck, Info, TriangleAlert};

use super::Example;
use crate::components::ui::alert::{
    Alert, AlertAction, AlertDescription, AlertTitle, AlertVariant,
};
use crate::components::ui::button::{Button, ButtonSize};

const PREVIEW: &str = "h-auto sm:h-72 p-6";

#[component]
pub fn Examples() -> Element {
    rsx! {
        div { class: "flex flex-col gap-10",
            AlertDemo {}
            AlertBasic {}
            AlertDestructive {}
            AlertActionExample {}
            AlertColors {}
            AlertRtl {}
        }
    }
}

#[component]
fn AlertDemo() -> Element {
    rsx! {
        Example { title: "Demo", class: PREVIEW,
            div { class: "grid w-full max-w-md items-start gap-4",
                Alert {
                    CircleCheck {}
                    AlertTitle { "Payment successful" }
                    AlertDescription {
                        "Your payment of $29.99 has been processed. A receipt has been sent to your email address."
                    }
                }
                Alert {
                    Info {}
                    AlertTitle { "New feature available" }
                    AlertDescription {
                        "We've added dark mode support. You can enable it in your account settings."
                    }
                }
            }
        }
    }
}

#[component]
fn AlertBasic() -> Element {
    rsx! {
        Example { title: "Basic", class: PREVIEW,
            Alert { class: "max-w-md",
                CircleCheck {}
                AlertTitle { "Account updated successfully" }
                AlertDescription {
                    "Your profile information has been saved. Changes will be reflected immediately."
                }
            }
        }
    }
}

#[component]
fn AlertDestructive() -> Element {
    rsx! {
        Example { title: "Destructive", class: PREVIEW,
            Alert { variant: AlertVariant::Destructive, class: "max-w-md",
                CircleAlert {}
                AlertTitle { "Payment failed" }
                AlertDescription {
                    "Your payment could not be processed. Please check your payment method and try again."
                }
            }
        }
    }
}

#[component]
fn AlertActionExample() -> Element {
    rsx! {
        Example { title: "Action", class: PREVIEW,
            Alert { class: "max-w-md",
                AlertTitle { "Dark mode is now available" }
                AlertDescription { "Enable it under your profile settings to get started." }
                AlertAction {
                    Button { size: ButtonSize::Xs, "Enable" }
                }
            }
        }
    }
}

#[component]
fn AlertColors() -> Element {
    rsx! {
        Example { title: "Colors", class: PREVIEW,
            Alert { class: "max-w-md border-amber-200 bg-amber-50 text-amber-900 dark:border-amber-900 dark:bg-amber-950 dark:text-amber-50",
                TriangleAlert {}
                AlertTitle { "Your subscription will expire in 3 days." }
                AlertDescription {
                    "Renew now to avoid service interruption or upgrade to a paid plan to continue using the service."
                }
            }
        }
    }
}

#[component]
fn AlertRtl() -> Element {
    rsx! {
        Example { title: "RTL", class: PREVIEW,
            div { class: "grid w-full max-w-md items-start gap-4", dir: "rtl",
                Alert {
                    CircleCheck {}
                    AlertTitle { "تم الدفع بنجاح" }
                    AlertDescription {
                        "تمت معالجة دفعتك البالغة 29.99 دولارًا. تم إرسال إيصال إلى عنوان بريدك الإلكتروني."
                    }
                }
                Alert {
                    Info {}
                    AlertTitle { "ميزة جديدة متاحة" }
                    AlertDescription {
                        "لقد أضفنا دعم الوضع الداكن. يمكنك تفعيله في إعدادات حسابك."
                    }
                }
            }
        }
    }
}
