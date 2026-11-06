//! Slider examples, ported from shadcn/ui `examples/base/slider-*.tsx`.

use dioxus::prelude::*;

use super::Example;
use crate::components::ui::label::Label;
use crate::components::ui::slider::{Slider, SliderOrientation};

#[component]
pub fn Examples() -> Element {
    rsx! {
        div { class: "flex flex-col gap-10",
            SliderDemo {}
            SliderRange {}
            SliderMultiple {}
            SliderVertical {}
            SliderControlled {}
            SliderDisabled {}
            SliderRtl {}
        }
    }
}

#[component]
fn SliderDemo() -> Element {
    rsx! {
        Example { title: "Demo",
            Slider {
                default_value: vec![75.0],
                max: 100.0,
                step: 1.0,
                class: "mx-auto w-full max-w-xs",
            }
        }
    }
}

#[component]
fn SliderRange() -> Element {
    rsx! {
        Example { title: "Range",
            Slider {
                default_value: vec![25.0, 50.0],
                max: 100.0,
                step: 5.0,
                class: "mx-auto w-full max-w-xs",
            }
        }
    }
}

#[component]
fn SliderMultiple() -> Element {
    rsx! {
        Example { title: "Multiple Thumbs",
            Slider {
                default_value: vec![10.0, 20.0, 70.0],
                max: 100.0,
                step: 10.0,
                class: "mx-auto w-full max-w-xs",
            }
        }
    }
}

#[component]
fn SliderVertical() -> Element {
    rsx! {
        Example { title: "Vertical",
            div { class: "mx-auto flex w-full max-w-xs items-center justify-center gap-6",
                Slider {
                    default_value: vec![50.0],
                    max: 100.0,
                    step: 1.0,
                    orientation: SliderOrientation::Vertical,
                    class: "h-40",
                }
                Slider {
                    default_value: vec![25.0],
                    max: 100.0,
                    step: 1.0,
                    orientation: SliderOrientation::Vertical,
                    class: "h-40",
                }
            }
        }
    }
}

#[component]
fn SliderControlled() -> Element {
    let mut value = use_signal(|| vec![0.3, 0.7]);
    let text = value
        .read()
        .iter()
        .map(f64::to_string)
        .collect::<Vec<_>>()
        .join(", ");

    rsx! {
        Example { title: "Controlled",
            div { class: "mx-auto grid w-full max-w-xs gap-3",
                div { class: "flex items-center justify-between gap-2",
                    Label { r#for: "slider-demo-temperature", "Temperature" }
                    span { class: "text-sm text-muted-foreground", "{text}" }
                }
                Slider {
                    id: "slider-demo-temperature",
                    value: value(),
                    on_value_change: move |next: Vec<f64>| value.set(next),
                    min: 0.0,
                    max: 1.0,
                    step: 0.1,
                }
            }
        }
    }
}

#[component]
fn SliderDisabled() -> Element {
    rsx! {
        Example { title: "Disabled",
            Slider {
                default_value: vec![50.0],
                max: 100.0,
                step: 1.0,
                disabled: true,
                class: "mx-auto w-full max-w-xs",
            }
        }
    }
}

#[component]
fn SliderRtl() -> Element {
    rsx! {
        Example { title: "RTL",
            Slider {
                default_value: vec![75.0],
                max: 100.0,
                step: 1.0,
                class: "mx-auto w-full max-w-xs",
                dir: "rtl",
            }
        }
    }
}
