//! Select: a button that opens a list of options and picks one of them.
//!
//! The list stays mounted and hidden while closed, so every item registers its label before the list
//! first opens, and `SelectValue` can show the chosen label straight away. `ArrowDown` and `ArrowUp` move
//! between options, `Home` and `End` jump to the first and last. `Enter` and `Space` pick through the
//! native button click. `Escape` and `Tab` close the list and return focus to the trigger, and a
//! transparent full-screen layer closes it on an outside click. The list is positioned under the trigger
//! with CSS and is not portalled. No `dioxus-primitives` crate is used: it is not published on crates.io.
//!
//! Installed by oniko-cn. This file is yours: edit it freely.

use std::sync::atomic::{AtomicUsize, Ordering};

use dioxus::prelude::*;
use oniko_cn::{cn, take_class};

static NEXT_ID: AtomicUsize = AtomicUsize::new(0);

const SELECT_TRIGGER_BASE: &str = "border-input data-placeholder:text-muted-foreground dark:bg-input/30 dark:hover:bg-input/50 focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 gap-1.5 rounded-md border bg-transparent py-2 pr-2 pl-2.5 text-sm shadow-xs transition-[color,box-shadow] focus-visible:ring-3 aria-invalid:ring-3 data-[size=default]:h-9 data-[size=sm]:h-8 *:data-[slot=select-value]:flex *:data-[slot=select-value]:gap-1.5 [&_svg:not([class*='size-'])]:size-4 flex w-fit items-center justify-between whitespace-nowrap outline-none select-none disabled:pointer-events-none disabled:opacity-50";

#[derive(Clone, Debug, PartialEq)]
struct Entry {
    id: usize,
    value: String,
    label: String,
}

#[derive(Clone, Copy)]
struct SelectContext {
    id: usize,
    /// Value from the `value` prop. `None` means the component owns the selection.
    controlled: Signal<Option<String>>,
    internal: Signal<Option<String>>,
    open: Signal<bool>,
    entries: Signal<Vec<Entry>>,
    disabled: Signal<bool>,
    on_value_change: Option<EventHandler<String>>,
}

impl SelectContext {
    fn value(self) -> Option<String> {
        match (self.controlled)() {
            Some(value) => Some(value),
            None => (self.internal)(),
        }
    }

    fn label_of(self, value: &str) -> Option<String> {
        self.entries
            .read()
            .iter()
            .find(|entry| entry.value == value)
            .map(|entry| entry.label.clone())
    }

    fn trigger_id(self) -> String {
        format!("select-trigger-{}", self.id)
    }

    fn content_id(self) -> String {
        format!("select-content-{}", self.id)
    }

    fn open_and_focus(mut self) {
        self.open.set(true);
        focus_list(self.content_id());
    }

    fn close(mut self) {
        self.open.set(false);
    }

    fn close_and_focus_trigger(mut self) {
        self.open.set(false);
        focus_element(self.trigger_id());
    }

    fn select(mut self, value: String) {
        if self.controlled.peek().is_none() {
            self.internal.set(Some(value.clone()));
        }
        self.open.set(false);
        focus_element(self.trigger_id());
        if let Some(handler) = self.on_value_change {
            handler.call(value);
        }
    }

    fn register(mut self, entry: Entry) {
        let mut entries = self.entries.write();
        entries.retain(|existing| existing.id != entry.id);
        entries.push(entry);
    }

    fn unregister(mut self, id: usize) {
        self.entries.write().retain(|entry| entry.id != id);
    }
}

/// Keeps a signal equal to a prop, so context readers never see a stale value.
fn use_prop<T: Clone + PartialEq + 'static>(value: T) -> Signal<T> {
    let mut signal = use_signal(|| value.clone());
    if *signal.peek() != value {
        signal.set(value);
    }
    signal
}

fn focus_element(id: String) {
    spawn(async move {
        let _ = document::eval(&format!("document.getElementById('{id}')?.focus()")).await;
    });
}

/// Focuses the selected option, or the first one, once the list has been shown.
fn focus_list(content_id: String) {
    spawn(async move {
        let script = format!(
            r#"requestAnimationFrame(() => {{
                const list = document.getElementById('{content_id}');
                (list?.querySelector('[data-selected]:not([disabled])') ?? list?.querySelector('[role="option"]:not([disabled])'))?.focus();
            }});"#
        );
        let _ = document::eval(&script).await;
    });
}

fn focus_option(content_id: String, target: &'static str) {
    spawn(async move {
        let script = format!(
            r#"
            const list = document.getElementById('{content_id}');
            const options = list ? [...list.querySelectorAll('[role="option"]:not([disabled])')] : [];
            if (options.length) {{
                const current = options.indexOf(document.activeElement);
                const last = options.length - 1;
                const next = {{
                    first: 0,
                    last,
                    down: current + 1 > last ? 0 : current + 1,
                    up: current <= 0 ? last : current - 1,
                }}["{target}"];
                options[next].focus();
            }}
            "#
        );
        let _ = document::eval(&script).await;
    });
}

/// Owns the selection and the open state. Pass `value` to control the selection.
#[component]
pub fn Select(
    /// Controlled selection. When set, the parent must update it from `on_value_change`.
    value: Option<String>,
    /// Selection at first render when `value` is not controlled.
    default_value: Option<String>,
    on_value_change: Option<EventHandler<String>>,
    #[props(default)] disabled: bool,
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(
        "relative inline-block",
        take_class(&mut attributes).as_deref()
    );
    let controlled = use_prop(value);
    let disabled_signal = use_prop(disabled);
    let internal = use_signal(|| default_value);
    let open = use_signal(|| false);
    let entries = use_signal(Vec::new);
    let id = use_hook(|| NEXT_ID.fetch_add(1, Ordering::Relaxed));
    let ctx = use_context_provider(|| SelectContext {
        id,
        controlled,
        internal,
        open,
        entries,
        disabled: disabled_signal,
        on_value_change,
    });

    rsx! {
        div {
            "data-slot": "select",
            "data-open": (ctx.open)().then_some("true"),
            "data-disabled": disabled_signal().then_some("true"),
            class: class,
            ..attributes,
            {children}
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum SelectTriggerSize {
    #[default]
    Default,
    Sm,
}

impl SelectTriggerSize {
    fn data(self) -> &'static str {
        match self {
            Self::Default => "default",
            Self::Sm => "sm",
        }
    }
}

/// The button that opens the list. Put a `SelectValue` inside it.
#[component]
pub fn SelectTrigger(
    #[props(default)] size: SelectTriggerSize,
    #[props(default)] aria_invalid: bool,
    #[props(extends = GlobalAttributes, extends = button)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let ctx = use_context::<SelectContext>();
    let mut attributes = attributes;
    let class = cn!(SELECT_TRIGGER_BASE, take_class(&mut attributes).as_deref());
    let open = (ctx.open)();
    let disabled = (ctx.disabled)();
    let placeholder = ctx.value().is_none().then_some("");
    let id = ctx.trigger_id();
    let controls = ctx.content_id();

    rsx! {
        button {
            id: id,
            "data-slot": "select-trigger",
            "data-size": size.data(),
            "data-state": if open { "open" } else { "closed" },
            "data-placeholder": placeholder,
            r#type: "button",
            role: "combobox",
            aria_haspopup: "listbox",
            aria_expanded: open,
            aria_controls: controls,
            aria_invalid: aria_invalid,
            disabled: disabled,
            class: class,
            onclick: move |_| {
                if open {
                    ctx.close();
                } else {
                    ctx.open_and_focus();
                }
            },
            onkeydown: move |event: KeyboardEvent| {
                match event.key() {
                    Key::ArrowDown | Key::ArrowUp => {
                        event.prevent_default();
                        if open {
                            focus_list(ctx.content_id());
                        } else {
                            ctx.open_and_focus();
                        }
                    }
                    Key::Escape if open => {
                        event.prevent_default();
                        event.stop_propagation();
                        ctx.close();
                    }
                    Key::Tab if open => ctx.close(),
                    _ => {}
                }
            },
            ..attributes,
            {children}
            svg {
                "data-slot": "select-icon",
                class: "text-muted-foreground size-4 pointer-events-none",
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
}

/// Shows the label of the selected item, or `placeholder` while nothing is selected.
#[component]
pub fn SelectValue(
    placeholder: Option<String>,
    #[props(extends = GlobalAttributes, extends = span)] attributes: Vec<Attribute>,
) -> Element {
    let ctx = use_context::<SelectContext>();
    let mut attributes = attributes;
    let class = cn!(
        "flex flex-1 text-left",
        take_class(&mut attributes).as_deref()
    );
    let placeholder_shown = ctx.value().is_none().then_some("");
    let text = match ctx.value() {
        Some(value) => ctx.label_of(&value).unwrap_or(value),
        None => placeholder.unwrap_or_default(),
    };

    rsx! {
        span {
            "data-slot": "select-value",
            "data-placeholder": placeholder_shown,
            class: class,
            ..attributes,
            "{text}"
        }
    }
}

/// The popup under the trigger. It is hidden while the list is closed, but stays mounted so its items
/// can register their labels.
#[component]
pub fn SelectContent(
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let ctx = use_context::<SelectContext>();
    let mut attributes = attributes;
    let open = (ctx.open)();
    let class = cn!(
        "bg-popover text-popover-foreground data-open:animate-in data-closed:animate-out data-closed:fade-out-0 data-open:fade-in-0 data-closed:zoom-out-95 data-open:zoom-in-95 data-[side=bottom]:slide-in-from-top-2 data-[side=left]:slide-in-from-right-2 data-[side=right]:slide-in-from-left-2 data-[side=top]:slide-in-from-bottom-2 ring-foreground/10 min-w-36 rounded-md shadow-md ring-1 duration-100 absolute top-full left-0 z-50 mt-1 max-h-72 w-max min-w-full overflow-y-auto outline-none",
        take_class(&mut attributes).as_deref()
    );
    let content_id = ctx.content_id();

    rsx! {
        if open {
            div {
                "data-slot": "select-overlay",
                class: "fixed inset-0 z-40",
                onclick: move |_| ctx.close(),
            }
        }
        div {
            id: content_id.clone(),
            "data-slot": "select-content",
            "data-open": open.then_some(""),
            "data-side": "bottom",
            role: "listbox",
            tabindex: "-1",
            hidden: !open,
            class: class,
            onkeydown: move |event: KeyboardEvent| {
                let target = match event.key() {
                    Key::ArrowDown => "down",
                    Key::ArrowUp => "up",
                    Key::Home => "first",
                    Key::End => "last",
                    Key::Escape | Key::Tab => {
                        event.prevent_default();
                        ctx.close_and_focus_trigger();
                        return;
                    }
                    _ => return,
                };
                event.prevent_default();
                focus_option(content_id.clone(), target);
            },
            ..attributes,
            {children}
        }
    }
}

#[component]
pub fn SelectGroup(
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!("scroll-my-1 p-1", take_class(&mut attributes).as_deref());

    rsx! {
        div {
            "data-slot": "select-group",
            role: "group",
            class: class,
            ..attributes,
            {children}
        }
    }
}

#[component]
pub fn SelectLabel(
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(
        "text-muted-foreground px-2 py-1.5 text-xs",
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        div {
            "data-slot": "select-label",
            class: class,
            ..attributes,
            {children}
        }
    }
}

/// One option. `label` is what `SelectValue` shows after selection; it defaults to `value`. Pass it
/// whenever `children` is not plain text.
#[component]
pub fn SelectItem(
    value: String,
    label: Option<String>,
    #[props(default)] disabled: bool,
    #[props(extends = GlobalAttributes, extends = button)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let ctx = use_context::<SelectContext>();
    let mut attributes = attributes;
    let class = cn!(
        "focus:bg-accent focus:text-accent-foreground not-data-[variant=destructive]:focus:**:text-accent-foreground gap-2 rounded-sm py-1.5 pr-8 pl-2 text-sm [&_svg:not([class*='size-'])]:size-4 *:[span]:last:flex *:[span]:last:items-center *:[span]:last:gap-2 relative flex w-full cursor-default items-center outline-hidden select-none disabled:pointer-events-none disabled:opacity-50",
        take_class(&mut attributes).as_deref()
    );
    let id = use_hook(|| NEXT_ID.fetch_add(1, Ordering::Relaxed));
    let label_text = label.unwrap_or_else(|| value.clone());
    let value = use_prop(value);
    let label = use_prop(label_text);

    use_effect(move || {
        ctx.register(Entry {
            id,
            value: value(),
            label: label(),
        });
    });
    use_drop(move || ctx.unregister(id));

    let item_value = value();
    let selected = ctx.value().as_deref() == Some(item_value.as_str());

    rsx! {
        button {
            "data-slot": "select-item",
            "data-selected": selected.then_some("true"),
            r#type: "button",
            role: "option",
            aria_selected: selected,
            tabindex: "-1",
            disabled: disabled,
            class: class,
            onclick: move |_| ctx.select(item_value.clone()),
            ..attributes,
            span { "data-slot": "select-item-text", class: "flex flex-1 gap-2", {children} }
            if selected {
                span { "data-slot": "select-item-indicator", class: "pointer-events-none absolute right-2 flex size-4 items-center justify-center",
                    svg {
                        view_box: "0 0 24 24",
                        fill: "none",
                        stroke: "currentColor",
                        stroke_width: "2",
                        stroke_linecap: "round",
                        stroke_linejoin: "round",
                        class: "pointer-events-none",
                        path { d: "M20 6 9 17l-5-5" }
                    }
                }
            }
        }
    }
}

#[component]
pub fn SelectSeparator(
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(
        "bg-border -mx-1 my-1 h-px",
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        div {
            "data-slot": "select-separator",
            role: "separator",
            class: class,
            ..attributes,
        }
    }
}
