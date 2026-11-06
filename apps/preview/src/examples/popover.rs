//! Popover examples, ported from shadcn/ui `examples/base/popover-*.tsx`.

use dioxus::prelude::*;

use super::Example;
use crate::components::ui::button::{ButtonSize, ButtonVariant, button_variants};
use crate::components::ui::field::Field;
use crate::components::ui::field::{FieldGroup, FieldLabel, FieldOrientation};
use crate::components::ui::input::Input;
use crate::components::ui::label::Label;
use crate::components::ui::popover::{
    Popover, PopoverAlign, PopoverContent, PopoverDescription, PopoverHeader, PopoverSide,
    PopoverTitle, PopoverTrigger,
};

#[component]
pub fn Examples() -> Element {
    rsx! {
        PopoverDemo {}
        PopoverBasic {}
        PopoverAlignments {}
        PopoverForm {}
        PopoverRtl {}
    }
}

#[component]
fn PopoverDemo() -> Element {
    rsx! {
        Example { title: "Demo",
            Popover {
                PopoverTrigger { class: button_variants(ButtonVariant::Outline, ButtonSize::Default, None),
                    "Open popover"
                }
                PopoverContent { class: "w-80",
                    div { class: "grid gap-4",
                        div { class: "space-y-2",
                            h4 { class: "leading-none font-medium", "Dimensions" }
                            p { class: "text-sm text-muted-foreground", "Set the dimensions for the layer." }
                        }
                        div { class: "grid gap-2",
                            div { class: "grid grid-cols-3 items-center gap-4",
                                Label { r#for: "width", "Width" }
                                Input { id: "width", value: "100%", class: "col-span-2 h-8" }
                            }
                            div { class: "grid grid-cols-3 items-center gap-4",
                                Label { r#for: "maxWidth", "Max. width" }
                                Input { id: "maxWidth", value: "300px", class: "col-span-2 h-8" }
                            }
                            div { class: "grid grid-cols-3 items-center gap-4",
                                Label { r#for: "height", "Height" }
                                Input { id: "height", value: "25px", class: "col-span-2 h-8" }
                            }
                            div { class: "grid grid-cols-3 items-center gap-4",
                                Label { r#for: "maxHeight", "Max. height" }
                                Input { id: "maxHeight", value: "none", class: "col-span-2 h-8" }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn PopoverBasic() -> Element {
    rsx! {
        Example { title: "Basic",
            Popover {
                PopoverTrigger { class: button_variants(ButtonVariant::Outline, ButtonSize::Default, Some("w-fit")),
                    "Open Popover"
                }
                PopoverContent { align: PopoverAlign::Start,
                    PopoverHeader {
                        PopoverTitle { "Dimensions" }
                        PopoverDescription { "Set the dimensions for the layer." }
                    }
                }
            }
        }
    }
}

#[component]
fn PopoverAlignments() -> Element {
    rsx! {
        Example { title: "Align",
            div { class: "flex gap-6",
                Popover {
                    PopoverTrigger { class: button_variants(ButtonVariant::Outline, ButtonSize::Sm, None),
                        "Start"
                    }
                    PopoverContent { align: PopoverAlign::Start, class: "w-40", "Aligned to start" }
                }
                Popover {
                    PopoverTrigger { class: button_variants(ButtonVariant::Outline, ButtonSize::Sm, None),
                        "Center"
                    }
                    PopoverContent { align: PopoverAlign::Center, class: "w-40", "Aligned to center" }
                }
                Popover {
                    PopoverTrigger { class: button_variants(ButtonVariant::Outline, ButtonSize::Sm, None),
                        "End"
                    }
                    PopoverContent { align: PopoverAlign::End, class: "w-40", "Aligned to end" }
                }
            }
        }
    }
}

#[component]
fn PopoverForm() -> Element {
    rsx! {
        Example { title: "With Form",
            Popover {
                PopoverTrigger { class: button_variants(ButtonVariant::Outline, ButtonSize::Default, None),
                    "Open Popover"
                }
                PopoverContent { class: "w-64", align: PopoverAlign::Start,
                    PopoverHeader {
                        PopoverTitle { "Dimensions" }
                        PopoverDescription { "Set the dimensions for the layer." }
                    }
                    FieldGroup { class: "gap-4",
                        Field { orientation: FieldOrientation::Horizontal,
                            FieldLabel { r#for: "width", class: "w-1/2", "Width" }
                            Input { id: "width", value: "100%" }
                        }
                        Field { orientation: FieldOrientation::Horizontal,
                            FieldLabel { r#for: "height", class: "w-1/2", "Height" }
                            Input { id: "height", value: "25px" }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn PopoverRtl() -> Element {
    let physical = [
        (PopoverSide::Left, "يسار"),
        (PopoverSide::Top, "أعلى"),
        (PopoverSide::Bottom, "أسفل"),
        (PopoverSide::Right, "يمين"),
    ];
    let logical = [
        (PopoverSide::InlineStart, "بداية السطر"),
        (PopoverSide::InlineEnd, "نهاية السطر"),
    ];

    rsx! {
        Example { title: "RTL",
            div { class: "grid gap-4", dir: "rtl",
                div { class: "flex flex-wrap justify-center gap-2",
                    for (side , label) in physical {
                        Popover { key: "{label}",
                            PopoverTrigger { class: button_variants(ButtonVariant::Outline, ButtonSize::Default, None),
                                "{label}"
                            }
                            PopoverContent { side, dir: "rtl",
                                PopoverHeader {
                                    PopoverTitle { "الأبعاد" }
                                    PopoverDescription { "تعيين الأبعاد للطبقة." }
                                }
                            }
                        }
                    }
                }
                div { class: "flex flex-wrap justify-center gap-2",
                    for (side , label) in logical {
                        Popover { key: "{label}",
                            PopoverTrigger { class: button_variants(ButtonVariant::Outline, ButtonSize::Default, None),
                                "{label}"
                            }
                            PopoverContent { side, dir: "rtl",
                                PopoverHeader {
                                    PopoverTitle { "الأبعاد" }
                                    PopoverDescription { "تعيين الأبعاد للطبقة." }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
