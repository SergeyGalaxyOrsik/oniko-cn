//! Collapsible examples, ported from shadcn/ui `examples/base/collapsible-*.tsx`.

use dioxus::prelude::*;
use lucide_dioxus::{ChevronDown, ChevronRight, ChevronsUpDown, File, Folder, Maximize, Minimize};

use super::Example;
use crate::components::ui::button::{Button, ButtonSize, ButtonVariant};
use crate::components::ui::card::{
    Card, CardContent, CardDescription, CardHeader, CardSize, CardTitle,
};
use crate::components::ui::collapsible::{Collapsible, CollapsibleContent, CollapsibleTrigger};
use crate::components::ui::field::{Field, FieldGroup, FieldLabel};
use crate::components::ui::input::Input;
use crate::components::ui::tabs::{Tabs, TabsList, TabsTrigger};

#[component]
pub fn Examples() -> Element {
    rsx! {
        CollapsibleDemo {}
        CollapsibleBasic {}
        CollapsibleSettings {}
        CollapsibleFileTree {}
        CollapsibleRtl {}
    }
}

#[component]
fn CollapsibleDemo() -> Element {
    let mut is_open = use_signal(|| false);
    rsx! {
        Example { title: "Demo",
            Collapsible {
                open: is_open(),
                on_open_change: move |open| is_open.set(open),
                class: "flex w-[350px] flex-col gap-2",
                div { class: "flex items-center justify-between gap-4 px-4",
                    h4 { class: "text-sm font-semibold", "Order #4189" }
                    CollapsibleTrigger {
                        variant: ButtonVariant::Ghost,
                        size: ButtonSize::Icon,
                        class: "size-8",
                        ChevronsUpDown {}
                        span { class: "sr-only", "Toggle details" }
                    }
                }
                div { class: "flex items-center justify-between rounded-md border px-4 py-2 text-sm",
                    span { class: "text-muted-foreground", "Status" }
                    span { class: "font-medium", "Shipped" }
                }
                CollapsibleContent { class: "flex flex-col gap-2",
                    div { class: "rounded-md border px-4 py-2 text-sm",
                        p { class: "font-medium", "Shipping address" }
                        p { class: "text-muted-foreground", "100 Market St, San Francisco" }
                    }
                    div { class: "rounded-md border px-4 py-2 text-sm",
                        p { class: "font-medium", "Items" }
                        p { class: "text-muted-foreground", "2x Studio Headphones" }
                    }
                }
            }
        }
    }
}

#[component]
fn CollapsibleBasic() -> Element {
    rsx! {
        Example { title: "Basic",
            Card { class: "mx-auto w-full max-w-sm",
                CardContent {
                    Collapsible { class: "rounded-md data-open:bg-muted",
                        CollapsibleTrigger { variant: ButtonVariant::Ghost, class: "w-full",
                            "Product details"
                            ChevronDown { class: "ml-auto group-data-panel-open/button:rotate-180" }
                        }
                        CollapsibleContent { class: "flex flex-col items-start gap-2 p-2.5 pt-0 text-sm",
                            div {
                                "This panel can be expanded or collapsed to reveal additional content."
                            }
                            Button { size: ButtonSize::Xs, "Learn More" }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn CollapsibleSettings() -> Element {
    let mut is_open = use_signal(|| false);
    rsx! {
        Example { title: "Settings Panel",
            Card { class: "mx-auto w-full max-w-xs", size: CardSize::Sm,
                CardHeader {
                    CardTitle { "Radius" }
                    CardDescription { "Set the corner radius of the element." }
                }
                CardContent {
                    Collapsible {
                        open: is_open(),
                        on_open_change: move |open| is_open.set(open),
                        class: "flex items-start gap-2",
                        FieldGroup { class: "grid w-full grid-cols-2 gap-2",
                            Field {
                                FieldLabel { r#for: "radius-x", class: "sr-only", "Radius X" }
                                Input { id: "radius", placeholder: "0", value: "0" }
                            }
                            Field {
                                FieldLabel { r#for: "radius-y", class: "sr-only", "Radius Y" }
                                Input { id: "radius", placeholder: "0", value: "0" }
                            }
                            CollapsibleContent { class: "col-span-full grid grid-cols-subgrid gap-2",
                                Field {
                                    FieldLabel { r#for: "radius-x", class: "sr-only", "Radius X" }
                                    Input { id: "radius", placeholder: "0", value: "0" }
                                }
                                Field {
                                    FieldLabel { r#for: "radius-y", class: "sr-only", "Radius Y" }
                                    Input { id: "radius", placeholder: "0", value: "0" }
                                }
                            }
                        }
                        CollapsibleTrigger { variant: ButtonVariant::Outline, size: ButtonSize::Icon,
                            if is_open() {
                                Minimize {}
                            } else {
                                Maximize {}
                            }
                        }
                    }
                }
            }
        }
    }
}

#[derive(Clone, PartialEq)]
struct FileTreeItem {
    name: &'static str,
    items: Option<Vec<FileTreeItem>>,
}

fn file(name: &'static str) -> FileTreeItem {
    FileTreeItem { name, items: None }
}

fn folder(name: &'static str, items: Vec<FileTreeItem>) -> FileTreeItem {
    FileTreeItem {
        name,
        items: Some(items),
    }
}

fn file_tree() -> Vec<FileTreeItem> {
    vec![
        folder(
            "components",
            vec![
                folder(
                    "ui",
                    vec![
                        file("button.tsx"),
                        file("card.tsx"),
                        file("dialog.tsx"),
                        file("input.tsx"),
                        file("select.tsx"),
                        file("table.tsx"),
                    ],
                ),
                file("login-form.tsx"),
                file("register-form.tsx"),
            ],
        ),
        folder("lib", vec![file("utils.ts"), file("cn.ts"), file("api.ts")]),
        folder(
            "hooks",
            vec![
                file("use-media-query.ts"),
                file("use-debounce.ts"),
                file("use-local-storage.ts"),
            ],
        ),
        folder("types", vec![file("index.d.ts"), file("api.d.ts")]),
        folder(
            "public",
            vec![file("favicon.ico"), file("logo.svg"), file("images")],
        ),
        file("app.tsx"),
        file("layout.tsx"),
        file("globals.css"),
        file("package.json"),
        file("tsconfig.json"),
        file("README.md"),
        file(".gitignore"),
    ]
}

#[component]
fn FileTreeNode(item: FileTreeItem) -> Element {
    match item.items {
        Some(items) => rsx! {
            Collapsible {
                CollapsibleTrigger {
                    variant: ButtonVariant::Ghost,
                    size: ButtonSize::Sm,
                    class: "group w-full justify-start transition-none hover:bg-accent hover:text-accent-foreground",
                    ChevronRight { class: "transition-transform group-data-[state=open]:rotate-90" }
                    Folder {}
                    "{item.name}"
                }
                CollapsibleContent { class: "mt-1 ml-5 style-lyra:ml-4",
                    div { class: "flex flex-col gap-1",
                        for child in items {
                            FileTreeNode { key: "{child.name}", item: child }
                        }
                    }
                }
            }
        },
        None => rsx! {
            Button {
                variant: ButtonVariant::Link,
                size: ButtonSize::Sm,
                class: "w-full justify-start gap-2 text-foreground",
                File {}
                span { "{item.name}" }
            }
        },
    }
}

#[component]
fn CollapsibleFileTree() -> Element {
    rsx! {
        Example { title: "File Tree", class: "h-[36rem]",
            Card { class: "mx-auto w-full max-w-[16rem] gap-2", size: CardSize::Sm,
                CardHeader {
                    Tabs { default_value: "explorer",
                        TabsList { class: "w-full",
                            TabsTrigger { value: "explorer", "Explorer" }
                            TabsTrigger { value: "settings", "Outline" }
                        }
                    }
                }
                CardContent {
                    div { class: "flex flex-col gap-1",
                        for item in file_tree() {
                            FileTreeNode { key: "{item.name}", item }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn CollapsibleRtl() -> Element {
    let mut is_open = use_signal(|| false);
    rsx! {
        Example { title: "RTL",
            Collapsible {
                open: is_open(),
                on_open_change: move |open| is_open.set(open),
                class: "flex w-[350px] flex-col gap-2",
                dir: "rtl",
                div { class: "flex items-center justify-between gap-4 px-4",
                    h4 { class: "text-sm font-semibold", "الطلب #4189" }
                    CollapsibleTrigger {
                        variant: ButtonVariant::Ghost,
                        size: ButtonSize::Icon,
                        class: "size-8",
                        ChevronsUpDown {}
                        span { class: "sr-only", "Toggle details" }
                    }
                }
                div { class: "flex items-center justify-between rounded-md border px-4 py-2 text-sm",
                    span { class: "text-muted-foreground", "الحالة" }
                    span { class: "font-medium", "تم الشحن" }
                }
                CollapsibleContent { class: "flex flex-col gap-2",
                    div { class: "rounded-md border px-4 py-2 text-sm",
                        p { class: "font-medium", "عنوان الشحن" }
                        p { class: "text-muted-foreground", "100 Market St, San Francisco" }
                    }
                    div { class: "rounded-md border px-4 py-2 text-sm",
                        p { class: "font-medium", "العناصر" }
                        p { class: "text-muted-foreground", "2x سماعات الاستوديو" }
                    }
                }
            }
        }
    }
}
