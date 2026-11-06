//! Model Selector: two dropdown menus for an input group, one picks the model the agent runs and the other
//! picks how hard it thinks. Put them in an `InputGroupAddon`.
//!
//! Installed by oniko-cn. This file is yours: edit it freely.

use dioxus::prelude::*;

use super::button::ButtonVariant;
use super::dropdown_menu::{
    DropdownMenu, DropdownMenuAlign, DropdownMenuContent, DropdownMenuGroup, DropdownMenuLabel,
    DropdownMenuRadioGroup, DropdownMenuRadioItem, DropdownMenuSeparator, DropdownMenuTrigger,
};
use super::input_group::{InputGroupButtonSize, input_group_button_variants};

#[derive(Clone, PartialEq)]
pub struct ModelOption {
    pub id: String,
    pub name: String,
}

#[derive(Clone, PartialEq)]
pub struct ModelProvider {
    pub name: String,
    pub models: Vec<ModelOption>,
}

fn chevron_down() -> Element {
    rsx! {
        svg {
            width: "24",
            height: "24",
            view_box: "0 0 24 24",
            fill: "none",
            stroke: "currentColor",
            stroke_width: "2",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            path { d: "m6 9 6 6 6-6" }
        }
    }
}

/// Radio menu of models, grouped by provider.
#[component]
pub fn ModelSelector(
    providers: Vec<ModelProvider>,
    value: String,
    on_value_change: EventHandler<String>,
    #[props(default)] class: String,
) -> Element {
    let current = providers
        .iter()
        .flat_map(|provider| provider.models.iter())
        .find(|model| model.id == value)
        .map(|model| model.name.clone())
        .unwrap_or_default();

    rsx! {
        DropdownMenu {
            DropdownMenuTrigger {
                class: input_group_button_variants(
                    ButtonVariant::Outline,
                    InputGroupButtonSize::Sm,
                    Some(class.as_str()),
                ),
                "data-size": "sm",
                aria_label: "Model: {current}",
                "{current}"
                {chevron_down()}
            }
            DropdownMenuContent { align: DropdownMenuAlign::Start, class: "w-48",
                DropdownMenuRadioGroup { value: value.clone(), on_value_change,
                    for (index , provider) in providers.iter().enumerate() {
                        Fragment { key: "{provider.name}",
                            if index > 0 {
                                DropdownMenuSeparator {}
                            }
                            DropdownMenuGroup {
                                DropdownMenuLabel { "{provider.name}" }
                                for model in provider.models.iter() {
                                    DropdownMenuRadioItem {
                                        key: "{model.id}",
                                        value: model.id.clone(),
                                        close_on_click: true,
                                        "{model.name}"
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

/// Radio menu of reasoning efforts, such as low, medium and high.
#[component]
pub fn EffortSelector(
    efforts: Vec<ModelOption>,
    value: String,
    on_value_change: EventHandler<String>,
    #[props(default)] class: String,
) -> Element {
    let current = efforts
        .iter()
        .find(|effort| effort.id == value)
        .map(|effort| effort.name.clone())
        .unwrap_or_default();

    rsx! {
        DropdownMenu {
            DropdownMenuTrigger {
                class: input_group_button_variants(
                    ButtonVariant::Outline,
                    InputGroupButtonSize::Sm,
                    Some(class.as_str()),
                ),
                "data-size": "sm",
                aria_label: "Effort: {current}",
                span { class: "text-muted-foreground", "Effort" }
                "{current}"
                {chevron_down()}
            }
            DropdownMenuContent { align: DropdownMenuAlign::Start, class: "w-36",
                DropdownMenuGroup {
                    DropdownMenuLabel { "Effort" }
                    DropdownMenuRadioGroup { value: value.clone(), on_value_change,
                        for effort in efforts.iter() {
                            DropdownMenuRadioItem {
                                key: "{effort.id}",
                                value: effort.id.clone(),
                                close_on_click: true,
                                "{effort.name}"
                            }
                        }
                    }
                }
            }
        }
    }
}
