//! Command: a filterable list of actions that the user searches and picks with the keyboard or the mouse.
//!
//! Filtering matches the item `value` and its `keywords` as a case-insensitive substring. Arrow keys
//! move the highlight in on-screen order, `Enter` selects the highlighted item, and hovering an item
//! highlights it.
//!
//! Installed by oniko-cn. This file is yours: edit it freely.

use std::rc::Rc;
use std::sync::atomic::{AtomicUsize, Ordering};

use dioxus::prelude::*;
use oniko_cn::{cn, take_class};

static NEXT_ID: AtomicUsize = AtomicUsize::new(0);

/// One registered item, as seen by the root for filtering and keyboard navigation.
#[derive(Clone)]
struct Entry {
    id: usize,
    value: String,
    group: Option<usize>,
    disabled: bool,
    visible: bool,
    on_select: Option<EventHandler<String>>,
    mounted: Option<Rc<MountedData>>,
}

#[derive(Clone, Copy)]
struct CommandContext {
    query: Signal<String>,
    active: Signal<Option<String>>,
    entries: Signal<Vec<Entry>>,
}

#[derive(Clone, Copy)]
struct GroupContext {
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

fn matches(query: &str, value: &str, keywords: &[String]) -> bool {
    let query = query.trim().to_lowercase();
    query.is_empty()
        || std::iter::once(value)
            .chain(keywords.iter().map(String::as_str))
            .any(|text| text.to_lowercase().contains(&query))
}

/// Enabled, visible items in the order they appear on screen.
async fn ordered_enabled(entries: Signal<Vec<Entry>>) -> Vec<Entry> {
    let snapshot = entries.peek().clone();
    let mut placed = Vec::new();
    for entry in snapshot
        .into_iter()
        .filter(|e| e.visible && !e.disabled)
    {
        let Some(mounted) = entry.mounted.clone() else {
            continue;
        };
        if let Ok(rect) = mounted.get_client_rect().await {
            placed.push((rect.origin.y, entry));
        }
    }
    placed.sort_by(|a, b| a.0.total_cmp(&b.0));
    placed.into_iter().map(|(_, entry)| entry).collect()
}

fn move_active(ctx: CommandContext, step: isize) {
    spawn(async move {
        let items = ordered_enabled(ctx.entries).await;
        if items.is_empty() {
            return;
        }
        let len = items.len() as isize;
        let current = ctx.active.peek().clone();
        let index = match current.and_then(|v| items.iter().position(|e| e.value == v)) {
            Some(i) => (i as isize + step).rem_euclid(len),
            None if step > 0 => 0,
            None => len - 1,
        } as usize;
        let target = &items[index];
        let mut active = ctx.active;
        active.set(Some(target.value.clone()));
        let js = format!(
            "document.getElementById('command-item-{}')?.scrollIntoView({{ block: 'nearest' }})",
            target.id
        );
        let _ = document::eval(&js).await;
    });
}

fn select_active(ctx: CommandContext) {
    spawn(async move {
        let items = ordered_enabled(ctx.entries).await;
        let current = ctx.active.peek().clone();
        let target = current
            .and_then(|v| items.iter().find(|e| e.value == v).cloned())
            .or_else(|| items.first().cloned());
        if let Some(entry) = target
            && let Some(handler) = entry.on_select
        {
            handler.call(entry.value);
        }
    });
}

/// Root of the command menu. Owns the query and the highlighted item.
#[component]
pub fn Command(#[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>, children: Element) -> Element {
    let ctx = CommandContext {
        query: use_signal(String::new),
        active: use_signal(|| None),
        entries: use_signal(Vec::new),
    };
    use_context_provider(|| ctx);

    let mut attributes = attributes;
    let class = cn!(
        "cn-command flex h-full w-full flex-col overflow-hidden",
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        div {
            "data-slot": "command",
            class: class,
            onkeydown: move |event: KeyboardEvent| {
                match event.key() {
                    Key::ArrowDown => {
                        event.prevent_default();
                        move_active(ctx, 1);
                    }
                    Key::ArrowUp => {
                        event.prevent_default();
                        move_active(ctx, -1);
                    }
                    Key::Enter => {
                        event.prevent_default();
                        select_active(ctx);
                    }
                    _ => {}
                }
            },
            ..attributes,
            {children}
        }
    }
}

#[component]
pub fn CommandInput(#[props(extends = GlobalAttributes, extends = input)] attributes: Vec<Attribute>) -> Element {
    let ctx = use_context::<CommandContext>();
    let mut query = ctx.query;
    let mut active = ctx.active;

    let mut attributes = attributes;
    let class = cn!(
        "cn-command-input bg-transparent outline-none",
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        div {
            "data-slot": "command-input-wrapper",
            class: "cn-command-input-wrapper",
            div {
                "data-slot": "command-input-group",
                class: "cn-command-input-group flex items-center gap-2 px-3",
                svg {
                    "data-slot": "command-input-icon",
                    class: "cn-command-input-icon pointer-events-none",
                    view_box: "0 0 24 24",
                    fill: "none",
                    stroke: "currentColor",
                    stroke_width: "2",
                    stroke_linecap: "round",
                    stroke_linejoin: "round",
                    circle { cx: "11", cy: "11", r: "8" }
                    path { d: "m21 21-4.3-4.3" }
                }
                input {
                    "data-slot": "command-input",
                    r#type: "text",
                    autocomplete: "off",
                    value: "{query}",
                    class: class,
                    oninput: move |event| {
                        query.set(event.value());
                        active.set(None);
                    },
                    ..attributes,
                }
            }
        }
    }
}

#[component]
pub fn CommandList(#[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>, children: Element) -> Element {
    let mut attributes = attributes;
    let class = cn!("cn-command-list", take_class(&mut attributes).as_deref());

    rsx! {
        div {
            "data-slot": "command-list",
            role: "listbox",
            class: class,
            ..attributes,
            {children}
        }
    }
}

/// Shown when no item matches the query.
#[component]
pub fn CommandEmpty(#[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>, children: Element) -> Element {
    let ctx = use_context::<CommandContext>();
    if ctx.entries.read().iter().any(|e| e.visible) {
        return rsx! {};
    }

    let mut attributes = attributes;
    let class = cn!("cn-command-empty", take_class(&mut attributes).as_deref());

    rsx! {
        div {
            "data-slot": "command-empty",
            class: class,
            ..attributes,
            {children}
        }
    }
}

/// A labelled set of items. Hidden while none of its items match the query.
#[component]
pub fn CommandGroup(
    /// Text shown above the group's items.
    heading: Option<String>,
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let ctx = use_context::<CommandContext>();
    let id = use_hook(|| NEXT_ID.fetch_add(1, Ordering::Relaxed));
    use_context_provider(|| GroupContext { id });

    // Children stay mounted while the group is hidden so they keep registering.
    let has_visible = ctx
        .entries
        .read()
        .iter()
        .any(|e| e.group == Some(id) && e.visible);

    let mut attributes = attributes;
    let class = cn!(
        "cn-command-group overflow-hidden",
        (!has_visible).then_some("hidden"),
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        div {
            "data-slot": "command-group",
            role: "presentation",
            "cmdk-group": "",
            class: class,
            ..attributes,
            if let Some(heading) = heading {
                div {
                    "data-slot": "command-group-heading",
                    "cmdk-group-heading": "",
                    aria_hidden: "true",
                    {heading}
                }
            }
            {children}
        }
    }
}

/// One selectable row. Renders nothing while it does not match the query.
#[component]
pub fn CommandItem(
    /// Identifies the item. Matched against the query and passed to `on_select`.
    value: String,
    /// Extra text the query also matches against.
    #[props(default)]
    keywords: Vec<String>,
    #[props(default)] disabled: bool,
    on_select: Option<EventHandler<String>>,
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let ctx = use_context::<CommandContext>();
    let group = try_use_context::<GroupContext>().map(|g| g.id);
    let id = use_hook(|| NEXT_ID.fetch_add(1, Ordering::Relaxed));
    let value = use_prop(value);
    let keywords = use_prop(keywords);
    let disabled = use_prop(disabled);
    let on_select = use_prop(on_select);
    let mut mounted = use_signal(|| None::<Rc<MountedData>>);
    let mut entries = ctx.entries;

    use_effect(move || {
        let entry = Entry {
            id,
            value: value(),
            group,
            disabled: disabled(),
            visible: matches(&(ctx.query)(), &value(), &keywords()),
            on_select: on_select(),
            mounted: mounted(),
        };
        let mut list = entries.write();
        match list.iter_mut().find(|e| e.id == id) {
            Some(existing) => *existing = entry,
            None => list.push(entry),
        }
    });
    use_drop(move || {
        if let Ok(mut list) = entries.try_write() {
            list.retain(|e| e.id != id);
        }
    });

    let visible = matches(&(ctx.query)(), &value(), &keywords());
    if !visible {
        return rsx! {};
    }

    let selected = ctx.active.read().as_deref() == Some(value().as_str());
    let mut attributes = attributes;
    let class = cn!(
        "cn-command-item group/command-item relative flex cursor-default items-center select-none outline-none data-disabled:pointer-events-none data-disabled:opacity-50",
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        div {
            "data-slot": "command-item",
            "data-selected": selected.then_some("true"),
            "data-disabled": disabled().then_some("true"),
            id: "command-item-{id}",
            role: "option",
            aria_selected: selected,
            aria_disabled: disabled(),
            class: class,
            onmounted: move |event| mounted.set(Some(event.data())),
            onmouseenter: move |_| {
                if !disabled() {
                    let mut active = ctx.active;
                    active.set(Some(value()));
                }
            },
            onclick: move |_| {
                if disabled() {
                    return;
                }
                let mut active = ctx.active;
                active.set(Some(value()));
                if let Some(handler) = on_select() {
                    handler.call(value());
                }
            },
            ..attributes,
            {children}
        }
    }
}

#[component]
pub fn CommandSeparator(#[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>) -> Element {
    let mut attributes = attributes;
    let class = cn!("cn-command-separator", take_class(&mut attributes).as_deref());

    rsx! {
        div {
            "data-slot": "command-separator",
            role: "separator",
            class: class,
            ..attributes,
        }
    }
}

/// Keyboard shortcut hint, aligned to the end of an item.
#[component]
pub fn CommandShortcut(#[props(extends = GlobalAttributes, extends = span)] attributes: Vec<Attribute>, children: Element) -> Element {
    let mut attributes = attributes;
    let class = cn!("cn-command-shortcut", take_class(&mut attributes).as_deref());

    rsx! {
        span {
            "data-slot": "command-shortcut",
            class: class,
            ..attributes,
            {children}
        }
    }
}
