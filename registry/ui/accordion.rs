//! Accordion: a vertically stacked set of headings that each reveal a section of content.
//!
//! Installed by oniko-cn. This file is yours: edit it freely.

use std::rc::Rc;
use std::sync::atomic::{AtomicUsize, Ordering};

use dioxus::prelude::*;
use oniko_cn::{cn, take_class};

static NEXT_ID: AtomicUsize = AtomicUsize::new(0);

#[derive(Clone, Copy)]
struct AccordionContext {
    open: Signal<Vec<String>>,
    multiple: Signal<bool>,
    disabled: Signal<bool>,
    on_value_change: Option<EventHandler<Vec<String>>>,
}

impl AccordionContext {
    fn toggle(mut self, value: &str) {
        let mut next = self.open.peek().clone();
        if let Some(index) = next.iter().position(|v| v == value) {
            next.remove(index);
        } else {
            if !*self.multiple.peek() {
                next.clear();
            }
            next.push(value.to_string());
        }
        self.open.set(next.clone());
        if let Some(handler) = self.on_value_change {
            handler.call(next);
        }
    }
}

#[derive(Clone, Copy)]
struct ItemContext {
    value: Signal<String>,
    disabled: Signal<bool>,
    inner: Signal<Option<Rc<MountedData>>>,
    panel_height: Signal<f64>,
    id: usize,
}

/// A signal that follows a prop, so context shared with children never goes stale.
fn use_prop<T: Clone + PartialEq + 'static>(value: T) -> Signal<T> {
    let mut signal = use_signal(|| value.clone());
    if *signal.peek() != value {
        signal.set(value);
    }
    signal
}

fn state(open: bool) -> &'static str {
    if open { "open" } else { "closed" }
}

#[component]
pub fn Accordion(
    /// Allow several items to be open at once.
    #[props(default)]
    multiple: bool,
    #[props(default)] disabled: bool,
    /// Values of the items that start open.
    #[props(default)]
    default_value: Vec<String>,
    on_value_change: Option<EventHandler<Vec<String>>>,
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(
        "cn-accordion flex w-full flex-col",
        take_class(&mut attributes)
    );
    let open = use_signal(|| default_value);
    let multiple = use_prop(multiple);
    let disabled = use_prop(disabled);
    use_context_provider(|| AccordionContext {
        open,
        multiple,
        disabled,
        on_value_change,
    });

    rsx! {
        div { "data-slot": "accordion", class: class, ..attributes, {children} }
    }
}

#[component]
pub fn AccordionItem(
    /// Identifies the item in the accordion's value.
    value: String,
    #[props(default)] disabled: bool,
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!("cn-accordion-item", take_class(&mut attributes));
    let root = use_context::<AccordionContext>();
    let id = use_hook(|| NEXT_ID.fetch_add(1, Ordering::Relaxed));
    let value = use_prop(value);
    let disabled = use_prop(disabled || *root.disabled.read());
    use_context_provider(|| ItemContext {
        value,
        disabled,
        inner: Signal::new(None),
        panel_height: Signal::new(0.0),
        id,
    });

    let open = root.open.read().contains(&value.read());
    rsx! {
        div {
            "data-slot": "accordion-item",
            "data-state": state(open),
            "data-disabled": disabled().then_some("true"),
            class: class,
            ..attributes,
            {children}
        }
    }
}

#[component]
pub fn AccordionTrigger(
    #[props(extends = GlobalAttributes, extends = button)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(
        "cn-accordion-trigger group/accordion-trigger relative flex flex-1 items-start justify-between border border-transparent transition-all outline-none aria-disabled:pointer-events-none aria-disabled:opacity-50",
        take_class(&mut attributes)
    );
    let root = use_context::<AccordionContext>();
    let item = use_context::<ItemContext>();
    let open = root.open.read().contains(&item.value.read());
    let disabled = (item.disabled)();
    let id = item.id;

    rsx! {
        h3 { "data-slot": "accordion-header", class: "flex",
            button {
                "data-slot": "accordion-trigger",
                "data-state": state(open),
                r#type: "button",
                id: "accordion-trigger-{id}",
                aria_controls: "accordion-panel-{id}",
                aria_expanded: open,
                aria_disabled: disabled,
                class: class,
                onclick: move |_| {
                    if disabled {
                        return;
                    }
                    // Measure before toggling so the height animation has a target.
                    let value = item.value.peek().clone();
                    let mut panel_height = item.panel_height;
                    spawn(async move {
                        if let Some(inner) = item.inner.peek().clone()
                            && let Ok(size) = inner.get_scroll_size().await
                        {
                            panel_height.set(size.height);
                        }
                        root.toggle(&value);
                    });
                },
                ..attributes,
                {children}
                ChevronDown {}
                ChevronUp {}
            }
        }
    }
}

#[component]
pub fn AccordionContent(
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let inner_class = cn!(
        "cn-accordion-content-inner [&_a]:underline [&_a]:underline-offset-3 [&_a]:hover:text-foreground [&_p:not(:last-child)]:mb-4",
        take_class(&mut attributes)
    );
    let root = use_context::<AccordionContext>();
    let mut item = use_context::<ItemContext>();
    let open = root.open.read().contains(&item.value.read());
    let id = item.id;
    let height = (item.panel_height)();

    // `inert` keeps collapsed content out of the tab order; `h-0` holds the
    // closed size once the exit animation has finished.
    rsx! {
        div {
            "data-slot": "accordion-content",
            "data-state": state(open),
            role: "region",
            id: "accordion-panel-{id}",
            aria_labelledby: "accordion-trigger-{id}",
            inert: (!open).then_some("true"),
            style: "--accordion-panel-height: {height}px",
            class: "cn-accordion-content overflow-hidden data-closed:h-0",
            ..attributes,
            div {
                class: inner_class,
                onmounted: move |event| item.inner.set(Some(event.data())),
                {children}
            }
        }
    }
}

#[component]
fn ChevronDown() -> Element {
    rsx! {
        svg {
            "data-slot": "accordion-trigger-icon",
            class: "pointer-events-none shrink-0 group-aria-expanded/accordion-trigger:hidden",
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

#[component]
fn ChevronUp() -> Element {
    rsx! {
        svg {
            "data-slot": "accordion-trigger-icon",
            class: "pointer-events-none hidden shrink-0 group-aria-expanded/accordion-trigger:inline",
            view_box: "0 0 24 24",
            fill: "none",
            stroke: "currentColor",
            stroke_width: "2",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            path { d: "m18 15-6-6-6 6" }
        }
    }
}
