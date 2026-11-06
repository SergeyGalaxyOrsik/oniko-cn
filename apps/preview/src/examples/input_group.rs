//! Input Group examples, ported from shadcn/ui `examples/base/input-group-*.tsx`.

use dioxus::prelude::*;
use lucide_dioxus::{
    Check, ChevronDown, Copy, CornerDownLeft, CreditCard, Ellipsis, EyeOff, FileCode, Info, Loader,
    Mail, RefreshCw, Search, Star,
};

use super::Example;
use crate::components::ui::button::{ButtonSize, ButtonVariant};
use crate::components::ui::dropdown_menu::{
    DropdownMenu, DropdownMenuAlign, DropdownMenuContent, DropdownMenuGroup, DropdownMenuItem,
    DropdownMenuTrigger,
};
use crate::components::ui::field::{Field, FieldDescription, FieldGroup, FieldLabel};
use crate::components::ui::input_group::{
    InputGroup, InputGroupAddon, InputGroupAlign, InputGroupButton, InputGroupButtonSize,
    InputGroupInput, InputGroupText, InputGroupTextarea, input_group_button_variants,
};
use crate::components::ui::kbd::Kbd;
use crate::components::ui::popover::{Popover, PopoverAlign, PopoverContent, PopoverTrigger};
use crate::components::ui::spinner::Spinner;

#[component]
pub fn Examples() -> Element {
    rsx! {
        InputGroupDemo {}
        InputGroupInlineStart {}
        InputGroupInlineEnd {}
        InputGroupBlockStart {}
        InputGroupBlockEnd {}
        InputGroupIcon {}
        InputGroupTextExample {}
        InputGroupButtonExample {}
        InputGroupKbd {}
        InputGroupDropdown {}
        InputGroupSpinner {}
        InputGroupTextareaExample {}
        InputGroupCustom {}
        InputGroupRtl {}
    }
}

#[component]
fn InputGroupDemo() -> Element {
    rsx! {
        Example { title: "Demo", class: "h-[26rem]",
            InputGroup { class: "max-w-xs",
                InputGroupInput { placeholder: "Search..." }
                InputGroupAddon {
                    Search { size: 16 }
                }
                InputGroupAddon { align: InputGroupAlign::InlineEnd, "12 results" }
            }
        }
    }
}

#[component]
fn InputGroupInlineStart() -> Element {
    rsx! {
        Example { title: "inline-start", class: "h-48",
            Field { class: "max-w-sm",
                FieldLabel { r#for: "inline-start-input", "Input" }
                InputGroup {
                    InputGroupInput { id: "inline-start-input", placeholder: "Search..." }
                    InputGroupAddon { align: InputGroupAlign::InlineStart,
                        Search { size: 16, class: "text-muted-foreground" }
                    }
                }
                FieldDescription { "Icon positioned at the start." }
            }
        }
    }
}

#[component]
fn InputGroupInlineEnd() -> Element {
    rsx! {
        Example { title: "inline-end", class: "h-48",
            Field { class: "max-w-sm",
                FieldLabel { r#for: "inline-end-input", "Input" }
                InputGroup {
                    InputGroupInput {
                        id: "inline-end-input",
                        r#type: "password",
                        placeholder: "Enter password",
                    }
                    InputGroupAddon { align: InputGroupAlign::InlineEnd,
                        EyeOff { size: 16 }
                    }
                }
                FieldDescription { "Icon positioned at the end." }
            }
        }
    }
}

#[component]
fn InputGroupBlockStart() -> Element {
    rsx! {
        Example { title: "block-start", class: "h-96",
            FieldGroup { class: "max-w-sm",
                Field {
                    FieldLabel { r#for: "block-start-input", "Input" }
                    InputGroup { class: "h-auto",
                        InputGroupInput { id: "block-start-input", placeholder: "Enter your name" }
                        InputGroupAddon { align: InputGroupAlign::BlockStart,
                            InputGroupText { "Full Name" }
                        }
                    }
                    FieldDescription { "Header positioned above the input." }
                }
                Field {
                    FieldLabel { r#for: "block-start-textarea", "Textarea" }
                    InputGroup {
                        InputGroupTextarea {
                            id: "block-start-textarea",
                            placeholder: "console.log('Hello, world!');",
                            class: "font-mono text-sm",
                        }
                        InputGroupAddon { align: InputGroupAlign::BlockStart,
                            FileCode { size: 16, class: "text-muted-foreground" }
                            InputGroupText { class: "font-mono", "script.js" }
                            InputGroupButton { size: InputGroupButtonSize::IconXs, class: "ml-auto",
                                Copy { size: 16 }
                                span { class: "sr-only", "Copy" }
                            }
                        }
                    }
                    FieldDescription { "Header positioned above the textarea." }
                }
            }
        }
    }
}

#[component]
fn InputGroupBlockEnd() -> Element {
    rsx! {
        Example { title: "block-end", class: "h-[26rem]",
            FieldGroup { class: "max-w-sm",
                Field {
                    FieldLabel { r#for: "block-end-input", "Input" }
                    InputGroup { class: "h-auto",
                        InputGroupInput { id: "block-end-input", placeholder: "Enter amount" }
                        InputGroupAddon { align: InputGroupAlign::BlockEnd,
                            InputGroupText { "USD" }
                        }
                    }
                    FieldDescription { "Footer positioned below the input." }
                }
                Field {
                    FieldLabel { r#for: "block-end-textarea", "Textarea" }
                    InputGroup {
                        InputGroupTextarea {
                            id: "block-end-textarea",
                            placeholder: "Write a comment...",
                        }
                        InputGroupAddon { align: InputGroupAlign::BlockEnd,
                            InputGroupText { "0/280" }
                            InputGroupButton {
                                variant: ButtonVariant::Default,
                                size: InputGroupButtonSize::Sm,
                                class: "ml-auto",
                                "Post"
                            }
                        }
                    }
                    FieldDescription { "Footer positioned below the textarea." }
                }
            }
        }
    }
}

#[component]
fn InputGroupIcon() -> Element {
    rsx! {
        Example { title: "Icon", class: "h-80",
            div { class: "grid w-full max-w-sm gap-6",
                InputGroup {
                    InputGroupInput { placeholder: "Search..." }
                    InputGroupAddon {
                        Search { size: 16 }
                    }
                }
                InputGroup {
                    InputGroupInput { r#type: "email", placeholder: "Enter your email" }
                    InputGroupAddon {
                        Mail { size: 16 }
                    }
                }
                InputGroup {
                    InputGroupInput { placeholder: "Card number" }
                    InputGroupAddon {
                        CreditCard { size: 16 }
                    }
                    InputGroupAddon { align: InputGroupAlign::InlineEnd,
                        Check { size: 16 }
                    }
                }
                InputGroup {
                    InputGroupInput { placeholder: "Card number" }
                    InputGroupAddon { align: InputGroupAlign::InlineEnd,
                        Star { size: 16 }
                        Info { size: 16 }
                    }
                }
            }
        }
    }
}

#[component]
fn InputGroupTextExample() -> Element {
    rsx! {
        Example { title: "Text", class: "h-80",
            div { class: "grid w-full max-w-sm gap-6",
                InputGroup {
                    InputGroupAddon {
                        InputGroupText { "$" }
                    }
                    InputGroupInput { placeholder: "0.00" }
                    InputGroupAddon { align: InputGroupAlign::InlineEnd,
                        InputGroupText { "USD" }
                    }
                }
                InputGroup {
                    InputGroupAddon {
                        InputGroupText { "https://" }
                    }
                    InputGroupInput { placeholder: "example.com", class: "pl-0.5!" }
                    InputGroupAddon { align: InputGroupAlign::InlineEnd,
                        InputGroupText { ".com" }
                    }
                }
                InputGroup {
                    InputGroupInput { placeholder: "Enter your username" }
                    InputGroupAddon { align: InputGroupAlign::InlineEnd,
                        InputGroupText { "@company.com" }
                    }
                }
                InputGroup {
                    InputGroupTextarea { placeholder: "Enter your message" }
                    InputGroupAddon { align: InputGroupAlign::BlockEnd,
                        InputGroupText { class: "text-xs text-muted-foreground", "120 characters left" }
                    }
                }
            }
        }
    }
}

#[component]
fn InputGroupButtonExample() -> Element {
    let mut copied = use_signal(|| false);
    let mut favorite = use_signal(|| false);

    rsx! {
        Example { title: "Button", class: "h-72",
            div { class: "grid w-full max-w-sm gap-6",
                InputGroup {
                    InputGroupInput { placeholder: "https://x.com/shadcn", readonly: true }
                    InputGroupAddon { align: InputGroupAlign::InlineEnd,
                        InputGroupButton {
                            "aria-label": "Copy",
                            title: "Copy",
                            size: InputGroupButtonSize::IconXs,
                            onclick: move |_| copied.set(true),
                            if copied() {
                                Check { size: 16 }
                            } else {
                                Copy { size: 16 }
                            }
                        }
                    }
                }
                InputGroup { class: "[--radius:9999px]",
                    InputGroupAddon {
                        Popover {
                            PopoverTrigger {
                                class: input_group_button_variants(
                                    ButtonVariant::Secondary,
                                    InputGroupButtonSize::IconXs,
                                    None,
                                ),
                                Info { size: 16 }
                            }
                            PopoverContent {
                                align: PopoverAlign::Start,
                                class: "flex flex-col gap-1 rounded-xl text-sm",
                                p { class: "font-medium", "Your connection is not secure." }
                                p { "You should not enter any sensitive information on this site." }
                            }
                        }
                    }
                    InputGroupAddon { class: "pl-1.5 text-muted-foreground", "https://" }
                    InputGroupInput { id: "input-secure-19" }
                    InputGroupAddon { align: InputGroupAlign::InlineEnd,
                        InputGroupButton {
                            size: InputGroupButtonSize::IconXs,
                            onclick: move |_| favorite.set(!favorite()),
                            Star {
                                size: 16,
                                class: if favorite() { "fill-blue-600 stroke-blue-600" } else { "" },
                            }
                        }
                    }
                }
                InputGroup {
                    InputGroupInput { placeholder: "Type to search..." }
                    InputGroupAddon { align: InputGroupAlign::InlineEnd,
                        InputGroupButton { variant: ButtonVariant::Secondary, "Search" }
                    }
                }
            }
        }
    }
}

#[component]
fn InputGroupKbd() -> Element {
    rsx! {
        Example { title: "Kbd", class: "h-40",
            InputGroup { class: "max-w-sm",
                InputGroupInput { placeholder: "Search..." }
                InputGroupAddon {
                    Search { size: 16, class: "text-muted-foreground" }
                }
                InputGroupAddon { align: InputGroupAlign::InlineEnd,
                    Kbd { "⌘K" }
                }
            }
        }
    }
}

#[component]
fn InputGroupDropdown() -> Element {
    rsx! {
        Example { title: "Dropdown", class: "h-56",
            div { class: "grid w-full max-w-sm gap-4",
                InputGroup {
                    InputGroupInput { placeholder: "Enter file name" }
                    InputGroupAddon { align: InputGroupAlign::InlineEnd,
                        DropdownMenu {
                            DropdownMenuTrigger {
                                variant: ButtonVariant::Ghost,
                                size: ButtonSize::Default,
                                class: input_group_button_variants(
                                    ButtonVariant::Ghost,
                                    InputGroupButtonSize::IconXs,
                                    None,
                                ),
                                "aria-label": "More",
                                Ellipsis { size: 16 }
                            }
                            DropdownMenuContent { align: DropdownMenuAlign::End,
                                DropdownMenuGroup {
                                    DropdownMenuItem { "Settings" }
                                    DropdownMenuItem { "Copy path" }
                                    DropdownMenuItem { "Open location" }
                                }
                            }
                        }
                    }
                }
                InputGroup {
                    InputGroupInput { placeholder: "Enter search query" }
                    InputGroupAddon { align: InputGroupAlign::InlineEnd,
                        DropdownMenu {
                            DropdownMenuTrigger {
                                variant: ButtonVariant::Ghost,
                                size: ButtonSize::Default,
                                class: input_group_button_variants(
                                    ButtonVariant::Ghost,
                                    InputGroupButtonSize::Xs,
                                    Some("pr-1.5! text-xs"),
                                ),
                                "Search In... "
                                ChevronDown { size: 12 }
                            }
                            DropdownMenuContent { align: DropdownMenuAlign::End,
                                DropdownMenuGroup {
                                    DropdownMenuItem { "Documentation" }
                                    DropdownMenuItem { "Blog Posts" }
                                    DropdownMenuItem { "Changelog" }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn InputGroupSpinner() -> Element {
    rsx! {
        Example { title: "Spinner", class: "h-80",
            div { class: "grid w-full max-w-sm gap-4",
                InputGroup {
                    InputGroupInput { placeholder: "Searching..." }
                    InputGroupAddon { align: InputGroupAlign::InlineEnd,
                        Spinner {}
                    }
                }
                InputGroup {
                    InputGroupInput { placeholder: "Processing..." }
                    InputGroupAddon {
                        Spinner {}
                    }
                }
                InputGroup {
                    InputGroupInput { placeholder: "Saving changes..." }
                    InputGroupAddon { align: InputGroupAlign::InlineEnd,
                        InputGroupText { "Saving..." }
                        Spinner {}
                    }
                }
                InputGroup {
                    InputGroupInput { placeholder: "Refreshing data..." }
                    InputGroupAddon {
                        Loader { size: 16, class: "animate-spin" }
                    }
                    InputGroupAddon { align: InputGroupAlign::InlineEnd,
                        InputGroupText { class: "text-muted-foreground", "Please wait..." }
                    }
                }
            }
        }
    }
}

#[component]
fn InputGroupTextareaExample() -> Element {
    rsx! {
        Example { title: "Textarea", class: "h-96",
            div { class: "grid w-full max-w-md gap-4",
                InputGroup {
                    InputGroupTextarea {
                        id: "textarea-code-32",
                        placeholder: "console.log('Hello, world!');",
                        class: "min-h-[200px]",
                    }
                    InputGroupAddon { align: InputGroupAlign::BlockEnd, class: "border-t",
                        InputGroupText { "Line 1, Column 1" }
                        InputGroupButton {
                            size: InputGroupButtonSize::Sm,
                            class: "ml-auto",
                            variant: ButtonVariant::Default,
                            "Run "
                            CornerDownLeft { size: 16 }
                        }
                    }
                    InputGroupAddon { align: InputGroupAlign::BlockStart, class: "border-b",
                        InputGroupText { class: "font-mono font-medium",
                            FileCode { size: 16 }
                            "script.js"
                        }
                        InputGroupButton { class: "ml-auto", size: InputGroupButtonSize::IconXs,
                            RefreshCw { size: 16 }
                        }
                        InputGroupButton {
                            variant: ButtonVariant::Ghost,
                            size: InputGroupButtonSize::IconXs,
                            Copy { size: 16 }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn InputGroupCustom() -> Element {
    rsx! {
        Example { title: "Custom Input", class: "h-56",
            div { class: "grid w-full max-w-sm gap-6",
                InputGroup {
                    textarea {
                        "data-slot": "input-group-control",
                        class: "flex field-sizing-content min-h-16 w-full resize-none rounded-md bg-transparent px-3 py-2.5 text-base transition-[color,box-shadow] outline-none md:text-sm",
                        placeholder: "Autoresize textarea...",
                    }
                    InputGroupAddon { align: InputGroupAlign::BlockEnd,
                        InputGroupButton {
                            class: "ml-auto",
                            size: InputGroupButtonSize::Sm,
                            variant: ButtonVariant::Default,
                            "Submit"
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn InputGroupRtl() -> Element {
    rsx! {
        Example { title: "RTL", class: "h-[30rem]",
            div { class: "grid w-full max-w-sm gap-6", dir: "rtl",
                InputGroup { class: "max-w-xs",
                    InputGroupInput { placeholder: "بحث..." }
                    InputGroupAddon {
                        Search { size: 16 }
                    }
                    InputGroupAddon { align: InputGroupAlign::InlineEnd, "١٢ نتيجة" }
                }
                InputGroup {
                    InputGroupInput { placeholder: "جاري البحث..." }
                    InputGroupAddon { align: InputGroupAlign::InlineEnd,
                        Spinner {}
                    }
                }
                InputGroup {
                    InputGroupInput { placeholder: "جاري حفظ التغييرات..." }
                    InputGroupAddon { align: InputGroupAlign::InlineEnd,
                        InputGroupText { "جاري الحفظ..." }
                        Spinner {}
                    }
                }
                FieldGroup { class: "max-w-sm",
                    Field {
                        FieldLabel { r#for: "rtl-textarea", "منطقة النص" }
                        InputGroup {
                            InputGroupTextarea { id: "rtl-textarea", placeholder: "اكتب تعليقًا..." }
                            InputGroupAddon { align: InputGroupAlign::BlockEnd,
                                InputGroupText { "٠/٢٨٠" }
                                InputGroupButton {
                                    variant: ButtonVariant::Default,
                                    size: InputGroupButtonSize::Sm,
                                    class: "ms-auto",
                                    "نشر"
                                }
                            }
                        }
                        FieldDescription { "تذييل موضع أسفل منطقة النص." }
                    }
                }
            }
        }
    }
}
