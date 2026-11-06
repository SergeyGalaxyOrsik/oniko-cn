//! Command examples, ported from shadcn/ui `examples/base/command-*.tsx`.

use dioxus::prelude::*;
use lucide_dioxus::{
    Bell, Calculator, Calendar, ClipboardPaste, Code, Copy, CreditCard, FileText, Folder,
    FolderPlus, House, Image, Inbox, LayoutGrid, List, Plus, Scissors, Settings, Trash, User,
    ZoomIn, ZoomOut,
};

use super::Example;
use crate::components::ui::button::{Button, ButtonVariant};
use crate::components::ui::command::{
    Command, CommandDialog, CommandEmpty, CommandGroup, CommandInput, CommandItem, CommandList,
    CommandSeparator, CommandShortcut,
};

#[component]
pub fn Examples() -> Element {
    rsx! {
        div { class: "flex flex-col gap-10",
            CommandDemo {}
            CommandBasic {}
            CommandShortcuts {}
            CommandGroups {}
            CommandScrollable {}
            CommandRtl {}
        }
    }
}

#[component]
fn CommandDemo() -> Element {
    rsx! {
        Example { title: "Demo", class: "h-[24.5rem]",
            Command { class: "max-w-sm rounded-lg border",
                CommandInput { placeholder: "Type a command or search..." }
                CommandList {
                    CommandEmpty { "No results found." }
                    CommandGroup { heading: "Suggestions",
                        CommandItem {
                            Calendar {}
                            span { "Calendar" }
                        }
                        CommandItem {
                            Smile {}
                            span { "Search Emoji" }
                        }
                        CommandItem { disabled: true,
                            Calculator {}
                            span { "Calculator" }
                        }
                    }
                    CommandSeparator {}
                    CommandGroup { heading: "Settings",
                        CommandItem {
                            User {}
                            span { "Profile" }
                            CommandShortcut { "⌘P" }
                        }
                        CommandItem {
                            CreditCard {}
                            span { "Billing" }
                            CommandShortcut { "⌘B" }
                        }
                        CommandItem {
                            Settings {}
                            span { "Settings" }
                            CommandShortcut { "⌘S" }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn CommandBasic() -> Element {
    let mut open = use_signal(|| false);

    rsx! {
        Example { title: "Basic",
            div { class: "flex flex-col gap-4",
                Button {
                    variant: ButtonVariant::Outline,
                    class: "w-fit",
                    onclick: move |_| open.set(true),
                    "Open Menu"
                }
                CommandDialog { open: open(), on_open_change: move |next| open.set(next),
                    Command {
                        CommandInput { placeholder: "Type a command or search..." }
                        CommandList {
                            CommandEmpty { "No results found." }
                            CommandGroup { heading: "Suggestions",
                                CommandItem { "Calendar" }
                                CommandItem { "Search Emoji" }
                                CommandItem { "Calculator" }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn CommandShortcuts() -> Element {
    let mut open = use_signal(|| false);

    rsx! {
        Example { title: "Shortcuts",
            div { class: "flex flex-col gap-4",
                Button {
                    variant: ButtonVariant::Outline,
                    class: "w-fit",
                    onclick: move |_| open.set(true),
                    "Open Menu"
                }
                CommandDialog { open: open(), on_open_change: move |next| open.set(next),
                    Command {
                        CommandInput { placeholder: "Type a command or search..." }
                        CommandList {
                            CommandEmpty { "No results found." }
                            CommandGroup { heading: "Settings",
                                CommandItem {
                                    User {}
                                    span { "Profile" }
                                    CommandShortcut { "⌘P" }
                                }
                                CommandItem {
                                    CreditCard {}
                                    span { "Billing" }
                                    CommandShortcut { "⌘B" }
                                }
                                CommandItem {
                                    Settings {}
                                    span { "Settings" }
                                    CommandShortcut { "⌘S" }
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
fn CommandGroups() -> Element {
    let mut open = use_signal(|| false);

    rsx! {
        Example { title: "Groups",
            div { class: "flex flex-col gap-4",
                Button {
                    variant: ButtonVariant::Outline,
                    class: "w-fit",
                    onclick: move |_| open.set(true),
                    "Open Menu"
                }
                CommandDialog { open: open(), on_open_change: move |next| open.set(next),
                    Command {
                        CommandInput { placeholder: "Type a command or search..." }
                        CommandList {
                            CommandEmpty { "No results found." }
                            CommandGroup { heading: "Suggestions",
                                CommandItem {
                                    Calendar {}
                                    span { "Calendar" }
                                }
                                CommandItem {
                                    Smile {}
                                    span { "Search Emoji" }
                                }
                                CommandItem {
                                    Calculator {}
                                    span { "Calculator" }
                                }
                            }
                            CommandSeparator {}
                            CommandGroup { heading: "Settings",
                                CommandItem {
                                    User {}
                                    span { "Profile" }
                                    CommandShortcut { "⌘P" }
                                }
                                CommandItem {
                                    CreditCard {}
                                    span { "Billing" }
                                    CommandShortcut { "⌘B" }
                                }
                                CommandItem {
                                    Settings {}
                                    span { "Settings" }
                                    CommandShortcut { "⌘S" }
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
fn CommandScrollable() -> Element {
    let mut open = use_signal(|| false);

    rsx! {
        Example { title: "Scrollable",
            div { class: "flex flex-col gap-4",
                Button {
                    variant: ButtonVariant::Outline,
                    class: "w-fit",
                    onclick: move |_| open.set(true),
                    "Open Menu"
                }
                CommandDialog { open: open(), on_open_change: move |next| open.set(next),
                    Command {
                        CommandInput { placeholder: "Type a command or search..." }
                        CommandList {
                            CommandEmpty { "No results found." }
                            CommandGroup { heading: "Navigation",
                                CommandItem {
                                    House {}
                                    span { "Home" }
                                    CommandShortcut { "⌘H" }
                                }
                                CommandItem {
                                    Inbox {}
                                    span { "Inbox" }
                                    CommandShortcut { "⌘I" }
                                }
                                CommandItem {
                                    FileText {}
                                    span { "Documents" }
                                    CommandShortcut { "⌘D" }
                                }
                                CommandItem {
                                    Folder {}
                                    span { "Folders" }
                                    CommandShortcut { "⌘F" }
                                }
                            }
                            CommandSeparator {}
                            CommandGroup { heading: "Actions",
                                CommandItem {
                                    Plus {}
                                    span { "New File" }
                                    CommandShortcut { "⌘N" }
                                }
                                CommandItem {
                                    FolderPlus {}
                                    span { "New Folder" }
                                    CommandShortcut { "⇧⌘N" }
                                }
                                CommandItem {
                                    Copy {}
                                    span { "Copy" }
                                    CommandShortcut { "⌘C" }
                                }
                                CommandItem {
                                    Scissors {}
                                    span { "Cut" }
                                    CommandShortcut { "⌘X" }
                                }
                                CommandItem {
                                    ClipboardPaste {}
                                    span { "Paste" }
                                    CommandShortcut { "⌘V" }
                                }
                                CommandItem {
                                    Trash {}
                                    span { "Delete" }
                                    CommandShortcut { "⌫" }
                                }
                            }
                            CommandSeparator {}
                            CommandGroup { heading: "View",
                                CommandItem {
                                    LayoutGrid {}
                                    span { "Grid View" }
                                }
                                CommandItem {
                                    List {}
                                    span { "List View" }
                                }
                                CommandItem {
                                    ZoomIn {}
                                    span { "Zoom In" }
                                    CommandShortcut { "⌘+" }
                                }
                                CommandItem {
                                    ZoomOut {}
                                    span { "Zoom Out" }
                                    CommandShortcut { "⌘-" }
                                }
                            }
                            CommandSeparator {}
                            CommandGroup { heading: "Account",
                                CommandItem {
                                    User {}
                                    span { "Profile" }
                                    CommandShortcut { "⌘P" }
                                }
                                CommandItem {
                                    CreditCard {}
                                    span { "Billing" }
                                    CommandShortcut { "⌘B" }
                                }
                                CommandItem {
                                    Settings {}
                                    span { "Settings" }
                                    CommandShortcut { "⌘S" }
                                }
                                CommandItem {
                                    Bell {}
                                    span { "Notifications" }
                                }
                                CommandItem {
                                    lucide_dioxus::CircleQuestionMark {}
                                    span { "Help & Support" }
                                }
                            }
                            CommandSeparator {}
                            CommandGroup { heading: "Tools",
                                CommandItem {
                                    Calculator {}
                                    span { "Calculator" }
                                }
                                CommandItem {
                                    Calendar {}
                                    span { "Calendar" }
                                }
                                CommandItem {
                                    Image {}
                                    span { "Image Editor" }
                                }
                                CommandItem {
                                    Code {}
                                    span { "Code Editor" }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

/// `lucide-dioxus` 3.63 does not export `Smile`, so this draws the same icon.
#[component]
fn Smile() -> Element {
    rsx! {
        svg {
            xmlns: "http://www.w3.org/2000/svg",
            width: "24",
            height: "24",
            view_box: "0 0 24 24",
            fill: "none",
            stroke: "currentColor",
            stroke_width: "2",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            circle { cx: "12", cy: "12", r: "10" }
            path { d: "M8 14s1.5 2 4 2 4-2 4-2" }
            line { x1: "9", x2: "9.01", y1: "9", y2: "9" }
            line { x1: "15", x2: "15.01", y1: "9", y2: "9" }
        }
    }
}

#[component]
fn CommandRtl() -> Element {
    rsx! {
        Example { title: "RTL", class: "h-[24.5rem]",
            Command { class: "max-w-sm rounded-lg border", dir: "rtl",
                CommandInput { placeholder: "اكتب أمرًا أو ابحث...", dir: "rtl" }
                CommandList {
                    CommandEmpty { "لم يتم العثور على نتائج." }
                    CommandGroup { heading: "اقتراحات",
                        CommandItem {
                            Calendar {}
                            span { "التقويم" }
                        }
                        CommandItem {
                            Smile {}
                            span { "البحث عن الرموز التعبيرية" }
                        }
                        CommandItem { disabled: true,
                            Calculator {}
                            span { "الآلة الحاسبة" }
                        }
                    }
                    CommandSeparator {}
                    CommandGroup { heading: "الإعدادات",
                        CommandItem {
                            User {}
                            span { "الملف الشخصي" }
                            CommandShortcut { "⌘P" }
                        }
                        CommandItem {
                            CreditCard {}
                            span { "الفوترة" }
                            CommandShortcut { "⌘B" }
                        }
                        CommandItem {
                            Settings {}
                            span { "الإعدادات" }
                            CommandShortcut { "⌘S" }
                        }
                    }
                }
            }
        }
    }
}
