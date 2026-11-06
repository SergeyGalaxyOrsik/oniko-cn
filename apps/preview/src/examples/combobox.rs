//! Combobox examples, ported from shadcn/ui `examples/base/combobox-*.tsx`.

use dioxus::prelude::*;
use lucide_dioxus::Globe;

use super::Example;
use crate::components::ui::button::ButtonVariant;
use crate::components::ui::combobox::{
    Combobox, ComboboxAlign, ComboboxChip, ComboboxChips, ComboboxChipsInput, ComboboxCollection,
    ComboboxContent, ComboboxEmpty, ComboboxGroup, ComboboxInput, ComboboxItem, ComboboxLabel,
    ComboboxList, ComboboxSeparator, ComboboxTrigger, ComboboxValue, use_combobox_anchor,
};
use crate::components::ui::field::{Field, FieldLabel};
use crate::components::ui::input_group::InputGroupAddon;
use crate::components::ui::item::{Item, ItemContent, ItemDescription, ItemSize, ItemTitle};

#[component]
pub fn Examples() -> Element {
    rsx! {
        div { class: "flex flex-col gap-10",
            ComboboxDemo {}
            ComboboxBasic {}
            ComboboxMultiple {}
            ComboboxClearButton {}
            ComboboxGroups {}
            ComboboxCustomItems {}
            ComboboxInvalid {}
            ComboboxDisabled {}
            ComboboxAutoHighlight {}
            ComboboxPopup {}
            ComboboxInputGroup {}
            ComboboxRtl {}
        }
    }
}

const FRAMEWORKS: [&str; 5] = ["Next.js", "SvelteKit", "Nuxt.js", "Remix", "Astro"];

/// The framework list that most examples share.
#[component]
fn FrameworkItems() -> Element {
    rsx! {
        ComboboxEmpty { "No items found." }
        ComboboxList {
            for item in FRAMEWORKS {
                ComboboxItem { key: "{item}", value: item, "{item}" }
            }
        }
    }
}

#[component]
fn ComboboxDemo() -> Element {
    rsx! {
        Example { title: "Demo",
            Combobox {
                ComboboxInput { placeholder: "Select a framework" }
                ComboboxContent { FrameworkItems {} }
            }
        }
    }
}

#[component]
fn ComboboxBasic() -> Element {
    rsx! {
        Example { title: "Basic",
            Combobox {
                ComboboxInput { placeholder: "Select a framework" }
                ComboboxContent { FrameworkItems {} }
            }
        }
    }
}

#[component]
fn ComboboxMultiple() -> Element {
    let anchor = use_combobox_anchor();

    rsx! {
        Example { title: "Multiple",
            Combobox {
                multiple: true,
                auto_highlight: true,
                default_values: vec![FRAMEWORKS[0].to_string()],
                ComboboxChips { anchor: anchor.clone(), class: "w-full max-w-xs",
                    ComboboxValue {
                        render: move |values: Vec<String>| rsx! {
                            for value in values {
                                ComboboxChip { key: "{value}", value: value.clone(), "{value}" }
                            }
                            ComboboxChipsInput {}
                        },
                    }
                }
                ComboboxContent { anchor: anchor.clone(), FrameworkItems {} }
            }
        }
    }
}

#[component]
fn ComboboxClearButton() -> Element {
    rsx! {
        Example { title: "Clear Button",
            Combobox { default_value: FRAMEWORKS[0],
                ComboboxInput { placeholder: "Select a framework", show_clear: true }
                ComboboxContent { FrameworkItems {} }
            }
        }
    }
}

const TIMEZONES: [(&str, [&str; 6]); 3] = [
    (
        "Americas",
        [
            "(GMT-5) New York",
            "(GMT-8) Los Angeles",
            "(GMT-6) Chicago",
            "(GMT-5) Toronto",
            "(GMT-8) Vancouver",
            "(GMT-3) São Paulo",
        ],
    ),
    (
        "Europe",
        [
            "(GMT+0) London",
            "(GMT+1) Paris",
            "(GMT+1) Berlin",
            "(GMT+1) Rome",
            "(GMT+1) Madrid",
            "(GMT+1) Amsterdam",
        ],
    ),
    (
        "Asia/Pacific",
        [
            "(GMT+9) Tokyo",
            "(GMT+8) Shanghai",
            "(GMT+8) Singapore",
            "(GMT+4) Dubai",
            "(GMT+11) Sydney",
            "(GMT+9) Seoul",
        ],
    ),
];

#[component]
fn ComboboxGroups() -> Element {
    rsx! {
        Example { title: "Groups",
            Combobox {
                ComboboxInput { placeholder: "Select a timezone" }
                ComboboxContent {
                    ComboboxEmpty { "No timezones found." }
                    ComboboxList {
                        for (index , (group , items)) in TIMEZONES.into_iter().enumerate() {
                            ComboboxGroup { key: "{group}",
                                ComboboxLabel { "{group}" }
                                ComboboxCollection {
                                    for item in items {
                                        ComboboxItem { key: "{item}", value: item, "{item}" }
                                    }
                                }
                                if index < TIMEZONES.len() - 1 {
                                    ComboboxSeparator {}
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

/// `(code, value, label, continent)`
const COUNTRIES: [(&str, &str, &str, &str); 20] = [
    ("", "", "Select country", ""),
    ("ar", "argentina", "Argentina", "South America"),
    ("au", "australia", "Australia", "Oceania"),
    ("br", "brazil", "Brazil", "South America"),
    ("ca", "canada", "Canada", "North America"),
    ("cn", "china", "China", "Asia"),
    ("co", "colombia", "Colombia", "South America"),
    ("eg", "egypt", "Egypt", "Africa"),
    ("fr", "france", "France", "Europe"),
    ("de", "germany", "Germany", "Europe"),
    ("it", "italy", "Italy", "Europe"),
    ("jp", "japan", "Japan", "Asia"),
    ("ke", "kenya", "Kenya", "Africa"),
    ("mx", "mexico", "Mexico", "North America"),
    ("nz", "new-zealand", "New Zealand", "Oceania"),
    ("ng", "nigeria", "Nigeria", "Africa"),
    ("za", "south-africa", "South Africa", "Africa"),
    ("kr", "south-korea", "South Korea", "Asia"),
    ("gb", "united-kingdom", "United Kingdom", "Europe"),
    ("us", "united-states", "United States", "North America"),
];

#[component]
fn ComboboxCustomItems() -> Element {
    rsx! {
        Example { title: "Custom Items",
            Combobox {
                ComboboxInput { placeholder: "Search countries..." }
                ComboboxContent {
                    ComboboxEmpty { "No countries found." }
                    ComboboxList {
                        for (code , value , label , continent) in COUNTRIES.into_iter().filter(|country| !country.0.is_empty()) {
                            ComboboxItem { key: "{code}", value, label,
                                Item { size: ItemSize::Xs, class: "p-0",
                                    ItemContent {
                                        ItemTitle { class: "whitespace-nowrap", "{label}" }
                                        ItemDescription { "{continent} ({code})" }
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

#[component]
fn ComboboxInvalid() -> Element {
    rsx! {
        Example { title: "Invalid",
            Combobox {
                ComboboxInput { placeholder: "Select a framework", "aria-invalid": "true" }
                ComboboxContent { FrameworkItems {} }
            }
        }
    }
}

#[component]
fn ComboboxDisabled() -> Element {
    rsx! {
        Example { title: "Disabled",
            Combobox {
                ComboboxInput { placeholder: "Select a framework", disabled: true }
                ComboboxContent { FrameworkItems {} }
            }
        }
    }
}

#[component]
fn ComboboxAutoHighlight() -> Element {
    rsx! {
        Example { title: "Auto Highlight",
            Combobox { auto_highlight: true,
                ComboboxInput { placeholder: "Select a framework" }
                ComboboxContent { FrameworkItems {} }
            }
        }
    }
}

#[component]
fn ComboboxPopup() -> Element {
    rsx! {
        Example { title: "Popup",
            Combobox { default_value: "",
                ComboboxTrigger {
                    variant: ButtonVariant::Outline,
                    class: "w-64 justify-between font-normal",
                    ComboboxValue {}
                }
                ComboboxContent {
                    ComboboxInput { show_trigger: false, placeholder: "Search" }
                    ComboboxEmpty { "No items found." }
                    ComboboxList {
                        for (code , value , label , _) in COUNTRIES {
                            ComboboxItem { key: "{code}", value, label, "{label}" }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn ComboboxInputGroup() -> Element {
    rsx! {
        Example { title: "Input Group",
            Combobox {
                ComboboxInput { placeholder: "Select a timezone",
                    InputGroupAddon {
                        Globe {}
                    }
                }
                ComboboxContent { align_offset: -28.0, class: "w-60",
                    ComboboxEmpty { "No timezones found." }
                    ComboboxList {
                        for (group , items) in TIMEZONES {
                            ComboboxGroup { key: "{group}",
                                ComboboxLabel { "{group}" }
                                ComboboxCollection {
                                    for item in items {
                                        ComboboxItem { key: "{item}", value: item, "{item}" }
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

const CATEGORIES: [(&str, &str); 6] = [
    ("technology", "التكنولوجيا"),
    ("design", "التصميم"),
    ("business", "الأعمال"),
    ("marketing", "التسويق"),
    ("education", "التعليم"),
    ("health", "الصحة"),
];

fn category_label(value: &str) -> String {
    CATEGORIES
        .iter()
        .find(|(key, _)| *key == value)
        .map_or_else(|| value.to_string(), |(_, label)| (*label).to_string())
}

#[component]
fn ComboboxRtl() -> Element {
    let anchor = use_combobox_anchor();

    rsx! {
        Example { title: "RTL",
            Field { class: "mx-auto w-full max-w-xs", dir: "rtl",
                FieldLabel { "الفئات" }
                Combobox {
                    multiple: true,
                    auto_highlight: true,
                    default_values: vec![CATEGORIES[0].0.to_string()],
                    ComboboxChips { anchor: anchor.clone(),
                        ComboboxValue {
                            render: move |values: Vec<String>| rsx! {
                                for value in values {
                                    ComboboxChip { key: "{value}", value: value.clone(), "{category_label(&value)}" }
                                }
                                ComboboxChipsInput { placeholder: "أضف فئات" }
                            },
                        }
                    }
                    ComboboxContent {
                        anchor: anchor.clone(),
                        align: ComboboxAlign::Start,
                        dir: "rtl",
                        "data-lang": "ar",
                        ComboboxEmpty { "لم يتم العثور على فئات." }
                        ComboboxList {
                            for (value , label) in CATEGORIES {
                                ComboboxItem { key: "{value}", value, label, "{label}" }
                            }
                        }
                    }
                }
            }
        }
    }
}
