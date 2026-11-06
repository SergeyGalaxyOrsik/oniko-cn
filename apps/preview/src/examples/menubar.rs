//! Menubar examples, ported from shadcn/ui `examples/base/menubar-*.tsx`.

use dioxus::prelude::*;
use lucide_dioxus::{CircleQuestionMark, File, Folder, Save, Settings, Trash};

use super::Example;
use crate::components::ui::menubar::{
    Menubar, MenubarAlign, MenubarCheckboxItem, MenubarContent, MenubarGroup, MenubarItem,
    MenubarItemVariant, MenubarMenu, MenubarRadioGroup, MenubarRadioItem, MenubarSeparator,
    MenubarShortcut, MenubarSide, MenubarSub, MenubarSubContent, MenubarSubTrigger, MenubarTrigger,
};

#[component]
pub fn Examples() -> Element {
    rsx! {
        MenubarDemo {}
        MenubarCheckbox {}
        MenubarRadio {}
        MenubarSubmenu {}
        MenubarIcons {}
        MenubarRtl {}
    }
}

#[component]
fn MenubarDemo() -> Element {
    rsx! {
        Example { title: "Demo",
            Menubar { class: "w-72",
                MenubarMenu {
                    MenubarTrigger { "File" }
                    MenubarContent {
                        MenubarGroup {
                            MenubarItem {
                                "New Tab "
                                MenubarShortcut { "⌘T" }
                            }
                            MenubarItem {
                                "New Window "
                                MenubarShortcut { "⌘N" }
                            }
                            MenubarItem { disabled: true, "New Incognito Window" }
                        }
                        MenubarSeparator {}
                        MenubarGroup {
                            MenubarSub {
                                MenubarSubTrigger { "Share" }
                                MenubarSubContent {
                                    MenubarGroup {
                                        MenubarItem { "Email link" }
                                        MenubarItem { "Messages" }
                                        MenubarItem { "Notes" }
                                    }
                                }
                            }
                        }
                        MenubarSeparator {}
                        MenubarGroup {
                            MenubarItem {
                                "Print... "
                                MenubarShortcut { "⌘P" }
                            }
                        }
                    }
                }
                MenubarMenu {
                    MenubarTrigger { "Edit" }
                    MenubarContent {
                        MenubarGroup {
                            MenubarItem {
                                "Undo "
                                MenubarShortcut { "⌘Z" }
                            }
                            MenubarItem {
                                "Redo "
                                MenubarShortcut { "⇧⌘Z" }
                            }
                        }
                        MenubarSeparator {}
                        MenubarGroup {
                            MenubarSub {
                                MenubarSubTrigger { "Find" }
                                MenubarSubContent {
                                    MenubarGroup {
                                        MenubarItem { "Search the web" }
                                    }
                                    MenubarSeparator {}
                                    MenubarGroup {
                                        MenubarItem { "Find..." }
                                        MenubarItem { "Find Next" }
                                        MenubarItem { "Find Previous" }
                                    }
                                }
                            }
                        }
                        MenubarSeparator {}
                        MenubarGroup {
                            MenubarItem { "Cut" }
                            MenubarItem { "Copy" }
                            MenubarItem { "Paste" }
                        }
                    }
                }
                MenubarMenu {
                    MenubarTrigger { "View" }
                    MenubarContent { class: "w-44",
                        MenubarGroup {
                            MenubarCheckboxItem { "Bookmarks Bar" }
                            MenubarCheckboxItem { checked: true, "Full URLs" }
                        }
                        MenubarSeparator {}
                        MenubarGroup {
                            MenubarItem { inset: true,
                                "Reload "
                                MenubarShortcut { "⌘R" }
                            }
                            MenubarItem { disabled: true, inset: true,
                                "Force Reload "
                                MenubarShortcut { "⇧⌘R" }
                            }
                        }
                        MenubarSeparator {}
                        MenubarGroup {
                            MenubarItem { inset: true, "Toggle Fullscreen" }
                        }
                        MenubarSeparator {}
                        MenubarGroup {
                            MenubarItem { inset: true, "Hide Sidebar" }
                        }
                    }
                }
                MenubarMenu {
                    MenubarTrigger { "Profiles" }
                    MenubarContent {
                        MenubarRadioGroup { value: "benoit",
                            MenubarRadioItem { value: "andy", "Andy" }
                            MenubarRadioItem { value: "benoit", "Benoit" }
                            MenubarRadioItem { value: "Luis", "Luis" }
                        }
                        MenubarSeparator {}
                        MenubarGroup {
                            MenubarItem { inset: true, "Edit..." }
                        }
                        MenubarSeparator {}
                        MenubarGroup {
                            MenubarItem { inset: true, "Add Profile..." }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn MenubarCheckbox() -> Element {
    rsx! {
        Example { title: "Checkbox",
            Menubar { class: "w-72",
                MenubarMenu {
                    MenubarTrigger { "View" }
                    MenubarContent { class: "w-64",
                        MenubarCheckboxItem { "Always Show Bookmarks Bar" }
                        MenubarCheckboxItem { checked: true, "Always Show Full URLs" }
                        MenubarSeparator {}
                        MenubarItem { inset: true,
                            "Reload "
                            MenubarShortcut { "⌘R" }
                        }
                        MenubarItem { disabled: true, inset: true,
                            "Force Reload "
                            MenubarShortcut { "⇧⌘R" }
                        }
                    }
                }
                MenubarMenu {
                    MenubarTrigger { "Format" }
                    MenubarContent {
                        MenubarCheckboxItem { checked: true, "Strikethrough" }
                        MenubarCheckboxItem { "Code" }
                        MenubarCheckboxItem { "Superscript" }
                    }
                }
            }
        }
    }
}

#[component]
fn MenubarRadio() -> Element {
    let mut user = use_signal(|| "benoit".to_string());
    let mut theme = use_signal(|| "system".to_string());

    rsx! {
        Example { title: "Radio",
            Menubar { class: "w-72",
                MenubarMenu {
                    MenubarTrigger { "Profiles" }
                    MenubarContent {
                        MenubarRadioGroup {
                            value: user(),
                            on_value_change: move |value| user.set(value),
                            MenubarRadioItem { value: "andy", "Andy" }
                            MenubarRadioItem { value: "benoit", "Benoit" }
                            MenubarRadioItem { value: "luis", "Luis" }
                        }
                        MenubarSeparator {}
                        MenubarItem { inset: true, "Edit..." }
                        MenubarItem { inset: true, "Add Profile..." }
                    }
                }
                MenubarMenu {
                    MenubarTrigger { "Theme" }
                    MenubarContent {
                        MenubarRadioGroup {
                            value: theme(),
                            on_value_change: move |value| theme.set(value),
                            MenubarRadioItem { value: "light", "Light" }
                            MenubarRadioItem { value: "dark", "Dark" }
                            MenubarRadioItem { value: "system", "System" }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn MenubarSubmenu() -> Element {
    rsx! {
        Example { title: "Submenu",
            Menubar { class: "w-72",
                MenubarMenu {
                    MenubarTrigger { "File" }
                    MenubarContent {
                        MenubarSub {
                            MenubarSubTrigger { "Share" }
                            MenubarSubContent {
                                MenubarItem { "Email link" }
                                MenubarItem { "Messages" }
                                MenubarItem { "Notes" }
                            }
                        }
                        MenubarSeparator {}
                        MenubarItem {
                            "Print... "
                            MenubarShortcut { "⌘P" }
                        }
                    }
                }
                MenubarMenu {
                    MenubarTrigger { "Edit" }
                    MenubarContent {
                        MenubarItem {
                            "Undo "
                            MenubarShortcut { "⌘Z" }
                        }
                        MenubarItem {
                            "Redo "
                            MenubarShortcut { "⇧⌘Z" }
                        }
                        MenubarSeparator {}
                        MenubarSub {
                            MenubarSubTrigger { "Find" }
                            MenubarSubContent {
                                MenubarItem { "Find..." }
                                MenubarItem { "Find Next" }
                                MenubarItem { "Find Previous" }
                            }
                        }
                        MenubarSeparator {}
                        MenubarItem { "Cut" }
                        MenubarItem { "Copy" }
                        MenubarItem { "Paste" }
                    }
                }
            }
        }
    }
}

#[component]
fn MenubarIcons() -> Element {
    rsx! {
        Example { title: "With Icons",
            Menubar { class: "w-72",
                MenubarMenu {
                    MenubarTrigger { "File" }
                    MenubarContent {
                        MenubarItem {
                            File {}
                            "New File "
                            MenubarShortcut { "⌘N" }
                        }
                        MenubarItem {
                            Folder {}
                            "Open Folder"
                        }
                        MenubarSeparator {}
                        MenubarItem {
                            Save {}
                            "Save "
                            MenubarShortcut { "⌘S" }
                        }
                    }
                }
                MenubarMenu {
                    MenubarTrigger { "More" }
                    MenubarContent {
                        MenubarGroup {
                            MenubarItem {
                                Settings {}
                                "Settings"
                            }
                            MenubarItem {
                                CircleQuestionMark {}
                                "Help"
                            }
                            MenubarSeparator {}
                            MenubarItem { variant: MenubarItemVariant::Destructive,
                                Trash {}
                                "Delete"
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn MenubarRtl() -> Element {
    let mut profile = use_signal(|| "benoit".to_string());

    rsx! {
        Example { title: "RTL",
            Menubar { class: "w-72", dir: "rtl",
                MenubarMenu {
                    MenubarTrigger { "ملف" }
                    MenubarContent { dir: "rtl", align: MenubarAlign::End,
                        MenubarGroup {
                            MenubarItem {
                                "علامة تبويب جديدة "
                                MenubarShortcut { "⌘T" }
                            }
                            MenubarItem {
                                "نافذة جديدة "
                                MenubarShortcut { "⌘N" }
                            }
                            MenubarItem { disabled: true, "نافذة التصفح المتخفي الجديدة" }
                        }
                        MenubarSeparator {}
                        MenubarGroup {
                            MenubarSub {
                                MenubarSubTrigger { "مشاركة" }
                                MenubarSubContent {
                                    side: MenubarSide::InlineEnd,
                                    dir: "rtl",
                                    "data-lang": "ar",
                                    MenubarGroup {
                                        MenubarItem { "رابط البريد الإلكتروني" }
                                        MenubarItem { "الرسائل" }
                                        MenubarItem { "الملاحظات" }
                                    }
                                }
                            }
                        }
                        MenubarSeparator {}
                        MenubarGroup {
                            MenubarItem {
                                "طباعة... "
                                MenubarShortcut { "⌘P" }
                            }
                        }
                    }
                }
                MenubarMenu {
                    MenubarTrigger { "تعديل" }
                    MenubarContent { dir: "rtl", align: MenubarAlign::End, "data-lang": "ar",
                        MenubarGroup {
                            MenubarItem {
                                "تراجع "
                                MenubarShortcut { "⌘Z" }
                            }
                            MenubarItem {
                                "إعادة "
                                MenubarShortcut { "⇧⌘Z" }
                            }
                        }
                        MenubarSeparator {}
                        MenubarGroup {
                            MenubarSub {
                                MenubarSubTrigger { "بحث" }
                                MenubarSubContent {
                                    side: MenubarSide::InlineEnd,
                                    dir: "rtl",
                                    "data-lang": "ar",
                                    MenubarGroup {
                                        MenubarItem { "البحث على الويب" }
                                    }
                                    MenubarSeparator {}
                                    MenubarGroup {
                                        MenubarItem { "بحث..." }
                                        MenubarItem { "البحث التالي" }
                                        MenubarItem { "البحث السابق" }
                                    }
                                }
                            }
                        }
                        MenubarSeparator {}
                        MenubarGroup {
                            MenubarItem { "قص" }
                            MenubarItem { "نسخ" }
                            MenubarItem { "لصق" }
                        }
                    }
                }
                MenubarMenu {
                    MenubarTrigger { "عرض" }
                    MenubarContent {
                        class: "w-44",
                        dir: "rtl",
                        align: MenubarAlign::End,
                        "data-lang": "ar",
                        MenubarGroup {
                            MenubarCheckboxItem { "شريط الإشارات المرجعية" }
                            MenubarCheckboxItem { checked: true, "عناوين URL الكاملة" }
                        }
                        MenubarSeparator {}
                        MenubarGroup {
                            MenubarItem { inset: true,
                                "إعادة تحميل "
                                MenubarShortcut { "⌘R" }
                            }
                            MenubarItem { disabled: true, inset: true,
                                "إعادة تحميل قسري "
                                MenubarShortcut { "⇧⌘R" }
                            }
                        }
                        MenubarSeparator {}
                        MenubarGroup {
                            MenubarItem { inset: true, "تبديل وضع ملء الشاشة" }
                        }
                        MenubarSeparator {}
                        MenubarGroup {
                            MenubarItem { inset: true, "إخفاء الشريط الجانبي" }
                        }
                    }
                }
                MenubarMenu {
                    MenubarTrigger { "الملفات الشخصية" }
                    MenubarContent { dir: "rtl", align: MenubarAlign::End, "data-lang": "ar",
                        MenubarRadioGroup {
                            value: profile(),
                            on_value_change: move |value| profile.set(value),
                            MenubarRadioItem { value: "andy", "Andy" }
                            MenubarRadioItem { value: "benoit", "Benoit" }
                            MenubarRadioItem { value: "Luis", "Luis" }
                        }
                        MenubarSeparator {}
                        MenubarGroup {
                            MenubarItem { inset: true, "تعديل..." }
                        }
                        MenubarSeparator {}
                        MenubarGroup {
                            MenubarItem { inset: true, "إضافة ملف شخصي..." }
                        }
                    }
                }
            }
        }
    }
}
