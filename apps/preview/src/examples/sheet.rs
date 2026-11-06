//! Sheet examples, ported from shadcn/ui `examples/base/sheet-*.tsx`.

use dioxus::prelude::*;

use super::Example;
use crate::components::ui::button::{Button, ButtonVariant};
use crate::components::ui::field::{Field, FieldGroup, FieldLabel};
use crate::components::ui::input::Input;
use crate::components::ui::label::Label;
use crate::components::ui::sheet::{
    Sheet, SheetClose, SheetContent, SheetDescription, SheetFooter, SheetHeader, SheetSide,
    SheetTitle, SheetTrigger,
};

const LOREM: &str = "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.";

#[component]
pub fn Examples() -> Element {
    rsx! {
        div { class: "flex flex-col gap-10",
            SheetDemo {}
            SheetSideExample {}
            SheetNoCloseButton {}
            SheetRtl {}
        }
    }
}

#[component]
fn SheetDemo() -> Element {
    rsx! {
        Example { title: "Demo",
            Sheet {
                SheetTrigger { variant: ButtonVariant::Outline, "Open" }
                SheetContent {
                    SheetHeader {
                        SheetTitle { "Edit profile" }
                        SheetDescription { "Make changes to your profile here. Click save when you're done." }
                    }
                    div { class: "grid flex-1 auto-rows-min gap-6 px-4",
                        div { class: "grid gap-3",
                            Label { r#for: "sheet-demo-name", "Name" }
                            Input { id: "sheet-demo-name", value: "Pedro Duarte" }
                        }
                        div { class: "grid gap-3",
                            Label { r#for: "sheet-demo-username", "Username" }
                            Input { id: "sheet-demo-username", value: "@peduarte" }
                        }
                    }
                    SheetFooter {
                        Button { r#type: "submit", "Save changes" }
                        SheetClose { variant: ButtonVariant::Outline, "Close" }
                    }
                }
            }
        }
    }
}

#[component]
fn SheetSideExample() -> Element {
    let sides = [
        ("top", SheetSide::Top),
        ("right", SheetSide::Right),
        ("bottom", SheetSide::Bottom),
        ("left", SheetSide::Left),
    ];

    rsx! {
        Example { title: "Side",
            div { class: "flex flex-wrap gap-2",
                for (name , side) in sides {
                    Sheet { key: "{name}",
                        SheetTrigger { variant: ButtonVariant::Outline, class: "capitalize", "{name}" }
                        SheetContent {
                            side,
                            class: "data-[side=bottom]:max-h-[50vh] data-[side=top]:max-h-[50vh]",
                            SheetHeader {
                                SheetTitle { "Edit profile" }
                                SheetDescription { "Make changes to your profile here. Click save when you're done." }
                            }
                            div { class: "no-scrollbar overflow-y-auto px-4",
                                for index in 0..10 {
                                    p { key: "{index}", class: "mb-2 leading-relaxed", {LOREM} }
                                }
                            }
                            SheetFooter {
                                Button { r#type: "submit", "Save changes" }
                                SheetClose { variant: ButtonVariant::Outline, "Cancel" }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn SheetNoCloseButton() -> Element {
    rsx! {
        Example { title: "No Close Button",
            Sheet {
                SheetTrigger { variant: ButtonVariant::Outline, "Open Sheet" }
                SheetContent { show_close_button: false,
                    SheetHeader {
                        SheetTitle { "No Close Button" }
                        SheetDescription {
                            "This sheet doesn't have a close button in the top-right corner. Click outside to close."
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn SheetRtl() -> Element {
    rsx! {
        Example { title: "RTL",
            Sheet {
                SheetTrigger { variant: ButtonVariant::Outline, "فتح" }
                SheetContent { side: SheetSide::Left, dir: "rtl", "data-lang": "ar",
                    SheetHeader {
                        SheetTitle { "تعديل الملف الشخصي" }
                        SheetDescription { "قم بإجراء تغييرات على ملفك الشخصي هنا. انقر حفظ عند الانتهاء." }
                    }
                    FieldGroup { class: "px-4",
                        Field {
                            FieldLabel { r#for: "sheet-rtl-name", "الاسم" }
                            Input { id: "sheet-rtl-name", value: "Pedro Duarte" }
                        }
                        Field {
                            FieldLabel { r#for: "sheet-rtl-username", "اسم المستخدم" }
                            Input { id: "sheet-rtl-username", value: "peduarte" }
                        }
                    }
                    SheetFooter {
                        Button { r#type: "submit", "حفظ التغييرات" }
                        SheetClose { variant: ButtonVariant::Outline, "إغلاق" }
                    }
                }
            }
        }
    }
}
