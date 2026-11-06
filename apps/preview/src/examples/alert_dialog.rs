//! AlertDialog examples, ported from shadcn/ui `examples/base/alert-dialog-*.tsx`.

use dioxus::prelude::*;
use lucide_dioxus::{Bluetooth, CircleFadingPlus, Trash};

use super::Example;
use crate::components::ui::alert_dialog::{
    AlertDialog, AlertDialogAction, AlertDialogCancel, AlertDialogContent, AlertDialogDescription,
    AlertDialogFooter, AlertDialogHeader, AlertDialogMedia, AlertDialogSize, AlertDialogTitle,
    AlertDialogTrigger,
};
use crate::components::ui::button::ButtonVariant;

#[component]
pub fn Examples() -> Element {
    rsx! {
        div { class: "flex flex-col gap-10",
            AlertDialogDemo {}
            AlertDialogBasic {}
            AlertDialogSmall {}
            AlertDialogWithMedia {}
            AlertDialogSmallWithMedia {}
            AlertDialogDestructive {}
            AlertDialogRtl {}
        }
    }
}

#[component]
fn AlertDialogDemo() -> Element {
    rsx! {
        Example { title: "Demo", class: "h-56",
            AlertDialog {
                AlertDialogTrigger { variant: ButtonVariant::Outline, "Show Dialog" }
                AlertDialogContent {
                    AlertDialogHeader {
                        AlertDialogTitle { "Are you absolutely sure?" }
                        AlertDialogDescription {
                            "This action cannot be undone. This will permanently delete your account from our servers."
                        }
                    }
                    AlertDialogFooter {
                        AlertDialogCancel { "Cancel" }
                        AlertDialogAction { "Continue" }
                    }
                }
            }
        }
    }
}

#[component]
fn AlertDialogBasic() -> Element {
    rsx! {
        Example { title: "Basic", class: "h-56",
            AlertDialog {
                AlertDialogTrigger { variant: ButtonVariant::Outline, "Show Dialog" }
                AlertDialogContent {
                    AlertDialogHeader {
                        AlertDialogTitle { "Are you absolutely sure?" }
                        AlertDialogDescription {
                            "This action cannot be undone. This will permanently delete your account and remove your data from our servers."
                        }
                    }
                    AlertDialogFooter {
                        AlertDialogCancel { "Cancel" }
                        AlertDialogAction { "Continue" }
                    }
                }
            }
        }
    }
}

#[component]
fn AlertDialogSmall() -> Element {
    rsx! {
        Example { title: "Small", class: "h-56",
            AlertDialog {
                AlertDialogTrigger { variant: ButtonVariant::Outline, "Show Dialog" }
                AlertDialogContent { size: AlertDialogSize::Sm,
                    AlertDialogHeader {
                        AlertDialogTitle { "Allow accessory to connect?" }
                        AlertDialogDescription {
                            "Do you want to allow the USB accessory to connect to this device?"
                        }
                    }
                    AlertDialogFooter {
                        AlertDialogCancel { "Don't allow" }
                        AlertDialogAction { "Allow" }
                    }
                }
            }
        }
    }
}

#[component]
fn AlertDialogWithMedia() -> Element {
    rsx! {
        Example { title: "Media", class: "h-56",
            AlertDialog {
                AlertDialogTrigger { variant: ButtonVariant::Outline, "Share Project" }
                AlertDialogContent {
                    AlertDialogHeader {
                        AlertDialogMedia {
                            CircleFadingPlus {}
                        }
                        AlertDialogTitle { "Share this project?" }
                        AlertDialogDescription {
                            "Anyone with the link will be able to view and edit this project."
                        }
                    }
                    AlertDialogFooter {
                        AlertDialogCancel { "Cancel" }
                        AlertDialogAction { "Share" }
                    }
                }
            }
        }
    }
}

#[component]
fn AlertDialogSmallWithMedia() -> Element {
    rsx! {
        Example { title: "Small with Media", class: "h-56",
            AlertDialog {
                AlertDialogTrigger { variant: ButtonVariant::Outline, "Show Dialog" }
                AlertDialogContent { size: AlertDialogSize::Sm,
                    AlertDialogHeader {
                        AlertDialogMedia {
                            Bluetooth {}
                        }
                        AlertDialogTitle { "Allow accessory to connect?" }
                        AlertDialogDescription {
                            "Do you want to allow the USB accessory to connect to this device?"
                        }
                    }
                    AlertDialogFooter {
                        AlertDialogCancel { "Don't allow" }
                        AlertDialogAction { "Allow" }
                    }
                }
            }
        }
    }
}

#[component]
fn AlertDialogDestructive() -> Element {
    rsx! {
        Example { title: "Destructive", class: "h-56",
            AlertDialog {
                AlertDialogTrigger { variant: ButtonVariant::Destructive, "Delete Chat" }
                AlertDialogContent { size: AlertDialogSize::Sm,
                    AlertDialogHeader {
                        AlertDialogMedia { class: "bg-destructive/10 text-destructive dark:bg-destructive/20 dark:text-destructive",
                            Trash {}
                        }
                        AlertDialogTitle { "Delete chat?" }
                        AlertDialogDescription {
                            "This will permanently delete this chat conversation. View "
                            a { href: "#", "Settings" }
                            " delete any memories saved during this chat."
                        }
                    }
                    AlertDialogFooter {
                        AlertDialogCancel { variant: ButtonVariant::Outline, "Cancel" }
                        AlertDialogAction { variant: ButtonVariant::Destructive, "Delete" }
                    }
                }
            }
        }
    }
}

#[component]
fn AlertDialogRtl() -> Element {
    rsx! {
        Example { title: "RTL", class: "h-56",
            div { class: "flex gap-4", dir: "rtl",
                AlertDialog {
                    AlertDialogTrigger { variant: ButtonVariant::Outline, "إظهار الحوار" }
                    AlertDialogContent { dir: "rtl", "data-lang": "ar",
                        AlertDialogHeader {
                            AlertDialogTitle { "هل أنت متأكد تمامًا؟" }
                            AlertDialogDescription {
                                "لا يمكن التراجع عن هذا الإجراء. سيؤدي هذا إلى حذف حسابك نهائيًا من خوادمنا."
                            }
                        }
                        AlertDialogFooter {
                            AlertDialogCancel { "إلغاء" }
                            AlertDialogAction { "متابعة" }
                        }
                    }
                }
                AlertDialog {
                    AlertDialogTrigger { variant: ButtonVariant::Outline, "إظهار الحوار (صغير)" }
                    AlertDialogContent { size: AlertDialogSize::Sm, dir: "rtl", "data-lang": "ar",
                        AlertDialogHeader {
                            AlertDialogMedia {
                                Bluetooth {}
                            }
                            AlertDialogTitle { "السماح للملحق بالاتصال؟" }
                            AlertDialogDescription {
                                "هل تريد السماح لملحق USB بالاتصال بهذا الجهاز؟"
                            }
                        }
                        AlertDialogFooter {
                            AlertDialogCancel { "عدم السماح" }
                            AlertDialogAction { "السماح" }
                        }
                    }
                }
            }
        }
    }
}
