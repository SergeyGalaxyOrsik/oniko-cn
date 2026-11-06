//! Avatar examples, ported from shadcn/ui `examples/base/avatar-*.tsx`.

use dioxus::prelude::*;
use lucide_dioxus::Plus;

use super::Example;
use crate::components::ui::avatar::{
    Avatar, AvatarBadge, AvatarFallback, AvatarGroup, AvatarGroupCount, AvatarImage, AvatarSize,
};
use crate::components::ui::button::{ButtonSize, ButtonVariant};
use crate::components::ui::dropdown_menu::{
    DropdownMenu, DropdownMenuContent, DropdownMenuGroup, DropdownMenuItem,
    DropdownMenuItemVariant, DropdownMenuSeparator, DropdownMenuTrigger,
};

#[component]
pub fn Examples() -> Element {
    rsx! {
        div { class: "flex flex-col gap-10",
            AvatarDemo {}
            AvatarBasic {}
            AvatarWithBadge {}
            AvatarBadgeIconExample {}
            AvatarGroupExample {}
            AvatarGroupCountExample {}
            AvatarGroupCountIconExample {}
            AvatarSizeExample {}
            AvatarDropdown {}
            AvatarRtl {}
        }
    }
}

#[component]
fn AvatarDemo() -> Element {
    rsx! {
        Example { title: "Demo", class: "h-72",
            div { class: "flex flex-row flex-wrap items-center gap-6 md:gap-12",
                Avatar {
                    AvatarImage {
                        src: "https://github.com/shadcn.png",
                        alt: "@shadcn",
                        class: "grayscale",
                    }
                    AvatarFallback { "CN" }
                }
                Avatar {
                    AvatarImage { src: "https://github.com/evilrabbit.png", alt: "@evilrabbit" }
                    AvatarFallback { "ER" }
                    AvatarBadge { class: "bg-green-600 dark:bg-green-800" }
                }
                AvatarGroup { class: "grayscale",
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
                    AvatarGroupCount { "+3" }
                }
            }
        }
    }
}

#[component]
fn AvatarBasic() -> Element {
    rsx! {
        Example { title: "Basic",
            Avatar {
                AvatarImage {
                    src: "https://github.com/shadcn.png",
                    alt: "@shadcn",
                    class: "grayscale",
                }
                AvatarFallback { "CN" }
            }
        }
    }
}

#[component]
fn AvatarWithBadge() -> Element {
    rsx! {
        Example { title: "Badge",
            Avatar {
                AvatarImage { src: "https://github.com/shadcn.png", alt: "@shadcn" }
                AvatarFallback { "CN" }
                AvatarBadge { class: "bg-green-600 dark:bg-green-800" }
            }
        }
    }
}

#[component]
fn AvatarBadgeIconExample() -> Element {
    rsx! {
        Example { title: "Badge with Icon",
            Avatar { class: "grayscale",
                AvatarImage { src: "https://github.com/pranathip.png", alt: "@pranathip" }
                AvatarFallback { "PP" }
                AvatarBadge {
                    Plus {}
                }
            }
        }
    }
}

#[component]
fn AvatarGroupExample() -> Element {
    rsx! {
        Example { title: "Avatar Group",
            AvatarGroup { class: "grayscale",
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
    }
}

#[component]
fn AvatarGroupCountExample() -> Element {
    rsx! {
        Example { title: "Avatar Group Count",
            AvatarGroup { class: "grayscale",
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
                AvatarGroupCount { "+3" }
            }
        }
    }
}

#[component]
fn AvatarGroupCountIconExample() -> Element {
    rsx! {
        Example { title: "Avatar Group Count with Icon",
            AvatarGroup { class: "grayscale",
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
                AvatarGroupCount {
                    Plus {}
                }
            }
        }
    }
}

#[component]
fn AvatarSizeExample() -> Element {
    rsx! {
        Example { title: "Size",
            div { class: "flex flex-wrap items-center gap-2 grayscale",
                Avatar { size: AvatarSize::Sm,
                    AvatarImage { src: "https://github.com/shadcn.png", alt: "@shadcn" }
                    AvatarFallback { "CN" }
                }
                Avatar {
                    AvatarImage { src: "https://github.com/shadcn.png", alt: "@shadcn" }
                    AvatarFallback { "CN" }
                }
                Avatar { size: AvatarSize::Lg,
                    AvatarImage { src: "https://github.com/shadcn.png", alt: "@shadcn" }
                    AvatarFallback { "CN" }
                }
            }
        }
    }
}

#[component]
fn AvatarDropdown() -> Element {
    rsx! {
        Example { title: "Dropdown",
            DropdownMenu {
                DropdownMenuTrigger {
                    variant: ButtonVariant::Ghost,
                    size: ButtonSize::Icon,
                    class: "rounded-full",
                    Avatar {
                        AvatarImage { src: "https://github.com/shadcn.png", alt: "shadcn" }
                        AvatarFallback { "CN" }
                    }
                }
                DropdownMenuContent { class: "w-32",
                    DropdownMenuGroup {
                        DropdownMenuItem { "Profile" }
                        DropdownMenuItem { "Billing" }
                        DropdownMenuItem { "Settings" }
                    }
                    DropdownMenuSeparator {}
                    DropdownMenuGroup {
                        DropdownMenuItem { variant: DropdownMenuItemVariant::Destructive, "Log out" }
                    }
                }
            }
        }
    }
}

#[component]
fn AvatarRtl() -> Element {
    rsx! {
        Example { title: "RTL", class: "h-72",
            div { class: "flex flex-row flex-wrap items-center gap-6 md:gap-12", dir: "rtl",
                Avatar {
                    AvatarImage {
                        src: "https://github.com/shadcn.png",
                        alt: "@shadcn",
                        class: "grayscale",
                    }
                    AvatarFallback { "CN" }
                }
                Avatar {
                    AvatarImage { src: "https://github.com/evilrabbit.png", alt: "@evilrabbit" }
                    AvatarFallback { "ER" }
                    AvatarBadge { class: "bg-green-600 dark:bg-green-800" }
                }
                AvatarGroup { class: "grayscale",
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
                    AvatarGroupCount { "+٣" }
                }
            }
        }
    }
}
