//! Combobox: a text input that filters a list of options and selects one of them.
//!
//! Items register themselves with the root while the list is open, so keyboard
//! navigation (ArrowUp, ArrowDown, Enter, Escape) and type-to-filter work over the
//! items that are actually rendered. Positioning is a plain absolutely positioned
//! list under the input.
//!
//! Installed by oniko-cn. This file is yours: edit it freely.

use std::sync::atomic::{AtomicUsize, Ordering};

use dioxus::prelude::*;
use oniko_cn::{cn, take_class};

static NEXT_ID: AtomicUsize = AtomicUsize::new(0);

#[derive(Clone, Debug, PartialEq)]
struct Entry {
    id: usize,
    value: String,
    label: String,
}

#[derive(Clone, Copy)]
struct ComboboxContext {
    /// Value from the `value` prop. `None` means the component owns the selection.
    controlled: Signal<Option<String>>,
    internal: Signal<Option<String>>,
    /// Label of the selected value, known once its item has been rendered.
    selected: Signal<Option<(String, String)>>,
    open: Signal<bool>,
    /// Whether the user has typed since the list opened. Only then does the text filter the list.
    typed: Signal<bool>,
    query: Signal<String>,
    highlighted: Signal<Option<usize>>,
    entries: Signal<Vec<Entry>>,
    disabled: Signal<bool>,
    on_value_change: Option<EventHandler<String>>,
    id: usize,
}

impl ComboboxContext {
    fn value(self) -> Option<String> {
        match (self.controlled)() {
            Some(value) => Some(value),
            None => (self.internal)(),
        }
    }

    fn selected_text(self) -> String {
        match self.value() {
            None => String::new(),
            Some(value) => match &*self.selected.read() {
                Some((selected, label)) if *selected == value => label.clone(),
                _ => value,
            },
        }
    }

    fn filter(self) -> String {
        if (self.typed)() {
            (self.query)()
        } else {
            String::new()
        }
    }

    fn visible(self) -> Vec<Entry> {
        let filter = self.filter().to_lowercase();
        let mut entries: Vec<Entry> = self
            .entries
            .read()
            .iter()
            .filter(|entry| entry.label.to_lowercase().contains(&filter))
            .cloned()
            .collect();
        entries.sort_by_key(|entry| entry.id);
        entries
    }

    fn show(mut self) {
        self.open.set(true);
    }

    fn close(mut self) {
        self.open.set(false);
        self.typed.set(false);
        self.highlighted.set(None);
    }

    fn select(mut self, value: String, label: String) {
        if self.controlled.peek().is_none() {
            self.internal.set(Some(value.clone()));
        }
        self.selected.set(Some((value.clone(), label)));
        self.close();
        if let Some(handler) = self.on_value_change {
            handler.call(value);
        }
    }

    /// The item Enter picks: the highlighted one, or the first match while typing.
    fn chosen(self) -> Option<Entry> {
        let visible = self.visible();
        let highlighted = (self.highlighted)();
        visible
            .iter()
            .find(|entry| Some(entry.id) == highlighted)
            .or_else(|| {
                if (self.typed)() {
                    visible.first()
                } else {
                    None
                }
            })
            .cloned()
    }

    fn step(mut self, delta: isize) {
        let visible = self.visible();
        if visible.is_empty() {
            return;
        }
        let len = visible.len() as isize;
        let current = (self.highlighted)().and_then(|id| visible.iter().position(|e| e.id == id));
        let next = match current {
            Some(index) => (index as isize + delta).rem_euclid(len),
            None if delta > 0 => 0,
            None => len - 1,
        } as usize;
        let id = visible[next].id;
        self.highlighted.set(Some(id));
        let script = format!(
            "document.getElementById('combobox-item-{id}')?.scrollIntoView({{ block: 'nearest' }})"
        );
        spawn(async move {
            let _ = document::eval(&script).await;
        });
    }

    fn focus_input(self) {
        let script = format!("document.getElementById('combobox-input-{}')?.focus()", self.id);
        spawn(async move {
            let _ = document::eval(&script).await;
        });
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

/// A signal that follows a prop, so context shared with children never goes stale.
fn use_prop<T: Clone + PartialEq + 'static>(value: T) -> Signal<T> {
    let mut signal = use_signal(|| value.clone());
    if *signal.peek() != value {
        signal.set(value);
    }
    signal
}

/// Owns the selection and the open state. Pass `value` to control the selection.
#[component]
pub fn Combobox(
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
    let class = cn!("relative w-full", take_class(&mut attributes));
    let controlled = use_prop(value);
    let disabled_signal = use_prop(disabled);
    let internal = use_signal(|| default_value);
    let selected = use_signal(|| None);
    let open = use_signal(|| false);
    let typed = use_signal(|| false);
    let query = use_signal(String::new);
    let highlighted = use_signal(|| None);
    let entries = use_signal(Vec::new);
    let id = use_hook(|| NEXT_ID.fetch_add(1, Ordering::Relaxed));
    let ctx = use_context_provider(|| ComboboxContext {
        controlled,
        internal,
        selected,
        open,
        typed,
        query,
        highlighted,
        entries,
        disabled: disabled_signal,
        on_value_change,
        id,
    });

    rsx! {
        div {
            "data-slot": "combobox",
            "data-open": (ctx.open)().then_some("true"),
            "data-disabled": disabled_signal().then_some("true"),
            class: class,
            ..attributes,
            {children}
        }
    }
}

/// The text field. Typing opens the list and filters it by the typed text.
#[component]
pub fn ComboboxInput(
    /// Shows the chevron button at the end of the field.
    #[props(default = true)]
    show_trigger: bool,
    onkeydown: Option<EventHandler<KeyboardEvent>>,
    #[props(extends = GlobalAttributes, extends = input)] attributes: Vec<Attribute>,
) -> Element {
    let mut ctx = use_context::<ComboboxContext>();
    let mut attributes = attributes;
    let class = cn!(
        "cn-input w-full min-w-0 outline-none disabled:pointer-events-none disabled:opacity-50",
        show_trigger.then_some("pr-9"),
        take_class(&mut attributes)
    );
    let open = (ctx.open)();
    let disabled = (ctx.disabled)();
    let display = if (ctx.typed)() {
        (ctx.query)()
    } else {
        ctx.selected_text()
    };
    let active = (ctx.highlighted)().map(|id| format!("combobox-item-{id}"));
    let id = ctx.id;

    rsx! {
        div { "data-slot": "combobox-input-group", class: "relative w-full",
            input {
                "data-slot": "combobox-input",
                id: "combobox-input-{id}",
                r#type: "text",
                role: "combobox",
                autocomplete: "off",
                aria_autocomplete: "list",
                aria_expanded: open,
                aria_controls: "combobox-list-{id}",
                aria_activedescendant: active,
                disabled: disabled,
                value: "{display}",
                class: class,
                onclick: move |_| ctx.show(),
                oninput: move |event| {
                    ctx.query.set(event.value());
                    ctx.typed.set(true);
                    ctx.open.set(true);
                    ctx.highlighted.set(None);
                },
                onblur: move |_| ctx.close(),
                onkeydown: move |event: KeyboardEvent| {
                    if let Some(handler) = onkeydown {
                        handler.call(event.clone());
                    }
                    match event.key() {
                        Key::ArrowDown | Key::ArrowUp => {
                            event.prevent_default();
                            if !(ctx.open)() {
                                ctx.show();
                            } else {
                                ctx.step(if event.key() == Key::ArrowDown { 1 } else { -1 });
                            }
                        }
                        Key::Enter if (ctx.open)() => {
                            event.prevent_default();
                            if let Some(entry) = ctx.chosen() {
                                ctx.select(entry.value, entry.label);
                            }
                        }
                        Key::Escape if (ctx.open)() => {
                            event.prevent_default();
                            event.stop_propagation();
                            ctx.close();
                        }
                        _ => {}
                    }
                },
                ..attributes,
            }
            if show_trigger {
                ComboboxTrigger {}
            }
        }
    }
}

/// The chevron button that opens and closes the list. Put it inside `ComboboxInput`.
#[component]
pub fn ComboboxTrigger(
    #[props(extends = GlobalAttributes, extends = button)] attributes: Vec<Attribute>,
) -> Element {
    let ctx = use_context::<ComboboxContext>();
    let mut attributes = attributes;
    let class = cn!(
        "cn-combobox-trigger absolute inset-y-0 right-0 flex items-center justify-center px-2 outline-none disabled:pointer-events-none disabled:opacity-50",
        take_class(&mut attributes)
    );
    let open = (ctx.open)();
    let disabled = (ctx.disabled)();

    rsx! {
        button {
            "data-slot": "combobox-trigger",
            r#type: "button",
            tabindex: "-1",
            aria_label: "Show options",
            aria_expanded: open,
            disabled: disabled,
            class: class,
            // Keeps focus in the input, so the input's blur does not close the list first.
            onmousedown: move |event| event.prevent_default(),
            onclick: move |_| {
                if open {
                    ctx.close();
                } else {
                    ctx.show();
                }
                ctx.focus_input();
            },
            ..attributes,
            svg {
                "data-slot": "combobox-trigger-icon",
                class: "cn-combobox-trigger-icon pointer-events-none",
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

/// The popup under the input. Renders nothing while the list is closed.
#[component]
pub fn ComboboxContent(
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let ctx = use_context::<ComboboxContext>();
    if !(ctx.open)() {
        return rsx! {};
    }
    let mut attributes = attributes;
    let empty = ctx.visible().is_empty();
    let class = cn!(
        "cn-combobox-content group/combobox-content absolute top-full left-0 z-50 mt-1 w-full",
        take_class(&mut attributes)
    );

    rsx! {
        div {
            "data-slot": "combobox-content",
            "data-open": "",
            "data-side": "bottom",
            "data-empty": empty.then_some("true"),
            class: class,
            ..attributes,
            {children}
        }
    }
}

/// The `listbox` that holds the items.
#[component]
pub fn ComboboxList(
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let ctx = use_context::<ComboboxContext>();
    let mut attributes = attributes;
    let empty = ctx.visible().is_empty();
    let class = cn!("cn-combobox-list", take_class(&mut attributes));

    rsx! {
        div {
            "data-slot": "combobox-list",
            role: "listbox",
            id: "combobox-list-{ctx.id}",
            "data-empty": empty.then_some("true"),
            class: class,
            ..attributes,
            {children}
        }
    }
}

/// One selectable option. `label` is what the user types against and what the field shows
/// after selection; it defaults to `value`. Pass it whenever `children` is not plain text.
#[component]
pub fn ComboboxItem(
    value: String,
    label: Option<String>,
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut ctx = use_context::<ComboboxContext>();
    let mut attributes = attributes;
    let class = cn!(
        "cn-combobox-item relative flex w-full cursor-default items-center outline-hidden select-none",
        take_class(&mut attributes)
    );
    let id = use_hook(|| NEXT_ID.fetch_add(1, Ordering::Relaxed));
    let label_text = label.clone().unwrap_or_else(|| value.clone());
    let value = use_prop(value);
    let label = use_prop(label_text);

    use_effect(move || {
        let value = value();
        let label = label();
        if ctx.value().as_deref() == Some(value.as_str()) {
            ctx.selected.set(Some((value.clone(), label.clone())));
        }
        ctx.register(Entry { id, value, label });
    });
    use_drop(move || ctx.unregister(id));

    let visible = label().to_lowercase().contains(&ctx.filter().to_lowercase());
    if !visible {
        return rsx! {};
    }
    let selected = ctx.value().as_deref() == Some(value().as_str());
    let highlighted = (ctx.highlighted)() == Some(id);
    let item_label = label();
    let item_value = value();

    rsx! {
        div {
            "data-slot": "combobox-item",
            id: "combobox-item-{id}",
            role: "option",
            aria_selected: selected,
            "data-highlighted": highlighted.then_some("true"),
            "data-selected": selected.then_some("true"),
            class: class,
            onmousedown: move |event| event.prevent_default(),
            onmouseenter: move |_| ctx.highlighted.set(Some(id)),
            onclick: move |_| ctx.select(item_value.clone(), item_label.clone()),
            ..attributes,
            span { "data-slot": "combobox-item-text", class: "cn-combobox-item-text", {children} }
            if selected {
                span { "data-slot": "combobox-item-indicator", class: "cn-combobox-item-indicator",
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
pub fn ComboboxGroup(
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!("flex flex-col", take_class(&mut attributes));

    rsx! {
        div {
            "data-slot": "combobox-group",
            role: "group",
            class: class,
            ..attributes,
            {children}
        }
    }
}

#[component]
pub fn ComboboxLabel(
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!("cn-combobox-label", take_class(&mut attributes));

    rsx! {
        div {
            "data-slot": "combobox-label",
            class: class,
            ..attributes,
            {children}
        }
    }
}

/// Shown inside `ComboboxContent` when no item matches the typed text.
#[component]
pub fn ComboboxEmpty(
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!("cn-combobox-empty", take_class(&mut attributes));

    rsx! {
        div {
            "data-slot": "combobox-empty",
            class: class,
            ..attributes,
            {children}
        }
    }
}

#[component]
pub fn ComboboxSeparator(
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
) -> Element {
    let mut attributes = attributes;
    let class = cn!("cn-combobox-separator", take_class(&mut attributes));

    rsx! {
        div {
            "data-slot": "combobox-separator",
            role: "separator",
            class: class,
            ..attributes,
        }
    }
}
