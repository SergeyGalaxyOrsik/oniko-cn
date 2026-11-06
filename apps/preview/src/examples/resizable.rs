//! Resizable examples, ported from shadcn/ui `examples/base/resizable-*.tsx`.

use dioxus::prelude::*;

use super::Example;
use crate::components::ui::resizable::{
    ResizableHandle, ResizableOrientation, ResizablePanel, ResizablePanelGroup,
};

#[component]
pub fn Examples() -> Element {
    rsx! {
        ResizableDemo {}
        ResizableVertical {}
        ResizableHandleDemo {}
        ResizableRtl {}
    }
}

#[component]
fn ResizableDemo() -> Element {
    rsx! {
        Example { title: "Demo", class: "h-80",
            ResizablePanelGroup {
                orientation: ResizableOrientation::Horizontal,
                class: "max-w-sm rounded-lg border",
                ResizablePanel { default_size: "50%",
                    div { class: "flex h-[200px] items-center justify-center p-6",
                        span { class: "font-semibold", "One" }
                    }
                }
                ResizableHandle { with_handle: true }
                ResizablePanel { default_size: "50%",
                    ResizablePanelGroup { orientation: ResizableOrientation::Vertical,
                        ResizablePanel { default_size: "25%",
                            div { class: "flex h-full items-center justify-center p-6",
                                span { class: "font-semibold", "Two" }
                            }
                        }
                        ResizableHandle { with_handle: true }
                        ResizablePanel { default_size: "75%",
                            div { class: "flex h-full items-center justify-center p-6",
                                span { class: "font-semibold", "Three" }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn ResizableVertical() -> Element {
    rsx! {
        Example { title: "Vertical", class: "h-80",
            ResizablePanelGroup {
                orientation: ResizableOrientation::Vertical,
                class: "min-h-[200px] max-w-sm rounded-lg border",
                ResizablePanel { default_size: "25%",
                    div { class: "flex h-full items-center justify-center p-6",
                        span { class: "font-semibold", "Header" }
                    }
                }
                ResizableHandle {}
                ResizablePanel { default_size: "75%",
                    div { class: "flex h-full items-center justify-center p-6",
                        span { class: "font-semibold", "Content" }
                    }
                }
            }
        }
    }
}

#[component]
fn ResizableHandleDemo() -> Element {
    rsx! {
        Example { title: "Handle", class: "h-80",
            ResizablePanelGroup {
                orientation: ResizableOrientation::Horizontal,
                class: "min-h-[200px] max-w-sm rounded-lg border",
                ResizablePanel { default_size: "25%",
                    div { class: "flex h-full items-center justify-center p-6",
                        span { class: "font-semibold", "Sidebar" }
                    }
                }
                ResizableHandle { with_handle: true }
                ResizablePanel { default_size: "75%",
                    div { class: "flex h-full items-center justify-center p-6",
                        span { class: "font-semibold", "Content" }
                    }
                }
            }
        }
    }
}

#[component]
fn ResizableRtl() -> Element {
    rsx! {
        Example { title: "RTL", class: "h-80",
            ResizablePanelGroup {
                orientation: ResizableOrientation::Horizontal,
                class: "max-w-sm rounded-lg border",
                dir: "rtl",
                ResizablePanel { default_size: "50%",
                    div { class: "flex h-[200px] items-center justify-center p-6",
                        span { class: "font-semibold", "واحد" }
                    }
                }
                ResizableHandle { with_handle: true }
                ResizablePanel { default_size: "50%",
                    ResizablePanelGroup { orientation: ResizableOrientation::Vertical, dir: "rtl",
                        ResizablePanel { default_size: "25%",
                            div { class: "flex h-full items-center justify-center p-6",
                                span { class: "font-semibold", "اثنان" }
                            }
                        }
                        ResizableHandle { with_handle: true }
                        ResizablePanel { default_size: "75%",
                            div { class: "flex h-full items-center justify-center p-6",
                                span { class: "font-semibold", "ثلاثة" }
                            }
                        }
                    }
                }
            }
        }
    }
}
