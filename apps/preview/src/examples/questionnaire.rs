//! Questionnaire examples, ported from shadcn/ui `examples/base/questionnaire-*.tsx`.

use std::collections::BTreeMap;

use dioxus::html::FormValue;
use dioxus::prelude::*;

use crate::components::ui::button::{Button, ButtonSize, ButtonVariant};
use crate::components::ui::card::{
    Card, CardAction, CardContent, CardDescription, CardDescriptionProps, CardFooter, CardHeader,
    CardTitle, CardTitleProps,
};
use crate::components::ui::dialog::{
    Dialog, DialogClose, DialogContent, DialogDescription, DialogDescriptionProps, DialogFooter,
    DialogHeader, DialogTitle, DialogTitleProps, DialogTrigger,
};
use crate::components::ui::native_select::{NativeSelect, NativeSelectOption};
use crate::components::ui::questionnaire::{
    Questionnaire, QuestionnaireActions, QuestionnaireChoice, QuestionnaireChoiceDefinition,
    QuestionnaireChoices, QuestionnaireDescription, QuestionnaireError, QuestionnaireInput,
    QuestionnaireItem, QuestionnaireItemDefinition, QuestionnaireItemStatus, QuestionnaireNext,
    QuestionnairePrevious, QuestionnaireProgress, QuestionnaireProgressState,
    QuestionnaireRenderProps, QuestionnaireShortcuts, QuestionnaireSkip, QuestionnaireSubmit,
    QuestionnaireTitle,
};
use crate::components::ui::toast::{ToastManager, ToastOptions, ToastProvider, Toaster, toast};

#[component]
pub fn Examples() -> Element {
    rsx! {
        ToastProvider {
            QuestionnaireDemo {}
            QuestionnaireMultiple {}
            QuestionnaireFreeform {}
            QuestionnaireSkipExample {}
            QuestionnaireShortcutsExample {}
            QuestionnaireValidation {}
            QuestionnaireControlled {}
            QuestionnaireResume {}
            QuestionnaireConditional {}
            QuestionnaireNavigationState {}
            QuestionnaireProgressExample {}
            QuestionnaireAnimated {}
            QuestionnaireCard {}
            QuestionnaireDialog {}
            Toaster {}
        }
    }
}

/// `formData.get(name)`.
fn form_get(event: &FormEvent, name: &str) -> Option<String> {
    form_get_all(event, name).into_iter().next()
}

/// `formData.getAll(name)`.
fn form_get_all(event: &FormEvent, name: &str) -> Vec<String> {
    event
        .values()
        .into_iter()
        .filter_map(|(key, value)| match value {
            FormValue::Text(text) if key == name => Some(text),
            _ => None,
        })
        .collect()
}

fn or_none(value: Option<String>) -> String {
    value.unwrap_or_else(|| "None".to_string())
}

fn joined_or_none(values: Vec<String>) -> String {
    if values.is_empty() {
        "None".to_string()
    } else {
        values.join(", ")
    }
}

fn show_toast(toast: ToastManager, title: &str, description: String) {
    toast.add(ToastOptions {
        description: Some(description),
        ..ToastOptions::new(title)
    });
}

struct DemoChoice {
    value: &'static str,
    label: &'static str,
    description: Option<&'static str>,
}

struct DemoInput {
    label: &'static str,
    placeholder: &'static str,
}

struct DemoQuestion {
    name: &'static str,
    title: &'static str,
    description: &'static str,
    required: bool,
    multiple: bool,
    choices: &'static [DemoChoice],
    input: Option<DemoInput>,
}

const DEMO_QUESTIONS: &[DemoQuestion] = &[
    DemoQuestion {
        choices: &[
            DemoChoice {
                description: Some("Show what the agent ran and what came back."),
                label: "Tool call timeline",
                value: "tool-calls",
            },
            DemoChoice {
                description: Some("Ask before sensitive or destructive actions."),
                label: "Approval checkpoints",
                value: "approvals",
            },
            DemoChoice {
                description: Some("Make delegated work and results easier to follow."),
                label: "Sub-agent handoffs",
                value: "handoffs",
            },
        ],
        description: "Choose a direction or describe another task.",
        input: Some(DemoInput {
            label: "Another agent feature",
            placeholder: "Describe another feature…",
        }),
        name: "direction",
        required: true,
        multiple: false,
        title: "What should the agent build next?",
    },
    DemoQuestion {
        choices: &[
            DemoChoice {
                label: "Progress",
                value: "progress",
                description: None,
            },
            DemoChoice {
                label: "Decisions",
                value: "decisions",
                description: None,
            },
            DemoChoice {
                label: "Risks",
                value: "risks",
                description: None,
            },
            DemoChoice {
                label: "Next step",
                value: "next-step",
                description: None,
            },
        ],
        description: "Select all that apply, or skip this question.",
        input: None,
        multiple: true,
        name: "signals",
        required: false,
        title: "What should every progress update include?",
    },
    DemoQuestion {
        choices: &[
            DemoChoice {
                label: "Start now",
                value: "now",
                description: None,
            },
            DemoChoice {
                label: "Next development cycle",
                value: "next-cycle",
                description: None,
            },
            DemoChoice {
                label: "Add it to the backlog",
                value: "backlog",
                description: None,
            },
        ],
        description: "Choose when the agent should begin the work.",
        input: None,
        multiple: false,
        name: "timing",
        required: true,
        title: "When should work begin?",
    },
];

fn demo_items() -> Vec<QuestionnaireItemDefinition> {
    DEMO_QUESTIONS
        .iter()
        .map(|question| QuestionnaireItemDefinition {
            choices: question
                .choices
                .iter()
                .map(|choice| QuestionnaireChoiceDefinition::new(choice.value))
                .collect(),
            ..QuestionnaireItemDefinition::new(question.name).required(question.required)
        })
        .collect()
}

#[component]
fn QuestionnaireDemo() -> Element {
    let toast = toast();
    let handle_submit = move |event: FormEvent| {
        event.prevent_default();
        let direction = form_get(&event, "direction");
        let signals = form_get_all(&event, "signals");
        let timing = form_get(&event, "timing");
        toast.add(ToastOptions {
            description: Some(format!(
                "Direction: {} · Progress signals: {} · Timing: {}",
                or_none(direction),
                joined_or_none(signals),
                or_none(timing)
            )),
            ..ToastOptions::new("Agent plan saved")
        });
    };

    rsx! {
        super::Example { title: "Questionnaire", class: "min-h-[560px] p-4 sm:p-8",
            Questionnaire {
                class: "mx-auto max-w-md",
                default_item: "direction",
                items: demo_items(),
                shortcuts: QuestionnaireShortcuts::Letters,
                onsubmit: handle_submit,
                QuestionnaireProgress {}
                for question in DEMO_QUESTIONS {
                    QuestionnaireItem {
                        key: "{question.name}",
                        multiple: question.multiple,
                        name: question.name,
                        required: question.required,
                        QuestionnaireTitle { "{question.title}" }
                        QuestionnaireDescription { "{question.description}" }
                        QuestionnaireChoices {
                            for choice in question.choices {
                                QuestionnaireChoice { key: "{choice.value}", value: choice.value,
                                    span { class: "font-medium", "{choice.label}" }
                                    if let Some(description) = choice.description {
                                        span { class: "text-muted-foreground", "{description}" }
                                    }
                                }
                            }
                            if let Some(input) = &question.input {
                                QuestionnaireInput {
                                    aria_label: input.label,
                                    placeholder: input.placeholder,
                                }
                            }
                        }
                        QuestionnaireError {}
                    }
                }
                QuestionnaireActions {
                    QuestionnairePrevious {}
                    QuestionnaireSkip {}
                    QuestionnaireNext { "Next" }
                    QuestionnaireSubmit { "Save plan" }
                }
            }
        }
    }
}

#[component]
fn QuestionnaireMultiple() -> Element {
    let toast = toast();
    let handle_submit = move |event: FormEvent| {
        event.prevent_default();
        let context = form_get_all(&event, "context");
        show_toast(
            toast,
            "Context selected",
            format!("Context: {}", joined_or_none(context)),
        );
    };

    rsx! {
        super::Example { title: "Multiple Selection", class: "min-h-[420px] p-4 sm:p-8",
            Questionnaire {
                class: "mx-auto max-w-md",
                items: vec![
                    QuestionnaireItemDefinition::new("context")
                        .required(true)
                        .choices(["source", "tests", "docs", "history"]),
                ],
                shortcuts: QuestionnaireShortcuts::Letters,
                onsubmit: handle_submit,
                QuestionnaireItem { name: "context", multiple: true, required: true,
                    QuestionnaireTitle { "What context should the agent inspect?" }
                    QuestionnaireDescription { "Select every source that may affect the implementation." }
                    QuestionnaireChoices {
                        QuestionnaireChoice { value: "source", "Relevant source files" }
                        QuestionnaireChoice { value: "tests", "Existing tests" }
                        QuestionnaireChoice { value: "docs", "Architecture documentation" }
                        QuestionnaireChoice { value: "history", "Recent commit history" }
                    }
                    QuestionnaireError {}
                }
                QuestionnaireActions {
                    QuestionnaireSubmit { "Share context" }
                }
            }
        }
    }
}

#[component]
fn QuestionnaireFreeform() -> Element {
    let toast = toast();
    let handle_submit = move |event: FormEvent| {
        event.prevent_default();
        let approach = form_get(&event, "approach");
        show_toast(
            toast,
            "Approach selected",
            format!("Approach: {}", or_none(approach)),
        );
    };

    rsx! {
        super::Example { title: "Freeform Answer", class: "min-h-[420px] p-4 sm:p-8",
            Questionnaire {
                class: "mx-auto max-w-md",
                items: vec![
                    QuestionnaireItemDefinition::new("approach")
                        .required(true)
                        .choices(["incremental", "module", "rewrite"]),
                ],
                shortcuts: QuestionnaireShortcuts::Letters,
                onsubmit: handle_submit,
                QuestionnaireItem { name: "approach", required: true,
                    QuestionnaireTitle { "How should the agent approach this refactor?" }
                    QuestionnaireDescription { "Choose a strategy or write a more specific instruction." }
                    QuestionnaireChoices {
                        QuestionnaireChoice { value: "incremental", "Make the smallest safe change" }
                        QuestionnaireChoice { value: "module", "Refactor one module at a time" }
                        QuestionnaireChoice { value: "rewrite", "Replace the implementation completely" }
                        QuestionnaireInput {
                            aria_label: "Another refactoring approach",
                            placeholder: "Describe another approach…",
                        }
                    }
                    QuestionnaireError {}
                }
                QuestionnaireActions {
                    QuestionnaireSubmit { "Use this approach" }
                }
            }
        }
    }
}

#[component]
fn QuestionnaireSkipExample() -> Element {
    let toast = toast();
    let mut constraint_status = use_signal(|| QuestionnaireItemStatus::Unanswered);
    let handle_submit = move |event: FormEvent| {
        event.prevent_default();
        let constraints = if constraint_status() == QuestionnaireItemStatus::Skipped {
            "Skipped".to_string()
        } else {
            or_none(form_get(&event, "constraints"))
        };
        show_toast(
            toast,
            "Agent brief submitted",
            format!(
                "Task: {} · Constraints: {} · Review: {}",
                or_none(form_get(&event, "task")),
                constraints,
                or_none(form_get(&event, "review"))
            ),
        );
    };

    rsx! {
        super::Example { title: "Explicit Skip", class: "min-h-[520px] p-4 sm:p-8",
            Questionnaire {
                class: "mx-auto max-w-md",
                default_item: "task",
                items: vec![
                    QuestionnaireItemDefinition::new("task").required(true),
                    QuestionnaireItemDefinition::new("constraints"),
                    QuestionnaireItemDefinition::new("review").required(true),
                ],
                onsubmit: handle_submit,
                QuestionnaireProgress {}

                QuestionnaireItem { name: "task", required: true,
                    QuestionnaireTitle { "What kind of change is this?" }
                    QuestionnaireDescription { "Choose the category that best describes the work." }
                    QuestionnaireChoices {
                        QuestionnaireChoice { value: "feature", "New feature" }
                        QuestionnaireChoice { value: "fix", "Bug fix" }
                        QuestionnaireChoice { value: "refactor", "Refactor" }
                    }
                    QuestionnaireError {}
                }

                QuestionnaireItem {
                    name: "constraints",
                    on_status_change: move |status| constraint_status.set(status),
                    QuestionnaireTitle { "Are there any implementation constraints?" }
                    QuestionnaireDescription { "Answer if needed, or intentionally skip this question." }
                    QuestionnaireChoices {
                        QuestionnaireChoice { value: "no-dependencies", "Do not add dependencies" }
                        QuestionnaireChoice { value: "no-migrations", "Do not change the database" }
                        QuestionnaireChoice { value: "preserve-api", "Preserve the public API" }
                        QuestionnaireInput {
                            aria_label: "Another implementation constraint",
                            placeholder: "Describe another constraint…",
                        }
                    }
                }

                QuestionnaireItem { name: "review", required: true,
                    QuestionnaireTitle { "How should the work be reviewed?" }
                    QuestionnaireDescription { "Choose the checks the agent should complete before handoff." }
                    QuestionnaireChoices {
                        QuestionnaireChoice { value: "tests", "Run the test suite" }
                        QuestionnaireChoice { value: "diff", "Review the final diff" }
                        QuestionnaireChoice { value: "both", "Tests and diff review" }
                    }
                    QuestionnaireError {}
                }

                QuestionnaireActions {
                    QuestionnairePrevious {}
                    QuestionnaireSkip {}
                    QuestionnaireNext { "Next" }
                    QuestionnaireSubmit { "Submit brief" }
                }
            }
        }
    }
}

#[component]
fn QuestionnaireShortcutsExample() -> Element {
    let toast = toast();
    let mut shortcuts = use_signal(|| Some(QuestionnaireShortcuts::Letters));
    let handle_submit = move |event: FormEvent| {
        event.prevent_default();
        show_toast(
            toast,
            "Next action selected",
            format!(
                "Action: {} · Shortcuts: {}",
                or_none(form_get(&event, "action")),
                shortcuts().map_or("none", QuestionnaireShortcuts::as_str)
            ),
        );
    };

    rsx! {
        super::Example { title: "Shortcuts", class: "min-h-[480px] p-4 sm:p-8",
            div { class: "relative mx-auto flex h-full w-full max-w-md flex-col",
                NativeSelect {
                    aria_label: "Shortcut style",
                    class: "absolute end-0 top-0",
                    value: shortcuts().map_or("none", QuestionnaireShortcuts::as_str),
                    onchange: move |event: FormEvent| {
                        shortcuts.set(match event.value().as_str() {
                            "letters" => Some(QuestionnaireShortcuts::Letters),
                            "numbers" => Some(QuestionnaireShortcuts::Numbers),
                            _ => None,
                        });
                    },
                    NativeSelectOption { value: "none", "No shortcuts" }
                    NativeSelectOption { value: "letters", "Letters" }
                    NativeSelectOption { value: "numbers", "Numbers" }
                }

                Questionnaire {
                    class: "mt-auto",
                    items: vec![
                        QuestionnaireItemDefinition::new("action")
                            .required(true)
                            .choices(["inspect", "tests", "patch"]),
                    ],
                    shortcuts: shortcuts(),
                    onsubmit: handle_submit,
                    QuestionnaireItem { name: "action", required: true,
                        QuestionnaireTitle { "What should the agent do next?" }
                        QuestionnaireDescription { "Use the displayed shortcut or navigate with the keyboard." }
                        QuestionnaireChoices {
                            QuestionnaireChoice { value: "inspect", "Inspect the implementation" }
                            QuestionnaireChoice { value: "tests", "Run the relevant tests" }
                            QuestionnaireChoice { value: "patch", "Prepare the patch" }
                        }
                        QuestionnaireError {}
                    }

                    QuestionnaireActions {
                        QuestionnaireSubmit { "Confirm action" }
                    }
                }
            }
        }
    }
}

/// The schema the upstream example checks with Zod: both answers come from fixed sets, and a
/// public answer must be complete.
fn validate_answers(event: &FormEvent) -> Result<(String, String), Vec<(&'static str, String)>> {
    let mut issues = Vec::new();
    let mut read = |name: &'static str, allowed: &[&str]| -> Option<String> {
        match form_get(event, name) {
            Some(value) if allowed.contains(&value.as_str()) => Some(value),
            Some(value) => {
                let expected = allowed
                    .iter()
                    .map(|option| format!("'{option}'"))
                    .collect::<Vec<_>>()
                    .join(" | ");
                issues.push((
                    name,
                    format!("Invalid enum value. Expected {expected}, received '{value}'"),
                ));
                None
            }
            None => {
                issues.push((name, "Required".to_string()));
                None
            }
        }
    };
    let detail = read("detail", &["summary", "complete"]);
    let audience = read("audience", &["team", "public"]);
    let (Some(detail), Some(audience)) = (detail, audience) else {
        return Err(issues);
    };
    if audience == "public" && detail == "summary" {
        return Err(vec![(
            "detail",
            "Public answers need enough context. Choose a complete answer.".to_string(),
        )]);
    }
    Ok((detail, audience))
}

#[component]
fn ValidationProgress() -> Element {
    rsx! {
        QuestionnaireProgress {
            class: "min-w-0",
            render: move |state: QuestionnaireProgressState| rsx! { "{state.current} / {state.total}" },
        }
    }
}

#[component]
fn ValidationError(name: &'static str, errors: Signal<BTreeMap<&'static str, String>>) -> Element {
    match errors.read().get(name).cloned() {
        Some(message) => rsx! {
            QuestionnaireError { "{message}" }
        },
        None => rsx! {
            QuestionnaireError {}
        },
    }
}

#[component]
fn QuestionnaireValidation() -> Element {
    let toast = toast();
    let mut item = use_signal(|| "detail".to_string());
    let mut errors = use_signal(BTreeMap::<&'static str, String>::new);
    let mut clear_error = move |name: &'static str| {
        if errors.read().contains_key(name) {
            errors.write().remove(name);
        }
    };
    let handle_submit = move |event: FormEvent| {
        event.prevent_default();
        match validate_answers(&event) {
            Ok((detail, audience)) => {
                errors.write().clear();
                show_toast(
                    toast,
                    "Agent response configured",
                    format!("Detail: {detail} · Audience: {audience}"),
                );
            }
            Err(issues) => {
                let mut next_errors = BTreeMap::new();
                for (name, message) in &issues {
                    next_errors.entry(*name).or_insert_with(|| message.clone());
                }
                errors.set(next_errors);
                if let Some((first_invalid, _)) = issues.first() {
                    item.set(first_invalid.to_string());
                }
            }
        }
    };

    rsx! {
        super::Example { title: "Custom Validation", class: "min-h-[520px] p-4 sm:p-8",
            Questionnaire {
                class: "mx-auto max-w-md",
                item: item(),
                items: vec![
                    QuestionnaireItemDefinition::new("detail").required(true),
                    QuestionnaireItemDefinition::new("audience").required(true),
                ],
                on_item_change: move |next| item.set(next),
                onsubmit: handle_submit,
                Card { class: "w-full",
                    QuestionnaireItem {
                        invalid: errors.read().contains_key("detail"),
                        name: "detail",
                        required: true,
                        CardHeader {
                            QuestionnaireTitle { "How much detail should the answer include?" }
                            QuestionnaireDescription { "Choose the response depth." }
                            CardAction {
                                ValidationProgress {}
                            }
                        }
                        CardContent {
                            QuestionnaireChoices {
                                QuestionnaireChoice {
                                    value: "summary",
                                    onchange: move |_| clear_error("detail"),
                                    "Concise summary"
                                }
                                QuestionnaireChoice {
                                    value: "complete",
                                    onchange: move |_| clear_error("detail"),
                                    "Complete answer"
                                }
                            }
                            ValidationError { name: "detail", errors }
                        }
                    }

                    QuestionnaireItem {
                        invalid: errors.read().contains_key("audience"),
                        name: "audience",
                        required: true,
                        CardHeader {
                            QuestionnaireTitle { "Who will read the answer?" }
                            QuestionnaireDescription { "Public answers require complete context." }
                            CardAction {
                                ValidationProgress {}
                            }
                        }
                        CardContent {
                            QuestionnaireChoices {
                                QuestionnaireChoice {
                                    value: "team",
                                    onchange: move |_| clear_error("audience"),
                                    "My team"
                                }
                                QuestionnaireChoice {
                                    value: "public",
                                    onchange: move |_| clear_error("audience"),
                                    "Public audience"
                                }
                            }
                            ValidationError { name: "audience", errors }
                        }
                    }

                    CardFooter {
                        QuestionnaireActions {
                            QuestionnairePrevious {}
                            QuestionnaireNext { "Next" }
                            QuestionnaireSubmit { "Validate answers" }
                        }
                    }
                }
            }
        }
    }
}

fn checkpoint_label(item: &str) -> &'static str {
    match item {
        "scope" => "Change scope",
        "checks" => "Verification",
        "output" => "Final output",
        _ => "",
    }
}

#[component]
fn QuestionnaireControlled() -> Element {
    let toast = toast();
    let mut item = use_signal(|| "scope".to_string());
    let handle_submit = move |event: FormEvent| {
        event.prevent_default();
        show_toast(
            toast,
            "Agent workflow configured",
            format!(
                "Scope: {} · Verification: {} · Output: {}",
                or_none(form_get(&event, "scope")),
                or_none(form_get(&event, "checks")),
                or_none(form_get(&event, "output"))
            ),
        );
    };

    rsx! {
        super::Example { title: "Controlled", class: "min-h-[520px] p-4 sm:p-8",
            div { class: "relative mx-auto flex h-full w-full max-w-md flex-col",
                p {
                    class: "absolute end-0 top-0 text-sm text-muted-foreground",
                    role: "status",
                    "Current checkpoint: {checkpoint_label(&item())}"
                }

                Questionnaire {
                    class: "mt-auto",
                    item: item(),
                    items: vec![
                        QuestionnaireItemDefinition::new("scope").required(true),
                        QuestionnaireItemDefinition::new("checks").required(true),
                        QuestionnaireItemDefinition::new("output").required(true),
                    ],
                    on_item_change: move |next| item.set(next),
                    onsubmit: handle_submit,
                    QuestionnaireProgress {}

                    QuestionnaireItem { name: "scope", required: true,
                        QuestionnaireTitle { "What may the agent change?" }
                        QuestionnaireDescription { "The host stores the active checkpoint while Questionnaire navigates." }
                        QuestionnaireChoices {
                            QuestionnaireChoice { value: "component", "Only the target component" }
                            QuestionnaireChoice { value: "tests", "Component and related tests" }
                            QuestionnaireChoice { value: "feature", "The complete feature area" }
                        }
                        QuestionnaireError {}
                    }

                    QuestionnaireItem { name: "checks", required: true,
                        QuestionnaireTitle { "Which verification level should it use?" }
                        QuestionnaireChoices {
                            QuestionnaireChoice { value: "targeted", "Targeted tests" }
                            QuestionnaireChoice { value: "package", "Package tests and typecheck" }
                            QuestionnaireChoice { value: "full", "Full workspace verification" }
                        }
                        QuestionnaireError {}
                    }

                    QuestionnaireItem { name: "output", required: true,
                        QuestionnaireTitle { "What should the agent return when finished?" }
                        QuestionnaireChoices {
                            QuestionnaireChoice { value: "summary", "Concise summary" }
                            QuestionnaireChoice { value: "diff", "Summary with changed files" }
                            QuestionnaireChoice { value: "handoff", "Detailed implementation handoff" }
                        }
                        QuestionnaireError {}
                    }

                    QuestionnaireActions {
                        QuestionnairePrevious {}
                        QuestionnaireNext { "Next" }
                        QuestionnaireSubmit { "Save workflow" }
                    }
                }
            }
        }
    }
}

#[component]
fn QuestionnaireResume() -> Element {
    let toast = toast();
    let handle_submit = move |event: FormEvent| {
        event.prevent_default();
        let notes = form_get(&event, "notes").filter(|notes| !notes.is_empty());
        toast.add(ToastOptions {
            description: Some(format!(
                "Migration: {} · Verification: {} · Notes: {}",
                or_none(form_get(&event, "change")),
                joined_or_none(form_get_all(&event, "verification")),
                or_none(notes)
            )),
            ..ToastOptions::new("Draft updated")
        });
    };

    rsx! {
        super::Example { title: "Resume", class: "min-h-[520px] p-4 sm:p-8",
            Questionnaire {
                class: "mx-auto max-w-md",
                default_item: "verification",
                items: vec![
                    QuestionnaireItemDefinition::new("change").required(true),
                    QuestionnaireItemDefinition::new("verification").required(true),
                    QuestionnaireItemDefinition::new("notes"),
                ],
                onreset: move |_| {
                    toast.add(ToastOptions::new("Saved answers restored"));
                },
                onsubmit: handle_submit,
                QuestionnaireProgress {}

                QuestionnaireItem { name: "change", required: true,
                    QuestionnaireTitle { "What kind of migration is this?" }
                    QuestionnaireDescription { "This answer was saved during the previous session." }
                    QuestionnaireChoices {
                        QuestionnaireChoice { value: "incremental", default_checked: true, "Incremental migration" }
                        QuestionnaireChoice { value: "cutover", "Single cutover" }
                    }
                    QuestionnaireError {}
                }

                QuestionnaireItem { name: "verification", multiple: true, required: true,
                    QuestionnaireTitle { "How should the migration be verified?" }
                    QuestionnaireDescription { "These checks were selected during the previous session." }
                    QuestionnaireChoices {
                        QuestionnaireChoice { value: "tests", default_checked: true, "Run migration tests" }
                        QuestionnaireChoice { value: "typecheck", default_checked: true, "Run the typecheck" }
                        QuestionnaireChoice { value: "manual", "Perform a manual smoke test" }
                    }
                    QuestionnaireError {}
                }

                QuestionnaireItem { name: "notes",
                    QuestionnaireTitle { "Anything else the agent should remember?" }
                    QuestionnaireDescription { "This note was saved with the draft." }
                    QuestionnaireInput {
                        aria_label: "Saved migration note",
                        default_value: "Keep the existing public API stable.",
                    }
                }

                QuestionnaireActions {
                    Button { r#type: "reset", variant: ButtonVariant::Outline, "Reset changes" }
                    QuestionnairePrevious {}
                    QuestionnaireNext { "Next" }
                    QuestionnaireSubmit { "Update draft" }
                }
            }
        }
    }
}

#[component]
fn QuestionnaireConditional() -> Element {
    let toast = toast();
    let mut runtime = use_signal(|| "local".to_string());
    let items = use_memo(move || {
        vec![
            QuestionnaireItemDefinition::new("runtime").required(true),
            QuestionnaireItemDefinition::new("environment")
                .disabled(runtime() != "cloud")
                .required(true),
            QuestionnaireItemDefinition::new("approval").required(true),
        ]
    });
    let handle_submit = move |event: FormEvent| {
        event.prevent_default();
        show_toast(
            toast,
            "Execution plan saved",
            format!(
                "Runtime: {} · Environment: {} · Approval: {}",
                or_none(form_get(&event, "runtime")),
                form_get(&event, "environment").unwrap_or_else(|| "Not applicable".to_string()),
                or_none(form_get(&event, "approval"))
            ),
        );
    };

    rsx! {
        super::Example { title: "Conditional Items", class: "min-h-[520px] p-4 sm:p-8",
            Questionnaire {
                class: "mx-auto max-w-md",
                default_item: "runtime",
                items: items(),
                onsubmit: handle_submit,
                QuestionnaireProgress {}

                QuestionnaireItem { name: "runtime", required: true,
                    QuestionnaireTitle { "Where should the agent run?" }
                    QuestionnaireDescription { "Cloud runs add an environment question to this flow." }
                    QuestionnaireChoices {
                        QuestionnaireChoice {
                            checked: runtime() == "local",
                            value: "local",
                            onchange: move |_| runtime.set("local".to_string()),
                            "Local workspace"
                        }
                        QuestionnaireChoice {
                            checked: runtime() == "cloud",
                            value: "cloud",
                            onchange: move |_| runtime.set("cloud".to_string()),
                            "Cloud workspace"
                        }
                    }
                    QuestionnaireError {}
                }

                QuestionnaireItem { disabled: runtime() != "cloud", name: "environment", required: true,
                    QuestionnaireTitle { "Which cloud environment should it use?" }
                    QuestionnaireChoices {
                        QuestionnaireChoice { value: "preview", "Preview" }
                        QuestionnaireChoice { value: "staging", "Staging" }
                        QuestionnaireChoice { value: "isolated", "Isolated sandbox" }
                    }
                    QuestionnaireError {}
                }

                QuestionnaireItem { name: "approval", required: true,
                    QuestionnaireTitle { "When should the agent request approval?" }
                    QuestionnaireChoices {
                        QuestionnaireChoice { value: "writes", "Before writing files" }
                        QuestionnaireChoice { value: "commands", "Before running commands" }
                        QuestionnaireChoice { value: "sensitive", "Only for sensitive actions" }
                    }
                    QuestionnaireError {}
                }

                QuestionnaireActions {
                    QuestionnairePrevious {}
                    QuestionnaireNext { "Next" }
                    QuestionnaireSubmit { "Save execution plan" }
                }
            }
        }
    }
}

#[component]
fn QuestionnaireNavigationState() -> Element {
    let toast = toast();
    let mut item = use_signal(|| "permission".to_string());
    let mut statuses = use_signal(|| {
        BTreeMap::from([
            ("permission", QuestionnaireItemStatus::Unanswered),
            ("verification", QuestionnaireItemStatus::Unanswered),
        ])
    });
    let unanswered =
        statuses.read().get(item().as_str()).copied() == Some(QuestionnaireItemStatus::Unanswered);
    let handle_submit = move |event: FormEvent| {
        event.prevent_default();
        show_toast(
            toast,
            "Permissions saved",
            format!(
                "Permission: {} · Verification: {}",
                or_none(form_get(&event, "permission")),
                or_none(form_get(&event, "verification"))
            ),
        );
    };

    rsx! {
        super::Example { title: "Navigation State", class: "min-h-[480px] p-4 sm:p-8",
            Questionnaire {
                class: "mx-auto max-w-md",
                item: item(),
                items: vec![
                    QuestionnaireItemDefinition::new("permission").required(true),
                    QuestionnaireItemDefinition::new("verification").required(true),
                ],
                on_item_change: move |next| item.set(next),
                onsubmit: handle_submit,
                QuestionnaireProgress {}

                QuestionnaireItem {
                    name: "permission",
                    required: true,
                    on_status_change: move |status| {
                        statuses.write().insert("permission", status);
                    },
                    QuestionnaireTitle { "What may the agent modify?" }
                    QuestionnaireDescription { "Next is intentionally disabled until an answer is selected." }
                    QuestionnaireChoices {
                        QuestionnaireChoice { value: "files", "Project files" }
                        QuestionnaireChoice { value: "tests", "Project files and tests" }
                        QuestionnaireChoice { value: "config", "Files, tests, and configuration" }
                    }
                    QuestionnaireError {}
                }

                QuestionnaireItem {
                    name: "verification",
                    required: true,
                    on_status_change: move |status| {
                        statuses.write().insert("verification", status);
                    },
                    QuestionnaireTitle { "What must pass before completion?" }
                    QuestionnaireChoices {
                        QuestionnaireChoice { value: "tests", "Tests" }
                        QuestionnaireChoice { value: "types", "Tests and types" }
                        QuestionnaireChoice { value: "all", "Tests, types, and visual QA" }
                    }
                    QuestionnaireError {}
                }

                QuestionnaireActions {
                    QuestionnairePrevious {}
                    QuestionnaireNext {
                        class: "data-[status=unanswered]:opacity-50",
                        disabled: unanswered,
                        variant: ButtonVariant::Secondary,
                        "Next"
                    }
                    QuestionnaireSubmit { disabled: unanswered, "Save permissions" }
                }
            }
        }
    }
}

#[component]
fn QuestionnaireProgressExample() -> Element {
    let toast = toast();
    let handle_submit = move |event: FormEvent| {
        event.prevent_default();
        show_toast(
            toast,
            "Pull request plan ready",
            format!(
                "Scope: {} · Commits: {} · Tests: {} · Delivery: {}",
                or_none(form_get(&event, "scope")),
                or_none(form_get(&event, "strategy")),
                or_none(form_get(&event, "tests")),
                or_none(form_get(&event, "delivery"))
            ),
        );
    };

    rsx! {
        super::Example { title: "Custom Progress", class: "min-h-[520px] p-4 sm:p-8",
            Questionnaire {
                class: "mx-auto max-w-md",
                default_item: "scope",
                items: vec![
                    QuestionnaireItemDefinition::new("scope").required(true),
                    QuestionnaireItemDefinition::new("strategy").required(true),
                    QuestionnaireItemDefinition::new("tests").required(true),
                    QuestionnaireItemDefinition::new("delivery").required(true),
                ],
                onsubmit: handle_submit,
                QuestionnaireProgress {
                    class: "w-full",
                    render: move |state: QuestionnaireProgressState| rsx! {
                        div { class: "mb-2 flex gap-1.5", aria_hidden: "true",
                            for index in 0..state.total {
                                span {
                                    key: "{index}",
                                    class: if index < state.current { "h-1.5 flex-1 rounded-full bg-primary" } else { "h-1.5 flex-1 rounded-full bg-muted" },
                                }
                            }
                        }
                        span { "Checkpoint {state.current} of {state.total}" }
                    },
                }

                QuestionnaireItem { name: "scope", required: true,
                    QuestionnaireTitle { "How large is the change?" }
                    QuestionnaireChoices {
                        QuestionnaireChoice { value: "small", "Small patch" }
                        QuestionnaireChoice { value: "medium", "Feature-sized change" }
                        QuestionnaireChoice { value: "large", "Cross-package change" }
                    }
                    QuestionnaireError {}
                }

                QuestionnaireItem { name: "strategy", required: true,
                    QuestionnaireTitle { "How should commits be organized?" }
                    QuestionnaireChoices {
                        QuestionnaireChoice { value: "single", "Single commit" }
                        QuestionnaireChoice { value: "logical", "Logical commits" }
                        QuestionnaireChoice { value: "squash", "Squash before review" }
                    }
                    QuestionnaireError {}
                }

                QuestionnaireItem { name: "tests", required: true,
                    QuestionnaireTitle { "Which tests should run?" }
                    QuestionnaireChoices {
                        QuestionnaireChoice { value: "targeted", "Targeted tests" }
                        QuestionnaireChoice { value: "package", "Package suite" }
                        QuestionnaireChoice { value: "workspace", "Full workspace" }
                    }
                    QuestionnaireError {}
                }

                QuestionnaireItem { name: "delivery", required: true,
                    QuestionnaireTitle { "How should the work be delivered?" }
                    QuestionnaireChoices {
                        QuestionnaireChoice { value: "patch", "Patch only" }
                        QuestionnaireChoice { value: "commit", "Committed locally" }
                        QuestionnaireChoice { value: "branch", "Push a review branch" }
                    }
                    QuestionnaireError {}
                }

                QuestionnaireActions {
                    QuestionnairePrevious {}
                    QuestionnaireNext { "Next" }
                    QuestionnaireSubmit { "Finish plan" }
                }
            }
        }
    }
}

const ANIMATED_ITEM_CLASS: &str = "data-active:animate-in data-active:fade-in-0 data-active:slide-in-from-bottom-2 data-active:duration-300 motion-reduce:animate-none";

#[component]
fn QuestionnaireAnimated() -> Element {
    let toast = toast();
    let handle_submit = move |event: FormEvent| {
        event.prevent_default();
        show_toast(
            toast,
            "Agent workflow saved",
            format!(
                "Task: {} · Review: {} · Delivery: {}",
                or_none(form_get(&event, "task")),
                or_none(form_get(&event, "review")),
                or_none(form_get(&event, "delivery"))
            ),
        );
    };

    rsx! {
        super::Example { title: "Animated Items", class: "min-h-[520px] p-4 sm:p-8",
            Questionnaire {
                class: "mx-auto max-w-md",
                default_item: "task",
                items: vec![
                    QuestionnaireItemDefinition::new("task").required(true),
                    QuestionnaireItemDefinition::new("review").required(true),
                    QuestionnaireItemDefinition::new("delivery").required(true),
                ],
                onsubmit: handle_submit,
                QuestionnaireProgress {}

                QuestionnaireItem { class: ANIMATED_ITEM_CLASS, name: "task", required: true,
                    QuestionnaireTitle { "What should the agent do?" }
                    QuestionnaireDescription { "Choose the task for this run." }
                    QuestionnaireChoices {
                        QuestionnaireChoice { value: "implement", "Implement the requested change" }
                        QuestionnaireChoice { value: "debug", "Debug the current behavior" }
                        QuestionnaireChoice { value: "review", "Review the implementation" }
                    }
                    QuestionnaireError {}
                }

                QuestionnaireItem { class: ANIMATED_ITEM_CLASS, name: "review", required: true,
                    QuestionnaireTitle { "How should the work be reviewed?" }
                    QuestionnaireDescription { "Select the verification depth." }
                    QuestionnaireChoices {
                        QuestionnaireChoice { value: "targeted", "Targeted checks" }
                        QuestionnaireChoice { value: "complete", "Complete test suite" }
                        QuestionnaireChoice { value: "manual", "Tests and manual QA" }
                    }
                    QuestionnaireError {}
                }

                QuestionnaireItem { class: ANIMATED_ITEM_CLASS, name: "delivery", required: true,
                    QuestionnaireTitle { "How should the result be delivered?" }
                    QuestionnaireDescription { "Choose the final handoff format." }
                    QuestionnaireChoices {
                        QuestionnaireChoice { value: "summary", "Concise summary" }
                        QuestionnaireChoice { value: "diff", "Summary and changed files" }
                        QuestionnaireChoice { value: "handoff", "Detailed review handoff" }
                    }
                    QuestionnaireError {}
                }

                QuestionnaireActions {
                    QuestionnairePrevious {}
                    QuestionnaireNext { "Next" }
                    QuestionnaireSubmit { "Save workflow" }
                }
            }
        }
    }
}

fn card_title(props: QuestionnaireRenderProps) -> Element {
    rsx! {
        CardTitle { ..CardTitleProps { attributes: props.attributes, children: props.children } }
    }
}

fn card_description(props: QuestionnaireRenderProps) -> Element {
    rsx! {
        CardDescription { ..CardDescriptionProps { attributes: props.attributes, children: props.children } }
    }
}

#[component]
fn QuestionnaireCard() -> Element {
    let toast = toast();
    let task_title_id = "questionnaire-card-task-title";
    let output_title_id = "questionnaire-card-output-title";
    let handle_submit = move |event: FormEvent| {
        event.prevent_default();
        show_toast(
            toast,
            "Agent task created",
            format!(
                "Task: {} · Handoff: {}",
                or_none(form_get(&event, "task")),
                or_none(form_get(&event, "output"))
            ),
        );
    };

    rsx! {
        super::Example { title: "Card", class: "min-h-[560px] p-4 sm:p-8",
            Questionnaire {
                class: "mx-auto max-w-md",
                default_item: "task",
                items: vec![
                    QuestionnaireItemDefinition::new("task")
                        .required(true)
                        .choices(["fix", "refactor", "docs"]),
                    QuestionnaireItemDefinition::new("output")
                        .required(true)
                        .choices(["summary", "files", "review"]),
                ],
                shortcuts: QuestionnaireShortcuts::Numbers,
                onsubmit: handle_submit,
                Card {
                    QuestionnaireItem { aria_labelledby: task_title_id, name: "task", required: true,
                        CardHeader {
                            QuestionnaireTitle { id: task_title_id, render: card_title,
                                "What should the agent work on?"
                            }
                            QuestionnaireDescription { render: card_description,
                                "Choose the task that should be handled next."
                            }
                            CardAction {
                                QuestionnaireProgress {}
                            }
                        }
                        CardContent {
                            QuestionnaireChoices {
                                QuestionnaireChoice { value: "fix", "Fix the failing tests" }
                                QuestionnaireChoice { value: "refactor", "Refactor the data layer" }
                                QuestionnaireChoice { value: "docs", "Update the integration guide" }
                            }
                            QuestionnaireError {}
                        }
                    }

                    QuestionnaireItem { aria_labelledby: output_title_id, name: "output", required: true,
                        CardHeader {
                            QuestionnaireTitle { id: output_title_id, render: card_title,
                                "What should the final handoff include?"
                            }
                            QuestionnaireDescription { render: card_description,
                                "Pick the level of detail needed for review."
                            }
                            CardAction {
                                QuestionnaireProgress {}
                            }
                        }
                        CardContent {
                            QuestionnaireChoices {
                                QuestionnaireChoice { value: "summary", "Summary only" }
                                QuestionnaireChoice { value: "files", "Summary and changed files" }
                                QuestionnaireChoice { value: "review", "Full review handoff" }
                            }
                            QuestionnaireError {}
                        }
                    }

                    CardFooter {
                        QuestionnaireActions { class: "w-full",
                            QuestionnairePrevious {}
                            QuestionnaireNext { "Next" }
                            QuestionnaireSubmit { "Create task" }
                        }
                    }
                }
            }
        }
    }
}

fn dialog_title(props: QuestionnaireRenderProps) -> Element {
    rsx! {
        DialogTitle { ..DialogTitleProps { attributes: props.attributes, children: props.children } }
    }
}

fn dialog_description(props: QuestionnaireRenderProps) -> Element {
    rsx! {
        DialogDescription { ..DialogDescriptionProps { attributes: props.attributes, children: props.children } }
    }
}

#[component]
fn QuestionnaireDialog() -> Element {
    let toast = toast();
    let mut open = use_signal(|| false);
    let handle_submit = move |event: FormEvent| {
        event.prevent_default();
        open.set(false);
        toast.add(ToastOptions {
            description: Some(format!(
                "Scope: {} · Verification: {}",
                or_none(form_get(&event, "scope")),
                or_none(form_get(&event, "tests"))
            )),
            ..ToastOptions::new("Clarification sent")
        });
    };

    rsx! {
        super::Example { title: "Dialog", class: "min-h-[320px] p-4 sm:p-8",
            Dialog { open: open(), on_open_change: move |next| open.set(next),
                DialogTrigger { variant: ButtonVariant::Outline, "Open clarification" }
                DialogContent {
                    Questionnaire {
                        default_item: "scope",
                        items: vec![
                            QuestionnaireItemDefinition::new("scope").required(true),
                            QuestionnaireItemDefinition::new("tests").required(true),
                        ],
                        onsubmit: handle_submit,
                        QuestionnaireItem { name: "scope", required: true,
                            DialogHeader {
                                QuestionnaireProgress {}
                                QuestionnaireTitle { render: dialog_title, "Which files are in scope?" }
                                QuestionnaireDescription { render: dialog_description,
                                    "Choose how broadly the agent can update the workspace."
                                }
                            }
                            QuestionnaireChoices {
                                QuestionnaireChoice { value: "component", "Component only" }
                                QuestionnaireChoice { value: "feature", "Complete feature directory" }
                                QuestionnaireChoice { value: "workspace", "Any related workspace file" }
                            }
                            QuestionnaireError {}
                        }

                        QuestionnaireItem { name: "tests", required: true,
                            DialogHeader {
                                QuestionnaireProgress {}
                                QuestionnaireTitle { render: dialog_title, "How much verification is needed?" }
                                QuestionnaireDescription { render: dialog_description,
                                    "Choose the checks the agent should run before handoff."
                                }
                            }
                            QuestionnaireChoices {
                                QuestionnaireChoice { value: "targeted", "Targeted tests" }
                                QuestionnaireChoice { value: "package", "Package tests" }
                                QuestionnaireChoice { value: "full", "Full workspace verification" }
                            }
                            QuestionnaireError {}
                        }

                        DialogFooter {
                            DialogClose {
                                r#type: "button",
                                variant: ButtonVariant::Outline,
                                size: ButtonSize::Default,
                                "Cancel"
                            }
                            QuestionnaireActions {
                                QuestionnairePrevious {}
                                QuestionnaireNext { "Next" }
                                QuestionnaireSubmit { "Send answer" }
                            }
                        }
                    }
                }
            }
        }
    }
}
