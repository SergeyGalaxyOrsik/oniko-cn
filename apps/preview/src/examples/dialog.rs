//! Dialog examples, ported from shadcn/ui `examples/base/dialog-*.tsx`.

use dioxus::prelude::*;

use super::Example;
use crate::components::ui::button::{Button, ButtonVariant};
use crate::components::ui::dialog::{
    Dialog, DialogClose, DialogContent, DialogDescription, DialogFooter, DialogHeader, DialogTitle,
    DialogTrigger,
};
use crate::components::ui::field::{Field, FieldGroup};
use crate::components::ui::input::Input;
use crate::components::ui::label::Label;

const LOREM: &str = "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.";

#[component]
pub fn Examples() -> Element {
    rsx! {
        div { class: "flex flex-col gap-10",
            DialogDemo {}
            DialogCloseButton {}
            DialogNoCloseButton {}
            DialogStickyFooter {}
            DialogScrollableContent {}
            DialogRtl {}
        }
    }
}

#[component]
fn DialogDemo() -> Element {
    rsx! {
        Example { title: "Demo",
            Dialog {
                DialogTrigger { variant: ButtonVariant::Outline, "Open Dialog" }
                DialogContent { class: "sm:max-w-sm",
                    DialogHeader {
                        DialogTitle { "Edit profile" }
                        DialogDescription { "Make changes to your profile here. Click save when you're done." }
                    }
                    FieldGroup {
                        Field {
                            Label { r#for: "name-1", "Name" }
                            Input { id: "name-1", name: "name", value: "Pedro Duarte" }
                        }
                        Field {
                            Label { r#for: "username-1", "Username" }
                            Input { id: "username-1", name: "username", value: "@peduarte" }
                        }
                    }
                    DialogFooter {
                        DialogClose { variant: ButtonVariant::Outline, "Cancel" }
                        Button { r#type: "submit", "Save changes" }
                    }
                }
            }
        }
    }
}

#[component]
fn DialogCloseButton() -> Element {
    rsx! {
        Example { title: "Close Button",
            Dialog {
                DialogTrigger { variant: ButtonVariant::Outline, "Share" }
                DialogContent { class: "sm:max-w-md",
                    DialogHeader {
                        DialogTitle { "Share link" }
                        DialogDescription { "Anyone who has this link will be able to view this." }
                    }
                    div { class: "flex items-center gap-2",
                        div { class: "grid flex-1 gap-2",
                            Label { r#for: "link", class: "sr-only", "Link" }
                            Input {
                                id: "link",
                                value: "https://ui.shadcn.com/docs/installation",
                                readonly: true,
                            }
                        }
                    }
                    DialogFooter { class: "sm:justify-start",
                        DialogClose { variant: ButtonVariant::Default, "Close" }
                    }
                }
            }
        }
    }
}

#[component]
fn DialogNoCloseButton() -> Element {
    rsx! {
        Example { title: "No Close Button",
            Dialog {
                DialogTrigger { variant: ButtonVariant::Outline, "No Close Button" }
                DialogContent { show_close_button: false,
                    DialogHeader {
                        DialogTitle { "No Close Button" }
                        DialogDescription { "This dialog doesn't have a close button in the top-right corner." }
                    }
                }
            }
        }
    }
}

#[component]
fn DialogStickyFooter() -> Element {
    rsx! {
        Example { title: "Sticky Footer",
            Dialog {
                DialogTrigger { variant: ButtonVariant::Outline, "Sticky Footer" }
                DialogContent {
                    DialogHeader {
                        DialogTitle { "Sticky Footer" }
                        DialogDescription { "This dialog has a sticky footer that stays visible while the content scrolls." }
                    }
                    div { class: "-mx-4 no-scrollbar max-h-[50vh] overflow-y-auto px-4",
                        for index in 0..10 {
                            p { key: "{index}", class: "mb-4 leading-normal", {LOREM} }
                        }
                    }
                    DialogFooter {
                        DialogClose { variant: ButtonVariant::Outline, "Close" }
                    }
                }
            }
        }
    }
}

#[component]
fn DialogScrollableContent() -> Element {
    rsx! {
        Example { title: "Scrollable Content",
            Dialog {
                DialogTrigger { variant: ButtonVariant::Outline, "Scrollable Content" }
                DialogContent {
                    DialogHeader {
                        DialogTitle { "Scrollable Content" }
                        DialogDescription { "This is a dialog with scrollable content." }
                    }
                    div { class: "-mx-4 no-scrollbar max-h-[50vh] overflow-y-auto px-4",
                        for index in 0..10 {
                            p { key: "{index}", class: "mb-4 leading-normal", {LOREM} }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn DialogRtl() -> Element {
    rsx! {
        Example { title: "RTL",
            Dialog {
                DialogTrigger { variant: ButtonVariant::Outline, "فتح الحوار" }
                DialogContent { class: "sm:max-w-sm", dir: "rtl", "data-lang": "ar",
                    DialogHeader {
                        DialogTitle { "تعديل الملف الشخصي" }
                        DialogDescription { "قم بإجراء تغييرات على ملفك الشخصي هنا. انقر فوق حفظ عند الانتهاء." }
                    }
                    FieldGroup {
                        Field {
                            Label { r#for: "name-1", "الاسم" }
                            Input { id: "name-1", name: "name", value: "Pedro Duarte" }
                        }
                        Field {
                            Label { r#for: "username-1", "اسم المستخدم" }
                            Input { id: "username-1", name: "username", value: "@peduarte" }
                        }
                    }
                    DialogFooter {
                        DialogClose { variant: ButtonVariant::Outline, "إلغاء" }
                        Button { r#type: "submit", "حفظ التغييرات" }
                    }
                }
            }
        }
    }
}
