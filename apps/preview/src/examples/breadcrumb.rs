//! Breadcrumb examples, ported from shadcn/ui `examples/base/breadcrumb-*.tsx`.

use dioxus::prelude::*;
use lucide_dioxus::{ChevronDown, Dot};

use super::Example;
use crate::components::ui::breadcrumb::{
    Breadcrumb, BreadcrumbEllipsis, BreadcrumbItem, BreadcrumbLink, BreadcrumbList, BreadcrumbPage,
    BreadcrumbSeparator,
};
use crate::components::ui::button::{ButtonSize, ButtonVariant};
use crate::components::ui::dropdown_menu::{
    DropdownMenu, DropdownMenuAlign, DropdownMenuContent, DropdownMenuGroup, DropdownMenuItem,
    DropdownMenuTrigger,
};

#[component]
pub fn Examples() -> Element {
    rsx! {
        div { class: "flex flex-col gap-10",
            BreadcrumbDemo {}
            BreadcrumbBasic {}
            BreadcrumbSeparatorDemo {}
            BreadcrumbDropdown {}
            BreadcrumbEllipsisDemo {}
            BreadcrumbLinkDemo {}
            BreadcrumbRtl {}
        }
    }
}

#[component]
fn BreadcrumbDemo() -> Element {
    rsx! {
        Example { title: "Demo", class: "p-2",
            Breadcrumb {
                BreadcrumbList {
                    BreadcrumbItem {
                        BreadcrumbLink { href: "#", "Home" }
                    }
                    BreadcrumbSeparator {}
                    BreadcrumbItem {
                        DropdownMenu {
                            DropdownMenuTrigger {
                                variant: ButtonVariant::Ghost,
                                size: ButtonSize::IconSm,
                                BreadcrumbEllipsis {}
                                span { class: "sr-only", "Toggle menu" }
                            }
                            DropdownMenuContent { align: DropdownMenuAlign::Start,
                                DropdownMenuGroup {
                                    DropdownMenuItem { "Documentation" }
                                    DropdownMenuItem { "Themes" }
                                    DropdownMenuItem { "GitHub" }
                                }
                            }
                        }
                    }
                    BreadcrumbSeparator {}
                    BreadcrumbItem {
                        BreadcrumbLink { href: "#", "Components" }
                    }
                    BreadcrumbSeparator {}
                    BreadcrumbItem {
                        BreadcrumbPage { "Breadcrumb" }
                    }
                }
            }
        }
    }
}

#[component]
fn BreadcrumbBasic() -> Element {
    rsx! {
        Example { title: "Basic",
            Breadcrumb {
                BreadcrumbList {
                    BreadcrumbItem {
                        BreadcrumbLink { href: "#", "Home" }
                    }
                    BreadcrumbSeparator {}
                    BreadcrumbItem {
                        BreadcrumbLink { href: "#", "Components" }
                    }
                    BreadcrumbSeparator {}
                    BreadcrumbItem {
                        BreadcrumbPage { "Breadcrumb" }
                    }
                }
            }
        }
    }
}

#[component]
fn BreadcrumbSeparatorDemo() -> Element {
    rsx! {
        Example { title: "Custom Separator",
            Breadcrumb {
                BreadcrumbList {
                    BreadcrumbItem {
                        BreadcrumbLink { href: "/", "Home" }
                    }
                    BreadcrumbSeparator {
                        Dot {}
                    }
                    BreadcrumbItem {
                        BreadcrumbLink { href: "/components", "Components" }
                    }
                    BreadcrumbSeparator {
                        Dot {}
                    }
                    BreadcrumbItem {
                        BreadcrumbPage { "Breadcrumb" }
                    }
                }
            }
        }
    }
}

#[component]
fn BreadcrumbDropdown() -> Element {
    rsx! {
        Example { title: "Dropdown",
            Breadcrumb {
                BreadcrumbList {
                    BreadcrumbItem {
                        BreadcrumbLink { href: "/", "Home" }
                    }
                    BreadcrumbSeparator {
                        Dot {}
                    }
                    BreadcrumbItem {
                        DropdownMenu {
                            DropdownMenuTrigger { class: "flex items-center gap-1",
                                "Components"
                                span { "data-icon": "inline-end", class: "contents",
                                    ChevronDown { class: "size-3.5" }
                                }
                            }
                            DropdownMenuContent { align: DropdownMenuAlign::Start,
                                DropdownMenuGroup {
                                    DropdownMenuItem { "Documentation" }
                                    DropdownMenuItem { "Themes" }
                                    DropdownMenuItem { "GitHub" }
                                }
                            }
                        }
                    }
                    BreadcrumbSeparator {
                        Dot {}
                    }
                    BreadcrumbItem {
                        BreadcrumbPage { "Breadcrumb" }
                    }
                }
            }
        }
    }
}

#[component]
fn BreadcrumbEllipsisDemo() -> Element {
    rsx! {
        Example { title: "Collapsed", class: "p-2",
            Breadcrumb {
                BreadcrumbList {
                    BreadcrumbItem {
                        BreadcrumbLink { href: "/", "Home" }
                    }
                    BreadcrumbSeparator {}
                    BreadcrumbItem {
                        BreadcrumbEllipsis {}
                    }
                    BreadcrumbSeparator {}
                    BreadcrumbItem {
                        BreadcrumbLink { href: "/docs/components", "Components" }
                    }
                    BreadcrumbSeparator {}
                    BreadcrumbItem {
                        BreadcrumbPage { "Breadcrumb" }
                    }
                }
            }
        }
    }
}

#[component]
fn BreadcrumbLinkDemo() -> Element {
    rsx! {
        Example { title: "Link Component",
            Breadcrumb {
                BreadcrumbList {
                    BreadcrumbItem {
                        BreadcrumbLink { href: "#link-component", "Home" }
                    }
                    BreadcrumbSeparator {}
                    BreadcrumbItem {
                        BreadcrumbLink { href: "#link-component", "Components" }
                    }
                    BreadcrumbSeparator {}
                    BreadcrumbItem {
                        BreadcrumbPage { "Breadcrumb" }
                    }
                }
            }
        }
    }
}

#[component]
fn BreadcrumbRtl() -> Element {
    rsx! {
        Example { title: "RTL", class: "p-2",
            Breadcrumb { dir: "rtl",
                BreadcrumbList {
                    BreadcrumbItem {
                        BreadcrumbLink { href: "/", "الرئيسية" }
                    }
                    BreadcrumbSeparator {
                        Dot {}
                    }
                    BreadcrumbItem {
                        DropdownMenu {
                            DropdownMenuTrigger { class: "flex items-center gap-1",
                                "المكونات"
                                span { "data-icon": "inline-end", class: "contents",
                                    ChevronDown { class: "size-3.5" }
                                }
                            }
                            DropdownMenuContent { align: DropdownMenuAlign::End, dir: "rtl",
                                DropdownMenuGroup {
                                    DropdownMenuItem { "التوثيق" }
                                    DropdownMenuItem { "السمات" }
                                    DropdownMenuItem { "جيت هاب" }
                                }
                            }
                        }
                    }
                    BreadcrumbSeparator {
                        Dot {}
                    }
                    BreadcrumbItem {
                        BreadcrumbPage { "مسار التنقل" }
                    }
                }
            }
        }
    }
}
