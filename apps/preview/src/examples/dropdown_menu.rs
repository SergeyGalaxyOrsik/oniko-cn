//! Dropdown Menu examples, ported from shadcn/ui `examples/base/dropdown-menu-*.tsx`.

use dioxus::prelude::*;
use lucide_dioxus::{
    BadgeCheck, Bell, Building, CircleQuestionMark, CreditCard, Download, Ellipsis, Eye, File,
    FileCode, FileText, Folder, FolderOpen, FolderSearch, Keyboard, Languages, LayoutPanelLeft,
    LogOut, Mail, MessageSquare, Monitor, Moon, Palette, Pencil, Save, Settings, Share, Shield,
    Sun, Trash, User, Wallet,
};

use super::Example;
use crate::components::ui::avatar::{Avatar, AvatarFallback, AvatarImage};
use crate::components::ui::button::{ButtonSize, ButtonVariant};
use crate::components::ui::dropdown_menu::{
    DropdownMenu, DropdownMenuAlign, DropdownMenuCheckboxItem, DropdownMenuContent,
    DropdownMenuGroup, DropdownMenuItem, DropdownMenuItemVariant, DropdownMenuLabel,
    DropdownMenuPortal, DropdownMenuRadioGroup, DropdownMenuRadioItem, DropdownMenuSeparator,
    DropdownMenuShortcut, DropdownMenuSide, DropdownMenuSub, DropdownMenuSubContent,
    DropdownMenuSubTrigger, DropdownMenuTrigger,
};

#[component]
pub fn Examples() -> Element {
    rsx! {
        DropdownMenuDemo {}
        DropdownMenuBasic {}
        DropdownMenuSubmenu {}
        DropdownMenuShortcuts {}
        DropdownMenuIcons {}
        DropdownMenuCheckboxes {}
        DropdownMenuCheckboxesIcons {}
        DropdownMenuRadioGroupDemo {}
        DropdownMenuRadioIcons {}
        DropdownMenuDestructive {}
        DropdownMenuAvatar {}
        DropdownMenuComplex {}
        DropdownMenuRtl {}
    }
}

#[component]
fn DropdownMenuDemo() -> Element {
    rsx! {
        Example { title: "Demo",
            DropdownMenu {
                DropdownMenuTrigger { variant: ButtonVariant::Outline, "Open" }
                DropdownMenuContent { class: "w-40", align: DropdownMenuAlign::Start,
                    DropdownMenuGroup {
                        DropdownMenuLabel { "My Account" }
                        DropdownMenuItem {
                            "Profile"
                            DropdownMenuShortcut { "⇧⌘P" }
                        }
                        DropdownMenuItem {
                            "Billing"
                            DropdownMenuShortcut { "⌘B" }
                        }
                        DropdownMenuItem {
                            "Settings"
                            DropdownMenuShortcut { "⌘S" }
                        }
                    }
                    DropdownMenuSeparator {}
                    DropdownMenuGroup {
                        DropdownMenuItem { "Team" }
                        DropdownMenuSub {
                            DropdownMenuSubTrigger { "Invite users" }
                            DropdownMenuPortal {
                                DropdownMenuSubContent {
                                    DropdownMenuItem { "Email" }
                                    DropdownMenuItem { "Message" }
                                    DropdownMenuSeparator {}
                                    DropdownMenuItem { "More..." }
                                }
                            }
                        }
                        DropdownMenuItem {
                            "New Team"
                            DropdownMenuShortcut { "⌘+T" }
                        }
                    }
                    DropdownMenuSeparator {}
                    DropdownMenuGroup {
                        DropdownMenuItem { "GitHub" }
                        DropdownMenuItem { "Support" }
                        DropdownMenuItem { disabled: true, "API" }
                    }
                    DropdownMenuSeparator {}
                    DropdownMenuGroup {
                        DropdownMenuItem {
                            "Log out"
                            DropdownMenuShortcut { "⇧⌘Q" }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn DropdownMenuBasic() -> Element {
    rsx! {
        Example { title: "Basic",
            DropdownMenu {
                DropdownMenuTrigger { variant: ButtonVariant::Outline, "Open" }
                DropdownMenuContent {
                    DropdownMenuGroup {
                        DropdownMenuLabel { "My Account" }
                        DropdownMenuItem { "Profile" }
                        DropdownMenuItem { "Billing" }
                        DropdownMenuItem { "Settings" }
                    }
                    DropdownMenuSeparator {}
                    DropdownMenuItem { "GitHub" }
                    DropdownMenuItem { "Support" }
                    DropdownMenuItem { disabled: true, "API" }
                }
            }
        }
    }
}

#[component]
fn DropdownMenuSubmenu() -> Element {
    rsx! {
        Example { title: "Submenu",
            DropdownMenu {
                DropdownMenuTrigger { variant: ButtonVariant::Outline, "Open" }
                DropdownMenuContent {
                    DropdownMenuGroup {
                        DropdownMenuItem { "Team" }
                        DropdownMenuSub {
                            DropdownMenuSubTrigger { "Invite users" }
                            DropdownMenuPortal {
                                DropdownMenuSubContent {
                                    DropdownMenuItem { "Email" }
                                    DropdownMenuItem { "Message" }
                                    DropdownMenuSub {
                                        DropdownMenuSubTrigger { "More options" }
                                        DropdownMenuPortal {
                                            DropdownMenuSubContent {
                                                DropdownMenuItem { "Calendly" }
                                                DropdownMenuItem { "Slack" }
                                                DropdownMenuSeparator {}
                                                DropdownMenuItem { "Webhook" }
                                            }
                                        }
                                    }
                                    DropdownMenuSeparator {}
                                    DropdownMenuItem { "Advanced..." }
                                }
                            }
                        }
                        DropdownMenuItem {
                            "New Team"
                            DropdownMenuShortcut { "⌘+T" }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn DropdownMenuShortcuts() -> Element {
    rsx! {
        Example { title: "Shortcuts",
            DropdownMenu {
                DropdownMenuTrigger { variant: ButtonVariant::Outline, "Open" }
                DropdownMenuContent {
                    DropdownMenuGroup {
                        DropdownMenuLabel { "My Account" }
                        DropdownMenuItem {
                            "Profile"
                            DropdownMenuShortcut { "⇧⌘P" }
                        }
                        DropdownMenuItem {
                            "Billing"
                            DropdownMenuShortcut { "⌘B" }
                        }
                        DropdownMenuItem {
                            "Settings"
                            DropdownMenuShortcut { "⌘S" }
                        }
                    }
                    DropdownMenuSeparator {}
                    DropdownMenuItem {
                        "Log out"
                        DropdownMenuShortcut { "⇧⌘Q" }
                    }
                }
            }
        }
    }
}

#[component]
fn DropdownMenuIcons() -> Element {
    rsx! {
        Example { title: "Icons",
            DropdownMenu {
                DropdownMenuTrigger { variant: ButtonVariant::Outline, "Open" }
                DropdownMenuContent {
                    DropdownMenuItem {
                        User {}
                        "Profile"
                    }
                    DropdownMenuItem {
                        CreditCard {}
                        "Billing"
                    }
                    DropdownMenuItem {
                        Settings {}
                        "Settings"
                    }
                    DropdownMenuSeparator {}
                    DropdownMenuItem { variant: DropdownMenuItemVariant::Destructive,
                        LogOut {}
                        "Log out"
                    }
                }
            }
        }
    }
}

#[component]
fn DropdownMenuCheckboxes() -> Element {
    let mut show_status_bar = use_signal(|| true);
    let mut show_activity_bar = use_signal(|| false);
    let mut show_panel = use_signal(|| false);

    rsx! {
        Example { title: "Checkboxes",
            DropdownMenu {
                DropdownMenuTrigger { variant: ButtonVariant::Outline, "Open" }
                DropdownMenuContent { class: "w-40",
                    DropdownMenuGroup {
                        DropdownMenuLabel { "Appearance" }
                        DropdownMenuCheckboxItem {
                            checked: show_status_bar(),
                            on_checked_change: move |checked| show_status_bar.set(checked),
                            "Status Bar"
                        }
                        DropdownMenuCheckboxItem {
                            checked: show_activity_bar(),
                            on_checked_change: move |checked| show_activity_bar.set(checked),
                            disabled: true,
                            "Activity Bar"
                        }
                        DropdownMenuCheckboxItem {
                            checked: show_panel(),
                            on_checked_change: move |checked| show_panel.set(checked),
                            "Panel"
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn DropdownMenuCheckboxesIcons() -> Element {
    let mut email = use_signal(|| true);
    let mut sms = use_signal(|| false);
    let mut push = use_signal(|| true);

    rsx! {
        Example { title: "Checkboxes Icons",
            DropdownMenu {
                DropdownMenuTrigger { variant: ButtonVariant::Outline, "Notifications" }
                DropdownMenuContent { class: "w-48",
                    DropdownMenuGroup {
                        DropdownMenuLabel { "Notification Preferences" }
                        DropdownMenuCheckboxItem {
                            checked: email(),
                            on_checked_change: move |checked| email.set(checked),
                            Mail {}
                            "Email notifications"
                        }
                        DropdownMenuCheckboxItem {
                            checked: sms(),
                            on_checked_change: move |checked| sms.set(checked),
                            MessageSquare {}
                            "SMS notifications"
                        }
                        DropdownMenuCheckboxItem {
                            checked: push(),
                            on_checked_change: move |checked| push.set(checked),
                            Bell {}
                            "Push notifications"
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn DropdownMenuRadioGroupDemo() -> Element {
    let mut position = use_signal(|| "bottom".to_string());

    rsx! {
        Example { title: "Radio Group",
            DropdownMenu {
                DropdownMenuTrigger { variant: ButtonVariant::Outline, "Open" }
                DropdownMenuContent { class: "w-32",
                    DropdownMenuGroup {
                        DropdownMenuLabel { "Panel Position" }
                        DropdownMenuRadioGroup {
                            value: position(),
                            on_value_change: move |value| position.set(value),
                            DropdownMenuRadioItem { value: "top", "Top" }
                            DropdownMenuRadioItem { value: "bottom", "Bottom" }
                            DropdownMenuRadioItem { value: "right", "Right" }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn DropdownMenuRadioIcons() -> Element {
    let mut payment_method = use_signal(|| "card".to_string());

    rsx! {
        Example { title: "Radio Icons",
            DropdownMenu {
                DropdownMenuTrigger { variant: ButtonVariant::Outline, "Payment Method" }
                DropdownMenuContent { class: "min-w-56",
                    DropdownMenuGroup {
                        DropdownMenuLabel { "Select Payment Method" }
                        DropdownMenuRadioGroup {
                            value: payment_method(),
                            on_value_change: move |value| payment_method.set(value),
                            DropdownMenuRadioItem { value: "card",
                                CreditCard {}
                                "Credit Card"
                            }
                            DropdownMenuRadioItem { value: "paypal",
                                Wallet {}
                                "PayPal"
                            }
                            DropdownMenuRadioItem { value: "bank",
                                Building {}
                                "Bank Transfer"
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn DropdownMenuDestructive() -> Element {
    rsx! {
        Example { title: "Destructive",
            DropdownMenu {
                DropdownMenuTrigger { variant: ButtonVariant::Outline, "Actions" }
                DropdownMenuContent {
                    DropdownMenuGroup {
                        DropdownMenuItem {
                            Pencil {}
                            "Edit"
                        }
                        DropdownMenuItem {
                            Share {}
                            "Share"
                        }
                    }
                    DropdownMenuSeparator {}
                    DropdownMenuGroup {
                        DropdownMenuItem { variant: DropdownMenuItemVariant::Destructive,
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
fn DropdownMenuAvatar() -> Element {
    rsx! {
        Example { title: "Avatar",
            DropdownMenu {
                DropdownMenuTrigger {
                    variant: ButtonVariant::Ghost,
                    size: ButtonSize::Icon,
                    class: "rounded-full",
                    Avatar {
                        AvatarImage { src: "https://github.com/shadcn.png", alt: "shadcn" }
                        AvatarFallback { "LR" }
                    }
                }
                DropdownMenuContent { align: DropdownMenuAlign::End,
                    DropdownMenuGroup {
                        DropdownMenuItem {
                            BadgeCheck {}
                            "Account"
                        }
                        DropdownMenuItem {
                            CreditCard {}
                            "Billing"
                        }
                        DropdownMenuItem {
                            Bell {}
                            "Notifications"
                        }
                    }
                    DropdownMenuSeparator {}
                    DropdownMenuItem {
                        LogOut {}
                        "Sign Out"
                    }
                }
            }
        }
    }
}

#[component]
fn DropdownMenuComplex() -> Element {
    let mut show_sidebar = use_signal(|| true);
    let mut show_status_bar = use_signal(|| false);
    let mut push = use_signal(|| true);
    let mut email = use_signal(|| true);
    let mut theme = use_signal(|| "light".to_string());

    rsx! {
        Example { title: "Complex",
            DropdownMenu {
                DropdownMenuTrigger { variant: ButtonVariant::Outline, "Complex Menu" }
                DropdownMenuContent { class: "w-44",
                    DropdownMenuGroup {
                        DropdownMenuLabel { "File" }
                        DropdownMenuItem {
                            File {}
                            "New File"
                            DropdownMenuShortcut { "⌘N" }
                        }
                        DropdownMenuItem {
                            Folder {}
                            "New Folder"
                            DropdownMenuShortcut { "⇧⌘N" }
                        }
                        DropdownMenuSub {
                            DropdownMenuSubTrigger {
                                FolderOpen {}
                                "Open Recent"
                            }
                            DropdownMenuPortal {
                                DropdownMenuSubContent {
                                    DropdownMenuGroup {
                                        DropdownMenuLabel { "Recent Projects" }
                                        DropdownMenuItem {
                                            FileCode {}
                                            "Project Alpha"
                                        }
                                        DropdownMenuItem {
                                            FileCode {}
                                            "Project Beta"
                                        }
                                        DropdownMenuSub {
                                            DropdownMenuSubTrigger {
                                                Ellipsis {}
                                                "More Projects"
                                            }
                                            DropdownMenuPortal {
                                                DropdownMenuSubContent {
                                                    DropdownMenuItem {
                                                        FileCode {}
                                                        "Project Gamma"
                                                    }
                                                    DropdownMenuItem {
                                                        FileCode {}
                                                        "Project Delta"
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    DropdownMenuSeparator {}
                                    DropdownMenuGroup {
                                        DropdownMenuItem {
                                            FolderSearch {}
                                            "Browse..."
                                        }
                                    }
                                }
                            }
                        }
                        DropdownMenuSeparator {}
                        DropdownMenuItem {
                            Save {}
                            "Save"
                            DropdownMenuShortcut { "⌘S" }
                        }
                        DropdownMenuItem {
                            Download {}
                            "Export"
                            DropdownMenuShortcut { "⇧⌘E" }
                        }
                    }
                    DropdownMenuSeparator {}
                    DropdownMenuGroup {
                        DropdownMenuLabel { "View" }
                        DropdownMenuCheckboxItem {
                            checked: show_sidebar(),
                            on_checked_change: move |checked| show_sidebar.set(checked),
                            Eye {}
                            "Show Sidebar"
                        }
                        DropdownMenuCheckboxItem {
                            checked: show_status_bar(),
                            on_checked_change: move |checked| show_status_bar.set(checked),
                            LayoutPanelLeft {}
                            "Show Status Bar"
                        }
                        DropdownMenuSub {
                            DropdownMenuSubTrigger {
                                Palette {}
                                "Theme"
                            }
                            DropdownMenuPortal {
                                DropdownMenuSubContent {
                                    DropdownMenuGroup {
                                        DropdownMenuLabel { "Appearance" }
                                        DropdownMenuRadioGroup {
                                            value: theme(),
                                            on_value_change: move |value| theme.set(value),
                                            DropdownMenuRadioItem { value: "light",
                                                Sun {}
                                                "Light"
                                            }
                                            DropdownMenuRadioItem { value: "dark",
                                                Moon {}
                                                "Dark"
                                            }
                                            DropdownMenuRadioItem { value: "system",
                                                Monitor {}
                                                "System"
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    DropdownMenuSeparator {}
                    DropdownMenuGroup {
                        DropdownMenuLabel { "Account" }
                        DropdownMenuItem {
                            User {}
                            "Profile"
                            DropdownMenuShortcut { "⇧⌘P" }
                        }
                        DropdownMenuItem {
                            CreditCard {}
                            "Billing"
                        }
                        DropdownMenuSub {
                            DropdownMenuSubTrigger {
                                Settings {}
                                "Settings"
                            }
                            DropdownMenuPortal {
                                DropdownMenuSubContent {
                                    DropdownMenuGroup {
                                        DropdownMenuLabel { "Preferences" }
                                        DropdownMenuItem {
                                            Keyboard {}
                                            "Keyboard Shortcuts"
                                        }
                                        DropdownMenuItem {
                                            Languages {}
                                            "Language"
                                        }
                                        DropdownMenuSub {
                                            DropdownMenuSubTrigger {
                                                Bell {}
                                                "Notifications"
                                            }
                                            DropdownMenuPortal {
                                                DropdownMenuSubContent {
                                                    DropdownMenuGroup {
                                                        DropdownMenuLabel { "Notification Types" }
                                                        DropdownMenuCheckboxItem {
                                                            checked: push(),
                                                            on_checked_change: move |checked| push.set(checked),
                                                            Bell {}
                                                            "Push Notifications"
                                                        }
                                                        DropdownMenuCheckboxItem {
                                                            checked: email(),
                                                            on_checked_change: move |checked| email.set(checked),
                                                            Mail {}
                                                            "Email Notifications"
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    DropdownMenuSeparator {}
                                    DropdownMenuGroup {
                                        DropdownMenuItem {
                                            Shield {}
                                            "Privacy & Security"
                                        }
                                    }
                                }
                            }
                        }
                    }
                    DropdownMenuSeparator {}
                    DropdownMenuGroup {
                        DropdownMenuItem {
                            CircleQuestionMark {}
                            "Help & Support"
                        }
                        DropdownMenuItem {
                            FileText {}
                            "Documentation"
                        }
                    }
                    DropdownMenuSeparator {}
                    DropdownMenuGroup {
                        DropdownMenuItem { variant: DropdownMenuItemVariant::Destructive,
                            LogOut {}
                            "Sign Out"
                            DropdownMenuShortcut { "⇧⌘Q" }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn DropdownMenuRtl() -> Element {
    let mut show_status_bar = use_signal(|| true);
    let mut show_activity_bar = use_signal(|| false);
    let mut show_panel = use_signal(|| false);
    let mut position = use_signal(|| "bottom".to_string());

    rsx! {
        Example { title: "RTL",
            DropdownMenu {
                DropdownMenuTrigger { variant: ButtonVariant::Outline, "افتح القائمة" }
                DropdownMenuContent { align: DropdownMenuAlign::End, dir: "rtl", class: "w-36", "data-lang": "ar",
                    DropdownMenuGroup {
                        DropdownMenuSub {
                            DropdownMenuSubTrigger { "الحساب" }
                            DropdownMenuPortal {
                                DropdownMenuSubContent { side: DropdownMenuSide::InlineEnd, dir: "rtl", "data-lang": "ar",
                                    DropdownMenuGroup {
                                        DropdownMenuItem {
                                            User {}
                                            "الملف الشخصي"
                                        }
                                        DropdownMenuItem {
                                            CreditCard {}
                                            "الفوترة"
                                        }
                                        DropdownMenuItem {
                                            Settings {}
                                            "الإعدادات"
                                        }
                                    }
                                }
                            }
                        }
                    }
                    DropdownMenuSeparator {}
                    DropdownMenuGroup {
                        DropdownMenuLabel { "الفريق" }
                        DropdownMenuItem { "الفريق" }
                        DropdownMenuSub {
                            DropdownMenuSubTrigger { "دعوة المستخدمين" }
                            DropdownMenuPortal {
                                DropdownMenuSubContent { side: DropdownMenuSide::InlineEnd, dir: "rtl", "data-lang": "ar",
                                    DropdownMenuItem { "البريد الإلكتروني" }
                                    DropdownMenuItem { "رسالة" }
                                    DropdownMenuSub {
                                        DropdownMenuSubTrigger { "المزيد" }
                                        DropdownMenuPortal {
                                            DropdownMenuSubContent { side: DropdownMenuSide::InlineEnd, dir: "rtl", "data-lang": "ar",
                                                DropdownMenuItem { "تقويم" }
                                                DropdownMenuItem { "دردشة" }
                                                DropdownMenuSeparator {}
                                                DropdownMenuItem { "خطاف ويب" }
                                            }
                                        }
                                    }
                                    DropdownMenuSeparator {}
                                    DropdownMenuItem { "متقدم..." }
                                }
                            }
                        }
                        DropdownMenuItem {
                            "فريق جديد"
                            DropdownMenuShortcut { "⌘+T" }
                        }
                    }
                    DropdownMenuSeparator {}
                    DropdownMenuGroup {
                        DropdownMenuLabel { "عرض" }
                        DropdownMenuCheckboxItem {
                            checked: show_status_bar(),
                            on_checked_change: move |checked| show_status_bar.set(checked),
                            "شريط الحالة"
                        }
                        DropdownMenuCheckboxItem {
                            checked: show_activity_bar(),
                            on_checked_change: move |checked| show_activity_bar.set(checked),
                            "شريط النشاط"
                        }
                        DropdownMenuCheckboxItem {
                            checked: show_panel(),
                            on_checked_change: move |checked| show_panel.set(checked),
                            "اللوحة"
                        }
                    }
                    DropdownMenuSeparator {}
                    DropdownMenuGroup {
                        DropdownMenuLabel { "الموضع" }
                        DropdownMenuRadioGroup {
                            value: position(),
                            on_value_change: move |value| position.set(value),
                            DropdownMenuRadioItem { value: "top", "أعلى" }
                            DropdownMenuRadioItem { value: "bottom", "أسفل" }
                            DropdownMenuRadioItem { value: "right", "يمين" }
                            DropdownMenuRadioItem { value: "left", "يسار" }
                        }
                    }
                    DropdownMenuSeparator {}
                    DropdownMenuGroup {
                        DropdownMenuItem { variant: DropdownMenuItemVariant::Destructive, "تسجيل الخروج" }
                    }
                }
            }
        }
    }
}
