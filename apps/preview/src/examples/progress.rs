//! Progress examples, ported from shadcn/ui `examples/base/progress-*.tsx`.

use dioxus::prelude::*;

use super::Example;
use crate::components::ui::progress::{Progress, ProgressLabel, ProgressValue};
use crate::components::ui::slider::Slider;

#[component]
pub fn Examples() -> Element {
    rsx! {
        div { class: "flex flex-col gap-10",
            ProgressDemo {}
            ProgressWithLabel {}
            ProgressControlled {}
            ProgressRtl {}
        }
    }
}

#[component]
fn ProgressDemo() -> Element {
    let mut progress = use_signal(|| 13.0);
    use_future(move || async move {
        let _ = document::eval("setTimeout(() => dioxus.send(true), 500)")
            .recv::<bool>()
            .await;
        progress.set(66.0);
    });

    rsx! {
        Example { title: "Demo",
            Progress { value: progress(), class: "w-[60%]" }
        }
    }
}

#[component]
fn ProgressWithLabel() -> Element {
    rsx! {
        Example { title: "Label",
            Progress { value: 56.0, class: "w-full max-w-sm",
                ProgressLabel { "Upload progress" }
                ProgressValue {}
            }
        }
    }
}

#[component]
fn ProgressControlled() -> Element {
    let mut value = use_signal(|| 50.0);

    rsx! {
        Example { title: "Controlled",
            div { class: "flex w-full max-w-sm flex-col gap-4",
                Progress { value: value(), class: "w-full" }
                Slider {
                    value: vec![value()],
                    on_value_change: move |next: Vec<f64>| {
                        if let Some(first) = next.first() {
                            value.set(*first);
                        }
                    },
                    min: 0.0,
                    max: 100.0,
                    step: 1.0,
                }
            }
        }
    }
}

#[component]
fn ProgressRtl() -> Element {
    rsx! {
        Example { title: "RTL",
            Progress { value: 56.0, class: "w-full max-w-sm", dir: "rtl",
                ProgressLabel { "تقدم الرفع" }
                ProgressValue {
                    span { class: "ms-auto", "٥٦%" }
                }
            }
        }
    }
}
