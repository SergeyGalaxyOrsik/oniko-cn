//! Empty examples, ported from shadcn/ui `examples/base/empty-*.tsx`.

use dioxus::prelude::*;
use lucide_dioxus::{ArrowUpRight, Bell, Cloud, FolderCode, Plus, RefreshCcw, Search};

use super::Example;
use crate::components::ui::avatar::{Avatar, AvatarFallback, AvatarImage};
use crate::components::ui::button::{Button, ButtonSize, ButtonVariant, button_variants};
use crate::components::ui::empty::{
    Empty, EmptyContent, EmptyDescription, EmptyHeader, EmptyMedia, EmptyMediaVariant, EmptyTitle,
};
use crate::components::ui::input_group::{
    InputGroup, InputGroupAddon, InputGroupAlign, InputGroupInput,
};
use crate::components::ui::kbd::Kbd;

#[component]
pub fn Examples() -> Element {
    rsx! {
        div { class: "flex flex-col gap-10",
            EmptyDemo {}
            EmptyOutline {}
            EmptyMuted {}
            EmptyAvatar {}
            EmptyAvatarGroup {}
            EmptyInputGroup {}
            EmptyRtl {}
        }
    }
}

#[component]
fn EmptyDemo() -> Element {
    rsx! {
        Example { title: "Demo", class: "h-96 p-0",
            Empty {
                EmptyHeader {
                    EmptyMedia { variant: EmptyMediaVariant::Icon,
                        FolderCode {}
                    }
                    EmptyTitle { "No Projects Yet" }
                    EmptyDescription {
                        "You haven't created any projects yet. Get started by creating your first project."
                    }
                }
                EmptyContent { class: "flex-row justify-center gap-2",
                    Button { "Create Project" }
                    Button { variant: ButtonVariant::Outline, "Import Project" }
                }
                a {
                    href: "#",
                    class: button_variants(ButtonVariant::Link, ButtonSize::Sm, Some("text-muted-foreground")),
                    "Learn More "
                    ArrowUpRight {}
                }
            }
        }
    }
}

#[component]
fn EmptyOutline() -> Element {
    rsx! {
        Example { title: "Outline", class: "h-96 p-6 md:p-10",
            Empty { class: "border border-dashed",
                EmptyHeader {
                    EmptyMedia { variant: EmptyMediaVariant::Icon,
                        Cloud {}
                    }
                    EmptyTitle { "Cloud Storage Empty" }
                    EmptyDescription { "Upload files to your cloud storage to access them anywhere." }
                }
                EmptyContent {
                    Button { variant: ButtonVariant::Outline, size: ButtonSize::Sm, "Upload Files" }
                }
            }
        }
    }
}

#[component]
fn EmptyMuted() -> Element {
    rsx! {
        Example { title: "Background", class: "h-96 p-0",
            Empty { class: "h-full bg-muted/30",
                EmptyHeader {
                    EmptyMedia { variant: EmptyMediaVariant::Icon,
                        Bell {}
                    }
                    EmptyTitle { "No Notifications" }
                    EmptyDescription { class: "max-w-xs text-pretty",
                        "You're all caught up. New notifications will appear here."
                    }
                }
                EmptyContent {
                    Button { variant: ButtonVariant::Outline,
                        span { "data-icon": "inline-start", class: "contents",
                            RefreshCcw {}
                        }
                        "Refresh"
                    }
                }
            }
        }
    }
}

#[component]
fn EmptyAvatar() -> Element {
    rsx! {
        Example { title: "Avatar", class: "h-96 p-0",
            Empty {
                EmptyHeader {
                    EmptyMedia { variant: EmptyMediaVariant::Default,
                        Avatar { class: "size-12",
                            AvatarImage { src: "https://github.com/shadcn.png", class: "grayscale" }
                            AvatarFallback { "LR" }
                        }
                    }
                    EmptyTitle { "User Offline" }
                    EmptyDescription {
                        "This user is currently offline. You can leave a message to notify them or try again later."
                    }
                }
                EmptyContent {
                    Button { size: ButtonSize::Sm, "Leave Message" }
                }
            }
        }
    }
}

#[component]
fn EmptyAvatarGroup() -> Element {
    rsx! {
        Example { title: "Avatar Group", class: "h-96 p-0",
            Empty {
                EmptyHeader {
                    EmptyMedia {
                        div { class: "flex -space-x-2 *:data-[slot=avatar]:size-12 *:data-[slot=avatar]:ring-2 *:data-[slot=avatar]:ring-background *:data-[slot=avatar]:grayscale",
                            Avatar {
                                AvatarImage { src: "https://github.com/shadcn.png", alt: "@shadcn" }
                                AvatarFallback { "CN" }
                            }
                            Avatar {
                                AvatarImage { src: "https://github.com/maxleiter.png", alt: "@maxleiter" }
                                AvatarFallback { "LR" }
                            }
                            Avatar {
                                AvatarImage { src: "https://github.com/evilrabbit.png", alt: "@evilrabbit" }
                                AvatarFallback { "ER" }
                            }
                        }
                    }
                    EmptyTitle { "No Team Members" }
                    EmptyDescription { "Invite your team to collaborate on this project." }
                }
                EmptyContent {
                    Button { size: ButtonSize::Sm,
                        Plus {}
                        "Invite Members"
                    }
                }
            }
        }
    }
}

#[component]
fn EmptyInputGroup() -> Element {
    rsx! {
        Example { title: "Input Group", class: "h-96 p-0",
            Empty {
                EmptyHeader {
                    EmptyTitle { "404 - Not Found" }
                    EmptyDescription {
                        "The page you're looking for doesn't exist. Try searching for what you need below."
                    }
                }
                EmptyContent {
                    InputGroup { class: "sm:w-3/4",
                        InputGroupInput { placeholder: "Try searching for pages..." }
                        InputGroupAddon {
                            Search {}
                        }
                        InputGroupAddon { align: InputGroupAlign::InlineEnd,
                            Kbd { "/" }
                        }
                    }
                    EmptyDescription {
                        "Need help? "
                        a { href: "#", "Contact support" }
                    }
                }
            }
        }
    }
}

#[component]
fn EmptyRtl() -> Element {
    rsx! {
        Example { title: "RTL",
            Empty { dir: "rtl",
                EmptyHeader {
                    EmptyMedia { variant: EmptyMediaVariant::Icon,
                        FolderCode {}
                    }
                    EmptyTitle { "لا توجد مشاريع بعد" }
                    EmptyDescription { "لم تقم بإنشاء أي مشاريع بعد. ابدأ بإنشاء مشروعك الأول." }
                }
                EmptyContent { class: "flex-row justify-center gap-2",
                    Button { "إنشاء مشروع" }
                    Button { variant: ButtonVariant::Outline, "استيراد مشروع" }
                }
                a {
                    href: "#",
                    class: button_variants(ButtonVariant::Link, ButtonSize::Sm, Some("text-muted-foreground")),
                    "تعرف على المزيد "
                    span { "data-icon": "inline-end", class: "contents",
                        ArrowUpRight { class: "rtl:rotate-270" }
                    }
                }
            }
        }
    }
}
