//! Hover Card examples, ported from shadcn/ui `examples/base/hover-card-*.tsx`.

use dioxus::prelude::*;

use super::Example;
use crate::components::ui::button::{ButtonSize, ButtonVariant, button_variants};
use crate::components::ui::hover_card::{
    HoverCard, HoverCardContent, HoverCardSide, HoverCardTrigger,
};

fn as_button((attributes, children): (Vec<Attribute>, Element)) -> Element {
    rsx! {
        button { r#type: "button", ..attributes, {children} }
    }
}

#[component]
pub fn Examples() -> Element {
    rsx! {
        HoverCardDemo {}
        HoverCardSides {}
        HoverCardRtl {}
    }
}

#[component]
fn HoverCardDemo() -> Element {
    rsx! {
        Example { title: "Demo", class: "h-80",
            HoverCard {
                HoverCardTrigger {
                    delay: 10,
                    close_delay: 100,
                    render: as_button,
                    class: button_variants(ButtonVariant::Link, ButtonSize::Default, None),
                    "Hover Here"
                }
                HoverCardContent { class: "flex w-64 flex-col gap-0.5",
                    div { class: "font-semibold", "@nextjs" }
                    div { "The React Framework – created and maintained by @vercel." }
                    div { class: "mt-1 text-xs text-muted-foreground", "Joined December 2021" }
                }
            }
        }
    }
}

#[component]
fn HoverCardSides() -> Element {
    let sides = [
        (HoverCardSide::Left, "left"),
        (HoverCardSide::Top, "top"),
        (HoverCardSide::Bottom, "bottom"),
        (HoverCardSide::Right, "right"),
    ];

    rsx! {
        Example { title: "Sides", class: "h-[22rem]",
            div { class: "flex flex-wrap justify-center gap-2",
                for (side , name) in sides {
                    HoverCard { key: "{name}",
                        HoverCardTrigger {
                            delay: 100,
                            close_delay: 100,
                            render: as_button,
                            class: button_variants(ButtonVariant::Outline, ButtonSize::Default, Some("capitalize")),
                            "{name}"
                        }
                        HoverCardContent { side,
                            div { class: "flex flex-col gap-1",
                                h4 { class: "font-medium", "Hover Card" }
                                p { "This hover card appears on the {name} side of the trigger." }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn HoverCardRtl() -> Element {
    let physical = [
        (HoverCardSide::Left, "يسار"),
        (HoverCardSide::Top, "أعلى"),
        (HoverCardSide::Bottom, "أسفل"),
        (HoverCardSide::Right, "يمين"),
    ];
    let logical = [
        (HoverCardSide::InlineStart, "بداية السطر"),
        (HoverCardSide::InlineEnd, "نهاية السطر"),
    ];

    rsx! {
        Example { title: "RTL", class: "h-80",
            div { class: "grid gap-4", dir: "rtl",
                div { class: "flex flex-wrap justify-center gap-2",
                    for (side , label) in physical {
                        HoverCard { key: "{label}",
                            HoverCardTrigger {
                                delay: 10,
                                close_delay: 100,
                                render: as_button,
                                class: button_variants(ButtonVariant::Outline, ButtonSize::Default, None),
                                "{label}"
                            }
                            HoverCardContent { side, class: "flex w-64 flex-col gap-1", dir: "rtl",
                                div { class: "font-semibold", "سماعات لاسلكية" }
                                div { class: "text-sm text-muted-foreground", "٩٩.٩٩ $" }
                            }
                        }
                    }
                }
                div { class: "flex flex-wrap justify-center gap-2",
                    for (side , label) in logical {
                        HoverCard { key: "{label}",
                            HoverCardTrigger {
                                delay: 10,
                                close_delay: 100,
                                render: as_button,
                                class: button_variants(ButtonVariant::Outline, ButtonSize::Default, None),
                                "{label}"
                            }
                            HoverCardContent { side, class: "flex w-64 flex-col gap-1", dir: "rtl",
                                div { class: "font-semibold", "سماعات لاسلكية" }
                                div { class: "text-sm text-muted-foreground", "٩٩.٩٩ $" }
                            }
                        }
                    }
                }
            }
        }
    }
}
