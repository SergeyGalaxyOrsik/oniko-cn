//! Questionnaire: a step-by-step form that asks one question at a time.
//! Questionnaire holds the answers and the current step. Its parts are QuestionnaireProgress,
//! QuestionnaireItem (one question per item, with index starting at 0), QuestionnaireTitle,
//! QuestionnaireDescription, QuestionnaireChoices, QuestionnaireChoice (radio, or checkbox when
//! `multiple`), QuestionnaireInput (free text answer), QuestionnaireError and QuestionnaireActions
//! with QuestionnairePrevious, QuestionnaireSkip, QuestionnaireNext and QuestionnaireSubmit.
//! Answers are keyed by the item `name`. Required items block Next and Submit until answered.
//!
//! Installed by oniko-cn. This file is yours: edit it freely.

use std::collections::BTreeMap;

use dioxus::prelude::*;
use oniko_cn::{cn, take_class};

use super::button::{ButtonSize, ButtonVariant, button_variants};

/// Answers keyed by item name. Each value holds the selected choice values, or the typed text.
pub type QuestionnaireAnswers = BTreeMap<String, Vec<String>>;

#[derive(Clone)]
struct ItemInfo {
    index: usize,
    name: String,
    required: bool,
}

#[derive(Clone)]
struct ItemContext {
    name: String,
    required: bool,
    multiple: bool,
}

#[derive(Clone, Copy)]
struct QuestionnaireContext {
    step: Signal<usize>,
    items: Signal<Vec<ItemInfo>>,
    answers: Signal<QuestionnaireAnswers>,
    attempted: Signal<bool>,
    onsubmit: Option<EventHandler<QuestionnaireAnswers>>,
}

fn is_answered(answers: &QuestionnaireAnswers, name: &str) -> bool {
    answers
        .get(name)
        .is_some_and(|values| values.iter().any(|value| !value.trim().is_empty()))
}

fn current_item(ctx: QuestionnaireContext) -> Option<ItemInfo> {
    let step = (ctx.step)();
    ctx.items
        .read()
        .iter()
        .find(|item| item.index == step)
        .cloned()
}

fn is_last_step(ctx: QuestionnaireContext) -> bool {
    let step = (ctx.step)();
    let count = ctx.items.read().len();
    count > 0 && step + 1 >= count
}

/// Root of a questionnaire. Pass `onsubmit` to receive the answers when the last item is submitted.
#[component]
pub fn Questionnaire(
    onsubmit: Option<EventHandler<QuestionnaireAnswers>>,
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(
        "cn-questionnaire flex flex-col",
        take_class(&mut attributes)
    );
    let step = use_signal(|| 0_usize);
    let items = use_signal(Vec::<ItemInfo>::new);
    let answers = use_signal(QuestionnaireAnswers::new);
    let attempted = use_signal(|| false);
    use_context_provider(|| QuestionnaireContext {
        step,
        items,
        answers,
        attempted,
        onsubmit,
    });

    rsx! {
        div {
            "data-slot": "questionnaire",
            class: class,
            ..attributes,
            {children}
        }
    }
}

/// Shows the position of the current item, such as "2 of 5".
#[component]
pub fn QuestionnaireProgress(
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
) -> Element {
    let ctx = use_context::<QuestionnaireContext>();
    let mut attributes = attributes;
    let class = cn!(
        "cn-questionnaire-progress text-muted-foreground",
        take_class(&mut attributes)
    );
    let total = ctx.items.read().len();
    let current = (ctx.step)() + 1;

    rsx! {
        div {
            "data-slot": "questionnaire-progress",
            class: class,
            ..attributes,
            {format!("{current} of {total}")}
        }
    }
}

/// One question. Items are rendered in every step, and only the one at `index` is shown.
/// Render them in order, with `index` counting from 0.
#[component]
pub fn QuestionnaireItem(
    index: usize,
    name: String,
    #[props(default)] required: bool,
    #[props(default)] multiple: bool,
    #[props(extends = GlobalAttributes, extends = fieldset)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut ctx = use_context::<QuestionnaireContext>();
    let mut attributes = attributes;
    let info = ItemInfo {
        index,
        name: name.clone(),
        required,
    };
    use_effect(move || {
        let info = info.clone();
        ctx.items.with_mut(|items| {
            items.retain(|item| item.index != info.index);
            items.push(info);
            items.sort_by_key(|item| item.index);
        });
    });
    use_context_provider(|| ItemContext {
        name,
        required,
        multiple,
    });

    let is_active = (ctx.step)() == index;
    let class = cn!("cn-questionnaire-item", take_class(&mut attributes));
    let class = if is_active {
        class
    } else {
        cn!(class, "hidden")
    };

    rsx! {
        fieldset {
            "data-slot": "questionnaire-item",
            class: class,
            ..attributes,
            {children}
        }
    }
}

/// The question text. Rendered as a legend of the item.
#[component]
pub fn QuestionnaireTitle(
    #[props(extends = GlobalAttributes, extends = legend)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!("cn-questionnaire-title", take_class(&mut attributes));

    rsx! {
        legend {
            "data-slot": "questionnaire-title",
            class: class,
            ..attributes,
            {children}
        }
    }
}

/// Extra text under the question.
#[component]
pub fn QuestionnaireDescription(
    #[props(extends = GlobalAttributes, extends = p)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!("cn-questionnaire-description", take_class(&mut attributes));

    rsx! {
        p {
            "data-slot": "questionnaire-description",
            class: class,
            ..attributes,
            {children}
        }
    }
}

/// Groups the choices of one item.
#[component]
pub fn QuestionnaireChoices(
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(
        "cn-questionnaire-choices flex flex-col",
        take_class(&mut attributes)
    );

    rsx! {
        div {
            "data-slot": "questionnaire-choices",
            class: class,
            ..attributes,
            {children}
        }
    }
}

/// One selectable answer. Picks a radio button, or a checkbox when the item is `multiple`.
/// The label text is the children. `value` is what gets stored in the answers.
#[component]
pub fn QuestionnaireChoice(
    value: String,
    #[props(default)] disabled: bool,
    #[props(extends = GlobalAttributes, extends = label)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let ctx = use_context::<QuestionnaireContext>();
    let mut answers = ctx.answers;
    let item = use_context::<ItemContext>();
    let mut attributes = attributes;

    let checked = answers
        .read()
        .get(&item.name)
        .is_some_and(|values| values.contains(&value));
    let multiple = item.multiple;
    let key = item.name.clone();
    let toggle_value = value.clone();
    let input_type = if multiple { "checkbox" } else { "radio" };
    let class = cn!(
        "cn-questionnaire-choice group/questionnaire-choice flex w-full cursor-pointer items-start has-[>input:disabled]:cursor-not-allowed has-[>input:disabled]:opacity-50",
        take_class(&mut attributes)
    );

    rsx! {
        label {
            "data-slot": "questionnaire-choice",
            "data-checked": checked.then_some("true"),
            class: class,
            ..attributes,
            input {
                "data-slot": "questionnaire-choice-input",
                r#type: input_type,
                name: item.name.clone(),
                value: value.clone(),
                checked: checked,
                disabled: disabled,
                class: "peer sr-only",
                onchange: move |_| {
                    let mut map = answers.write();
                    let entry = map.entry(key.clone()).or_default();
                    if multiple {
                        match entry.iter().position(|selected| selected == &toggle_value) {
                            Some(position) => {
                                entry.remove(position);
                            }
                            None => entry.push(toggle_value.clone()),
                        }
                    } else {
                        *entry = vec![toggle_value.clone()];
                    }
                },
            }
            span {
                "data-slot": "questionnaire-choice-indicator",
                class: "cn-questionnaire-choice-indicator flex shrink-0 items-center justify-center border",
                if checked && multiple {
                    svg {
                        "data-slot": "questionnaire-choice-indicator-check",
                        class: "cn-questionnaire-choice-indicator-check pointer-events-none",
                        view_box: "0 0 24 24",
                        fill: "none",
                        stroke: "currentColor",
                        stroke_width: "2",
                        stroke_linecap: "round",
                        stroke_linejoin: "round",
                        path { d: "M20 6 9 17l-5-5" }
                    }
                }
                if checked && !multiple {
                    span {
                        "data-slot": "questionnaire-choice-indicator-dot",
                        class: "cn-questionnaire-choice-indicator-dot rounded-full",
                    }
                }
            }
            span {
                "data-slot": "questionnaire-choice-content",
                class: "cn-questionnaire-choice-content flex flex-col",
                {children}
            }
        }
    }
}

/// Secondary text inside a choice.
#[component]
pub fn QuestionnaireChoiceDescription(
    #[props(extends = GlobalAttributes, extends = p)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(
        "cn-questionnaire-choice-description text-sm",
        take_class(&mut attributes)
    );

    rsx! {
        p {
            "data-slot": "questionnaire-choice-description",
            class: class,
            ..attributes,
            {children}
        }
    }
}

/// Free text answer for the item. Its text is stored as the item's answer.
/// Give it an accessible name with `aria-label`, `aria-labelledby` or a visible label.
#[component]
pub fn QuestionnaireInput(
    #[props(extends = GlobalAttributes, extends = input)] attributes: Vec<Attribute>,
) -> Element {
    let ctx = use_context::<QuestionnaireContext>();
    let mut answers = ctx.answers;
    let item = use_context::<ItemContext>();
    let mut draft = use_signal(String::new);
    let mut attributes = attributes;
    let class = cn!(
        "cn-questionnaire-input flex w-full min-w-0 outline-none",
        take_class(&mut attributes)
    );
    let key = item.name.clone();

    rsx! {
        div {
            "data-slot": "questionnaire-input-wrapper",
            class: "cn-questionnaire-input-wrapper",
            input {
                "data-slot": "questionnaire-input",
                r#type: "text",
                class: class,
                value: draft(),
                oninput: move |event| {
                    let text = event.value();
                    let stored = if text.trim().is_empty() {
                        Vec::new()
                    } else {
                        vec![text.clone()]
                    };
                    answers.write().insert(key.clone(), stored);
                    draft.set(text);
                },
                ..attributes,
            }
        }
    }
}

/// Validation message for the item. Shown after Next or Submit was pressed on an unanswered required item.
#[component]
pub fn QuestionnaireError(
    message: Option<String>,
    #[props(extends = GlobalAttributes, extends = p)] attributes: Vec<Attribute>,
) -> Element {
    let ctx = use_context::<QuestionnaireContext>();
    let item = use_context::<ItemContext>();
    let mut attributes = attributes;
    let class = cn!("cn-questionnaire-error", take_class(&mut attributes));
    let missing =
        item.required && (ctx.attempted)() && !is_answered(&ctx.answers.read(), &item.name);
    let message = message.unwrap_or_else(|| "Answer this question to continue.".to_string());

    rsx! {
        if missing {
            p {
                "data-slot": "questionnaire-error",
                role: "alert",
                class: class,
                ..attributes,
                {message}
            }
        }
    }
}

/// Row with the navigation buttons.
#[component]
pub fn QuestionnaireActions(
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(
        "cn-questionnaire-actions flex flex-wrap items-center justify-between",
        take_class(&mut attributes)
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

/// Goes back one item. Disabled on the first item.
#[component]
pub fn QuestionnairePrevious(
    #[props(default = ButtonVariant::Outline)] variant: ButtonVariant,
    #[props(default)] size: ButtonSize,
    #[props(extends = GlobalAttributes, extends = button)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let ctx = use_context::<QuestionnaireContext>();
    let mut step = ctx.step;
    let mut attributes = attributes;
    let class = button_variants(variant, size, take_class(&mut attributes).as_deref());
    let disabled = (ctx.step)() == 0;

    rsx! {
        button {
            "data-slot": "questionnaire-previous",
            r#type: "button",
            class: class,
            disabled: disabled,
            onclick: move |_| {
                let current = step();
                step.set(current.saturating_sub(1));
            },
            ..attributes,
            {children}
        }
    }
}

/// Moves to the next item without an answer. Disabled on required items and on the last item.
#[component]
pub fn QuestionnaireSkip(
    #[props(default = ButtonVariant::Ghost)] variant: ButtonVariant,
    #[props(default)] size: ButtonSize,
    #[props(extends = GlobalAttributes, extends = button)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let ctx = use_context::<QuestionnaireContext>();
    let mut step = ctx.step;
    let mut attempted = ctx.attempted;
    let mut attributes = attributes;
    let class = button_variants(variant, size, take_class(&mut attributes).as_deref());
    let required = current_item(ctx).is_some_and(|item| item.required);
    let disabled = required || is_last_step(ctx);

    rsx! {
        button {
            "data-slot": "questionnaire-skip",
            r#type: "button",
            class: class,
            disabled: disabled,
            onclick: move |_| {
                let current = step();
                step.set(current + 1);
                attempted.set(false);
            },
            ..attributes,
            {children}
        }
    }
}

/// Moves to the next item. Blocked while the current required item is unanswered.
/// Hidden on the last item, where QuestionnaireSubmit is shown instead.
#[component]
pub fn QuestionnaireNext(
    #[props(default)] variant: ButtonVariant,
    #[props(default)] size: ButtonSize,
    #[props(extends = GlobalAttributes, extends = button)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let ctx = use_context::<QuestionnaireContext>();
    let mut step = ctx.step;
    let mut attempted = ctx.attempted;
    let mut attributes = attributes;
    let class = button_variants(variant, size, take_class(&mut attributes).as_deref());
    let last = is_last_step(ctx);

    rsx! {
        if !last {
            button {
                "data-slot": "questionnaire-next",
                r#type: "button",
                class: class,
                onclick: move |_| {
                    let current = step();
                    let blocked = current_item(ctx)
                        .is_some_and(|item| item.required && !is_answered(&ctx.answers.read(), &item.name));
                    if blocked {
                        attempted.set(true);
                        return;
                    }
                    step.set(current + 1);
                    attempted.set(false);
                },
                ..attributes,
                {children}
            }
        }
    }
}

/// Submits the answers. Shown on the last item. If a required item is unanswered, jumps to the
/// first one instead of calling `onsubmit`.
#[component]
pub fn QuestionnaireSubmit(
    #[props(default)] variant: ButtonVariant,
    #[props(default)] size: ButtonSize,
    #[props(extends = GlobalAttributes, extends = button)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let ctx = use_context::<QuestionnaireContext>();
    let mut step = ctx.step;
    let mut attempted = ctx.attempted;
    let mut attributes = attributes;
    let class = button_variants(variant, size, take_class(&mut attributes).as_deref());
    let last = is_last_step(ctx);

    rsx! {
        if last {
            button {
                "data-slot": "questionnaire-submit",
                r#type: "button",
                class: class,
                onclick: move |_| {
                    let answers = ctx.answers.read().clone();
                    let first_missing = ctx
                        .items
                        .read()
                        .iter()
                        .filter(|item| item.required && !is_answered(&answers, &item.name))
                        .map(|item| item.index)
                        .min();
                    if let Some(index) = first_missing {
                        step.set(index);
                        attempted.set(true);
                        return;
                    }
                    attempted.set(false);
                    if let Some(handler) = ctx.onsubmit {
                        handler.call(answers);
                    }
                },
                ..attributes,
                {children}
            }
        }
    }
}
