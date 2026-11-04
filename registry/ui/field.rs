//! Field: combines a label, a control, helper text and a validation message into one accessible form field.
//!
//! Compose a `FieldSet` with a `FieldLegend` and a `FieldGroup` of `Field`s. Each `Field` holds a `FieldLabel`,
//! the control itself, a `FieldDescription` and a `FieldError`. Pass `r#for` to `FieldLabel` to point it at the control.
//!
//! Installed by oniko-cn. This file is yours: edit it freely.

use dioxus::prelude::*;
use oniko_cn::{cn, take_class};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum FieldOrientation {
    /// Label above the control.
    #[default]
    Vertical,
    /// Label beside the control, as in a switch or checkbox row.
    Horizontal,
    /// Vertical in a narrow container, horizontal once the enclosing `FieldGroup` is wide enough.
    Responsive,
}

impl FieldOrientation {
    /// Value of the `data-orientation` attribute.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Vertical => "vertical",
            Self::Horizontal => "horizontal",
            Self::Responsive => "responsive",
        }
    }

    pub fn class(self) -> &'static str {
        match self {
            Self::Vertical => "flex-col [&>*]:w-full [&>.sr-only]:w-auto",
            Self::Horizontal => {
                "flex-row items-center [&>[data-slot=field-label]]:flex-auto has-[>[data-slot=field-content]]:items-start has-[>[data-slot=field-content]]:[&>[role=checkbox],[role=radio],[role=switch]]:mt-px"
            }
            Self::Responsive => {
                "flex-col [&>*]:w-full @md/field-group:flex-row @md/field-group:items-center @md/field-group:[&>*]:w-auto @md/field-group:[&>[data-slot=field-label]]:flex-auto @md/field-group:has-[>[data-slot=field-content]]:items-start @md/field-group:has-[>[data-slot=field-content]]:[&>[role=checkbox],[role=radio],[role=switch]]:mt-px"
            }
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum FieldLegendVariant {
    #[default]
    Legend,
    Label,
}

impl FieldLegendVariant {
    /// Value of the `data-variant` attribute that the styles key off.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Legend => "legend",
            Self::Label => "label",
        }
    }
}

/// One field: a row or column that lays out its label, control, description and error.
///
/// Set `invalid` to show the error state, and `disabled` to dim the whole field.
#[component]
pub fn Field(
    #[props(default)] orientation: FieldOrientation,
    #[props(default)] invalid: bool,
    #[props(default)] disabled: bool,
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(
        "cn-field group/field flex w-full",
        orientation.class(),
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        div {
            "data-slot": "field",
            role: "group",
            "data-orientation": orientation.as_str(),
            "data-invalid": invalid.then_some("true"),
            "data-disabled": disabled.then_some("true"),
            class: class,
            ..attributes,
            {children}
        }
    }
}

/// A semantic group of fields with a legend, usually wrapping a `FieldGroup`.
#[component]
pub fn FieldSet(
    #[props(extends = GlobalAttributes, extends = fieldset)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(
        "cn-field-set flex flex-col",
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        fieldset {
            "data-slot": "field-set",
            class: class,
            ..attributes,
            {children}
        }
    }
}

#[component]
pub fn FieldLegend(
    #[props(default)] variant: FieldLegendVariant,
    #[props(extends = GlobalAttributes, extends = legend)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!("cn-field-legend", take_class(&mut attributes).as_deref());

    rsx! {
        legend {
            "data-slot": "field-legend",
            "data-variant": variant.as_str(),
            class: class,
            ..attributes,
            {children}
        }
    }
}

/// Stacks related fields and separators with consistent spacing.
#[component]
pub fn FieldGroup(
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(
        "cn-field-group group/field-group @container/field-group flex w-full flex-col",
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        div {
            "data-slot": "field-group",
            class: class,
            ..attributes,
            {children}
        }
    }
}

/// Groups the label and description of a field when they sit beside the control.
#[component]
pub fn FieldContent(
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(
        "cn-field-content group/field-content flex flex-1 flex-col leading-snug",
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        div {
            "data-slot": "field-content",
            class: class,
            ..attributes,
            {children}
        }
    }
}

/// The label of a field. Pass `r#for` with the id of the control it names.
#[component]
pub fn FieldLabel(
    #[props(extends = GlobalAttributes, extends = label)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(
        "cn-field-label group/field-label peer/field-label flex w-fit",
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        label {
            "data-slot": "field-label",
            class: class,
            ..attributes,
            {children}
        }
    }
}

/// A title for a field or a choice, without the form-control semantics of `FieldLabel`.
#[component]
pub fn FieldTitle(
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(
        "cn-field-title flex w-fit items-center",
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        div {
            "data-slot": "field-title",
            class: class,
            ..attributes,
            {children}
        }
    }
}

#[component]
pub fn FieldDescription(
    #[props(extends = GlobalAttributes, extends = p)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(
        "cn-field-description leading-normal font-normal last:mt-0 nth-last-2:-mt-1 group-has-[[data-orientation=horizontal]]/field:text-balance [&>a]:underline [&>a]:underline-offset-4 [&>a:hover]:text-primary",
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        p {
            "data-slot": "field-description",
            class: class,
            ..attributes,
            {children}
        }
    }
}

/// A divider between sections of a `FieldGroup`. Optional children are shown centred on the line.
#[component]
pub fn FieldSeparator(
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Option<Element>,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(
        "cn-field-separator flex items-center",
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        div {
            "data-slot": "field-separator",
            class: class,
            ..attributes,
            div { "data-slot": "field-separator-line", class: "cn-separator cn-separator-horizontal flex-1" }
            if let Some(children) = children {
                span {
                    "data-slot": "field-separator-content",
                    class: "cn-field-separator-content",
                    {children}
                }
                div { "data-slot": "field-separator-line", class: "cn-separator cn-separator-horizontal flex-1" }
            }
        }
    }
}

/// The validation message of a field. Shown to assistive technology as an alert.
#[component]
pub fn FieldError(
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(
        "cn-field-error font-normal",
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        div {
            "data-slot": "field-error",
            role: "alert",
            class: class,
            ..attributes,
            {children}
        }
    }
}
