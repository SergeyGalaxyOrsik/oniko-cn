//! Select examples, ported from shadcn/ui `examples/base/select-*.tsx`.

use dioxus::prelude::*;

use super::Example;
use crate::components::ui::field::{
    Field, FieldContent, FieldDescription, FieldError, FieldGroup, FieldLabel, FieldOrientation,
};
use crate::components::ui::select::{
    Select, SelectContent, SelectGroup, SelectItem, SelectLabel, SelectOption, SelectSeparator,
    SelectTrigger, SelectValue,
};
use crate::components::ui::switch::Switch;

#[component]
pub fn Examples() -> Element {
    rsx! {
        div { class: "flex flex-col gap-10",
            SelectDemo {}
            SelectAlignItem {}
            SelectGroups {}
            SelectScrollable {}
            SelectDisabled {}
            SelectInvalid {}
            SelectRtl {}
        }
    }
}

fn fruits(placeholder: &str) -> Vec<SelectOption> {
    vec![
        SelectOption::none(placeholder),
        SelectOption::new("Apple", "apple"),
        SelectOption::new("Banana", "banana"),
        SelectOption::new("Blueberry", "blueberry"),
        SelectOption::new("Grapes", "grapes"),
        SelectOption::new("Pineapple", "pineapple"),
    ]
}

#[component]
fn SelectDemo() -> Element {
    let items = fruits("Select a fruit");

    rsx! {
        Example { title: "Demo",
            Select { items: items.clone(),
                SelectTrigger { class: "w-full max-w-48", SelectValue {} }
                SelectContent {
                    SelectGroup {
                        SelectLabel { "Fruits" }
                        for item in items {
                            SelectItem { key: "{item.label}", value: item.value.clone(), "{item.label}" }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn SelectAlignItem() -> Element {
    let mut align_item_with_trigger = use_signal(|| true);
    let items = fruits("Select a fruit");

    rsx! {
        Example { title: "Align Item with Trigger",
            FieldGroup { class: "w-full max-w-xs",
                Field { orientation: FieldOrientation::Horizontal,
                    FieldContent {
                        FieldLabel { r#for: "align-item", "Align Item" }
                        FieldDescription { "Toggle to align the item with the trigger." }
                    }
                    Switch {
                        id: "align-item",
                        checked: align_item_with_trigger(),
                        on_checked_change: move |checked| align_item_with_trigger.set(checked),
                    }
                }
                Field {
                    Select { items: items.clone(), default_value: "banana",
                        SelectTrigger { SelectValue {} }
                        SelectContent { align_item_with_trigger: align_item_with_trigger(),
                            SelectGroup {
                                for item in items {
                                    SelectItem { key: "{item.label}", value: item.value.clone(), "{item.label}" }
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
fn SelectGroups() -> Element {
    let fruits = [
        ("Apple", "apple"),
        ("Banana", "banana"),
        ("Blueberry", "blueberry"),
    ];
    let vegetables = [
        ("Carrot", "carrot"),
        ("Broccoli", "broccoli"),
        ("Spinach", "spinach"),
    ];
    let mut items = vec![SelectOption::none("Select a fruit")];
    items.extend(
        fruits
            .iter()
            .chain(vegetables.iter())
            .map(|(label, value)| SelectOption::new(*label, *value)),
    );

    rsx! {
        Example { title: "Groups",
            Select { items,
                SelectTrigger { class: "w-full max-w-48", SelectValue {} }
                SelectContent {
                    SelectGroup {
                        SelectLabel { "Fruits" }
                        for (label , value) in fruits {
                            SelectItem { key: "{value}", value, "{label}" }
                        }
                    }
                    SelectSeparator {}
                    SelectGroup {
                        SelectLabel { "Vegetables" }
                        for (label , value) in vegetables {
                            SelectItem { key: "{value}", value, "{label}" }
                        }
                    }
                }
            }
        }
    }
}

const NORTH_AMERICA: [(&str, &str); 6] = [
    ("Eastern Standard Time", "est"),
    ("Central Standard Time", "cst"),
    ("Mountain Standard Time", "mst"),
    ("Pacific Standard Time", "pst"),
    ("Alaska Standard Time", "akst"),
    ("Hawaii Standard Time", "hst"),
];

const EUROPE_AFRICA: [(&str, &str); 6] = [
    ("Greenwich Mean Time", "gmt"),
    ("Central European Time", "cet"),
    ("Eastern European Time", "eet"),
    ("Western European Summer Time", "west"),
    ("Central Africa Time", "cat"),
    ("East Africa Time", "eat"),
];

const ASIA: [(&str, &str); 6] = [
    ("Moscow Time", "msk"),
    ("India Standard Time", "ist"),
    ("China Standard Time", "cst_china"),
    ("Japan Standard Time", "jst"),
    ("Korea Standard Time", "kst"),
    ("Indonesia Central Standard Time", "ist_indonesia"),
];

const AUSTRALIA_PACIFIC: [(&str, &str); 5] = [
    ("Australian Western Standard Time", "awst"),
    ("Australian Central Standard Time", "acst"),
    ("Australian Eastern Standard Time", "aest"),
    ("New Zealand Standard Time", "nzst"),
    ("Fiji Time", "fjt"),
];

const SOUTH_AMERICA: [(&str, &str); 4] = [
    ("Argentina Time", "art"),
    ("Bolivia Time", "bot"),
    ("Brasilia Time", "brt"),
    ("Chile Standard Time", "clt"),
];

#[component]
fn SelectScrollable() -> Element {
    let groups: [(&str, &[(&str, &str)]); 5] = [
        ("North America", &NORTH_AMERICA),
        ("Europe & Africa", &EUROPE_AFRICA),
        ("Asia", &ASIA),
        ("Australia & Pacific", &AUSTRALIA_PACIFIC),
        ("South America", &SOUTH_AMERICA),
    ];
    let mut items = vec![SelectOption::none("Select a timezone")];
    for (_, group) in groups {
        items.extend(
            group
                .iter()
                .map(|(label, value)| SelectOption::new(*label, *value)),
        );
    }

    rsx! {
        Example { title: "Scrollable",
            Select { items,
                SelectTrigger { class: "w-full max-w-64", SelectValue {} }
                SelectContent {
                    for (title , group) in groups {
                        SelectGroup { key: "{title}",
                            SelectLabel { "{title}" }
                            for (label , value) in group.iter().copied() {
                                SelectItem { key: "{value}", value, "{label}" }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn SelectDisabled() -> Element {
    let items = fruits("Select a fruit");

    rsx! {
        Example { title: "Disabled",
            Select { items: items.clone(), disabled: true,
                SelectTrigger { class: "w-full max-w-48", SelectValue {} }
                SelectContent {
                    SelectGroup {
                        for item in items {
                            SelectItem {
                                key: "{item.label}",
                                value: item.value.clone(),
                                disabled: item.value.as_deref() == Some("grapes"),
                                "{item.label}"
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn SelectInvalid() -> Element {
    let items = vec![
        SelectOption::none("Select a fruit"),
        SelectOption::new("Apple", "apple"),
        SelectOption::new("Banana", "banana"),
        SelectOption::new("Blueberry", "blueberry"),
    ];

    rsx! {
        Example { title: "Invalid",
            Field { "data-invalid": "true", class: "w-full max-w-48",
                FieldLabel { "Fruit" }
                Select { items: items.clone(),
                    SelectTrigger { "aria-invalid": "true", SelectValue {} }
                    SelectContent {
                        SelectGroup {
                            for item in items {
                                SelectItem { key: "{item.label}", value: item.value.clone(), "{item.label}" }
                            }
                        }
                    }
                }
                FieldError { "Please select a fruit." }
            }
        }
    }
}

#[component]
fn SelectRtl() -> Element {
    let mut selected = use_signal(String::new);
    let fruits = [
        ("تفاح", "apple"),
        ("موز", "banana"),
        ("توت أزرق", "blueberry"),
        ("عنب", "grapes"),
        ("أناناس", "pineapple"),
    ];
    let vegetables = [
        ("جزر", "carrot"),
        ("بروكلي", "broccoli"),
        ("سبانخ", "spinach"),
    ];
    let mut items = vec![SelectOption::none("اختر فاكهة")];
    items.extend(
        fruits
            .iter()
            .chain(vegetables.iter())
            .map(|(label, value)| SelectOption::new(*label, *value)),
    );

    rsx! {
        Example { title: "RTL",
            Select {
                items,
                value: selected(),
                on_value_change: move |value: Option<String>| selected.set(value.unwrap_or_default()),
                SelectTrigger { class: "w-32", dir: "rtl", SelectValue {} }
                SelectContent { dir: "rtl", "data-lang": "ar",
                    SelectGroup {
                        SelectLabel { "الفواكه" }
                        for (label , value) in fruits {
                            SelectItem { key: "{value}", value, "{label}" }
                        }
                    }
                    SelectSeparator {}
                    SelectGroup {
                        SelectLabel { "الخضروات" }
                        for (label , value) in vegetables {
                            SelectItem { key: "{value}", value, "{label}" }
                        }
                    }
                }
            }
        }
    }
}
