//! Context Menu examples, ported from shadcn/ui `examples/base/context-menu-*.tsx`.

use dioxus::prelude::*;
use lucide_dioxus::{
    ArrowLeft, ArrowRight, ClipboardPaste, Copy, Pencil, RotateCw, Scissors, Share, Trash,
};

use super::Example;
use crate::components::ui::context_menu::{
    ContextMenu, ContextMenuCheckboxItem, ContextMenuContent, ContextMenuGroup, ContextMenuItem,
    ContextMenuItemVariant, ContextMenuLabel, ContextMenuRadioGroup, ContextMenuRadioItem,
    ContextMenuSeparator, ContextMenuShortcut, ContextMenuSide, ContextMenuSub,
    ContextMenuSubContent, ContextMenuSubTrigger, ContextMenuTrigger,
};

const TRIGGER_CLASS: &str = "flex aspect-video w-full max-w-xs items-center justify-center rounded-xl border border-dashed text-sm";

#[component]
pub fn Examples() -> Element {
    rsx! {
        ContextMenuDemo {}
        ContextMenuBasic {}
        ContextMenuSubmenu {}
        ContextMenuShortcuts {}
        ContextMenuGroups {}
        ContextMenuIcons {}
        ContextMenuCheckboxes {}
        ContextMenuRadio {}
        ContextMenuDestructive {}
        ContextMenuSides {}
        ContextMenuRtl {}
    }
}

#[component]
fn TriggerLabel(fine: String, coarse: String) -> Element {
    rsx! {
        span { class: "hidden pointer-fine:inline-block", "{fine}" }
        span { class: "hidden pointer-coarse:inline-block", "{coarse}" }
    }
}

#[component]
fn ContextMenuDemo() -> Element {
    let mut person = use_signal(|| "pedro".to_string());

    rsx! {
        Example { title: "Demo",
            ContextMenu {
                ContextMenuTrigger { class: TRIGGER_CLASS,
                    TriggerLabel { fine: "Right click here", coarse: "Long press here" }
                }
                ContextMenuContent { class: "w-48",
                    ContextMenuGroup {
                        ContextMenuItem {
                            "Back"
                            ContextMenuShortcut { "⌘[" }
                        }
                        ContextMenuItem { disabled: true,
                            "Forward"
                            ContextMenuShortcut { "⌘]" }
                        }
                        ContextMenuItem {
                            "Reload"
                            ContextMenuShortcut { "⌘R" }
                        }
                        ContextMenuSub {
                            ContextMenuSubTrigger { "More Tools" }
                            ContextMenuSubContent { class: "w-44",
                                ContextMenuGroup {
                                    ContextMenuItem { "Save Page..." }
                                    ContextMenuItem { "Create Shortcut..." }
                                    ContextMenuItem { "Name Window..." }
                                }
                                ContextMenuSeparator {}
                                ContextMenuGroup {
                                    ContextMenuItem { "Developer Tools" }
                                }
                                ContextMenuSeparator {}
                                ContextMenuGroup {
                                    ContextMenuItem { variant: ContextMenuItemVariant::Destructive, "Delete" }
                                }
                            }
                        }
                    }
                    ContextMenuSeparator {}
                    ContextMenuGroup {
                        ContextMenuCheckboxItem { checked: true, "Show Bookmarks" }
                        ContextMenuCheckboxItem { "Show Full URLs" }
                    }
                    ContextMenuSeparator {}
                    ContextMenuGroup {
                        ContextMenuRadioGroup {
                            value: person(),
                            on_value_change: move |value| person.set(value),
                            ContextMenuLabel { "People" }
                            ContextMenuRadioItem { value: "pedro", "Pedro Duarte" }
                            ContextMenuRadioItem { value: "colm", "Colm Tuite" }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn ContextMenuBasic() -> Element {
    rsx! {
        Example { title: "Basic",
            ContextMenu {
                ContextMenuTrigger { class: TRIGGER_CLASS,
                    TriggerLabel { fine: "Right click here", coarse: "Long press here" }
                }
                ContextMenuContent {
                    ContextMenuGroup {
                        ContextMenuItem { "Back" }
                        ContextMenuItem { disabled: true, "Forward" }
                        ContextMenuItem { "Reload" }
                    }
                }
            }
        }
    }
}

#[component]
fn ContextMenuSubmenu() -> Element {
    rsx! {
        Example { title: "Submenu",
            ContextMenu {
                ContextMenuTrigger { class: TRIGGER_CLASS,
                    TriggerLabel { fine: "Right click here", coarse: "Long press here" }
                }
                ContextMenuContent {
                    ContextMenuGroup {
                        ContextMenuItem {
                            "Copy"
                            ContextMenuShortcut { "⌘C" }
                        }
                        ContextMenuItem {
                            "Cut"
                            ContextMenuShortcut { "⌘X" }
                        }
                    }
                    ContextMenuSub {
                        ContextMenuSubTrigger { "More Tools" }
                        ContextMenuSubContent {
                            ContextMenuGroup {
                                ContextMenuItem { "Save Page..." }
                                ContextMenuItem { "Create Shortcut..." }
                                ContextMenuItem { "Name Window..." }
                            }
                            ContextMenuSeparator {}
                            ContextMenuGroup {
                                ContextMenuItem { "Developer Tools" }
                            }
                            ContextMenuSeparator {}
                            ContextMenuGroup {
                                ContextMenuItem { variant: ContextMenuItemVariant::Destructive, "Delete" }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn ContextMenuShortcuts() -> Element {
    rsx! {
        Example { title: "Shortcuts",
            ContextMenu {
                ContextMenuTrigger { class: TRIGGER_CLASS,
                    TriggerLabel { fine: "Right click here", coarse: "Long press here" }
                }
                ContextMenuContent {
                    ContextMenuGroup {
                        ContextMenuItem {
                            "Back"
                            ContextMenuShortcut { "⌘[" }
                        }
                        ContextMenuItem { disabled: true,
                            "Forward"
                            ContextMenuShortcut { "⌘]" }
                        }
                        ContextMenuItem {
                            "Reload"
                            ContextMenuShortcut { "⌘R" }
                        }
                    }
                    ContextMenuSeparator {}
                    ContextMenuGroup {
                        ContextMenuItem {
                            "Save"
                            ContextMenuShortcut { "⌘S" }
                        }
                        ContextMenuItem {
                            "Save As..."
                            ContextMenuShortcut { "⇧⌘S" }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn ContextMenuGroups() -> Element {
    rsx! {
        Example { title: "Groups",
            ContextMenu {
                ContextMenuTrigger { class: TRIGGER_CLASS,
                    TriggerLabel { fine: "Right click here", coarse: "Long press here" }
                }
                ContextMenuContent {
                    ContextMenuGroup {
                        ContextMenuLabel { "File" }
                        ContextMenuItem {
                            "New File"
                            ContextMenuShortcut { "⌘N" }
                        }
                        ContextMenuItem {
                            "Open File"
                            ContextMenuShortcut { "⌘O" }
                        }
                        ContextMenuItem {
                            "Save"
                            ContextMenuShortcut { "⌘S" }
                        }
                    }
                    ContextMenuSeparator {}
                    ContextMenuGroup {
                        ContextMenuLabel { "Edit" }
                        ContextMenuItem {
                            "Undo"
                            ContextMenuShortcut { "⌘Z" }
                        }
                        ContextMenuItem {
                            "Redo"
                            ContextMenuShortcut { "⇧⌘Z" }
                        }
                    }
                    ContextMenuSeparator {}
                    ContextMenuGroup {
                        ContextMenuItem {
                            "Cut"
                            ContextMenuShortcut { "⌘X" }
                        }
                        ContextMenuItem {
                            "Copy"
                            ContextMenuShortcut { "⌘C" }
                        }
                        ContextMenuItem {
                            "Paste"
                            ContextMenuShortcut { "⌘V" }
                        }
                    }
                    ContextMenuSeparator {}
                    ContextMenuGroup {
                        ContextMenuItem { variant: ContextMenuItemVariant::Destructive,
                            "Delete"
                            ContextMenuShortcut { "⌫" }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn ContextMenuIcons() -> Element {
    rsx! {
        Example { title: "Icons",
            ContextMenu {
                ContextMenuTrigger { class: TRIGGER_CLASS,
                    TriggerLabel { fine: "Right click here", coarse: "Long press here" }
                }
                ContextMenuContent {
                    ContextMenuGroup {
                        ContextMenuItem {
                            Copy {}
                            "Copy"
                        }
                        ContextMenuItem {
                            Scissors {}
                            "Cut"
                        }
                        ContextMenuItem {
                            ClipboardPaste {}
                            "Paste"
                        }
                    }
                    ContextMenuSeparator {}
                    ContextMenuGroup {
                        ContextMenuItem { variant: ContextMenuItemVariant::Destructive,
                            Trash {}
                            "Delete"
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn ContextMenuCheckboxes() -> Element {
    rsx! {
        Example { title: "Checkboxes",
            ContextMenu {
                ContextMenuTrigger { class: TRIGGER_CLASS,
                    TriggerLabel { fine: "Right click here", coarse: "Long press here" }
                }
                ContextMenuContent {
                    ContextMenuGroup {
                        ContextMenuCheckboxItem { default_checked: true, "Show Bookmarks Bar" }
                        ContextMenuCheckboxItem { "Show Full URLs" }
                        ContextMenuCheckboxItem { default_checked: true, "Show Developer Tools" }
                    }
                }
            }
        }
    }
}

#[component]
fn ContextMenuRadio() -> Element {
    let mut user = use_signal(|| "pedro".to_string());
    let mut theme = use_signal(|| "light".to_string());

    rsx! {
        Example { title: "Radio",
            ContextMenu {
                ContextMenuTrigger { class: TRIGGER_CLASS,
                    TriggerLabel { fine: "Right click here", coarse: "Long press here" }
                }
                ContextMenuContent {
                    ContextMenuGroup {
                        ContextMenuLabel { "People" }
                        ContextMenuRadioGroup {
                            value: user(),
                            on_value_change: move |value| user.set(value),
                            ContextMenuRadioItem { value: "pedro", "Pedro Duarte" }
                            ContextMenuRadioItem { value: "colm", "Colm Tuite" }
                        }
                    }
                    ContextMenuSeparator {}
                    ContextMenuGroup {
                        ContextMenuLabel { "Theme" }
                        ContextMenuRadioGroup {
                            value: theme(),
                            on_value_change: move |value| theme.set(value),
                            ContextMenuRadioItem { value: "light", "Light" }
                            ContextMenuRadioItem { value: "dark", "Dark" }
                            ContextMenuRadioItem { value: "system", "System" }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn ContextMenuDestructive() -> Element {
    rsx! {
        Example { title: "Destructive",
            ContextMenu {
                ContextMenuTrigger { class: TRIGGER_CLASS,
                    TriggerLabel { fine: "Right click here", coarse: "Long press here" }
                }
                ContextMenuContent {
                    ContextMenuGroup {
                        ContextMenuItem {
                            Pencil {}
                            "Edit"
                        }
                        ContextMenuItem {
                            Share {}
                            "Share"
                        }
                    }
                    ContextMenuSeparator {}
                    ContextMenuGroup {
                        ContextMenuItem { variant: ContextMenuItemVariant::Destructive,
                            Trash {}
                            "Delete"
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn SideMenu(side: ContextMenuSide, name: &'static str) -> Element {
    rsx! {
        ContextMenu {
            ContextMenuTrigger { class: TRIGGER_CLASS,
                TriggerLabel {
                    fine: format!("Right click ({name})"),
                    coarse: format!("Long press ({name})"),
                }
            }
            ContextMenuContent { side,
                ContextMenuGroup {
                    ContextMenuItem { "Back" }
                    ContextMenuItem { "Forward" }
                    ContextMenuItem { "Reload" }
                }
            }
        }
    }
}

#[component]
fn ContextMenuSides() -> Element {
    rsx! {
        Example { title: "Sides",
            div { class: "grid w-full max-w-sm grid-cols-2 gap-4",
                SideMenu { side: ContextMenuSide::Top, name: "top" }
                SideMenu { side: ContextMenuSide::Right, name: "right" }
                SideMenu { side: ContextMenuSide::Bottom, name: "bottom" }
                SideMenu { side: ContextMenuSide::Left, name: "left" }
            }
        }
    }
}

#[component]
fn ContextMenuRtl() -> Element {
    let mut people = use_signal(|| "pedro".to_string());

    rsx! {
        Example { title: "RTL",
            ContextMenu {
                ContextMenuTrigger { class: TRIGGER_CLASS,
                    TriggerLabel {
                        fine: "انقر بزر الماوس الأيمن هنا",
                        coarse: "اضغط مطولاً هنا",
                    }
                }
                ContextMenuContent { class: "w-48", dir: "rtl", "data-lang": "ar",
                    ContextMenuGroup {
                        ContextMenuSub {
                            ContextMenuSubTrigger { "التنقل" }
                            ContextMenuSubContent {
                                class: "w-44",
                                side: ContextMenuSide::InlineEnd,
                                dir: "rtl",
                                "data-lang": "ar",
                                ContextMenuGroup {
                                    ContextMenuItem {
                                        ArrowLeft {}
                                        "رجوع"
                                        ContextMenuShortcut { "⌘[" }
                                    }
                                    ContextMenuItem { disabled: true,
                                        ArrowRight {}
                                        "تقدم"
                                        ContextMenuShortcut { "⌘]" }
                                    }
                                    ContextMenuItem {
                                        RotateCw {}
                                        "إعادة تحميل"
                                        ContextMenuShortcut { "⌘R" }
                                    }
                                }
                            }
                        }
                        ContextMenuSub {
                            ContextMenuSubTrigger { "المزيد من الأدوات" }
                            ContextMenuSubContent {
                                class: "w-44",
                                side: ContextMenuSide::InlineEnd,
                                dir: "rtl",
                                "data-lang": "ar",
                                ContextMenuGroup {
                                    ContextMenuItem { "حفظ الصفحة..." }
                                    ContextMenuItem { "إنشاء اختصار..." }
                                    ContextMenuItem { "تسمية النافذة..." }
                                }
                                ContextMenuSeparator {}
                                ContextMenuGroup {
                                    ContextMenuItem { "أدوات المطور" }
                                }
                                ContextMenuSeparator {}
                                ContextMenuGroup {
                                    ContextMenuItem { variant: ContextMenuItemVariant::Destructive, "حذف" }
                                }
                            }
                        }
                    }
                    ContextMenuSeparator {}
                    ContextMenuGroup {
                        ContextMenuCheckboxItem { checked: true, "إظهار الإشارات المرجعية" }
                        ContextMenuCheckboxItem { "إظهار عناوين URL الكاملة" }
                    }
                    ContextMenuSeparator {}
                    ContextMenuGroup {
                        ContextMenuRadioGroup {
                            value: people(),
                            on_value_change: move |value| people.set(value),
                            ContextMenuLabel { "الأشخاص" }
                            ContextMenuRadioItem { value: "pedro", "Pedro Duarte" }
                            ContextMenuRadioItem { value: "colm", "Colm Tuite" }
                        }
                    }
                }
            }
        }
    }
}
