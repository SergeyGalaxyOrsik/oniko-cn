//! Item examples, ported from shadcn/ui `examples/base/item-*.tsx`.

use dioxus::prelude::*;
use lucide_dioxus::{
    BadgeCheck, ChevronDown, ChevronRight, ExternalLink, Inbox, Plus, ShieldAlert,
};

use super::Example;
use crate::components::ui::avatar::{Avatar, AvatarFallback, AvatarImage};
use crate::components::ui::button::{Button, ButtonSize, ButtonVariant};
use crate::components::ui::dropdown_menu::{
    DropdownMenu, DropdownMenuContent, DropdownMenuGroup, DropdownMenuItem, DropdownMenuTrigger,
};
use crate::components::ui::item::{
    Item, ItemActions, ItemContent, ItemDescription, ItemGroup, ItemHeader, ItemMedia,
    ItemMediaVariant, ItemSize, ItemTitle, ItemVariant, item_variants,
};

struct Person {
    username: &'static str,
    avatar: &'static str,
    email: &'static str,
}

const PEOPLE: [Person; 3] = [
    Person {
        username: "shadcn",
        avatar: "https://github.com/shadcn.png",
        email: "shadcn@vercel.com",
    },
    Person {
        username: "maxleiter",
        avatar: "https://github.com/maxleiter.png",
        email: "maxleiter@vercel.com",
    },
    Person {
        username: "evilrabbit",
        avatar: "https://github.com/evilrabbit.png",
        email: "evilrabbit@vercel.com",
    },
];

struct Song {
    title: &'static str,
    artist: &'static str,
    album: &'static str,
    duration: &'static str,
}

const MUSIC: [Song; 3] = [
    Song {
        title: "Midnight City Lights",
        artist: "Neon Dreams",
        album: "Electric Nights",
        duration: "3:45",
    },
    Song {
        title: "Coffee Shop Conversations",
        artist: "The Morning Brew",
        album: "Urban Stories",
        duration: "4:05",
    },
    Song {
        title: "Digital Rain",
        artist: "Cyber Symphony",
        album: "Binary Beats",
        duration: "3:30",
    },
];

struct Model {
    name: &'static str,
    description: &'static str,
    image: &'static str,
}

const MODELS: [Model; 3] = [
    Model {
        name: "v0-1.5-sm",
        description: "Everyday tasks and UI generation.",
        image: "https://images.unsplash.com/photo-1650804068570-7fb2e3dbf888?q=80&w=640&auto=format&fit=crop",
    },
    Model {
        name: "v0-1.5-lg",
        description: "Advanced thinking or reasoning.",
        image: "https://images.unsplash.com/photo-1610280777472-54133d004c8c?q=80&w=640&auto=format&fit=crop",
    },
    Model {
        name: "v0-2.0-mini",
        description: "Open Source model for everyone.",
        image: "https://images.unsplash.com/photo-1602146057681-08560aee8cde?q=80&w=640&auto=format&fit=crop",
    },
];

#[component]
pub fn Examples() -> Element {
    rsx! {
        div { class: "flex flex-col gap-10",
            ItemDemo {}
            ItemVariantExample {}
            ItemSizeDemo {}
            ItemIcon {}
            ItemAvatar {}
            ItemImage {}
            ItemGroupExample {}
            ItemHeaderDemo {}
            ItemLink {}
            ItemDropdown {}
            ItemRtl {}
        }
    }
}

#[component]
fn ItemDemo() -> Element {
    rsx! {
        Example { title: "Demo",
            div { class: "flex w-full max-w-md flex-col gap-6",
                Item { variant: ItemVariant::Outline,
                    ItemContent {
                        ItemTitle { "Basic Item" }
                        ItemDescription { "A simple item with title and description." }
                    }
                    ItemActions {
                        Button { variant: ButtonVariant::Outline, size: ButtonSize::Sm, "Action" }
                    }
                }
                a {
                    "data-slot": "item",
                    "data-variant": ItemVariant::Outline.as_str(),
                    "data-size": ItemSize::Sm.as_str(),
                    href: "#",
                    class: item_variants(ItemVariant::Outline, ItemSize::Sm, None),
                    ItemMedia {
                        BadgeCheck { class: "size-5" }
                    }
                    ItemContent {
                        ItemTitle { "Your profile has been verified." }
                    }
                    ItemActions {
                        ChevronRight { class: "size-4" }
                    }
                }
            }
        }
    }
}

#[component]
fn ItemVariantExample() -> Element {
    rsx! {
        Example { title: "Variant", class: "h-96",
            div { class: "flex w-full max-w-md flex-col gap-6",
                Item {
                    ItemMedia { variant: ItemMediaVariant::Icon,
                        Inbox {}
                    }
                    ItemContent {
                        ItemTitle { "Default Variant" }
                        ItemDescription { "Transparent background with no border." }
                    }
                }
                Item { variant: ItemVariant::Outline,
                    ItemMedia { variant: ItemMediaVariant::Icon,
                        Inbox {}
                    }
                    ItemContent {
                        ItemTitle { "Outline Variant" }
                        ItemDescription { "Outlined style with a visible border." }
                    }
                }
                Item { variant: ItemVariant::Muted,
                    ItemMedia { variant: ItemMediaVariant::Icon,
                        Inbox {}
                    }
                    ItemContent {
                        ItemTitle { "Muted Variant" }
                        ItemDescription { "Muted background for secondary content." }
                    }
                }
            }
        }
    }
}

#[component]
fn ItemSizeDemo() -> Element {
    rsx! {
        Example { title: "Size", class: "h-96",
            div { class: "flex w-full max-w-md flex-col gap-6",
                Item { variant: ItemVariant::Outline,
                    ItemMedia { variant: ItemMediaVariant::Icon,
                        Inbox {}
                    }
                    ItemContent {
                        ItemTitle { "Default Size" }
                        ItemDescription { "The standard size for most use cases." }
                    }
                }
                Item { variant: ItemVariant::Outline, size: ItemSize::Sm,
                    ItemMedia { variant: ItemMediaVariant::Icon,
                        Inbox {}
                    }
                    ItemContent {
                        ItemTitle { "Small Size" }
                        ItemDescription { "A compact size for dense layouts." }
                    }
                }
                Item { variant: ItemVariant::Outline, size: ItemSize::Xs,
                    ItemMedia { variant: ItemMediaVariant::Icon,
                        Inbox {}
                    }
                    ItemContent {
                        ItemTitle { "Extra Small Size" }
                        ItemDescription { "The most compact size available." }
                    }
                }
            }
        }
    }
}

#[component]
fn ItemIcon() -> Element {
    rsx! {
        Example { title: "Icon",
            div { class: "flex w-full max-w-lg flex-col gap-6",
                Item { variant: ItemVariant::Outline,
                    ItemMedia { variant: ItemMediaVariant::Icon,
                        ShieldAlert {}
                    }
                    ItemContent {
                        ItemTitle { "Security Alert" }
                        ItemDescription { "New login detected from unknown device." }
                    }
                    ItemActions {
                        Button { size: ButtonSize::Sm, variant: ButtonVariant::Outline, "Review" }
                    }
                }
            }
        }
    }
}

#[component]
fn ItemAvatar() -> Element {
    rsx! {
        Example { title: "Avatar",
            div { class: "flex w-full max-w-lg flex-col gap-6",
                Item { variant: ItemVariant::Outline,
                    ItemMedia {
                        Avatar { class: "size-10",
                            AvatarImage { src: "https://github.com/evilrabbit.png" }
                            AvatarFallback { "ER" }
                        }
                    }
                    ItemContent {
                        ItemTitle { "Evil Rabbit" }
                        ItemDescription { "Last seen 5 months ago" }
                    }
                    ItemActions {
                        Button {
                            size: ButtonSize::IconSm,
                            variant: ButtonVariant::Outline,
                            class: "rounded-full",
                            aria_label: "Invite",
                            Plus {}
                        }
                    }
                }
                Item { variant: ItemVariant::Outline,
                    ItemMedia {
                        div { class: "flex -space-x-2 *:data-[slot=avatar]:ring-2 *:data-[slot=avatar]:ring-background *:data-[slot=avatar]:grayscale",
                            Avatar { class: "hidden sm:flex",
                                AvatarImage { src: "https://github.com/shadcn.png", alt: "@shadcn" }
                                AvatarFallback { "CN" }
                            }
                            Avatar { class: "hidden sm:flex",
                                AvatarImage { src: "https://github.com/maxleiter.png", alt: "@maxleiter" }
                                AvatarFallback { "LR" }
                            }
                            Avatar {
                                AvatarImage { src: "https://github.com/evilrabbit.png", alt: "@evilrabbit" }
                                AvatarFallback { "ER" }
                            }
                        }
                    }
                    ItemContent {
                        ItemTitle { "No Team Members" }
                        ItemDescription { "Invite your team to collaborate on this project." }
                    }
                    ItemActions {
                        Button { size: ButtonSize::Sm, variant: ButtonVariant::Outline, "Invite" }
                    }
                }
            }
        }
    }
}

#[component]
fn ItemImage() -> Element {
    rsx! {
        Example { title: "Image",
            div { class: "flex w-full max-w-md flex-col gap-6",
                ItemGroup { class: "gap-4",
                    for song in MUSIC {
                        a {
                            key: "{song.title}",
                            "data-slot": "item",
                            "data-variant": ItemVariant::Outline.as_str(),
                            "data-size": ItemSize::Default.as_str(),
                            role: "listitem",
                            href: "#",
                            class: item_variants(ItemVariant::Outline, ItemSize::Default, None),
                            ItemMedia { variant: ItemMediaVariant::Image,
                                img {
                                    src: "https://avatar.vercel.sh/{song.title}",
                                    alt: song.title,
                                    width: "32",
                                    height: "32",
                                    class: "object-cover grayscale",
                                }
                            }
                            ItemContent {
                                ItemTitle { class: "line-clamp-1",
                                    "{song.title} - "
                                    span { class: "text-muted-foreground", "{song.album}" }
                                }
                                ItemDescription { "{song.artist}" }
                            }
                            ItemContent { class: "flex-none text-center",
                                ItemDescription { "{song.duration}" }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn ItemGroupExample() -> Element {
    rsx! {
        Example { title: "Group", class: "h-96",
            ItemGroup { class: "max-w-sm",
                for person in PEOPLE {
                    Item { key: "{person.username}", variant: ItemVariant::Outline,
                        ItemMedia {
                            Avatar {
                                AvatarImage { src: person.avatar, class: "grayscale" }
                                AvatarFallback { "{person.username.chars().next().unwrap_or_default()}" }
                            }
                        }
                        ItemContent { class: "gap-1",
                            ItemTitle { "{person.username}" }
                            ItemDescription { "{person.email}" }
                        }
                        ItemActions {
                            Button {
                                variant: ButtonVariant::Ghost,
                                size: ButtonSize::Icon,
                                class: "rounded-full",
                                Plus {}
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn ItemHeaderDemo() -> Element {
    rsx! {
        Example { title: "Header", class: "h-96",
            div { class: "flex w-full max-w-xl flex-col gap-6",
                ItemGroup { class: "grid grid-cols-3 gap-4",
                    for model in MODELS {
                        Item { key: "{model.name}", variant: ItemVariant::Outline,
                            ItemHeader {
                                img {
                                    src: model.image,
                                    alt: model.name,
                                    width: "128",
                                    height: "128",
                                    class: "aspect-square w-full rounded-sm object-cover",
                                }
                            }
                            ItemContent {
                                ItemTitle { "{model.name}" }
                                ItemDescription { "{model.description}" }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn ItemLink() -> Element {
    rsx! {
        Example { title: "Link",
            div { class: "flex w-full max-w-md flex-col gap-4",
                a {
                    "data-slot": "item",
                    "data-variant": ItemVariant::Default.as_str(),
                    "data-size": ItemSize::Default.as_str(),
                    href: "#",
                    class: item_variants(ItemVariant::Default, ItemSize::Default, None),
                    ItemContent {
                        ItemTitle { "Visit our documentation" }
                        ItemDescription { "Learn how to get started with our components." }
                    }
                    ItemActions {
                        ChevronRight { class: "size-4" }
                    }
                }
                a {
                    "data-slot": "item",
                    "data-variant": ItemVariant::Outline.as_str(),
                    "data-size": ItemSize::Default.as_str(),
                    href: "#",
                    target: "_blank",
                    rel: "noopener noreferrer",
                    class: item_variants(ItemVariant::Outline, ItemSize::Default, None),
                    ItemContent {
                        ItemTitle { "External resource" }
                        ItemDescription { "Opens in a new tab with security attributes." }
                    }
                    ItemActions {
                        ExternalLink { class: "size-4" }
                    }
                }
            }
        }
    }
}

#[component]
fn ItemDropdown() -> Element {
    rsx! {
        Example { title: "Dropdown",
            DropdownMenu {
                DropdownMenuTrigger { variant: ButtonVariant::Outline,
                    "Select "
                    ChevronDown {}
                }
                DropdownMenuContent { class: "w-48",
                    DropdownMenuGroup {
                        for person in PEOPLE {
                            DropdownMenuItem { key: "{person.username}",
                                Item { size: ItemSize::Xs, class: "w-full p-2",
                                    ItemMedia {
                                        Avatar { class: "size-[--spacing(6.5)]",
                                            AvatarImage { src: person.avatar, class: "grayscale" }
                                            AvatarFallback { "{person.username.chars().next().unwrap_or_default()}" }
                                        }
                                    }
                                    ItemContent { class: "gap-0",
                                        ItemTitle { "{person.username}" }
                                        ItemDescription { class: "leading-none", "{person.email}" }
                                    }
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
fn ItemRtl() -> Element {
    rsx! {
        Example { title: "RTL",
            div { class: "flex w-full max-w-md flex-col gap-6", dir: "rtl",
                Item { variant: ItemVariant::Outline, dir: "rtl",
                    ItemContent {
                        ItemTitle { "عنصر أساسي" }
                        ItemDescription { "عنصر بسيط يحتوي على عنوان ووصف." }
                    }
                    ItemActions {
                        Button { variant: ButtonVariant::Outline, size: ButtonSize::Sm, "إجراء" }
                    }
                }
                a {
                    "data-slot": "item",
                    "data-variant": ItemVariant::Outline.as_str(),
                    "data-size": ItemSize::Sm.as_str(),
                    href: "#",
                    dir: "rtl",
                    class: item_variants(ItemVariant::Outline, ItemSize::Sm, None),
                    ItemMedia {
                        BadgeCheck { class: "size-5" }
                    }
                    ItemContent {
                        ItemTitle { "تم التحقق من ملفك الشخصي." }
                    }
                    ItemActions {
                        ChevronRight { class: "size-4" }
                    }
                }
            }
        }
    }
}
