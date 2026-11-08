//! Questionnaire: a multi-step questionnaire with single-choice, multiple-choice, freeform and
//! skippable questions.
//!
//! `Questionnaire` renders a `<form>` and owns the ordered items, the active item, the answers,
//! validation, progress and navigation. Its parts are `QuestionnaireProgress`,
//! `QuestionnaireItem` (a `<fieldset>` holding one question), `QuestionnaireTitle` (its
//! `<legend>`), `QuestionnaireDescription`, `QuestionnaireChoices`, `QuestionnaireChoice` (a radio,
//! or a checkbox when the item is `multiple`), `QuestionnaireChoiceDescription`,
//! `QuestionnaireInput` (a freeform answer), `QuestionnaireError`, and `QuestionnaireActions` with
//! `QuestionnairePrevious`, `QuestionnaireSkip`, `QuestionnaireNext` and `QuestionnaireSubmit`.
//!
//! Answers are native form values named after their item. Read them in `onsubmit` with
//! `event.values()`, and call `event.prevent_default()` there to stay on the page. Pass `items` so
//! progress, actions and shortcuts know every item before it renders. Items are keyed by `name`;
//! `item` and `on_item_change` control the active one.
//!
//! Keyboard: Arrow Up and Arrow Down move between answers, Arrow Left and Arrow Right go to the
//! previous and next item, Enter on a filled answer or Cmd/Ctrl+Enter confirms the item, and with
//! `shortcuts` a letter or a number picks an answer.
//!
//! Installed by oniko-cn. This file is yours: edit it freely.

use std::sync::atomic::{AtomicUsize, Ordering};

use dioxus::core::AttributeValue;
use dioxus::prelude::*;
use oniko_cn::{cn, take_class};

use super::button::{ButtonSize, ButtonVariant, button_variants};

static NEXT_ID: AtomicUsize = AtomicUsize::new(0);

fn next_id() -> usize {
    NEXT_ID.fetch_add(1, Ordering::Relaxed)
}

fn flag(on: bool) -> Option<&'static str> {
    on.then_some("")
}

/// Whether an item has an answer, was skipped on purpose, or neither.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum QuestionnaireItemStatus {
    #[default]
    Unanswered,
    Answered,
    Skipped,
}

impl QuestionnaireItemStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Unanswered => "unanswered",
            Self::Answered => "answered",
            Self::Skipped => "skipped",
        }
    }
}

/// Which keys pick a choice: `A`-`Z` or `1`-`9`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum QuestionnaireShortcuts {
    #[default]
    Letters,
    Numbers,
}

impl QuestionnaireShortcuts {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Letters => "letters",
            Self::Numbers => "numbers",
        }
    }

    fn keys(self) -> Vec<String> {
        match self {
            Self::Letters => ('A'..='Z').map(String::from).collect(),
            Self::Numbers => (1..=9).map(|n: u8| n.to_string()).collect(),
        }
    }

    fn shortcut_for_key(self, key: &str) -> Option<String> {
        let key = match self {
            Self::Letters => key.to_uppercase(),
            Self::Numbers => key.to_string(),
        };
        self.keys().contains(&key).then_some(key)
    }
}

/// The `type` of a `QuestionnaireInput`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum QuestionnaireInputType {
    Date,
    DatetimeLocal,
    Email,
    Month,
    Number,
    Password,
    Search,
    Tel,
    #[default]
    Text,
    Time,
    Url,
    Week,
}

impl QuestionnaireInputType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Date => "date",
            Self::DatetimeLocal => "datetime-local",
            Self::Email => "email",
            Self::Month => "month",
            Self::Number => "number",
            Self::Password => "password",
            Self::Search => "search",
            Self::Tel => "tel",
            Self::Text => "text",
            Self::Time => "time",
            Self::Url => "url",
            Self::Week => "week",
        }
    }

    /// Text-like inputs where Arrow Up and Arrow Down still move between answers while empty.
    fn navigable(self) -> bool {
        matches!(
            self,
            Self::Email | Self::Password | Self::Search | Self::Tel | Self::Text | Self::Url
        )
    }
}

/// One fixed choice of an item in `Questionnaire::items`. Shortcut keys follow the choice order.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct QuestionnaireChoiceDefinition {
    pub value: String,
    pub disabled: bool,
}

impl QuestionnaireChoiceDefinition {
    pub fn new(value: impl Into<String>) -> Self {
        Self {
            value: value.into(),
            disabled: false,
        }
    }
}

/// One item in `Questionnaire::items`, in questionnaire order.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct QuestionnaireItemDefinition {
    pub name: String,
    pub required: bool,
    pub disabled: bool,
    pub choices: Vec<QuestionnaireChoiceDefinition>,
}

impl QuestionnaireItemDefinition {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            ..Self::default()
        }
    }

    pub fn required(mut self, required: bool) -> Self {
        self.required = required;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn choices<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.choices = values
            .into_iter()
            .map(QuestionnaireChoiceDefinition::new)
            .collect();
        self
    }
}

/// Position of the active item, passed to `QuestionnaireProgress::render`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct QuestionnaireProgressState {
    /// 1-based position of the active item, 0 when there is none.
    pub current: usize,
    pub total: usize,
    pub first: bool,
    pub last: bool,
}

/// What `QuestionnaireTitle` and `QuestionnaireDescription` hand to `render` when the caller
/// renders another element in their place. Spread `attributes` on it and put `children` inside.
#[derive(Clone)]
pub struct QuestionnaireRenderProps {
    pub attributes: Vec<Attribute>,
    pub children: Element,
}

#[derive(Clone, Debug, PartialEq)]
enum AnswerKind {
    Choice {
        value: String,
    },
    Input {
        navigable: bool,
        has_value: bool,
        default_filled: bool,
    },
}

#[derive(Clone, Debug, PartialEq)]
struct Answer {
    id: usize,
    dom_id: String,
    own_disabled: bool,
    kind: AnswerKind,
}

#[derive(Clone, Debug, Default, PartialEq)]
struct ItemEntry {
    name: String,
    seq: usize,
    dom_id: String,
    /// False while only answers have registered and the item itself has not rendered yet.
    mounted: bool,
    required: bool,
    multiple: bool,
    disabled: bool,
    external_invalid: bool,
    answers: Vec<Answer>,
    selected: Vec<usize>,
    defaults: Vec<usize>,
    skipped: bool,
    attempted: bool,
    reset_version: usize,
}

impl ItemEntry {
    fn enabled_answers(&self) -> impl Iterator<Item = &Answer> {
        self.answers
            .iter()
            .filter(|answer| !(self.disabled || answer.own_disabled))
    }

    fn status(&self) -> QuestionnaireItemStatus {
        if self.skipped {
            QuestionnaireItemStatus::Skipped
        } else if self
            .enabled_answers()
            .any(|answer| self.selected.contains(&answer.id))
        {
            QuestionnaireItemStatus::Answered
        } else {
            QuestionnaireItemStatus::Unanswered
        }
    }

    fn intentionally_skipped(&self) -> bool {
        self.status() == QuestionnaireItemStatus::Skipped && !self.required
    }

    fn valid(&self) -> bool {
        self.disabled
            || self.intentionally_skipped()
            || (!self.external_invalid && self.status() == QuestionnaireItemStatus::Answered)
    }

    fn invalid(&self) -> bool {
        !self.disabled
            && !self.intentionally_skipped()
            && (self.external_invalid || (self.attempted && !self.valid()))
    }

    fn has_input_answer(&self) -> bool {
        self.enabled_answers()
            .any(|answer| matches!(answer.kind, AnswerKind::Input { .. }))
    }

    fn is_filled(&self, answer: &Answer) -> bool {
        self.selected.contains(&answer.id)
            && match answer.kind {
                AnswerKind::Choice { .. } => true,
                AnswerKind::Input { has_value, .. } => has_value,
            }
    }

    fn is_radio(&self, answer: &Answer) -> bool {
        !self.multiple && matches!(answer.kind, AnswerKind::Choice { .. })
    }

    fn update_selected(&mut self, id: usize, selected: bool) {
        if !selected {
            self.selected.retain(|current| *current != id);
        } else if !self.multiple {
            self.selected = vec![id];
        } else if !self.selected.contains(&id) {
            self.selected.push(id);
        }
    }

    fn skip(&mut self) {
        if self.required {
            return;
        }
        self.selected.clear();
        self.skipped = true;
    }

    fn reset(&mut self) {
        self.attempted = false;
        self.skipped = false;
        self.selected = if self.multiple {
            self.defaults.clone()
        } else {
            self.defaults.first().copied().into_iter().collect()
        };
        for answer in &mut self.answers {
            if let AnswerKind::Input {
                has_value,
                default_filled,
                ..
            } = &mut answer.kind
            {
                *has_value = *default_filled;
            }
        }
        self.reset_version += 1;
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
struct RootProps {
    default_item: Option<String>,
    item: Option<String>,
    items: Option<Vec<QuestionnaireItemDefinition>>,
    shortcuts: Option<QuestionnaireShortcuts>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum FocusTarget {
    Item,
    Invalid,
}

#[derive(Clone, Debug, PartialEq)]
struct FocusRequest {
    name: String,
    target: FocusTarget,
}

/// The element a keydown started on, recorded by the part that owns it while the event bubbles
/// up to the form.
#[derive(Clone, Debug, PartialEq)]
enum KeyTarget {
    Answer {
        item: String,
        id: usize,
        text_entry: bool,
        radio: bool,
    },
    Item(String),
}

#[derive(Clone, Debug, Default)]
struct RootView {
    logical: Vec<String>,
    active: Option<String>,
    index: Option<usize>,
    active_entry: Option<ItemEntry>,
    active_required: Option<bool>,
    active_status: Option<QuestionnaireItemStatus>,
    shortcuts: Option<QuestionnaireShortcuts>,
    items: Option<Vec<QuestionnaireItemDefinition>>,
}

impl RootView {
    fn total(&self) -> usize {
        self.logical.len()
    }

    fn first(&self) -> bool {
        self.total() > 0 && self.index == Some(0)
    }

    fn last(&self) -> bool {
        self.total() > 0 && self.index == Some(self.total() - 1)
    }

    fn progress(&self) -> QuestionnaireProgressState {
        QuestionnaireProgressState {
            current: self.index.map_or(0, |index| index + 1),
            total: self.total(),
            first: self.first(),
            last: self.last(),
        }
    }

    fn next_name(&self) -> Option<String> {
        self.index
            .and_then(|index| self.logical.get(index + 1))
            .cloned()
    }
}

fn initial_item_name(
    items: Option<&[QuestionnaireItemDefinition]>,
    default_item: Option<&str>,
) -> Option<String> {
    let Some(items) = items else {
        return default_item.map(str::to_string);
    };
    if let Some(default_item) = default_item
        && items
            .iter()
            .any(|item| item.name == default_item && !item.disabled)
    {
        return Some(default_item.to_string());
    }
    items
        .iter()
        .find(|item| !item.disabled)
        .map(|item| item.name.clone())
}

fn shortcut_by_choice_value(
    definition: Option<&QuestionnaireItemDefinition>,
    shortcuts: QuestionnaireShortcuts,
) -> Vec<(String, String)> {
    let Some(definition) = definition else {
        return Vec::new();
    };
    definition
        .choices
        .iter()
        .filter(|choice| !choice.disabled)
        .zip(shortcuts.keys())
        .map(|(choice, key)| (choice.value.clone(), key))
        .collect()
}

fn shortcut_by_answer_id(
    entry: &ItemEntry,
    shortcuts: QuestionnaireShortcuts,
) -> Vec<(usize, String)> {
    entry
        .enabled_answers()
        .filter(|answer| matches!(answer.kind, AnswerKind::Choice { .. }))
        .zip(shortcuts.keys())
        .map(|(answer, key)| (answer.id, key))
        .collect()
}

fn choice_shortcut(view: &RootView, entry: &ItemEntry, value: &str, id: usize) -> Option<String> {
    let shortcuts = view.shortcuts?;
    match &view.items {
        Some(items) => {
            shortcut_by_choice_value(items.iter().find(|item| item.name == entry.name), shortcuts)
                .into_iter()
                .find(|(choice, _)| choice == value)
                .map(|(_, key)| key)
        }
        None => shortcut_by_answer_id(entry, shortcuts)
            .into_iter()
            .find(|(answer, _)| *answer == id)
            .map(|(_, key)| key),
    }
}

fn answer_key_shortcuts(shortcut: Option<&str>, filled: bool) -> Option<String> {
    let keys: Vec<&str> = [shortcut, filled.then_some("Enter")]
        .into_iter()
        .flatten()
        .collect();
    (!keys.is_empty()).then(|| keys.join(" "))
}

fn is_unset(children: &Element) -> bool {
    matches!(children, Ok(node) if *node == VNode::placeholder())
}

fn take_text_attribute(attributes: &mut Vec<Attribute>, name: &str) -> Option<String> {
    let mut found = None;
    attributes.retain(|attribute| {
        if attribute.name != name || attribute.namespace.is_some() {
            return true;
        }
        if let AttributeValue::Text(text) = &attribute.value {
            found = Some(text.clone());
        }
        false
    });
    found
}

fn render_attributes(
    slot: &'static str,
    class: String,
    attributes: Vec<Attribute>,
) -> Vec<Attribute> {
    let mut all = vec![
        Attribute::new("data-slot", slot, None, false),
        Attribute::new("class", class, None, false),
    ];
    all.extend(attributes);
    all
}

fn focus_element(dom_id: &str) {
    document::eval(&format!(
        "requestAnimationFrame(() => document.getElementById({dom_id:?})?.focus())"
    ));
}

#[derive(Clone, Copy)]
struct RootCtx {
    form_id: CopyValue<String>,
    props: Signal<RootProps>,
    uncontrolled: Signal<Option<String>>,
    store: Signal<Vec<ItemEntry>>,
    focus_request: Signal<Option<FocusRequest>>,
    submit_request: Signal<bool>,
    key_target: CopyValue<Option<KeyTarget>>,
    on_item_change: CopyValue<Option<EventHandler<String>>>,
}

impl RootCtx {
    fn is_controlled(&self) -> bool {
        self.props.read().item.is_some()
    }

    fn view(&self) -> RootView {
        let props = self.props.read();
        let store = self.store.read();
        let logical: Vec<String> = match &props.items {
            Some(items) => items
                .iter()
                .filter(|item| !item.disabled)
                .map(|item| item.name.clone())
                .collect(),
            None => {
                let mut mounted: Vec<&ItemEntry> = store
                    .iter()
                    .filter(|entry| entry.mounted && !entry.disabled)
                    .collect();
                mounted.sort_by_key(|entry| entry.seq);
                mounted.iter().map(|entry| entry.name.clone()).collect()
            }
        };
        let active = props
            .item
            .clone()
            .or_else(|| self.uncontrolled.read().clone());
        let index = active
            .as_ref()
            .and_then(|active| logical.iter().position(|name| name == active));
        let active_entry = index
            .and(active.as_ref())
            .and_then(|active| {
                store
                    .iter()
                    .find(|entry| entry.mounted && !entry.disabled && entry.name == *active)
            })
            .cloned();
        let definition = active.as_ref().and_then(|active| {
            props
                .items
                .as_ref()
                .and_then(|items| items.iter().find(|item| item.name == *active))
        });
        let active_required = index.map(|_| match definition {
            Some(definition) => definition.required,
            None => active_entry.as_ref().is_some_and(|entry| entry.required),
        });
        let active_status = index.map(|_| {
            active_entry
                .as_ref()
                .map(ItemEntry::status)
                .unwrap_or_default()
        });
        RootView {
            logical,
            active,
            index,
            active_entry,
            active_required,
            active_status,
            shortcuts: props.shortcuts,
            items: props.items.clone(),
        }
    }

    fn entry(&self, name: &str) -> ItemEntry {
        self.store
            .read()
            .iter()
            .find(|entry| entry.name == name)
            .cloned()
            .unwrap_or_default()
    }

    fn with_entry<R>(&self, name: &str, update: impl FnOnce(&mut ItemEntry) -> R) -> Option<R> {
        let mut store = self.store;
        let mut store = store.write();
        store
            .iter_mut()
            .find(|entry| entry.name == name)
            .map(update)
    }

    fn set_item(&self, next: String, target: FocusTarget) {
        let active = self
            .props
            .peek()
            .item
            .clone()
            .or_else(|| self.uncontrolled.peek().clone());
        if active.as_deref() == Some(next.as_str()) {
            return;
        }
        let mut focus_request = self.focus_request;
        focus_request.set(Some(FocusRequest {
            name: next.clone(),
            target,
        }));
        if self.props.peek().item.is_none() {
            let mut uncontrolled = self.uncontrolled;
            uncontrolled.set(Some(next.clone()));
        }
        let handler = *self.on_item_change.peek();
        if let Some(handler) = handler {
            handler.call(next);
        }
    }

    fn validate(&self, name: &str) -> bool {
        self.with_entry(name, |entry| {
            entry.attempted = true;
            entry.valid()
        })
        .unwrap_or(true)
    }

    fn item_dom_id(&self, name: &str) -> Option<String> {
        self.store
            .peek()
            .iter()
            .find(|entry| entry.name == name && entry.mounted)
            .map(|entry| entry.dom_id.clone())
    }

    fn focus_item(&self, name: &str) {
        if let Some(dom_id) = self.item_dom_id(name) {
            focus_element(&dom_id);
        }
    }

    fn focus_invalid(&self, name: &str) {
        if let Some(dom_id) = self.item_dom_id(name) {
            document::eval(&format!(
                r#"requestAnimationFrame(() => {{
                    const item = document.getElementById({dom_id:?});
                    if (!item) return;
                    const target =
                        item.querySelector("input[data-filled][name]:not(:disabled)") ??
                        item.querySelector("input:not([type=hidden]):not(:disabled), textarea:not(:disabled)") ??
                        item;
                    target.focus();
                }})"#
            ));
        }
    }

    fn focus_answer(&self, answer: &Answer, click: bool) {
        document::eval(&format!(
            "const answer = document.getElementById({:?}); if (answer) {{ answer.focus(); if ({click}) answer.click(); }}",
            answer.dom_id
        ));
    }

    fn request_submit(&self) {
        let mut submit_request = self.submit_request;
        submit_request.set(true);
    }

    fn go_previous(&self) {
        let view = self.view();
        if let Some(index) = view.index
            && index > 0
        {
            self.set_item(view.logical[index - 1].clone(), FocusTarget::Item);
        }
    }

    fn go_next(&self) {
        let view = self.view();
        let (Some(entry), Some(next)) = (view.active_entry.as_ref(), view.next_name()) else {
            return;
        };
        if !self.validate(&entry.name) {
            self.focus_invalid(&entry.name);
            return;
        }
        self.set_item(next, FocusTarget::Item);
    }

    fn confirm_current(&self) {
        let view = self.view();
        let Some(entry) = view.active_entry.as_ref() else {
            return;
        };
        if !self.validate(&entry.name) {
            self.focus_invalid(&entry.name);
            return;
        }
        if view.last() {
            self.request_submit();
            return;
        }
        if let Some(next) = view.next_name() {
            self.set_item(next, FocusTarget::Item);
        }
    }

    fn skip_current(&self) {
        let view = self.view();
        let Some(entry) = view.active_entry.as_ref() else {
            return;
        };
        if entry.required {
            return;
        }
        self.with_entry(&entry.name, ItemEntry::skip);
        match view.next_name() {
            Some(next) if !view.last() => self.set_item(next, FocusTarget::Item),
            _ => self.request_submit(),
        }
    }

    fn mark_key_target(&self, target: KeyTarget) {
        let mut key_target = self.key_target;
        if key_target.peek().is_none() {
            key_target.set(Some(target));
        }
    }

    fn take_key_target(&self) -> Option<KeyTarget> {
        let mut key_target = self.key_target;
        key_target.write().take()
    }

    fn move_answer_focus(&self, entry: &ItemEntry, target: Option<&KeyTarget>, down: bool) -> bool {
        let answers: Vec<&Answer> = entry.enabled_answers().collect();
        if answers.is_empty() {
            return false;
        }
        let (current_index, text_entry, radio_target) = match target {
            Some(KeyTarget::Answer {
                item,
                id,
                text_entry,
                radio,
            }) if *item == entry.name => (
                answers.iter().position(|answer| answer.id == *id),
                *text_entry,
                *radio,
            ),
            Some(KeyTarget::Answer {
                text_entry, radio, ..
            }) => (None, *text_entry, *radio),
            _ => (None, false, false),
        };
        let current = current_index.map(|index| answers[index]);
        let empty_navigable_input = matches!(
            current.map(|answer| &answer.kind),
            Some(AnswerKind::Input {
                navigable: true,
                has_value: false,
                ..
            })
        );
        let on_item = matches!(target, Some(KeyTarget::Item(name)) if *name == entry.name);
        if (text_entry && !empty_navigable_input) || (current_index.is_none() && !on_item) {
            return false;
        }
        let count = answers.len();
        let next = match current_index {
            None => answers
                .iter()
                .copied()
                .find(|answer| entry.is_filled(answer))
                .unwrap_or(if down { answers[0] } else { answers[count - 1] }),
            Some(index) => answers[(index + if down { 1 } else { count - 1 }) % count],
        };
        if current.is_some_and(|current| current.id == next.id) {
            return false;
        }
        let next_radio = entry.is_radio(next);
        if current_index.is_some() && radio_target && next_radio {
            return false;
        }
        self.focus_answer(next, next_radio);
        true
    }

    fn answer_by_shortcut(
        &self,
        view: &RootView,
        entry: &ItemEntry,
        shortcut: &str,
    ) -> Option<Answer> {
        let shortcuts = view.shortcuts?;
        match &view.items {
            Some(items) => {
                let value = shortcut_by_choice_value(
                    items.iter().find(|item| item.name == entry.name),
                    shortcuts,
                )
                .into_iter()
                .find(|(_, key)| key == shortcut)
                .map(|(value, _)| value)?;
                entry
                    .enabled_answers()
                    .find(|answer| {
                        matches!(&answer.kind, AnswerKind::Choice { value: choice } if *choice == value)
                    })
                    .cloned()
            }
            None => {
                let id = shortcut_by_answer_id(entry, shortcuts)
                    .into_iter()
                    .find(|(_, key)| key == shortcut)
                    .map(|(id, _)| id)?;
                entry
                    .enabled_answers()
                    .find(|answer| answer.id == id)
                    .cloned()
            }
        }
    }

    fn handle_keydown(&self, event: &KeyboardEvent, target: Option<KeyTarget>) {
        if !event.default_action_enabled() || event.is_composing() {
            return;
        }
        let view = self.view();
        let Some(entry) = view.active_entry.clone() else {
            return;
        };
        let modifiers = event.modifiers();
        let meta_or_ctrl =
            modifiers.contains(Modifiers::META) || modifiers.contains(Modifiers::CONTROL);
        let alt = modifiers.contains(Modifiers::ALT);
        let repeat = event.is_auto_repeating();
        let key = event.key();

        if key == Key::Enter && meta_or_ctrl && !alt && !modifiers.contains(Modifiers::SHIFT) {
            event.prevent_default();
            if !repeat {
                self.confirm_current();
            }
            return;
        }
        if meta_or_ctrl || alt {
            return;
        }

        let (text_entry, radio_target) = match &target {
            Some(KeyTarget::Answer {
                text_entry, radio, ..
            }) => (*text_entry, *radio),
            _ => (false, false),
        };

        if (key == Key::ArrowUp || key == Key::ArrowDown)
            && self.move_answer_focus(&entry, target.as_ref(), key == Key::ArrowDown)
        {
            event.prevent_default();
            return;
        }

        if (key == Key::ArrowLeft || key == Key::ArrowRight) && !text_entry && !radio_target {
            event.prevent_default();
            if repeat {
                return;
            }
            if key == Key::ArrowLeft {
                self.go_previous();
            } else if entry.status() != QuestionnaireItemStatus::Unanswered {
                self.go_next();
            }
            return;
        }

        if key == Key::Enter {
            let answer = match &target {
                Some(KeyTarget::Answer { item, id, .. }) if *item == entry.name => {
                    entry.enabled_answers().find(|answer| answer.id == *id)
                }
                _ => None,
            };
            let Some(answer) = answer else {
                return;
            };
            event.prevent_default();
            if !repeat && entry.is_filled(answer) {
                self.confirm_current();
            }
            return;
        }

        let Some(shortcuts) = view.shortcuts else {
            return;
        };
        if text_entry {
            return;
        }
        let Key::Character(character) = &key else {
            return;
        };
        let Some(shortcut) = shortcuts.shortcut_for_key(character) else {
            return;
        };
        let Some(answer) = self.answer_by_shortcut(&view, &entry, &shortcut) else {
            return;
        };
        event.prevent_default();
        if repeat {
            return;
        }
        self.focus_answer(&answer, matches!(answer.kind, AnswerKind::Choice { .. }));
    }

    fn handle_submit(&self, event: &FormEvent) -> bool {
        let view = self.view();
        let mounted: Vec<String> = {
            let store = self.store.peek();
            view.logical
                .iter()
                .filter(|name| {
                    store
                        .iter()
                        .any(|entry| entry.mounted && !entry.disabled && entry.name == **name)
                })
                .cloned()
                .collect()
        };
        let Some(first_invalid) = mounted.into_iter().find(|name| !self.validate(name)) else {
            return true;
        };
        event.prevent_default();
        let is_active = view.active.as_deref() == Some(first_invalid.as_str());
        self.set_item(first_invalid.clone(), FocusTarget::Invalid);
        if is_active {
            self.focus_invalid(&first_invalid);
            let mut focus_request = self.focus_request;
            focus_request.set(None);
        }
        false
    }

    fn handle_reset(&self) {
        let mut store = self.store;
        store.write().iter_mut().for_each(ItemEntry::reset);
        let props = self.props.peek().clone();
        let reset_name = match props.items.as_deref() {
            Some(items) => initial_item_name(Some(items), props.default_item.as_deref()),
            None => {
                let store = self.store.peek();
                let mut mounted: Vec<&ItemEntry> = store
                    .iter()
                    .filter(|entry| entry.mounted && !entry.disabled)
                    .collect();
                mounted.sort_by_key(|entry| entry.seq);
                mounted
                    .iter()
                    .find(|entry| Some(entry.name.as_str()) == props.default_item.as_deref())
                    .or(mounted.first())
                    .map(|entry| entry.name.clone())
            }
        };
        if let Some(name) = reset_name {
            self.set_item(name, FocusTarget::Item);
        }
    }

    fn sync_item(&self, seq: usize, name: &str, dom_id: &str, item: ItemProps) {
        let up_to_date = self.store.peek().iter().any(|entry| {
            entry.name == name
                && entry.mounted
                && entry.seq == seq
                && entry.dom_id == dom_id
                && entry.required == item.required
                && entry.multiple == item.multiple
                && entry.disabled == item.disabled
                && entry.external_invalid == item.invalid
        });
        if up_to_date {
            return;
        }
        let mut store = self.store;
        let mut store = store.write();
        let index = match store.iter().position(|entry| entry.name == name) {
            // Another item already uses this name; the first one keeps it.
            Some(index) if store[index].mounted && store[index].seq != seq => return,
            Some(index) => index,
            None => {
                store.push(ItemEntry {
                    name: name.to_string(),
                    ..ItemEntry::default()
                });
                store.len() - 1
            }
        };
        let entry = &mut store[index];
        if entry.multiple && !item.multiple {
            let keep = entry
                .enabled_answers()
                .find(|answer| entry.selected.contains(&answer.id))
                .map(|answer| answer.id);
            entry.selected = keep.into_iter().collect();
        }
        entry.mounted = true;
        entry.seq = seq;
        entry.dom_id = dom_id.to_string();
        entry.required = item.required;
        entry.multiple = item.multiple;
        entry.disabled = item.disabled;
        entry.external_invalid = item.invalid;
    }

    fn remove_item(&self, seq: usize) {
        let mut store = self.store;
        if let Ok(mut store) = store.try_write() {
            store.retain(|entry| !(entry.mounted && entry.seq == seq));
        }
    }

    fn rename_item(&self, from: &str, to: &str) {
        self.with_entry(from, |entry| entry.name = to.to_string());
    }

    fn sync_answer(
        &self,
        item: &str,
        id: usize,
        build: impl FnOnce(Option<&Answer>) -> Answer,
        default_selected: bool,
    ) {
        let (candidate, unchanged) = {
            let store = self.store.peek();
            let entry = store.iter().find(|entry| entry.name == item);
            let existing = entry.and_then(|entry| entry.answers.iter().find(|a| a.id == id));
            let candidate = build(existing);
            let unchanged = existing == Some(&candidate)
                && entry.is_some_and(|entry| entry.defaults.contains(&id) == default_selected);
            (candidate, unchanged)
        };
        if unchanged {
            return;
        }
        let mut store = self.store;
        let mut store = store.write();
        let index = match store.iter().position(|entry| entry.name == item) {
            Some(index) => index,
            None => {
                store.push(ItemEntry {
                    name: item.to_string(),
                    ..ItemEntry::default()
                });
                store.len() - 1
            }
        };
        let entry = &mut store[index];
        match entry.answers.iter().position(|answer| answer.id == id) {
            Some(position) => entry.answers[position] = candidate,
            None => {
                entry.answers.push(candidate);
                entry.answers.sort_by_key(|answer| answer.id);
                if default_selected {
                    if !entry.multiple {
                        if entry.selected.is_empty() {
                            entry.selected = vec![id];
                        }
                    } else if !entry.selected.contains(&id) {
                        entry.selected.push(id);
                    }
                }
            }
        }
        if default_selected {
            if !entry.defaults.contains(&id) {
                entry.defaults.push(id);
            }
        } else {
            entry.defaults.retain(|current| *current != id);
        }
    }

    fn remove_answer(&self, item: &str, id: usize) {
        let mut store = self.store;
        if let Ok(mut store) = store.try_write()
            && let Some(entry) = store.iter_mut().find(|entry| entry.name == item)
        {
            entry.answers.retain(|answer| answer.id != id);
            entry.defaults.retain(|current| *current != id);
            entry.selected.retain(|current| *current != id);
        }
    }

    fn select_from_interaction(&self, item: &str, id: usize, selected: bool) {
        self.with_entry(item, |entry| {
            entry.skipped = false;
            entry.update_selected(id, selected);
        });
    }

    fn sync_controlled_selection(&self, item: &str, id: usize, selected: bool) {
        self.with_entry(item, |entry| {
            if selected {
                entry.skipped = false;
            }
            entry.update_selected(id, selected);
        });
    }

    fn set_input_filled(&self, item: &str, id: usize, filled: bool) {
        self.with_entry(item, |entry| {
            if let Some(AnswerKind::Input { has_value, .. }) = entry
                .answers
                .iter_mut()
                .find(|answer| answer.id == id)
                .map(|answer| &mut answer.kind)
            {
                *has_value = filled;
            }
            entry.skipped = false;
            entry.update_selected(id, filled);
        });
    }

    fn reset_version(&self, item: &str) -> usize {
        self.store
            .peek()
            .iter()
            .find(|entry| entry.name == item)
            .map_or(0, |entry| entry.reset_version)
    }
}

#[derive(Clone, Copy, PartialEq)]
struct ItemProps {
    required: bool,
    multiple: bool,
    disabled: bool,
    invalid: bool,
}

#[derive(Clone, Copy)]
struct ItemCtx {
    name: Signal<String>,
    descriptions: Signal<Vec<String>>,
    errors: Signal<Vec<String>>,
}

fn use_root() -> RootCtx {
    use_context::<RootCtx>()
}

fn use_item() -> ItemCtx {
    use_context::<ItemCtx>()
}

/// Keeps `id` in `list` while the calling part is mounted.
fn use_registered_id(list: Signal<Vec<String>>, id: String) {
    let mut registered = use_hook(|| CopyValue::new(None::<String>));
    if registered.peek().as_deref() != Some(id.as_str()) {
        let previous = registered.peek().clone();
        let mut list = list;
        list.with_mut(|ids| {
            ids.retain(|current| Some(current) != previous.as_ref());
            if !ids.contains(&id) {
                ids.push(id.clone());
            }
        });
        registered.set(Some(id));
    }
    use_drop(move || {
        let mut list = list;
        if let (Ok(registered), Ok(mut ids)) = (registered.try_read(), list.try_write()) {
            ids.retain(|current| Some(current) != registered.as_ref());
        }
    });
}

/// The root `<form>`. Pass `item` with `on_item_change` to control the active item by name, or
/// `default_item` to choose the first one.
#[component]
pub fn Questionnaire(
    default_item: Option<String>,
    item: Option<String>,
    /// Every item in order. Lets progress, actions and shortcuts be complete before items render.
    items: Option<Vec<QuestionnaireItemDefinition>>,
    on_item_change: Option<EventHandler<String>>,
    shortcuts: Option<QuestionnaireShortcuts>,
    /// Runs when every item is valid. Call `event.prevent_default()` to stay on the page.
    onsubmit: Option<EventHandler<FormEvent>>,
    onreset: Option<EventHandler<FormEvent>>,
    onkeydown: Option<EventHandler<KeyboardEvent>>,
    #[props(extends = GlobalAttributes, extends = form)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(
        "gap-4 flex w-full min-w-0 flex-col",
        take_class(&mut attributes).as_deref()
    );
    let root_props = RootProps {
        default_item,
        item,
        items,
        shortcuts,
    };
    let form_id = use_hook(|| CopyValue::new(format!("questionnaire-{}", next_id())));
    let mut props = use_signal(|| root_props.clone());
    if *props.peek() != root_props {
        props.set(root_props.clone());
    }
    let uncontrolled = use_signal(|| {
        initial_item_name(
            root_props.items.as_deref(),
            root_props.default_item.as_deref(),
        )
    });
    let store = use_signal(Vec::<ItemEntry>::new);
    let focus_request = use_signal(|| None::<FocusRequest>);
    let submit_request = use_signal(|| false);
    let key_target = use_hook(|| CopyValue::new(None::<KeyTarget>));
    let mut on_item_change_cell = use_hook(|| CopyValue::new(on_item_change));
    on_item_change_cell.set(on_item_change);

    let ctx = use_context_provider(|| RootCtx {
        form_id,
        props,
        uncontrolled,
        store,
        focus_request,
        submit_request,
        key_target,
        on_item_change: on_item_change_cell,
    });

    let initial_active = root_props
        .item
        .clone()
        .or_else(|| uncontrolled.peek().clone());
    let mut previous_active = use_hook(|| CopyValue::new(initial_active));
    use_effect(move || {
        let view = ctx.view();
        let pending = (ctx.focus_request)();
        let Some(first) = view.logical.first().cloned() else {
            return;
        };
        let Some(active) = view.index.and(view.active.clone()) else {
            if !ctx.is_controlled() && view.active.is_none() {
                let mut uncontrolled = ctx.uncontrolled;
                uncontrolled.set(Some(first));
            } else {
                ctx.set_item(first, FocusTarget::Item);
            }
            return;
        };
        let changed = previous_active.peek().as_deref() != Some(active.as_str());
        previous_active.set(Some(active.clone()));
        let mut focus_request = ctx.focus_request;
        match pending {
            Some(request) if request.name == active => {
                focus_request.set(None);
                match request.target {
                    FocusTarget::Invalid => ctx.focus_invalid(&active),
                    FocusTarget::Item => ctx.focus_item(&active),
                }
            }
            _ => {
                if ctx.is_controlled() && changed {
                    focus_request.set(None);
                    ctx.focus_item(&active);
                }
            }
        }
    });
    use_effect(move || {
        if (ctx.submit_request)() {
            let mut submit_request = ctx.submit_request;
            submit_request.set(false);
            document::eval(&format!(
                "requestAnimationFrame(() => document.getElementById({:?})?.requestSubmit())",
                ctx.form_id.peek()
            ));
        }
    });

    let state = ctx.view().progress();
    let id = form_id.peek().clone();

    rsx! {
        form {
            id: "{id}",
            "data-slot": "questionnaire",
            class: class,
            novalidate: true,
            "data-shortcuts": shortcuts.map(QuestionnaireShortcuts::as_str),
            "data-current": "{state.current}",
            "data-total": "{state.total}",
            "data-first": flag(state.first),
            "data-last": flag(state.last),
            onkeydown: move |event: KeyboardEvent| {
                let target = ctx.take_key_target();
                if let Some(handler) = onkeydown {
                    handler.call(event.clone());
                }
                ctx.handle_keydown(&event, target);
            },
            onsubmit: move |event: FormEvent| {
                if ctx.handle_submit(&event)
                    && let Some(handler) = onsubmit
                {
                    handler.call(event);
                }
            },
            onreset: move |event: FormEvent| {
                if let Some(handler) = onreset {
                    handler.call(event.clone());
                }
                if event.default_action_enabled() {
                    ctx.handle_reset();
                }
            },
            ..attributes,
            {children}
        }
    }
}

/// Shows "Question n of N" as a progressbar. Pass `render` to draw the content from the state.
#[component]
pub fn QuestionnaireProgress(
    render: Option<Callback<QuestionnaireProgressState, Element>>,
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let root = use_root();
    let mut attributes = attributes;
    let class = cn!(
        "text-[0.625rem] min-h-[1lh] w-fit min-w-[14ch] font-medium text-muted-foreground tabular-nums",
        take_class(&mut attributes).as_deref()
    );
    let state = root.view().progress();
    let has_items = state.total > 0;
    let label = has_items.then(|| format!("Question {} of {}", state.current, state.total));
    let content = match render {
        Some(render) => render.call(state),
        None if !is_unset(&children) => children,
        None => rsx! {
            if let Some(label) = label.clone() {
                "{label}"
            }
        },
    };

    rsx! {
        div {
            "data-slot": "questionnaire-progress",
            class: class,
            role: "progressbar",
            "aria-label": "Questionnaire progress",
            "aria-live": "polite",
            "aria-valuemax": has_items.then(|| state.total.to_string()),
            "aria-valuemin": has_items.then_some("1"),
            "aria-valuenow": has_items.then(|| state.current.to_string()),
            "aria-valuetext": label,
            "data-current": "{state.current}",
            "data-total": "{state.total}",
            "data-first": flag(state.first),
            "data-last": flag(state.last),
            ..attributes,
            {content}
        }
    }
}

/// One question, keyed by `name`. Only the active item is shown; the others are hidden and inert.
#[component]
pub fn QuestionnaireItem(
    name: String,
    #[props(default)] required: bool,
    /// Answers are checkboxes and the form value holds every checked one.
    #[props(default)]
    multiple: bool,
    /// Marks the item invalid from outside, for example after a schema check.
    #[props(default)]
    invalid: bool,
    #[props(default)] disabled: bool,
    on_status_change: Option<EventHandler<QuestionnaireItemStatus>>,
    #[props(extends = GlobalAttributes, extends = fieldset)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let root = use_root();
    let seq = use_hook(next_id);
    let dom_id = use_hook(|| format!("questionnaire-item-{seq}"));
    let mut name_signal = use_signal(|| name.clone());
    let descriptions = use_signal(Vec::<String>::new);
    let errors = use_signal(Vec::<String>::new);
    use_context_provider(|| ItemCtx {
        name: name_signal,
        descriptions,
        errors,
    });

    if *name_signal.peek() != name {
        let previous = name_signal.peek().clone();
        root.rename_item(&previous, &name);
        name_signal.set(name.clone());
    }
    root.sync_item(
        seq,
        &name,
        &dom_id,
        ItemProps {
            required,
            multiple,
            disabled,
            invalid,
        },
    );
    use_drop(move || root.remove_item(seq));

    let mut previous_status = use_hook(|| CopyValue::new(QuestionnaireItemStatus::Unanswered));
    use_effect(move || {
        let status = root.entry(&name_signal()).status();
        if *previous_status.peek() != status {
            previous_status.set(status);
            if let Some(handler) = on_status_change {
                handler.call(status);
            }
        }
    });

    let view = root.view();
    let entry = root.entry(&name);
    let active = !disabled && view.active.as_deref() == Some(name.as_str());
    let status = entry.status();
    let item_invalid = entry.invalid();
    let has_answers = entry.enabled_answers().next().is_some();

    let mut attributes = attributes;
    let class = cn!(
        "flex flex-col gap-3 min-w-0 border-0 p-0 outline-none",
        take_class(&mut attributes).as_deref()
    );
    let own_described_by = take_text_attribute(&mut attributes, "aria-describedby");
    let own_key_shortcuts = take_text_attribute(&mut attributes, "aria-keyshortcuts");
    let mut described_by = descriptions.read().clone();
    if item_invalid {
        described_by.extend(errors.read().iter().cloned());
    }
    described_by.extend(own_described_by);
    let described_by = (!described_by.is_empty()).then(|| described_by.join(" "));
    let key_shortcuts: Vec<&str> = [
        own_key_shortcuts.as_deref(),
        active.then_some("Meta+Enter Control+Enter"),
        (active && has_answers).then_some("ArrowUp ArrowDown"),
        (active && !view.first()).then_some("ArrowLeft"),
        (active && !view.last() && status != QuestionnaireItemStatus::Unanswered)
            .then_some("ArrowRight"),
    ]
    .into_iter()
    .flatten()
    .collect();
    let key_shortcuts = (!key_shortcuts.is_empty()).then(|| key_shortcuts.join(" "));
    let key_name = name.clone();

    rsx! {
        fieldset {
            id: "{dom_id}",
            "data-slot": "questionnaire-item",
            class: class,
            tabindex: "-1",
            disabled: disabled,
            hidden: !active,
            inert: (!active).then_some(true),
            "aria-describedby": described_by,
            "aria-invalid": item_invalid.then_some("true"),
            "aria-keyshortcuts": key_shortcuts,
            "data-active": flag(active),
            "data-disabled": flag(disabled),
            "data-invalid": flag(item_invalid),
            "data-multiple": flag(multiple),
            "data-required": flag(required),
            "data-status": status.as_str(),
            onkeydown: move |_| root.mark_key_target(KeyTarget::Item(key_name.clone())),
            ..attributes,
            {children}
        }
    }
}

/// The question, rendered as the item's `<legend>`. Pass `render` to use another element.
#[component]
pub fn QuestionnaireTitle(
    render: Option<Callback<QuestionnaireRenderProps, Element>>,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    use_item();
    let mut attributes = attributes;
    let class = cn!(
        "text-sm font-semibold [&:not(:has(~[data-slot=questionnaire-description]))]:mb-3 font-heading text-pretty",
        take_class(&mut attributes).as_deref()
    );
    if let Some(render) = render {
        return render.call(QuestionnaireRenderProps {
            attributes: render_attributes("questionnaire-title", class, attributes),
            children,
        });
    }

    rsx! {
        legend {
            "data-slot": "questionnaire-title",
            class: class,
            ..attributes,
            {children}
        }
    }
}

/// Extra context for the question. The item is described by it.
#[component]
pub fn QuestionnaireDescription(
    id: Option<String>,
    render: Option<Callback<QuestionnaireRenderProps, Element>>,
    #[props(extends = GlobalAttributes, extends = p)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let item = use_item();
    let generated = use_hook(|| format!("questionnaire-description-{}", next_id()));
    let id = id.unwrap_or(generated);
    use_registered_id(item.descriptions, id.clone());
    let mut attributes = attributes;
    let class = cn!(
        "text-xs/relaxed text-pretty text-muted-foreground",
        take_class(&mut attributes).as_deref()
    );
    if let Some(render) = render {
        let mut attributes = attributes;
        attributes.insert(0, Attribute::new("id", id, None, false));
        return render.call(QuestionnaireRenderProps {
            attributes: render_attributes("questionnaire-description", class, attributes),
            children,
        });
    }

    rsx! {
        p {
            id: "{id}",
            "data-slot": "questionnaire-description",
            class: class,
            ..attributes,
            {children}
        }
    }
}

/// Holds the choices and the freeform input of an item.
#[component]
pub fn QuestionnaireChoices(
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    use_item();
    let root = use_root();
    let shortcuts = root.props.read().shortcuts;
    let mut attributes = attributes;
    let class = cn!(
        "gap-1.5 group/questionnaire-choices grid min-w-0",
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        div {
            "data-slot": "questionnaire-choices",
            class: class,
            "data-shortcuts": shortcuts.map(QuestionnaireShortcuts::as_str),
            ..attributes,
            {children}
        }
    }
}

/// One fixed answer: a radio, or a checkbox when the item is `multiple`. Pass `checked` with
/// `onchange` to control it.
#[component]
pub fn QuestionnaireChoice(
    value: String,
    checked: Option<bool>,
    #[props(default)] default_checked: bool,
    #[props(default)] disabled: bool,
    onchange: Option<EventHandler<FormEvent>>,
    #[props(extends = GlobalAttributes, extends = label)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let root = use_root();
    let item = use_item();
    let id = use_hook(next_id);
    let dom_id = use_hook(|| format!("questionnaire-answer-{id}"));
    let item_name = item.name.read().clone();
    let mut registered_item = use_hook(|| CopyValue::new(item_name.clone()));
    if *registered_item.peek() != item_name {
        registered_item.set(item_name.clone());
    }

    root.sync_answer(
        &item_name,
        id,
        |_| Answer {
            id,
            dom_id: dom_id.clone(),
            own_disabled: disabled,
            kind: AnswerKind::Choice {
                value: value.clone(),
            },
        },
        checked.unwrap_or(default_checked),
    );
    let mut last_controlled = use_hook(|| CopyValue::new(None::<(bool, usize)>));
    if let Some(checked) = checked {
        let synced = (checked, root.reset_version(&item_name));
        if *last_controlled.peek() != Some(synced) {
            last_controlled.set(Some(synced));
            root.sync_controlled_selection(&item_name, id, checked);
        }
    }
    use_drop(move || {
        if let Ok(item_name) = registered_item.try_read() {
            root.remove_answer(&item_name, id);
        }
    });

    let view = root.view();
    let entry = root.entry(&item_name);
    let status = entry.status();
    let is_disabled = entry.disabled || disabled;
    let is_checked = match checked {
        Some(checked) => status != QuestionnaireItemStatus::Skipped && checked,
        None => entry.selected.contains(&id),
    };
    let multiple = entry.multiple;
    let input_type = if multiple { "checkbox" } else { "radio" };
    let shortcut = choice_shortcut(&view, &entry, &value, id);
    let invalid = entry.invalid();
    let required = entry.required && !multiple && !entry.has_input_answer();
    let name = (status != QuestionnaireItemStatus::Skipped).then(|| item_name.clone());
    let key_shortcuts = answer_key_shortcuts(shortcut.as_deref(), !is_disabled && is_checked);

    let mut attributes = attributes;
    let class = cn!(
        "border-input hover:bg-input/40 data-checked:border-primary/40 data-checked:bg-primary/10 data-invalid:border-destructive has-[>input:focus-visible]:border-ring has-[>input:focus-visible]:ring-ring/30 gap-2.5 rounded-xl border px-3 py-2.5 text-xs/relaxed has-[>input:focus-visible]:ring-2 group/questionnaire-choice relative flex min-h-11 cursor-pointer items-start text-start transition-colors outline-none select-none",
        "data-disabled:pointer-events-none data-disabled:cursor-not-allowed data-disabled:opacity-50",
        take_class(&mut attributes).as_deref()
    );
    let change_item = item_name.clone();
    let key_item = item_name.clone();

    rsx! {
        label {
            "data-slot": "questionnaire-choice",
            class: class,
            "data-checked": flag(is_checked),
            "data-unchecked": flag(!is_checked),
            "data-disabled": flag(is_disabled),
            "data-invalid": flag(invalid),
            "data-shortcut": shortcut.clone(),
            "data-type": input_type,
            ..attributes,
            input {
                "data-slot": "questionnaire-choice-input",
                class: "absolute inset-0 z-10 size-full cursor-pointer opacity-0",
                id: "{dom_id}",
                r#type: input_type,
                name: name,
                value: "{value}",
                checked: is_checked,
                initial_checked: checked.unwrap_or(default_checked),
                disabled: is_disabled,
                required: required,
                "aria-invalid": invalid.then_some("true"),
                "aria-keyshortcuts": key_shortcuts,
                "data-checked": flag(is_checked),
                "data-unchecked": flag(!is_checked),
                "data-disabled": flag(is_disabled),
                "data-invalid": flag(invalid),
                "data-shortcut": shortcut.clone(),
                "data-type": input_type,
                onchange: move |event: FormEvent| {
                    if let Some(handler) = onchange {
                        handler.call(event.clone());
                    }
                    if !event.default_action_enabled() {
                        return;
                    }
                    let now_checked = if multiple { event.checked() } else { true };
                    match checked {
                        None => root.select_from_interaction(&change_item, id, now_checked),
                        Some(checked) => {
                            if status == QuestionnaireItemStatus::Skipped && checked == now_checked {
                                root.select_from_interaction(&change_item, id, checked);
                            }
                        }
                    }
                },
                onkeydown: move |_| {
                    root.mark_key_target(KeyTarget::Answer {
                        item: key_item.clone(),
                        id,
                        text_entry: false,
                        radio: !multiple,
                    })
                },
            }
            span {
                "aria-hidden": "true",
                "data-slot": "questionnaire-choice-indicator",
                class: "border-input dark:bg-input/30 group-data-checked/questionnaire-choice:bg-primary dark:group-data-checked/questionnaire-choice:bg-primary group-data-checked/questionnaire-choice:text-primary-foreground group-data-checked/questionnaire-choice:border-primary size-4 translate-y-[--spacing(0.45)] group-has-data-[slot=questionnaire-choice-description]/questionnaire-choice:translate-y-0.5 rounded-[4px] pointer-events-none relative flex shrink-0 items-center justify-center border group-data-[type=radio]/questionnaire-choice:rounded-full",
                span {
                    "data-slot": "questionnaire-choice-indicator-dot",
                    class: "bg-primary-foreground size-2 hidden rounded-full group-data-[type=checkbox]/questionnaire-choice:hidden group-data-checked/questionnaire-choice:block",
                }
                svg {
                    "data-slot": "questionnaire-choice-indicator-check",
                    class: "size-3.5 hidden group-data-[type=radio]/questionnaire-choice:hidden group-data-checked/questionnaire-choice:block",
                    xmlns: "http://www.w3.org/2000/svg",
                    width: "24",
                    height: "24",
                    view_box: "0 0 24 24",
                    fill: "none",
                    stroke: "currentColor",
                    stroke_width: "2",
                    stroke_linecap: "round",
                    stroke_linejoin: "round",
                    path { d: "M20 6 9 17l-5-5" }
                }
            }
            span {
                "data-slot": "questionnaire-choice-label",
                class: "gap-0.5 flex min-w-0 flex-1 flex-col leading-snug",
                {children}
            }
            span {
                "data-slot": "questionnaire-choice-shortcut",
                class: "border-input bg-background/80 text-muted-foreground size-4 translate-y-[--spacing(0.45)] group-has-data-[slot=questionnaire-choice-description]/questionnaire-choice:translate-y-0.5 items-center justify-center rounded-sm border font-mono text-[0.5625rem] font-medium leading-none pointer-events-none ms-auto hidden shrink-0 group-data-[shortcut]/questionnaire-choice:inline-flex",
                "aria-hidden": "true",
                hidden: shortcut.is_none(),
                "data-shortcut": shortcut.clone(),
                if let Some(shortcut) = shortcut.clone() {
                    "{shortcut}"
                }
            }
        }
    }
}

/// Secondary text under a choice label.
#[component]
pub fn QuestionnaireChoiceDescription(
    #[props(extends = GlobalAttributes, extends = span)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(
        "text-muted-foreground",
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        span {
            "data-slot": "questionnaire-choice-description",
            class: class,
            ..attributes,
            {children}
        }
    }
}

/// A freeform answer. It counts as the item's answer while it holds text. Give it an accessible
/// name with `aria_label` or a visible label.
#[component]
pub fn QuestionnaireInput(
    #[props(default)] r#type: QuestionnaireInputType,
    /// Controlled value. When omitted, the input keeps its own value.
    value: Option<String>,
    default_value: Option<String>,
    #[props(default)] disabled: bool,
    oninput: Option<EventHandler<FormEvent>>,
    #[props(extends = GlobalAttributes, extends = input)] attributes: Vec<Attribute>,
) -> Element {
    let root = use_root();
    let item = use_item();
    let id = use_hook(next_id);
    let dom_id = use_hook(|| format!("questionnaire-answer-{id}"));
    let item_name = item.name.read().clone();
    let mut registered_item = use_hook(|| CopyValue::new(item_name.clone()));
    if *registered_item.peek() != item_name {
        registered_item.set(item_name.clone());
    }

    let has_text = |text: Option<&String>| text.is_some_and(|text| !text.trim().is_empty());
    let controlled_filled = value.as_ref().map(|value| has_text(Some(value)));
    let default_filled = has_text(default_value.as_ref());
    let initial_default_filled = use_hook(|| default_filled);
    root.sync_answer(
        &item_name,
        id,
        |existing| Answer {
            id,
            dom_id: dom_id.clone(),
            own_disabled: disabled,
            kind: AnswerKind::Input {
                navigable: r#type.navigable(),
                has_value: controlled_filled.unwrap_or_else(|| match existing.map(|a| &a.kind) {
                    Some(AnswerKind::Input { has_value, .. }) => *has_value,
                    _ => initial_default_filled,
                }),
                default_filled,
            },
        },
        default_filled,
    );
    let mut last_controlled = use_hook(|| CopyValue::new(None::<(bool, usize)>));
    if let Some(filled) = controlled_filled {
        let synced = (filled, root.reset_version(&item_name));
        if *last_controlled.peek() != Some(synced) {
            last_controlled.set(Some(synced));
            root.sync_controlled_selection(&item_name, id, filled);
        }
    }
    use_drop(move || {
        if let Ok(item_name) = registered_item.try_read() {
            root.remove_answer(&item_name, id);
        }
    });

    let entry = root.entry(&item_name);
    let is_disabled = entry.disabled || disabled;
    let filled = controlled_filled.unwrap_or_else(|| {
        entry
            .answers
            .iter()
            .find(|answer| answer.id == id)
            .is_some_and(|answer| {
                matches!(
                    answer.kind,
                    AnswerKind::Input {
                        has_value: true,
                        ..
                    }
                )
            })
    });
    let selected = entry.selected.contains(&id);
    let invalid = entry.invalid();
    let key_shortcuts = answer_key_shortcuts(None, !is_disabled && filled && selected);

    let mut attributes = attributes;
    let class = cn!(
        "bg-input/20 dark:bg-input/30 border-input focus-visible:border-ring focus-visible:ring-ring/30 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 h-7 rounded-md border px-2 py-0.5 text-sm focus-visible:ring-2 aria-invalid:ring-2 md:text-xs/relaxed min-h-11 w-full min-w-0 transition-[color,box-shadow,background-color] outline-none disabled:pointer-events-none disabled:cursor-not-allowed disabled:opacity-50 sm:min-h-0",
        "selection:bg-primary selection:text-primary-foreground placeholder:text-muted-foreground",
        take_class(&mut attributes).as_deref()
    );
    // `value` is only set when controlled: an unset `value` would clear the default value.
    if let Some(value) = &value {
        attributes.push(Attribute::new("value", value.clone(), None, true));
    }
    let initial_value = value.clone().or(default_value).unwrap_or_default();
    let controlled = value.is_some();
    let input_item = item_name.clone();
    let key_item = item_name.clone();

    rsx! {
        div {
            "data-slot": "questionnaire-input-wrapper",
            class: "w-full group/questionnaire-input relative min-w-0",
            input {
                "data-slot": "questionnaire-input",
                class: class,
                id: "{dom_id}",
                r#type: r#type.as_str(),
                name: selected.then(|| item_name.clone()),
                form: (!selected).then_some(""),
                disabled: is_disabled,
                initial_value: initial_value,
                "aria-invalid": invalid.then_some("true"),
                "aria-keyshortcuts": key_shortcuts,
                "data-disabled": flag(is_disabled),
                "data-filled": flag(filled),
                "data-empty": flag(!filled),
                "data-invalid": flag(invalid),
                oninput: move |event: FormEvent| {
                    if let Some(handler) = oninput {
                        handler.call(event.clone());
                    }
                    if !event.default_action_enabled() || controlled {
                        return;
                    }
                    let filled = !event.value().trim().is_empty();
                    root.set_input_filled(&input_item, id, filled);
                },
                onkeydown: move |_| {
                    root.mark_key_target(KeyTarget::Answer {
                        item: key_item.clone(),
                        id,
                        text_entry: true,
                        radio: false,
                    })
                },
                ..attributes,
            }
        }
    }
}

/// The item's error. Hidden until the item is invalid; shows a default message without children.
#[component]
pub fn QuestionnaireError(
    id: Option<String>,
    #[props(extends = GlobalAttributes, extends = p)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let root = use_root();
    let item = use_item();
    let generated = use_hook(|| format!("questionnaire-error-{}", next_id()));
    let id = id.unwrap_or(generated);
    use_registered_id(item.errors, id.clone());
    let entry = root.entry(&item.name.read());
    let invalid = entry.invalid();
    let content = if is_unset(&children) {
        let message = if entry.required {
            "Choose an answer to continue."
        } else {
            "Choose an answer or skip this question."
        };
        rsx! { "{message}" }
    } else {
        children
    };
    let mut attributes = attributes;
    let class = cn!(
        "mt-2 text-xs/relaxed text-destructive",
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        p {
            id: "{id}",
            "data-slot": "questionnaire-error",
            class: class,
            hidden: !invalid,
            role: invalid.then_some("alert"),
            "data-invalid": flag(invalid),
            ..attributes,
            {content}
        }
    }
}

/// Lays out Previous, Skip, Next and Submit in one row.
#[component]
pub fn QuestionnaireActions(
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(
        "gap-1.5 sm:min-h-7 grid min-h-11 w-full grid-cols-[minmax(0,1fr)_auto_auto] items-center",
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        div {
            "data-slot": "questionnaire-actions",
            class: class,
            ..attributes,
            {children}
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Navigation {
    Previous,
    Skip,
    Next,
    Submit,
}

fn variant_name(variant: ButtonVariant) -> &'static str {
    match variant {
        ButtonVariant::Default => "default",
        ButtonVariant::Outline => "outline",
        ButtonVariant::Secondary => "secondary",
        ButtonVariant::Ghost => "ghost",
        ButtonVariant::Destructive => "destructive",
        ButtonVariant::Link => "link",
    }
}

fn size_name(size: ButtonSize) -> &'static str {
    match size {
        ButtonSize::Default => "default",
        ButtonSize::Xs => "xs",
        ButtonSize::Sm => "sm",
        ButtonSize::Lg => "lg",
        ButtonSize::Icon => "icon",
        ButtonSize::IconXs => "icon-xs",
        ButtonSize::IconSm => "icon-sm",
        ButtonSize::IconLg => "icon-lg",
    }
}

fn navigation_button(
    kind: Navigation,
    variant: ButtonVariant,
    size: ButtonSize,
    disabled: bool,
    onclick: Option<EventHandler<MouseEvent>>,
    attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let root = use_root();
    let view = root.view();
    let (visible, slot, layout, label, button_type) = match kind {
        Navigation::Previous => (
            view.total() > 1 && !view.first(),
            "questionnaire-previous",
            "col-start-1 row-start-1 min-h-11 justify-self-start sm:min-h-0",
            "Previous",
            "button",
        ),
        Navigation::Skip => (
            view.active_required == Some(false),
            "questionnaire-skip",
            "col-start-2 row-start-1 min-h-11 justify-self-end sm:min-h-0",
            "Skip",
            "button",
        ),
        Navigation::Next => (
            view.total() > 1 && !view.last(),
            "questionnaire-next",
            "col-start-3 row-start-1 min-h-11 justify-self-end sm:min-h-0",
            "Next",
            "button",
        ),
        Navigation::Submit => (
            view.total() > 0 && view.last(),
            "questionnaire-submit",
            "col-start-3 row-start-1 min-h-11 justify-self-end sm:min-h-0",
            "Submit",
            "submit",
        ),
    };
    let shortcut = matches!(kind, Navigation::Next | Navigation::Submit).then_some("Enter");
    let active_shortcut = shortcut.filter(|_| visible && !disabled);
    let status = view.active_status.map(QuestionnaireItemStatus::as_str);
    let mut attributes = attributes;
    let extra = cn!(layout, take_class(&mut attributes).as_deref());
    let class = button_variants(variant, size, Some(extra.as_str()));
    let content = if is_unset(&children) {
        rsx! { "{label}" }
    } else {
        children
    };

    rsx! {
        button {
            "data-slot": slot,
            "data-size": size_name(size),
            "data-variant": variant_name(variant),
            class: class,
            r#type: button_type,
            disabled: disabled,
            hidden: !visible,
            inert: (!visible).then_some(true),
            tabindex: (!visible).then_some("-1"),
            "aria-hidden": (!visible).then_some("true"),
            "aria-keyshortcuts": active_shortcut,
            "data-disabled": flag(disabled),
            "data-shortcut": active_shortcut,
            "data-status": status,
            "data-visible": flag(visible),
            "data-hidden": flag(!visible),
            onclick: move |event: MouseEvent| {
                if let Some(handler) = onclick {
                    handler.call(event.clone());
                }
                if !event.default_action_enabled() {
                    return;
                }
                match kind {
                    Navigation::Previous => root.go_previous(),
                    Navigation::Skip => root.skip_current(),
                    Navigation::Next => root.go_next(),
                    Navigation::Submit => {}
                }
            },
            ..attributes,
            {content}
        }
    }
}

/// Goes back one item. Hidden on the first item.
#[component]
pub fn QuestionnairePrevious(
    #[props(default = ButtonVariant::Outline)] variant: ButtonVariant,
    #[props(default)] size: ButtonSize,
    #[props(default)] disabled: bool,
    onclick: Option<EventHandler<MouseEvent>>,
    #[props(extends = GlobalAttributes, extends = button)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    navigation_button(
        Navigation::Previous,
        variant,
        size,
        disabled,
        onclick,
        attributes,
        children,
    )
}

/// Leaves an optional item unanswered on purpose and moves on. Hidden on required items.
#[component]
pub fn QuestionnaireSkip(
    #[props(default = ButtonVariant::Outline)] variant: ButtonVariant,
    #[props(default)] size: ButtonSize,
    #[props(default)] disabled: bool,
    onclick: Option<EventHandler<MouseEvent>>,
    #[props(extends = GlobalAttributes, extends = button)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    navigation_button(
        Navigation::Skip,
        variant,
        size,
        disabled,
        onclick,
        attributes,
        children,
    )
}

/// Validates the active item and goes to the next one. Hidden on the last item.
#[component]
pub fn QuestionnaireNext(
    #[props(default)] variant: ButtonVariant,
    #[props(default)] size: ButtonSize,
    #[props(default)] disabled: bool,
    onclick: Option<EventHandler<MouseEvent>>,
    #[props(extends = GlobalAttributes, extends = button)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    navigation_button(
        Navigation::Next,
        variant,
        size,
        disabled,
        onclick,
        attributes,
        children,
    )
}

/// Submits the form. Shown only on the last item. Invalid items stop the submit and take focus.
#[component]
pub fn QuestionnaireSubmit(
    #[props(default)] variant: ButtonVariant,
    #[props(default)] size: ButtonSize,
    #[props(default)] disabled: bool,
    onclick: Option<EventHandler<MouseEvent>>,
    #[props(extends = GlobalAttributes, extends = button)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    navigation_button(
        Navigation::Submit,
        variant,
        size,
        disabled,
        onclick,
        attributes,
        children,
    )
}
