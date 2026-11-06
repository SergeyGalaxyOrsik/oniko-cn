//! Carousel examples, ported from shadcn/ui `examples/base/carousel-*.tsx`.

use dioxus::prelude::*;

use super::Example;
use crate::components::ui::card::{Card, CardContent};
use crate::components::ui::carousel::{
    Carousel, CarouselAlign, CarouselApi, CarouselAutoplay, CarouselContent, CarouselItem,
    CarouselNext, CarouselOptions, CarouselOrientation, CarouselPrevious,
};

#[component]
pub fn Examples() -> Element {
    rsx! {
        CarouselDemo {}
        CarouselSize {}
        CarouselSpacing {}
        CarouselOrientationDemo {}
        CarouselApiDemo {}
        CarouselPlugin {}
        CarouselRtl {}
    }
}

fn to_arabic_numerals(num: usize) -> String {
    const DIGITS: [char; 10] = ['٠', '١', '٢', '٣', '٤', '٥', '٦', '٧', '٨', '٩'];
    num.to_string()
        .chars()
        .filter_map(|c| c.to_digit(10))
        .map(|d| DIGITS[d as usize])
        .collect()
}

#[component]
fn CarouselDemo() -> Element {
    rsx! {
        Example { title: "Demo", class: "h-80 sm:h-[32rem]",
            Carousel { class: "w-full max-w-[12rem] sm:max-w-xs",
                CarouselContent {
                    for index in 0..5 {
                        CarouselItem { key: "{index}",
                            div { class: "p-1",
                                Card {
                                    CardContent { class: "flex aspect-square items-center justify-center p-6",
                                        span { class: "text-4xl font-semibold", "{index + 1}" }
                                    }
                                }
                            }
                        }
                    }
                }
                CarouselPrevious {}
                CarouselNext {}
            }
        }
    }
}

#[component]
fn CarouselSize() -> Element {
    rsx! {
        Example { title: "Sizes", class: "h-80 sm:h-[32rem]",
            Carousel {
                opts: CarouselOptions {
                    align: CarouselAlign::Start,
                    ..Default::default()
                },
                class: "w-full max-w-[12rem] sm:max-w-xs md:max-w-sm",
                CarouselContent {
                    for index in 0..5 {
                        CarouselItem { key: "{index}", class: "basis-1/2 lg:basis-1/3",
                            div { class: "p-1",
                                Card {
                                    CardContent { class: "flex aspect-square items-center justify-center p-6",
                                        span { class: "text-3xl font-semibold", "{index + 1}" }
                                    }
                                }
                            }
                        }
                    }
                }
                CarouselPrevious {}
                CarouselNext {}
            }
        }
    }
}

#[component]
fn CarouselSpacing() -> Element {
    rsx! {
        Example { title: "Spacing", class: "h-80 sm:h-[32rem]",
            Carousel { class: "w-full max-w-[12rem] sm:max-w-xs md:max-w-sm",
                CarouselContent { class: "-ml-1",
                    for index in 0..5 {
                        CarouselItem { key: "{index}", class: "basis-1/2 pl-1 lg:basis-1/3",
                            div { class: "p-1",
                                Card {
                                    CardContent { class: "flex aspect-square items-center justify-center p-6",
                                        span { class: "text-2xl font-semibold", "{index + 1}" }
                                    }
                                }
                            }
                        }
                    }
                }
                CarouselPrevious {}
                CarouselNext {}
            }
        }
    }
}

#[component]
fn CarouselOrientationDemo() -> Element {
    rsx! {
        Example { title: "Orientation", class: "h-[32rem]",
            Carousel {
                opts: CarouselOptions {
                    align: CarouselAlign::Start,
                    ..Default::default()
                },
                orientation: CarouselOrientation::Vertical,
                class: "w-full max-w-xs",
                CarouselContent { class: "-mt-1 h-[270px]",
                    for index in 0..5 {
                        CarouselItem { key: "{index}", class: "basis-1/2 pt-1",
                            div { class: "p-1",
                                Card {
                                    CardContent { class: "flex items-center justify-center p-6",
                                        span { class: "text-3xl font-semibold", "{index + 1}" }
                                    }
                                }
                            }
                        }
                    }
                }
                CarouselPrevious {}
                CarouselNext {}
            }
        }
    }
}

#[component]
fn CarouselApiDemo() -> Element {
    let mut api = use_signal(|| None::<CarouselApi>);
    let current = api().map(|api| api.selected_scroll_snap() + 1).unwrap_or(0);
    let count = api().map(|api| api.scroll_snap_count()).unwrap_or(0);
    rsx! {
        Example { title: "API", class: "sm:h-[32rem]",
            div { class: "mx-auto max-w-[10rem] sm:max-w-xs",
                Carousel {
                    set_api: move |handle| api.set(Some(handle)),
                    class: "w-full max-w-xs",
                    CarouselContent {
                        for index in 0..5 {
                            CarouselItem { key: "{index}",
                                Card { class: "m-px",
                                    CardContent { class: "flex aspect-square items-center justify-center p-6",
                                        span { class: "text-4xl font-semibold", "{index + 1}" }
                                    }
                                }
                            }
                        }
                    }
                    CarouselPrevious {}
                    CarouselNext {}
                }
                div { class: "py-2 text-center text-sm text-muted-foreground",
                    "Slide {current} of {count}"
                }
            }
        }
    }
}

#[component]
fn CarouselPlugin() -> Element {
    rsx! {
        Example { title: "Plugin", class: "sm:h-[32rem]",
            Carousel {
                autoplay: CarouselAutoplay {
                    delay_ms: 2000,
                    stop_on_interaction: true,
                    stop_on_mouse_enter: true,
                },
                class: "w-full max-w-[10rem] sm:max-w-xs",
                CarouselContent {
                    for index in 0..5 {
                        CarouselItem { key: "{index}",
                            div { class: "p-1",
                                Card {
                                    CardContent { class: "flex aspect-square items-center justify-center p-6",
                                        span { class: "text-4xl font-semibold", "{index + 1}" }
                                    }
                                }
                            }
                        }
                    }
                }
                CarouselPrevious {}
                CarouselNext {}
            }
        }
    }
}

#[component]
fn CarouselRtl() -> Element {
    rsx! {
        Example { title: "RTL", class: "h-80 sm:h-[32rem]",
            Carousel { dir: "rtl", class: "w-full max-w-[12rem] sm:max-w-xs",
                CarouselContent {
                    for index in 0..5usize {
                        CarouselItem { key: "{index}",
                            div { class: "p-1",
                                Card {
                                    CardContent { class: "flex aspect-square items-center justify-center p-6",
                                        span { class: "text-4xl font-semibold", "{to_arabic_numerals(index + 1)}" }
                                    }
                                }
                            }
                        }
                    }
                }
                CarouselPrevious {}
                CarouselNext {}
            }
        }
    }
}
