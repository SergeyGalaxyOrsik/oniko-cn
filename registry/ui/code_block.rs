//! Code block: displays code, terminal output or a diff in a surface with an optional header, footer and copy button.
//!
//! Installed by oniko-cn. This file is yours: edit it freely.

use dioxus::prelude::*;
use oniko_cn::{cn, take_class};

use super::button::{ButtonSize, ButtonVariant, button_variants};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum CodeBlockVariant {
    #[default]
    Default,
    Destructive,
}

impl CodeBlockVariant {
    pub fn class(self) -> &'static str {
        match self {
            Self::Default => "cn-code-block-variant-default",
            Self::Destructive => "cn-code-block-variant-destructive",
        }
    }

    /// Value of the `data-variant` attribute that the styles key off.
    fn as_str(self) -> &'static str {
        match self {
            Self::Default => "default",
            Self::Destructive => "destructive",
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum CodeBlockSize {
    #[default]
    Default,
    Sm,
}

impl CodeBlockSize {
    /// Value of the `data-size` attribute that the styles key off.
    fn as_str(self) -> &'static str {
        match self {
            Self::Default => "default",
            Self::Sm => "sm",
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum CodeBlockLineVariant {
    #[default]
    Default,
    Added,
    Removed,
    Highlighted,
}

impl CodeBlockLineVariant {
    pub fn class(self) -> &'static str {
        match self {
            Self::Default => "",
            Self::Added => "cn-code-block-line-variant-added",
            Self::Removed => "cn-code-block-line-variant-removed",
            Self::Highlighted => "cn-code-block-line-variant-highlighted",
        }
    }
}

const COPY_SCRIPT: &str = "const text = await dioxus.recv(); try { await navigator.clipboard.writeText(text); dioxus.send(true); } catch (error) { dioxus.send(false); }";

#[component]
pub fn CodeBlock(
    #[props(default)] variant: CodeBlockVariant,
    #[props(default)] size: CodeBlockSize,
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(
        "cn-code-block group/code-block relative flex flex-col",
        variant.class(),
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        div {
            "data-slot": "code-block",
            "data-variant": variant.as_str(),
            "data-size": size.as_str(),
            class: class,
            ..attributes,
            {children}
        }
    }
}

#[component]
pub fn CodeBlockHeader(
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(
        "cn-code-block-header flex items-center",
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        div {
            "data-slot": "code-block-header",
            class: class,
            ..attributes,
            {children}
        }
    }
}

#[component]
pub fn CodeBlockTitle(
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(
        "cn-code-block-title flex min-w-0 items-center truncate",
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        div {
            "data-slot": "code-block-title",
            class: class,
            ..attributes,
            {children}
        }
    }
}

/// Holds buttons. Inside a header it sits at the end; as a direct child of `CodeBlock` it floats over the content.
#[component]
pub fn CodeBlockActions(
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(
        "cn-code-block-actions flex items-center [[data-slot=code-block]>&]:absolute [[data-slot=code-block]>&]:opacity-0 [[data-slot=code-block]:hover>&]:opacity-100 [[data-slot=code-block]:focus-within>&]:opacity-100 [@media(hover:none)]:opacity-100",
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        div {
            "data-slot": "code-block-actions",
            class: class,
            ..attributes,
            {children}
        }
    }
}

/// Copies `value` to the clipboard. Announces "Copied" for two seconds after a successful copy.
#[component]
pub fn CodeBlockCopyButton(
    value: String,
    onclick: Option<EventHandler<MouseEvent>>,
    #[props(extends = GlobalAttributes, extends = button)] attributes: Vec<Attribute>,
) -> Element {
    let mut attributes = attributes;
    let own_class = cn!(
        "cn-code-block-copy-button",
        take_class(&mut attributes).as_deref()
    );
    let class = button_variants(
        ButtonVariant::Ghost,
        ButtonSize::IconSm,
        Some(own_class.as_str()),
    );
    let mut copied = use_signal(|| false);

    rsx! {
        button {
            "data-slot": "button",
            r#type: "button",
            aria_label: "Copy code",
            class: class,
            onclick: move |event| {
                if let Some(handler) = onclick {
                    handler.call(event);
                }
                let text = value.clone();
                spawn(async move {
                    let mut eval = document::eval(COPY_SCRIPT);
                    let _ = eval.send(text);
                    let ok = eval.recv::<bool>().await.unwrap_or(false);
                    copied.set(ok);
                    if ok {
                        let _ = document::eval(
                            "await new Promise((resolve) => setTimeout(resolve, 2000))",
                        )
                        .await;
                        copied.set(false);
                    }
                });
            },
            ..attributes,
            if copied() {
                svg {
                    width: "16",
                    height: "16",
                    view_box: "0 0 24 24",
                    fill: "none",
                    stroke: "currentColor",
                    stroke_width: "2",
                    stroke_linecap: "round",
                    stroke_linejoin: "round",
                    "aria-hidden": "true",
                    path { d: "M5 12.5l4.5 4.5L19 7.5" }
                }
            } else {
                svg {
                    width: "16",
                    height: "16",
                    view_box: "0 0 24 24",
                    fill: "none",
                    stroke: "currentColor",
                    stroke_width: "2",
                    stroke_linecap: "round",
                    stroke_linejoin: "round",
                    "aria-hidden": "true",
                    path { d: "M9 9h11v11H9z" }
                    path { d: "M5 15V5h10" }
                }
            }
        }
        span {
            class: "sr-only",
            role: "status",
            aria_live: "polite",
            if copied() {
                "Copied"
            }
        }
    }
}

/// The code itself. Pass a string, or a list of `CodeBlockLine` for per-line styling.
#[component]
pub fn CodeBlockContent(
    #[props(default)] wrap: bool,
    #[props(default)] line_numbers: bool,
    #[props(default = 1)] start_line: i64,
    #[props(extends = GlobalAttributes, extends = pre)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let wrap_class = if wrap {
        "whitespace-pre-wrap break-words"
    } else {
        "whitespace-pre"
    };
    let class = cn!(
        "cn-code-block-content block overflow-auto font-mono outline-none",
        wrap_class,
        take_class(&mut attributes).as_deref()
    );
    let counter_reset = line_numbers.then(|| format!("counter-reset: line {}", start_line - 1));

    rsx! {
        pre {
            "data-slot": "code-block-content",
            "data-numbered": line_numbers.then_some("true"),
            dir: "ltr",
            tabindex: "0",
            style: counter_reset,
            class: class,
            ..attributes,
            {children}
        }
    }
}

/// One line of code inside `CodeBlockContent`.
#[component]
pub fn CodeBlockLine(
    #[props(default)] variant: CodeBlockLineVariant,
    #[props(extends = GlobalAttributes, extends = span)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(
        "cn-code-block-line relative block after:absolute in-data-numbered:flex in-data-numbered:[counter-increment:line] in-data-numbered:before:inline-block in-data-numbered:before:content-[counter(line)] in-data-numbered:before:select-none",
        variant.class(),
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        span {
            "data-slot": "code-block-line",
            class: class,
            ..attributes,
            {children}
        }
    }
}

#[component]
pub fn CodeBlockFooter(
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(
        "cn-code-block-footer flex items-center",
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        div {
            "data-slot": "code-block-footer",
            class: class,
            ..attributes,
            {children}
        }
    }
}
