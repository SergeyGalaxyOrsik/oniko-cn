//! Button Group examples, ported from shadcn/ui `examples/base/button-group-*.tsx`.

use dioxus::prelude::*;
use lucide_dioxus::{
    Archive, ArrowLeft, ArrowRight, AudioLines, Bot, CalendarPlus, Check, ChevronDown, Clock, Copy,
    Ellipsis, ListFilter, MailCheck, Minus, Plus, Search, Share, Tag, Trash, TriangleAlert,
    UserRoundX, VolumeOff,
};

use super::Example;
use crate::components::ui::button::{Button, ButtonSize, ButtonVariant, button_variants};
use crate::components::ui::button_group::{
    ButtonGroup, ButtonGroupOrientation, ButtonGroupSeparator,
};
use crate::components::ui::dropdown_menu::{
    DropdownMenu, DropdownMenuAlign, DropdownMenuContent, DropdownMenuGroup, DropdownMenuItem,
    DropdownMenuItemVariant, DropdownMenuRadioGroup, DropdownMenuRadioItem, DropdownMenuSeparator,
    DropdownMenuSub, DropdownMenuSubContent, DropdownMenuSubTrigger, DropdownMenuTrigger,
};
use crate::components::ui::field::{Field, FieldDescription, FieldLabel};
use crate::components::ui::input::Input;
use crate::components::ui::input_group::{
    InputGroup, InputGroupAddon, InputGroupAlign, InputGroupButton, InputGroupButtonSize,
    InputGroupInput,
};
use crate::components::ui::popover::{
    Popover, PopoverAlign, PopoverContent, PopoverDescription, PopoverHeader, PopoverTitle,
    PopoverTrigger,
};
use crate::components::ui::select::{
    Select, SelectContent, SelectGroup, SelectItem, SelectTrigger, SelectValue,
};
use crate::components::ui::textarea::Textarea;
use crate::components::ui::tooltip::{Tooltip, TooltipContent, TooltipTrigger};

#[component]
pub fn Examples() -> Element {
    rsx! {
        div { class: "flex flex-col gap-10",
            Example { title: "Demo", ButtonGroupDemo {} }
            Example { title: "Orientation", ButtonGroupOrientationExample {} }
            Example { title: "Size", ButtonGroupSize {} }
            Example { title: "Nested", ButtonGroupNested {} }
            Example { title: "Separator", ButtonGroupSeparatorDemo {} }
            Example { title: "Split", ButtonGroupSplit {} }
            Example { title: "Input", ButtonGroupInputExample {} }
            Example { title: "Input Group", ButtonGroupInputGroup {} }
            Example { title: "Dropdown Menu", ButtonGroupDropdown {} }
            Example { title: "Select", ButtonGroupSelect {} }
            Example { title: "Popover", ButtonGroupPopover {} }
            Example { title: "RTL", ButtonGroupRtl {} }
        }
    }
}

#[component]
pub fn ButtonGroupDemo() -> Element {
    let mut label = use_signal(|| "personal".to_string());

    rsx! {
        ButtonGroup {
            ButtonGroup { class: "hidden sm:flex",
                Button { variant: ButtonVariant::Outline, size: ButtonSize::Icon, aria_label: "Go Back",
                    ArrowLeft {}
                }
            }
            ButtonGroup {
                Button { variant: ButtonVariant::Outline, "Archive" }
                Button { variant: ButtonVariant::Outline, "Report" }
            }
            ButtonGroup {
                Button { variant: ButtonVariant::Outline, "Snooze" }
                DropdownMenu {
                    DropdownMenuTrigger {
                        variant: ButtonVariant::Outline,
                        size: ButtonSize::Icon,
                        aria_label: "More Options",
                        Ellipsis {}
                    }
                    DropdownMenuContent { align: DropdownMenuAlign::End, class: "w-40",
                        DropdownMenuGroup {
                            DropdownMenuItem {
                                MailCheck {}
                                "Mark as Read"
                            }
                            DropdownMenuItem {
                                Archive {}
                                "Archive"
                            }
                        }
                        DropdownMenuSeparator {}
                        DropdownMenuGroup {
                            DropdownMenuItem {
                                Clock {}
                                "Snooze"
                            }
                            DropdownMenuItem {
                                CalendarPlus {}
                                "Add to Calendar"
                            }
                            DropdownMenuItem {
                                ListFilter {}
                                "Add to List"
                            }
                            DropdownMenuSub {
                                DropdownMenuSubTrigger {
                                    Tag {}
                                    "Label As..."
                                }
                                DropdownMenuSubContent {
                                    DropdownMenuRadioGroup {
                                        value: label(),
                                        on_value_change: move |value| label.set(value),
                                        DropdownMenuRadioItem { value: "personal", "Personal" }
                                        DropdownMenuRadioItem { value: "work", "Work" }
                                        DropdownMenuRadioItem { value: "other", "Other" }
                                    }
                                }
                            }
                        }
                        DropdownMenuSeparator {}
                        DropdownMenuGroup {
                            DropdownMenuItem { variant: DropdownMenuItemVariant::Destructive,
                                Trash {}
                                "Trash"
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn ButtonGroupOrientationExample() -> Element {
    rsx! {
        ButtonGroup {
            orientation: ButtonGroupOrientation::Vertical,
            aria_label: "Media controls",
            class: "h-fit",
            Button { variant: ButtonVariant::Outline, size: ButtonSize::Icon,
                Plus {}
            }
            Button { variant: ButtonVariant::Outline, size: ButtonSize::Icon,
                Minus {}
            }
        }
    }
}

#[component]
fn ButtonGroupSize() -> Element {
    rsx! {
        div { class: "flex flex-col items-start gap-8",
            ButtonGroup {
                Button { variant: ButtonVariant::Outline, size: ButtonSize::Sm, "Small" }
                Button { variant: ButtonVariant::Outline, size: ButtonSize::Sm, "Button" }
                Button { variant: ButtonVariant::Outline, size: ButtonSize::Sm, "Group" }
                Button { variant: ButtonVariant::Outline, size: ButtonSize::IconSm,
                    Plus {}
                }
            }
            ButtonGroup {
                Button { variant: ButtonVariant::Outline, "Default" }
                Button { variant: ButtonVariant::Outline, "Button" }
                Button { variant: ButtonVariant::Outline, "Group" }
                Button { variant: ButtonVariant::Outline, size: ButtonSize::Icon,
                    Plus {}
                }
            }
            ButtonGroup {
                Button { variant: ButtonVariant::Outline, size: ButtonSize::Lg, "Large" }
                Button { variant: ButtonVariant::Outline, size: ButtonSize::Lg, "Button" }
                Button { variant: ButtonVariant::Outline, size: ButtonSize::Lg, "Group" }
                Button { variant: ButtonVariant::Outline, size: ButtonSize::IconLg,
                    Plus {}
                }
            }
        }
    }
}

#[component]
fn ButtonGroupNested() -> Element {
    rsx! {
        ButtonGroup {
            ButtonGroup {
                Button { variant: ButtonVariant::Outline, size: ButtonSize::Icon,
                    Plus {}
                }
            }
            ButtonGroup {
                InputGroup {
                    InputGroupInput { placeholder: "Send a message..." }
                    Tooltip {
                        TooltipTrigger {
                            InputGroupAddon { align: InputGroupAlign::InlineEnd,
                                AudioLines {}
                            }
                        }
                        TooltipContent { "Voice Mode" }
                    }
                }
            }
        }
    }
}

#[component]
fn ButtonGroupSeparatorDemo() -> Element {
    rsx! {
        ButtonGroup {
            Button { variant: ButtonVariant::Secondary, size: ButtonSize::Sm, "Copy" }
            ButtonGroupSeparator {}
            Button { variant: ButtonVariant::Secondary, size: ButtonSize::Sm, "Paste" }
        }
    }
}

#[component]
fn ButtonGroupSplit() -> Element {
    rsx! {
        ButtonGroup {
            Button { variant: ButtonVariant::Secondary, "Button" }
            ButtonGroupSeparator {}
            Button { size: ButtonSize::Icon, variant: ButtonVariant::Secondary,
                Plus {}
            }
        }
    }
}

#[component]
fn ButtonGroupInputExample() -> Element {
    rsx! {
        ButtonGroup {
            Input { placeholder: "Search..." }
            Button { variant: ButtonVariant::Outline, aria_label: "Search",
                Search {}
            }
        }
    }
}

#[component]
fn ButtonGroupInputGroup() -> Element {
    let mut voice_enabled = use_signal(|| false);

    rsx! {
        ButtonGroup { class: "[--radius:9999rem]",
            ButtonGroup {
                Button { variant: ButtonVariant::Outline, size: ButtonSize::Icon,
                    Plus {}
                }
            }
            ButtonGroup {
                InputGroup {
                    InputGroupInput {
                        placeholder: if voice_enabled() { "Record and send audio..." } else { "Send a message..." },
                        disabled: voice_enabled(),
                    }
                    InputGroupAddon { align: InputGroupAlign::InlineEnd,
                        Tooltip {
                            TooltipTrigger {
                                InputGroupButton {
                                    onclick: move |_| voice_enabled.toggle(),
                                    size: InputGroupButtonSize::IconXs,
                                    "data-active": voice_enabled(),
                                    class: "data-[active=true]:bg-orange-100 data-[active=true]:text-orange-700 dark:data-[active=true]:bg-orange-800 dark:data-[active=true]:text-orange-100",
                                    aria_pressed: voice_enabled(),
                                    AudioLines {}
                                }
                            }
                            TooltipContent { "Voice Mode" }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn ButtonGroupDropdown() -> Element {
    rsx! {
        ButtonGroup {
            Button { variant: ButtonVariant::Outline, "Follow" }
            DropdownMenu {
                DropdownMenuTrigger { variant: ButtonVariant::Outline, class: "pl-2!",
                    ChevronDown {}
                }
                DropdownMenuContent { align: DropdownMenuAlign::End, class: "w-44",
                    DropdownMenuGroup {
                        DropdownMenuItem {
                            VolumeOff {}
                            "Mute Conversation"
                        }
                        DropdownMenuItem {
                            Check {}
                            "Mark as Read"
                        }
                        DropdownMenuItem {
                            TriangleAlert {}
                            "Report Conversation"
                        }
                        DropdownMenuItem {
                            UserRoundX {}
                            "Block User"
                        }
                        DropdownMenuItem {
                            Share {}
                            "Share Conversation"
                        }
                        DropdownMenuItem {
                            Copy {}
                            "Copy Conversation"
                        }
                    }
                    DropdownMenuSeparator {}
                    DropdownMenuGroup {
                        DropdownMenuItem { variant: DropdownMenuItemVariant::Destructive,
                            Trash {}
                            "Delete Conversation"
                        }
                    }
                }
            }
        }
    }
}

const CURRENCIES: [(&str, &str); 3] = [("US Dollar", "$"), ("Euro", "€"), ("British Pound", "£")];

#[component]
fn ButtonGroupSelect() -> Element {
    let mut currency = use_signal(|| "$".to_string());

    rsx! {
        ButtonGroup {
            ButtonGroup {
                Select {
                    value: currency(),
                    on_value_change: move |value: Option<String>| {
                        if let Some(value) = value {
                            currency.set(value);
                        }
                    },
                    SelectTrigger { class: "font-mono", SelectValue {} }
                    SelectContent {
                        SelectGroup {
                            for (name , symbol) in CURRENCIES {
                                SelectItem { key: "{symbol}", value: symbol, label: symbol,
                                    "{symbol} "
                                    span { class: "text-muted-foreground", "{name}" }
                                }
                            }
                        }
                    }
                }
                Input { placeholder: "10.00", pattern: "[0-9]*" }
            }
            ButtonGroup {
                Button { aria_label: "Send", size: ButtonSize::Icon, variant: ButtonVariant::Outline,
                    ArrowRight {}
                }
            }
        }
    }
}

#[component]
fn ButtonGroupPopover() -> Element {
    rsx! {
        ButtonGroup {
            Button { variant: ButtonVariant::Outline,
                Bot {}
                " Copilot"
            }
            Popover {
                PopoverTrigger {
                    aria_label: "Open Popover",
                    class: button_variants(ButtonVariant::Outline, ButtonSize::Icon, None),
                    ChevronDown {}
                }
                PopoverContent { align: PopoverAlign::End, class: "rounded-xl text-sm",
                    PopoverHeader {
                        PopoverTitle { "Start a new task with Copilot" }
                        PopoverDescription { "Describe your task in natural language." }
                    }
                    Field {
                        FieldLabel { r#for: "task", class: "sr-only", "Task Description" }
                        Textarea { id: "task", placeholder: "I need to...", class: "resize-none" }
                        FieldDescription { "Copilot will open a pull request for review." }
                    }
                }
            }
        }
    }
}

#[component]
fn ButtonGroupRtl() -> Element {
    let mut label = use_signal(|| "personal".to_string());

    rsx! {
        div { dir: "rtl",
            ButtonGroup {
                ButtonGroup { class: "hidden sm:flex",
                    Button { variant: ButtonVariant::Outline, size: ButtonSize::Icon, aria_label: "Go Back",
                        ArrowLeft { class: "rtl:rotate-180" }
                    }
                }
                ButtonGroup {
                    Button { variant: ButtonVariant::Outline, "أرشفة" }
                    Button { variant: ButtonVariant::Outline, "تقرير" }
                }
                ButtonGroup {
                    Button { variant: ButtonVariant::Outline, "تأجيل" }
                    DropdownMenu {
                        DropdownMenuTrigger {
                            variant: ButtonVariant::Outline,
                            size: ButtonSize::Icon,
                            aria_label: "More Options",
                            Ellipsis {}
                        }
                        DropdownMenuContent { class: "w-40", dir: "rtl",
                            DropdownMenuGroup {
                                DropdownMenuItem {
                                    MailCheck {}
                                    "وضع علامة كمقروء"
                                }
                                DropdownMenuItem {
                                    Archive {}
                                    "أرشفة"
                                }
                            }
                            DropdownMenuSeparator {}
                            DropdownMenuGroup {
                                DropdownMenuItem {
                                    Clock {}
                                    "تأجيل"
                                }
                                DropdownMenuItem {
                                    CalendarPlus {}
                                    "إضافة إلى التقويم"
                                }
                                DropdownMenuItem {
                                    ListFilter {}
                                    "إضافة إلى القائمة"
                                }
                                DropdownMenuSub {
                                    DropdownMenuSubTrigger {
                                        Tag {}
                                        "تصنيف كـ..."
                                    }
                                    DropdownMenuSubContent { dir: "rtl",
                                        DropdownMenuRadioGroup {
                                            value: label(),
                                            on_value_change: move |value| label.set(value),
                                            DropdownMenuRadioItem { value: "personal", "شخصي" }
                                            DropdownMenuRadioItem { value: "work", "عمل" }
                                            DropdownMenuRadioItem { value: "other", "آخر" }
                                        }
                                    }
                                }
                            }
                            DropdownMenuSeparator {}
                            DropdownMenuGroup {
                                DropdownMenuItem { variant: DropdownMenuItemVariant::Destructive,
                                    Trash {}
                                    "سلة المهملات"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
