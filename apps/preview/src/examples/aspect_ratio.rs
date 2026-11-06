//! Aspect Ratio examples, ported from shadcn/ui `examples/base/aspect-ratio-*.tsx`.

use dioxus::prelude::*;

use super::Example;
use crate::components::ui::aspect_ratio::AspectRatio;

const IMAGE_CLASS: &str =
    "absolute inset-0 size-full rounded-lg object-cover grayscale dark:brightness-20";

#[component]
pub fn Examples() -> Element {
    rsx! {
        div { class: "flex flex-col gap-10",
            AspectRatioDemo {}
            AspectRatioSquare {}
            AspectRatioPortrait {}
            AspectRatioRtl {}
        }
    }
}

#[component]
fn AspectRatioDemo() -> Element {
    rsx! {
        Example { title: "Demo",
            AspectRatio { ratio: 16.0 / 9.0, class: "w-full max-w-sm rounded-lg bg-muted",
                img { src: "https://avatar.vercel.sh/shadcn1", alt: "Photo", class: IMAGE_CLASS }
            }
        }
    }
}

#[component]
fn AspectRatioSquare() -> Element {
    rsx! {
        Example { title: "Square",
            AspectRatio { ratio: 1.0, class: "w-full max-w-[12rem] rounded-lg bg-muted",
                img { src: "https://avatar.vercel.sh/shadcn1", alt: "Photo", class: IMAGE_CLASS }
            }
        }
    }
}

#[component]
fn AspectRatioPortrait() -> Element {
    rsx! {
        Example { title: "Portrait", class: "h-96",
            AspectRatio { ratio: 9.0 / 16.0, class: "w-full max-w-[10rem] rounded-lg bg-muted",
                img { src: "https://avatar.vercel.sh/shadcn1", alt: "Photo", class: IMAGE_CLASS }
            }
        }
    }
}

#[component]
fn AspectRatioRtl() -> Element {
    rsx! {
        Example { title: "RTL", class: "h-96",
            figure { class: "w-full max-w-sm", dir: "rtl",
                AspectRatio { ratio: 16.0 / 9.0, class: "rounded-lg bg-muted",
                    img { src: "https://avatar.vercel.sh/shadcn1", alt: "Photo", class: IMAGE_CLASS }
                }
                figcaption { class: "mt-2 text-center text-sm text-muted-foreground",
                    "منظر طبيعي جميل"
                }
            }
        }
    }
}
