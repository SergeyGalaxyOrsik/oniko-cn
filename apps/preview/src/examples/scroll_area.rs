//! Scroll Area examples, ported from shadcn/ui `examples/base/scroll-area-*.tsx`.

use dioxus::prelude::*;

use super::Example;
use crate::components::ui::scroll_area::{ScrollArea, ScrollBar, ScrollBarOrientation};
use crate::components::ui::separator::Separator;

#[component]
pub fn Examples() -> Element {
    rsx! {
        ScrollAreaDemo {}
        ScrollAreaHorizontalDemo {}
        ScrollAreaRtl {}
    }
}

fn tags() -> Vec<String> {
    (0..50).map(|i| format!("v1.2.0-beta.{}", 50 - i)).collect()
}

#[component]
fn ScrollAreaDemo() -> Element {
    rsx! {
        Example { title: "Demo", class: "h-96",
            ScrollArea { class: "h-72 w-48 rounded-md border",
                div { class: "p-4",
                    h4 { class: "mb-4 text-sm leading-none font-medium", "Tags" }
                    for tag in tags() {
                        div { key: "{tag}",
                            div { class: "text-sm", "{tag}" }
                            Separator { class: "my-2" }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn ScrollAreaHorizontalDemo() -> Element {
    let works = [
        (
            "Ornella Binni",
            "https://images.unsplash.com/photo-1465869185982-5a1a7522cbcb?auto=format&fit=crop&w=300&q=80",
        ),
        (
            "Tom Byrom",
            "https://images.unsplash.com/photo-1548516173-3cabfa4607e9?auto=format&fit=crop&w=300&q=80",
        ),
        (
            "Vladimir Malyavko",
            "https://images.unsplash.com/photo-1494337480532-3725c85fd2ab?auto=format&fit=crop&w=300&q=80",
        ),
    ];
    rsx! {
        Example { title: "Horizontal", class: "h-auto",
            ScrollArea { class: "w-96 rounded-md border whitespace-nowrap",
                div { class: "flex w-max space-x-4 p-4",
                    for (artist , art) in works {
                        figure { key: "{artist}", class: "shrink-0",
                            div { class: "overflow-hidden rounded-md",
                                img {
                                    src: art,
                                    alt: "Photo by {artist}",
                                    class: "aspect-[3/4] h-fit w-fit object-cover",
                                    width: "300",
                                    height: "400",
                                }
                            }
                            figcaption { class: "pt-2 text-xs text-muted-foreground",
                                "Photo by "
                                span { class: "font-semibold text-foreground", "{artist}" }
                            }
                        }
                    }
                }
                ScrollBar { orientation: ScrollBarOrientation::Horizontal }
            }
        }
    }
}

#[component]
fn ScrollAreaRtl() -> Element {
    rsx! {
        Example { title: "RTL", class: "h-auto",
            ScrollArea { class: "h-72 w-48 rounded-md border", dir: "rtl",
                div { class: "p-4",
                    h4 { class: "mb-4 text-sm leading-none font-medium", "العلامات" }
                    for tag in tags() {
                        div { key: "{tag}",
                            div { class: "text-sm", "{tag}" }
                            Separator { class: "my-2" }
                        }
                    }
                }
            }
        }
    }
}
