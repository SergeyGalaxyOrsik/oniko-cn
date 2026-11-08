//! Field: combines a label, a control, helper text and a validation message into one accessible form field.
//!
//! Compose a `FieldSet` with a `FieldLegend` and a `FieldGroup` of `Field`s. Each `Field` holds a `FieldLabel`,
//! the control itself, a `FieldDescription` and a `FieldError`. Pass `r#for` to `FieldLabel` to point it at the control.
//! To show the invalid or disabled state, set `"data-invalid": "true"` or `"data-disabled": "true"` on the `Field`.
//!
//! Installed by oniko-cn. This file is yours: edit it freely.

use dioxus::prelude::*;
use oniko_cn::{cn, take_class};

use super::label::label_variants;
use super::separator::Separator;

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
            Self::Vertical => "flex-col *:w-full [&>.sr-only]:w-auto",
            Self::Horizontal => {
                "flex-row items-center has-[>[data-slot=field-content]]:items-start *:data-[slot=field-label]:flex-auto has-[>[data-slot=field-content]]:[&>[role=checkbox],[role=radio]]:mt-px"
            }
            Self::Responsive => {
                "flex-col *:w-full @md/field-group:flex-row @md/field-group:items-center @md/field-group:*:w-auto @md/field-group:has-[>[data-slot=field-content]]:items-start @md/field-group:*:data-[slot=field-label]:flex-auto [&>.sr-only]:w-auto @md/field-group:has-[>[data-slot=field-content]]:[&>[role=checkbox],[role=radio]]:mt-px"
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
#[component]
pub fn Field(
    #[props(default)] orientation: FieldOrientation,
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(
        "data-[invalid=true]:text-destructive gap-2 group/field flex w-full",
        orientation.class(),
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        div {
            role: "group",
            "data-slot": "field",
            "data-orientation": orientation.as_str(),
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
        "gap-4 has-[>[data-slot=checkbox-group]]:gap-3 has-[>[data-slot=radio-group]]:gap-3 flex flex-col",
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
    let class = cn!(
        "mb-2 font-medium data-[variant=label]:text-xs/relaxed data-[variant=legend]:text-sm",
        take_class(&mut attributes).as_deref()
    );

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
        "gap-4 data-[slot=checkbox-group]:gap-3 *:data-[slot=field-group]:gap-4 group/field-group @container/field-group flex w-full flex-col",
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
        "gap-0.5 group/field-content flex flex-1 flex-col leading-snug",
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
    let extra = cn!(
        "has-data-checked:bg-primary/5 dark:has-data-checked:bg-primary/10 gap-2 leading-snug group-data-[disabled=true]/field:opacity-50 has-[>[data-slot=field]]:rounded-md has-[>[data-slot=field]]:border has-[>[data-slot=field]]:not-has-[:disabled,[data-disabled]]:hover:bg-input/40 has-[>[data-slot=field]]:has-[:focus-visible]:border-ring has-[>[data-slot=field]]:has-[:focus-visible]:ring-ring/30 has-[>[data-slot=field]]:has-[:focus-visible]:ring-2 *:data-[slot=field]:p-2 group/field-label peer/field-label flex w-fit",
        "has-[>[data-slot=field]]:w-full has-[>[data-slot=field]]:flex-col",
        take_class(&mut attributes).as_deref()
    );
    let class = label_variants(Some(extra.as_str()));

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
/// It carries the `field-label` slot, so parents that target labels style it too.
#[component]
pub fn FieldTitle(
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(
        "gap-2 text-xs/relaxed font-medium group-data-[disabled=true]/field:opacity-50 flex w-fit items-center",
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        div {
            "data-slot": "field-label",
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
        "text-muted-foreground text-left text-xs/relaxed [[data-variant=legend]+&]:-mt-1.5 leading-normal font-normal group-has-data-horizontal/field:text-balance",
        "last:mt-0 nth-last-2:-mt-1",
        "[&>a]:underline [&>a]:underline-offset-4 [&>a:hover]:text-primary",
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
        "-my-2 h-5 text-xs/relaxed group-data-[variant=outline]/field-group:-mb-2 relative",
        take_class(&mut attributes).as_deref()
    );
    let has_content = children.is_some();

    rsx! {
        div {
            "data-slot": "field-separator",
            "data-content": if has_content { "true" } else { "false" },
            class: class,
            ..attributes,
            Separator { class: "absolute inset-0 top-1/2" }
            if let Some(children) = children {
                span {
                    class: "text-muted-foreground px-2 relative mx-auto block w-fit bg-background",
                    "data-slot": "field-separator-content",
                    {children}
                }
            }
        }
    }
}

/// The validation message of a field, shown to assistive technology as an alert.
///
/// Pass `children`, or `errors` with the messages to show. Renders nothing when both are empty.
/// Several different messages become a bulleted list.
#[component]
pub fn FieldError(
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    #[props(default)] errors: Vec<Option<String>>,
    children: Option<Element>,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(
        "text-destructive text-xs/relaxed font-normal",
        take_class(&mut attributes).as_deref()
    );

    let mut unique: Vec<Option<String>> = Vec::new();
    for error in errors {
        if !unique.contains(&error) {
            unique.push(error);
        }
    }

    let content = match children {
        Some(children) => children,
        None if unique.is_empty() => return rsx! {},
        None if unique.len() == 1 => {
            rsx! { {unique.first().cloned().flatten()} }
        }
        None => rsx! {
            ul { class: "ml-4 flex list-disc flex-col gap-1",
                for message in unique.into_iter().flatten() {
                    li { "{message}" }
                }
            }
        },
    };

    rsx! {
        div {
            role: "alert",
            "data-slot": "field-error",
            class: class,
            ..attributes,
            {content}
        }
    }
}
