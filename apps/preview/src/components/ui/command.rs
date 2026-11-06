//! Command: a filterable list of actions that the user searches and picks with the keyboard or the mouse.
//!
//! It follows `cmdk`, the library shadcn/ui wraps. Items are matched against the search text with the
//! same fuzzy scoring (an item matches when the typed letters appear in it in order, with word starts
//! scoring higher), so groups and the empty state react to the search. The first item is highlighted
//! after every search; `ArrowDown`, `ArrowUp`, `Home`, `End`, `Ctrl+N`, `Ctrl+P`, `Ctrl+J`, `Ctrl+K`,
//! `Cmd+Arrow` and `Alt+Arrow` (groups) move the highlight and `Enter` runs the highlighted item's
//! `on_select`. Items keep the document order instead of being sorted by score. `CommandDialog` puts
//! the menu in a `Dialog`.
//!
//! Installed by oniko-cn. This file is yours: edit it freely.

use std::collections::HashMap;
use std::sync::atomic::{AtomicUsize, Ordering};

use dioxus::prelude::*;
use oniko_cn::{cn, take_class};

use super::dialog::{Dialog, DialogContent, DialogDescription, DialogHeader, DialogTitle};
use super::input_group::{InputGroup, InputGroupAddon};

static NEXT_ID: AtomicUsize = AtomicUsize::new(0);

const NAVIGATE_SCRIPT: &str = r#"
const root = document.getElementById("__ROOT__");
const items = root ? [...root.querySelectorAll('[cmdk-item]:not([aria-disabled="true"])')] : [];
const action = "__ACTION__";
const current = items.findIndex((item) => item.id === "__CURRENT__");
let next = -1;
if (items.length) {
  const last = items.length - 1;
  if (action === "first") next = 0;
  else if (action === "last") next = last;
  else if (action === "next") next = current < 0 ? 0 : current + 1 > last ? (__LOOP__ ? 0 : last) : current + 1;
  else if (action === "prev") next = current < 0 ? last : current - 1 < 0 ? (__LOOP__ ? last : 0) : current - 1;
  else {
    const groups = [...root.querySelectorAll("[cmdk-group]")].filter((group) => group.querySelector('[cmdk-item]:not([aria-disabled="true"])'));
    const at = groups.indexOf(current < 0 ? null : items[current].closest("[cmdk-group]"));
    const target = action === "next-group" ? groups[at + 1] : groups[at - 1];
    const firstOf = target && target.querySelector('[cmdk-item]:not([aria-disabled="true"])');
    next = firstOf ? items.indexOf(firstOf) : current;
  }
}
const item = next >= 0 ? items[next] : null;
if (item) {
  item.scrollIntoView({ block: "nearest" });
  const group = item.closest("[cmdk-group]");
  if (group && group.querySelector('[cmdk-item]:not([aria-disabled="true"])') === item) {
    const heading = group.querySelector("[cmdk-group-heading]");
    if (heading) heading.scrollIntoView({ block: "nearest" });
  }
}
dioxus.send(item ? item.id : "");
"#;

const FIRST_SCRIPT: &str = r#"
await new Promise((resolve) => requestAnimationFrame(() => requestAnimationFrame(() => resolve())));
const root = document.getElementById("__ROOT__");
const item = root && root.querySelector('[cmdk-item]:not([aria-disabled="true"])');
if (item) {
  const list = item.closest("[cmdk-list]");
  if (list) list.scrollTop = 0;
}
dioxus.send(item ? item.id : "");
"#;

/// A signal that follows a prop, so context shared with children never goes stale.
fn use_prop<T: Clone + PartialEq + 'static>(value: T) -> Signal<T> {
    let mut signal = use_signal(|| value.clone());
    if *signal.peek() != value {
        signal.set(value);
    }
    signal
}

type Filter = Callback<(String, String, Vec<String>), f64>;

/// One registered item, as the root sees it for filtering and selection.
#[derive(Clone)]
struct Entry {
    id: usize,
    value: String,
    keywords: Vec<String>,
    group: Option<usize>,
    on_select: Option<EventHandler<String>>,
}

#[derive(Clone, Copy)]
struct CommandContext {
    id: usize,
    query: Signal<String>,
    /// Id of the highlighted item, when the highlight is not controlled.
    active: Signal<Option<usize>>,
    controlled_value: Signal<Option<String>>,
    entries: Signal<Vec<Entry>>,
    ready: Signal<bool>,
    should_filter: Signal<bool>,
    filter: Option<Filter>,
    loop_around: Signal<bool>,
    on_value_change: Option<EventHandler<String>>,
}

impl CommandContext {
    fn root_id(self) -> String {
        format!("command-{}", self.id)
    }

    fn input_id(self) -> String {
        format!("command-{}-input", self.id)
    }

    fn list_id(self) -> String {
        format!("command-{}-list", self.id)
    }

    fn item_dom_id(self, item: usize) -> String {
        format!("command-{}-item-{item}", self.id)
    }

    fn score(self, value: &str, keywords: &[String]) -> f64 {
        let query = (self.query)();
        if !(self.should_filter)() || query.is_empty() {
            return 1.0;
        }
        match self.filter {
            Some(filter) => filter.call((value.to_string(), query, keywords.to_vec())),
            None => command_score(value.trim(), query.trim(), keywords),
        }
    }

    fn entry_visible(self, entry: &Entry) -> bool {
        self.score(&entry.value, &entry.keywords) > 0.0
    }

    fn any_visible(self, group: Option<usize>) -> bool {
        self.entries
            .read()
            .iter()
            .any(|entry| (group.is_none() || entry.group == group) && self.entry_visible(entry))
    }

    fn active_id(self) -> Option<usize> {
        if let Some(value) = (self.controlled_value)() {
            return self
                .entries
                .read()
                .iter()
                .find(|entry| entry.value == value)
                .map(|entry| entry.id);
        }
        (self.active)()
    }

    fn activate(self, id: usize) {
        let mut active = self.active;
        if *active.peek() == Some(id) {
            return;
        }
        active.set(Some(id));
        if let Some(handler) = self.on_value_change
            && let Some(entry) = self.entries.peek().iter().find(|entry| entry.id == id)
        {
            handler.call(entry.value.clone());
        }
    }

    fn activate_dom_id(self, dom_id: &str) {
        let prefix = format!("command-{}-item-", self.id);
        if let Some(id) = dom_id
            .strip_prefix(&prefix)
            .and_then(|rest| rest.parse::<usize>().ok())
        {
            self.activate(id);
        }
    }

    fn navigate(self, action: &'static str) {
        let current = self
            .active_id()
            .map(|id| self.item_dom_id(id))
            .unwrap_or_default();
        let script = NAVIGATE_SCRIPT
            .replace("__ROOT__", &self.root_id())
            .replace("__ACTION__", action)
            .replace("__CURRENT__", &current)
            .replace(
                "__LOOP__",
                if *self.loop_around.peek() {
                    "true"
                } else {
                    "false"
                },
            );
        spawn(async move {
            let mut eval = document::eval(&script);
            if let Ok(dom_id) = eval.recv::<String>().await {
                self.activate_dom_id(&dom_id);
            }
        });
    }

    /// Highlights the first enabled item once the list has rendered.
    fn select_first(self) {
        let script = FIRST_SCRIPT.replace("__ROOT__", &self.root_id());
        spawn(async move {
            let mut eval = document::eval(&script);
            if let Ok(dom_id) = eval.recv::<String>().await {
                self.activate_dom_id(&dom_id);
            }
        });
    }

    fn run_active(self) {
        let Some(id) = self.active_id() else {
            return;
        };
        let entry = self
            .entries
            .peek()
            .iter()
            .find(|entry| entry.id == id)
            .cloned();
        if let Some(entry) = entry
            && let Some(handler) = entry.on_select
        {
            handler.call(entry.value);
        }
    }
}

fn is_gap(c: char) -> bool {
    matches!(
        c,
        '\\' | '/' | '_' | '+' | '.' | '#' | '"' | '@' | '[' | '(' | '{' | '&'
    )
}

fn is_space(c: char) -> bool {
    c.is_whitespace() || c == '-'
}

fn lower(text: &str) -> Vec<char> {
    text.chars()
        .map(|c| {
            if is_space(c) {
                ' '
            } else {
                c.to_lowercase().next().unwrap_or(c)
            }
        })
        .collect()
}

/// Fuzzy score of `abbreviation` against `string` plus `keywords`, the algorithm of cmdk's
/// `command-score`: 0 means no match, higher is better.
fn command_score(string: &str, abbreviation: &str, keywords: &[String]) -> f64 {
    let mut full = string.to_string();
    if !keywords.is_empty() {
        full.push(' ');
        full.push_str(&keywords.join(" "));
    }
    let chars: Vec<char> = full.chars().collect();
    let lowered = lower(&full);
    let abbreviation_chars: Vec<char> = abbreviation.chars().collect();
    let lowered_abbreviation = lower(abbreviation);
    if chars.len() != lowered.len() || abbreviation_chars.len() != lowered_abbreviation.len() {
        return f64::from(u8::from(
            full.to_lowercase().contains(&abbreviation.to_lowercase()),
        ));
    }
    score_inner(
        &Text {
            string: &chars,
            lowered: &lowered,
            abbreviation: &abbreviation_chars,
            lowered_abbreviation: &lowered_abbreviation,
        },
        0,
        0,
        &mut HashMap::new(),
    )
}

struct Text<'a> {
    string: &'a [char],
    lowered: &'a [char],
    abbreviation: &'a [char],
    lowered_abbreviation: &'a [char],
}

fn score_inner(
    text: &Text,
    string_index: usize,
    abbreviation_index: usize,
    memo: &mut HashMap<(usize, usize), f64>,
) -> f64 {
    const CONTINUOUS_MATCH: f64 = 1.0;
    const SPACE_WORD_JUMP: f64 = 0.9;
    const NON_SPACE_WORD_JUMP: f64 = 0.8;
    const CHARACTER_JUMP: f64 = 0.17;
    const TRANSPOSITION: f64 = 0.1;
    const SKIPPED: f64 = 0.999;
    const CASE_MISMATCH: f64 = 0.9999;
    const NOT_COMPLETE: f64 = 0.99;

    if abbreviation_index == text.abbreviation.len() {
        return if string_index == text.string.len() {
            CONTINUOUS_MATCH
        } else {
            NOT_COMPLETE
        };
    }
    if let Some(score) = memo.get(&(string_index, abbreviation_index)) {
        return *score;
    }

    let wanted = text.lowered_abbreviation[abbreviation_index];
    let next_wanted = text
        .lowered_abbreviation
        .get(abbreviation_index + 1)
        .copied();
    let find = |from: usize| {
        text.lowered
            .get(from..)
            .and_then(|rest| rest.iter().position(|c| *c == wanted))
            .map(|offset| offset + from)
    };

    let mut high = 0.0_f64;
    let mut index = find(string_index);
    while let Some(at) = index {
        let before = at.checked_sub(1).map(|i| text.string[i]);
        let mut score = score_inner(text, at + 1, abbreviation_index + 1, memo);
        if score > high {
            if at == string_index {
                score *= CONTINUOUS_MATCH;
            } else if before.is_some_and(is_gap) {
                score *= NON_SPACE_WORD_JUMP;
                let breaks = text.string[string_index..at - 1]
                    .iter()
                    .filter(|c| is_gap(**c))
                    .count();
                if breaks > 0 && string_index > 0 {
                    score *= SKIPPED.powi(i32::try_from(breaks).unwrap_or(i32::MAX));
                }
            } else if before.is_some_and(is_space) {
                score *= SPACE_WORD_JUMP;
                let breaks = text.string[string_index..at - 1]
                    .iter()
                    .filter(|c| is_space(**c))
                    .count();
                if breaks > 0 && string_index > 0 {
                    score *= SKIPPED.powi(i32::try_from(breaks).unwrap_or(i32::MAX));
                }
            } else {
                score *= CHARACTER_JUMP;
                if string_index > 0 {
                    score *= SKIPPED.powi(i32::try_from(at - string_index).unwrap_or(i32::MAX));
                }
            }
            if text.string[at] != text.abbreviation[abbreviation_index] {
                score *= CASE_MISMATCH;
            }
        }

        let before_lowered = at.checked_sub(1).map(|i| text.lowered[i]);
        let transposed =
            (score < TRANSPOSITION && before_lowered.is_some() && before_lowered == next_wanted)
                || (next_wanted == Some(wanted) && before_lowered != Some(wanted));
        if transposed && abbreviation_index + 2 <= text.abbreviation.len() {
            let transposed_score = score_inner(text, at + 1, abbreviation_index + 2, memo);
            if transposed_score * TRANSPOSITION > score {
                score = transposed_score * TRANSPOSITION;
            }
        }
        if score > high {
            high = score;
        }
        index = find(at + 1);
    }

    memo.insert((string_index, abbreviation_index), high);
    high
}

/// Root of the command menu. Owns the search text and the highlighted item.
#[component]
pub fn Command(
    /// Value of the highlighted item. When set, `on_value_change` is the only way to move it.
    value: Option<String>,
    on_value_change: Option<EventHandler<String>>,
    /// Accessible name, announced for the input and the list.
    label: Option<String>,
    /// Turn off the built-in search, for example when the items come from the server.
    #[props(default = true)]
    should_filter: bool,
    /// Scores an item: `(value, search, keywords)`. `0.0` hides it. The default is cmdk's fuzzy score.
    filter: Option<Filter>,
    /// Wrap from the last item to the first and back.
    #[props(default)]
    r#loop: bool,
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let id = use_hook(|| NEXT_ID.fetch_add(1, Ordering::Relaxed));
    let mut ready = use_signal(|| false);
    let ctx = use_context_provider(|| CommandContext {
        id,
        query: Signal::new(String::new()),
        active: Signal::new(None),
        controlled_value: Signal::new(None),
        entries: Signal::new(Vec::new()),
        ready,
        should_filter: Signal::new(true),
        filter,
        loop_around: Signal::new(false),
        on_value_change,
    });
    let mut controlled_value = ctx.controlled_value;
    if *controlled_value.peek() != value {
        controlled_value.set(value);
    }
    let mut should_filter_signal = ctx.should_filter;
    if *should_filter_signal.peek() != should_filter {
        should_filter_signal.set(should_filter);
    }
    let mut loop_signal = ctx.loop_around;
    if *loop_signal.peek() != r#loop {
        loop_signal.set(r#loop);
    }
    use_hook(|| {
        spawn(async move {
            let mut eval = document::eval(
                "await new Promise((resolve) => requestAnimationFrame(() => requestAnimationFrame(() => resolve()))); dioxus.send(true);",
            );
            let _ = eval.recv::<bool>().await;
            ready.set(true);
            ctx.select_first();
            // A dialog focuses its popup; hand focus to the search box so the keys reach the menu.
            let _ = document::eval(&format!(
                "const root = document.getElementById({:?}); if (root && root.closest('[data-slot=\"dialog-content\"]')) document.getElementById({:?})?.focus();",
                ctx.root_id(),
                ctx.input_id()
            ))
            .await;
        });
    });

    let mut attributes = attributes;
    let class = cn!(
        "bg-popover text-popover-foreground rounded-xl! p-1 flex size-full flex-col overflow-hidden",
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        div {
            id: ctx.root_id(),
            "data-slot": "command",
            "cmdk-root": "",
            tabindex: "-1",
            class: class,
            onkeydown: move |event: KeyboardEvent| {
                let modifiers = event.modifiers();
                let ctrl = modifiers.ctrl();
                let action = match event.key() {
                    Key::ArrowDown if modifiers.meta() => "last",
                    Key::ArrowDown if modifiers.alt() => "next-group",
                    Key::ArrowDown => "next",
                    Key::ArrowUp if modifiers.meta() => "first",
                    Key::ArrowUp if modifiers.alt() => "prev-group",
                    Key::ArrowUp => "prev",
                    Key::Home => "first",
                    Key::End => "last",
                    Key::Character(ref key) if ctrl && (key == "n" || key == "j") => "next",
                    Key::Character(ref key) if ctrl && (key == "p" || key == "k") => "prev",
                    Key::Enter if !event.is_composing() => {
                        event.prevent_default();
                        ctx.run_active();
                        return;
                    }
                    _ => return,
                };
                event.prevent_default();
                ctx.navigate(action);
            },
            ..attributes,
            if let Some(label) = label {
                label {
                    "cmdk-label": "",
                    r#for: ctx.input_id(),
                    class: "sr-only",
                    "{label}"
                }
            }
            {children}
        }
    }
}

/// A `Dialog` that holds a command menu. Put a `Command`, or `CommandInput` and `CommandList`, inside.
#[component]
pub fn CommandDialog(
    open: Option<bool>,
    #[props(default)] default_open: bool,
    on_open_change: Option<EventHandler<bool>>,
    #[props(default = "Command Palette".to_string())] title: String,
    #[props(default = "Search for a command to run...".to_string())] description: String,
    #[props(default)] show_close_button: bool,
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(
        "rounded-xl! top-1/3 translate-y-0 overflow-hidden p-0",
        take_class(&mut attributes).as_deref()
    );
    attributes.push(Attribute::new("class", class, None, false));

    rsx! {
        Dialog { open, default_open, on_open_change,
            DialogHeader { class: "sr-only",
                DialogTitle { "{title}" }
                DialogDescription { "{description}" }
            }
            DialogContent { show_close_button, attributes, {children} }
        }
    }
}

/// The search box. `value` and `on_value_change` control the search text.
#[component]
pub fn CommandInput(
    value: Option<String>,
    on_value_change: Option<EventHandler<String>>,
    #[props(extends = GlobalAttributes, extends = input)] attributes: Vec<Attribute>,
) -> Element {
    let ctx = use_context::<CommandContext>();
    let mut query = ctx.query;
    let mut attributes = attributes;
    let class = cn!(
        "w-full text-sm outline-hidden disabled:cursor-not-allowed disabled:opacity-50",
        take_class(&mut attributes).as_deref()
    );
    let text = value.unwrap_or(query());
    let active = ctx.active_id().map(|id| ctx.item_dom_id(id));

    rsx! {
        div { "data-slot": "command-input-wrapper", class: "p-1 pb-0",
            InputGroup { class: "bg-input/30 border-input/30 h-8! rounded-lg! shadow-none! *:data-[slot=input-group-addon]:pl-2!",
                input {
                    id: ctx.input_id(),
                    "data-slot": "command-input",
                    "cmdk-input": "",
                    r#type: "text",
                    role: "combobox",
                    autocomplete: "off",
                    autocorrect: "off",
                    spellcheck: false,
                    aria_autocomplete: "list",
                    aria_expanded: true,
                    aria_controls: ctx.list_id(),
                    aria_activedescendant: active,
                    value: text,
                    class: class,
                    oninput: move |event| {
                        let next = event.value();
                        query.set(next.clone());
                        if let Some(handler) = on_value_change {
                            handler.call(next);
                        }
                        ctx.select_first();
                    },
                    ..attributes,
                }
                InputGroupAddon {
                    svg {
                        class: "size-4 shrink-0 opacity-50",
                        "aria-hidden": "true",
                        view_box: "0 0 24 24",
                        fill: "none",
                        stroke: "currentColor",
                        stroke_width: "2",
                        stroke_linecap: "round",
                        stroke_linejoin: "round",
                        path { d: "m21 21-4.34-4.34" }
                        circle { cx: "11", cy: "11", r: "8" }
                    }
                }
            }
        }
    }
}

#[component]
pub fn CommandList(
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let ctx = use_context::<CommandContext>();
    let mut attributes = attributes;
    let class = cn!(
        "no-scrollbar max-h-72 scroll-py-1 outline-none overflow-x-hidden overflow-y-auto",
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        div {
            id: ctx.list_id(),
            "data-slot": "command-list",
            "cmdk-list": "",
            role: "listbox",
            tabindex: "-1",
            aria_label: "Suggestions",
            class: class,
            ..attributes,
            div { "cmdk-list-sizer": "", {children} }
        }
    }
}

/// Shown while no item matches the search.
#[component]
pub fn CommandEmpty(
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let ctx = use_context::<CommandContext>();
    if !(ctx.ready)() || ctx.any_visible(None) {
        return rsx! {};
    }
    let mut attributes = attributes;
    let class = cn!(
        "py-6 text-center text-sm",
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        div {
            "data-slot": "command-empty",
            "cmdk-empty": "",
            role: "presentation",
            class: class,
            ..attributes,
            {children}
        }
    }
}

#[derive(Clone, Copy)]
struct GroupContext {
    id: usize,
}

/// A labelled set of items. Hidden while none of its items match the search.
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
    let mut attributes = attributes;
    let class = cn!(
        "text-foreground **:[[cmdk-group-heading]]:text-muted-foreground overflow-hidden p-1 **:[[cmdk-group-heading]]:px-2 **:[[cmdk-group-heading]]:py-1.5 **:[[cmdk-group-heading]]:text-xs **:[[cmdk-group-heading]]:font-medium",
        take_class(&mut attributes).as_deref()
    );
    let hidden = !ctx.any_visible(Some(id));

    rsx! {
        div {
            "data-slot": "command-group",
            "cmdk-group": "",
            role: "presentation",
            hidden: hidden,
            class: class,
            ..attributes,
            if let Some(heading) = heading {
                div {
                    id: "command-{ctx.id}-group-{id}",
                    "cmdk-group-heading": "",
                    aria_hidden: "true",
                    "{heading}"
                }
            }
            div {
                "cmdk-group-items": "",
                role: "group",
                aria_labelledby: "command-{ctx.id}-group-{id}",
                {children}
            }
        }
    }
}

#[component]
pub fn CommandSeparator(
    /// Keep the separator while searching. By default it is hidden then.
    #[props(default)]
    always_render: bool,
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
) -> Element {
    let ctx = use_context::<CommandContext>();
    if !always_render && !(ctx.query)().is_empty() {
        return rsx! {};
    }
    let mut attributes = attributes;
    let class = cn!(
        "bg-border -mx-1 h-px w-auto",
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        div {
            "data-slot": "command-separator",
            "cmdk-separator": "",
            role: "separator",
            class: class,
            ..attributes,
        }
    }
}

/// One selectable row. Renders nothing while it does not match the search.
///
/// `value` defaults to the item's text. `on_select` runs on click and on `Enter` with the item's value.
#[component]
pub fn CommandItem(
    value: Option<String>,
    /// Extra words the search also matches against.
    #[props(default)]
    keywords: Vec<String>,
    #[props(default)] disabled: bool,
    on_select: Option<EventHandler<String>>,
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let ctx = use_context::<CommandContext>();
    let group = try_use_context::<GroupContext>().map(|group| group.id);
    let id = use_hook(|| NEXT_ID.fetch_add(1, Ordering::Relaxed));
    let dom_id = ctx.item_dom_id(id);
    let value_prop = use_prop(value);
    let keywords = use_prop(keywords);
    let on_select = use_prop(on_select);
    let mut text = use_signal(String::new);
    let mut entries = ctx.entries;

    let resolved = value_prop().unwrap_or(text());
    use_effect(move || {
        let entry = Entry {
            id,
            value: value_prop().unwrap_or(text()),
            keywords: keywords(),
            group,
            on_select: on_select(),
        };
        let mut list = entries.write();
        match list.iter_mut().find(|existing| existing.id == id) {
            Some(existing) => *existing = entry,
            None => list.push(entry),
        }
    });
    use_drop(move || {
        if let Ok(mut list) = entries.try_write() {
            list.retain(|entry| entry.id != id);
        }
    });

    if ctx.score(&resolved, &keywords()) <= 0.0 {
        return rsx! {};
    }

    let selected = ctx.active_id() == Some(id);
    let mut attributes = attributes;
    let class = cn!(
        "data-selected:bg-muted data-selected:text-foreground data-selected:**:[svg]:text-foreground relative flex cursor-default items-center gap-2 rounded-sm px-2 py-1.5 text-sm outline-hidden select-none in-data-[slot=dialog-content]:rounded-lg! [&_svg:not([class*='size-'])]:size-4 group/command-item data-[disabled=true]:pointer-events-none data-[disabled=true]:opacity-50 [&_svg]:pointer-events-none [&_svg]:shrink-0",
        take_class(&mut attributes).as_deref()
    );
    let read_text = {
        let dom_id = dom_id.clone();
        move |_| {
            let dom_id = dom_id.clone();
            async move {
                let script = format!(
                    "dioxus.send((document.getElementById({dom_id:?})?.textContent ?? '').trim());"
                );
                let mut eval = document::eval(&script);
                if let Ok(content) = eval.recv::<String>().await
                    && *text.peek() != content
                {
                    text.set(content);
                }
            }
        }
    };

    rsx! {
        div {
            id: dom_id,
            "data-slot": "command-item",
            "cmdk-item": "",
            "data-value": resolved.clone(),
            "data-selected": selected.then_some("true"),
            "data-disabled": disabled.then_some("true"),
            role: "option",
            aria_selected: selected,
            aria_disabled: disabled,
            class: class,
            onmounted: read_text,
            onpointermove: move |_| {
                if !disabled {
                    ctx.activate(id);
                }
            },
            onclick: move |_| {
                if disabled {
                    return;
                }
                ctx.activate(id);
                if let Some(handler) = on_select() {
                    handler.call(value_prop().unwrap_or(text()));
                }
            },
            ..attributes,
            {children}
            svg {
                class: "ml-auto opacity-0 group-has-data-[slot=command-shortcut]/command-item:hidden group-data-[checked=true]/command-item:opacity-100",
                "aria-hidden": "true",
                view_box: "0 0 24 24",
                fill: "none",
                stroke: "currentColor",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
                path { d: "M20 6 9 17l-5-5" }
            }
        }
    }
}

/// Keyboard shortcut hint, aligned to the end of an item.
#[component]
pub fn CommandShortcut(
    #[props(extends = GlobalAttributes, extends = span)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(
        "text-muted-foreground group-data-selected/command-item:text-foreground ml-auto text-xs tracking-widest",
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        span {
            "data-slot": "command-shortcut",
            class: class,
            ..attributes,
            {children}
        }
    }
}
