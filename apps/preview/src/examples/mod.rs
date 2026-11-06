//! One module per docs page of shadcn/ui. Each exports `Examples`, which
//! renders every example that page shows, in the same order.

mod accordion;
mod alert;
mod alert_dialog;
mod aspect_ratio;
mod attachment;
mod avatar;
mod badge;
mod breadcrumb;
mod bubble;
mod button;
mod button_group;
mod calendar;
mod card;
mod carousel;
mod chain_of_thought;
mod chart;
mod chat;
mod checkbox;
mod checkpoint;
mod code_block;
mod collapsible;
mod combobox;
mod command;
mod confirmation;
mod context;
mod context_menu;
mod data_table;
mod date_picker;
mod dialog;
mod direction;
mod drawer;
mod dropdown_menu;
mod empty;
mod field;
mod file_tree;
mod hover_card;
mod inline_citation;
mod input;
mod input_group;
mod input_otp;
mod item;
mod kbd;
mod label;
mod marker;
mod menubar;
mod message;
mod message_scroller;
mod model_selector;
mod native_select;
mod navigation_menu;
mod pagination;
mod plan;
mod popover;
mod progress;
mod prompt_input;
mod questionnaire;
mod queue;
mod radio_group;
mod reasoning;
mod resizable;
mod scroll_area;
mod select;
mod separator;
mod sheet;
mod sidebar;
mod skeleton;
mod slider;
mod sources;
mod spinner;
mod stack_trace;
mod subagent;
mod suggestion;
mod switch;
mod table;
mod tabs;
mod task;
mod terminal;
mod test_results;
mod textarea;
mod toast;
mod toggle;
mod toggle_group;
mod tool;
mod tooltip;
mod trace;
mod typography;

use dioxus::prelude::*;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Group {
    Components,
    AiElements,
}

impl Group {
    pub fn title(self) -> &'static str {
        match self {
            Group::Components => "Components",
            Group::AiElements => "AI Elements",
        }
    }
}

pub struct Page {
    pub slug: &'static str,
    pub title: &'static str,
    pub group: Group,
}

pub const PAGES: &[Page] = &[
    Page {
        slug: "accordion",
        title: "Accordion",
        group: Group::Components,
    },
    Page {
        slug: "alert-dialog",
        title: "Alert Dialog",
        group: Group::Components,
    },
    Page {
        slug: "alert",
        title: "Alert",
        group: Group::Components,
    },
    Page {
        slug: "aspect-ratio",
        title: "Aspect Ratio",
        group: Group::Components,
    },
    Page {
        slug: "attachment",
        title: "Attachment",
        group: Group::Components,
    },
    Page {
        slug: "avatar",
        title: "Avatar",
        group: Group::Components,
    },
    Page {
        slug: "badge",
        title: "Badge",
        group: Group::Components,
    },
    Page {
        slug: "breadcrumb",
        title: "Breadcrumb",
        group: Group::Components,
    },
    Page {
        slug: "bubble",
        title: "Bubble",
        group: Group::Components,
    },
    Page {
        slug: "button-group",
        title: "Button Group",
        group: Group::Components,
    },
    Page {
        slug: "button",
        title: "Button",
        group: Group::Components,
    },
    Page {
        slug: "calendar",
        title: "Calendar",
        group: Group::Components,
    },
    Page {
        slug: "card",
        title: "Card",
        group: Group::Components,
    },
    Page {
        slug: "carousel",
        title: "Carousel",
        group: Group::Components,
    },
    Page {
        slug: "chart",
        title: "Chart",
        group: Group::Components,
    },
    Page {
        slug: "checkbox",
        title: "Checkbox",
        group: Group::Components,
    },
    Page {
        slug: "code-block",
        title: "Code Block",
        group: Group::Components,
    },
    Page {
        slug: "collapsible",
        title: "Collapsible",
        group: Group::Components,
    },
    Page {
        slug: "combobox",
        title: "Combobox",
        group: Group::Components,
    },
    Page {
        slug: "command",
        title: "Command",
        group: Group::Components,
    },
    Page {
        slug: "context-menu",
        title: "Context Menu",
        group: Group::Components,
    },
    Page {
        slug: "data-table",
        title: "Data Table",
        group: Group::Components,
    },
    Page {
        slug: "date-picker",
        title: "Date Picker",
        group: Group::Components,
    },
    Page {
        slug: "dialog",
        title: "Dialog",
        group: Group::Components,
    },
    Page {
        slug: "direction",
        title: "Direction",
        group: Group::Components,
    },
    Page {
        slug: "drawer",
        title: "Drawer",
        group: Group::Components,
    },
    Page {
        slug: "dropdown-menu",
        title: "Dropdown Menu",
        group: Group::Components,
    },
    Page {
        slug: "empty",
        title: "Empty",
        group: Group::Components,
    },
    Page {
        slug: "field",
        title: "Field",
        group: Group::Components,
    },
    Page {
        slug: "hover-card",
        title: "Hover Card",
        group: Group::Components,
    },
    Page {
        slug: "input-group",
        title: "Input Group",
        group: Group::Components,
    },
    Page {
        slug: "input-otp",
        title: "Input OTP",
        group: Group::Components,
    },
    Page {
        slug: "input",
        title: "Input",
        group: Group::Components,
    },
    Page {
        slug: "item",
        title: "Item",
        group: Group::Components,
    },
    Page {
        slug: "kbd",
        title: "Kbd",
        group: Group::Components,
    },
    Page {
        slug: "label",
        title: "Label",
        group: Group::Components,
    },
    Page {
        slug: "marker",
        title: "Marker",
        group: Group::Components,
    },
    Page {
        slug: "menubar",
        title: "Menubar",
        group: Group::Components,
    },
    Page {
        slug: "message-scroller",
        title: "Message Scroller",
        group: Group::Components,
    },
    Page {
        slug: "message",
        title: "Message",
        group: Group::Components,
    },
    Page {
        slug: "native-select",
        title: "Native Select",
        group: Group::Components,
    },
    Page {
        slug: "navigation-menu",
        title: "Navigation Menu",
        group: Group::Components,
    },
    Page {
        slug: "pagination",
        title: "Pagination",
        group: Group::Components,
    },
    Page {
        slug: "popover",
        title: "Popover",
        group: Group::Components,
    },
    Page {
        slug: "progress",
        title: "Progress",
        group: Group::Components,
    },
    Page {
        slug: "questionnaire",
        title: "Questionnaire",
        group: Group::Components,
    },
    Page {
        slug: "radio-group",
        title: "Radio Group",
        group: Group::Components,
    },
    Page {
        slug: "resizable",
        title: "Resizable",
        group: Group::Components,
    },
    Page {
        slug: "scroll-area",
        title: "Scroll Area",
        group: Group::Components,
    },
    Page {
        slug: "select",
        title: "Select",
        group: Group::Components,
    },
    Page {
        slug: "separator",
        title: "Separator",
        group: Group::Components,
    },
    Page {
        slug: "sheet",
        title: "Sheet",
        group: Group::Components,
    },
    Page {
        slug: "sidebar",
        title: "Sidebar",
        group: Group::Components,
    },
    Page {
        slug: "skeleton",
        title: "Skeleton",
        group: Group::Components,
    },
    Page {
        slug: "slider",
        title: "Slider",
        group: Group::Components,
    },
    Page {
        slug: "spinner",
        title: "Spinner",
        group: Group::Components,
    },
    Page {
        slug: "switch",
        title: "Switch",
        group: Group::Components,
    },
    Page {
        slug: "table",
        title: "Table",
        group: Group::Components,
    },
    Page {
        slug: "tabs",
        title: "Tabs",
        group: Group::Components,
    },
    Page {
        slug: "textarea",
        title: "Textarea",
        group: Group::Components,
    },
    Page {
        slug: "toast",
        title: "Toast",
        group: Group::Components,
    },
    Page {
        slug: "toggle-group",
        title: "Toggle Group",
        group: Group::Components,
    },
    Page {
        slug: "toggle",
        title: "Toggle",
        group: Group::Components,
    },
    Page {
        slug: "tooltip",
        title: "Tooltip",
        group: Group::Components,
    },
    Page {
        slug: "trace",
        title: "Trace",
        group: Group::Components,
    },
    Page {
        slug: "typography",
        title: "Typography",
        group: Group::Components,
    },
    Page {
        slug: "chain-of-thought",
        title: "Chain of Thought",
        group: Group::AiElements,
    },
    Page {
        slug: "chat",
        title: "Chat",
        group: Group::AiElements,
    },
    Page {
        slug: "checkpoint",
        title: "Checkpoint",
        group: Group::AiElements,
    },
    Page {
        slug: "confirmation",
        title: "Confirmation",
        group: Group::AiElements,
    },
    Page {
        slug: "context",
        title: "Context",
        group: Group::AiElements,
    },
    Page {
        slug: "file-tree",
        title: "File Tree",
        group: Group::AiElements,
    },
    Page {
        slug: "inline-citation",
        title: "Inline Citation",
        group: Group::AiElements,
    },
    Page {
        slug: "model-selector",
        title: "Model Selector",
        group: Group::AiElements,
    },
    Page {
        slug: "plan",
        title: "Plan",
        group: Group::AiElements,
    },
    Page {
        slug: "prompt-input",
        title: "Prompt Input",
        group: Group::AiElements,
    },
    Page {
        slug: "queue",
        title: "Queue",
        group: Group::AiElements,
    },
    Page {
        slug: "reasoning",
        title: "Reasoning",
        group: Group::AiElements,
    },
    Page {
        slug: "sources",
        title: "Sources",
        group: Group::AiElements,
    },
    Page {
        slug: "stack-trace",
        title: "Stack Trace",
        group: Group::AiElements,
    },
    Page {
        slug: "subagent",
        title: "Subagent",
        group: Group::AiElements,
    },
    Page {
        slug: "suggestion",
        title: "Suggestion",
        group: Group::AiElements,
    },
    Page {
        slug: "task",
        title: "Task",
        group: Group::AiElements,
    },
    Page {
        slug: "terminal",
        title: "Terminal",
        group: Group::AiElements,
    },
    Page {
        slug: "test-results",
        title: "Test Results",
        group: Group::AiElements,
    },
    Page {
        slug: "tool",
        title: "Tool",
        group: Group::AiElements,
    },
];

#[component]
pub fn PageExamples(slug: String) -> Element {
    match slug.as_str() {
        "accordion" => rsx! { accordion::Examples {} },
        "alert-dialog" => rsx! { alert_dialog::Examples {} },
        "alert" => rsx! { alert::Examples {} },
        "aspect-ratio" => rsx! { aspect_ratio::Examples {} },
        "attachment" => rsx! { attachment::Examples {} },
        "avatar" => rsx! { avatar::Examples {} },
        "badge" => rsx! { badge::Examples {} },
        "breadcrumb" => rsx! { breadcrumb::Examples {} },
        "bubble" => rsx! { bubble::Examples {} },
        "button-group" => rsx! { button_group::Examples {} },
        "button" => rsx! { button::Examples {} },
        "calendar" => rsx! { calendar::Examples {} },
        "card" => rsx! { card::Examples {} },
        "carousel" => rsx! { carousel::Examples {} },
        "chart" => rsx! { chart::Examples {} },
        "checkbox" => rsx! { checkbox::Examples {} },
        "code-block" => rsx! { code_block::Examples {} },
        "collapsible" => rsx! { collapsible::Examples {} },
        "combobox" => rsx! { combobox::Examples {} },
        "command" => rsx! { command::Examples {} },
        "context-menu" => rsx! { context_menu::Examples {} },
        "data-table" => rsx! { data_table::Examples {} },
        "date-picker" => rsx! { date_picker::Examples {} },
        "dialog" => rsx! { dialog::Examples {} },
        "direction" => rsx! { direction::Examples {} },
        "drawer" => rsx! { drawer::Examples {} },
        "dropdown-menu" => rsx! { dropdown_menu::Examples {} },
        "empty" => rsx! { empty::Examples {} },
        "field" => rsx! { field::Examples {} },
        "hover-card" => rsx! { hover_card::Examples {} },
        "input-group" => rsx! { input_group::Examples {} },
        "input-otp" => rsx! { input_otp::Examples {} },
        "input" => rsx! { input::Examples {} },
        "item" => rsx! { item::Examples {} },
        "kbd" => rsx! { kbd::Examples {} },
        "label" => rsx! { label::Examples {} },
        "marker" => rsx! { marker::Examples {} },
        "menubar" => rsx! { menubar::Examples {} },
        "message-scroller" => rsx! { message_scroller::Examples {} },
        "message" => rsx! { message::Examples {} },
        "native-select" => rsx! { native_select::Examples {} },
        "navigation-menu" => rsx! { navigation_menu::Examples {} },
        "pagination" => rsx! { pagination::Examples {} },
        "popover" => rsx! { popover::Examples {} },
        "progress" => rsx! { progress::Examples {} },
        "questionnaire" => rsx! { questionnaire::Examples {} },
        "radio-group" => rsx! { radio_group::Examples {} },
        "resizable" => rsx! { resizable::Examples {} },
        "scroll-area" => rsx! { scroll_area::Examples {} },
        "select" => rsx! { select::Examples {} },
        "separator" => rsx! { separator::Examples {} },
        "sheet" => rsx! { sheet::Examples {} },
        "sidebar" => rsx! { sidebar::Examples {} },
        "skeleton" => rsx! { skeleton::Examples {} },
        "slider" => rsx! { slider::Examples {} },
        "spinner" => rsx! { spinner::Examples {} },
        "switch" => rsx! { switch::Examples {} },
        "table" => rsx! { table::Examples {} },
        "tabs" => rsx! { tabs::Examples {} },
        "textarea" => rsx! { textarea::Examples {} },
        "toast" => rsx! { toast::Examples {} },
        "toggle-group" => rsx! { toggle_group::Examples {} },
        "toggle" => rsx! { toggle::Examples {} },
        "tooltip" => rsx! { tooltip::Examples {} },
        "trace" => rsx! { trace::Examples {} },
        "typography" => rsx! { typography::Examples {} },
        "chain-of-thought" => rsx! { chain_of_thought::Examples {} },
        "chat" => rsx! { chat::Examples {} },
        "checkpoint" => rsx! { checkpoint::Examples {} },
        "confirmation" => rsx! { confirmation::Examples {} },
        "context" => rsx! { context::Examples {} },
        "file-tree" => rsx! { file_tree::Examples {} },
        "inline-citation" => rsx! { inline_citation::Examples {} },
        "model-selector" => rsx! { model_selector::Examples {} },
        "plan" => rsx! { plan::Examples {} },
        "prompt-input" => rsx! { prompt_input::Examples {} },
        "queue" => rsx! { queue::Examples {} },
        "reasoning" => rsx! { reasoning::Examples {} },
        "sources" => rsx! { sources::Examples {} },
        "stack-trace" => rsx! { stack_trace::Examples {} },
        "subagent" => rsx! { subagent::Examples {} },
        "suggestion" => rsx! { suggestion::Examples {} },
        "task" => rsx! { task::Examples {} },
        "terminal" => rsx! { terminal::Examples {} },
        "test-results" => rsx! { test_results::Examples {} },
        "tool" => rsx! { tool::Examples {} },
        _ => rsx! { NotPorted {} },
    }
}

/// One example, framed like a preview on ui.shadcn.com.
#[component]
pub fn Example(
    title: String,
    /// Extra classes for the preview area, like upstream's `previewClassName`.
    #[props(default)]
    class: String,
    children: Element,
) -> Element {
    rsx! {
        section { class: "flex flex-col gap-3",
            h3 { class: "text-sm font-medium text-muted-foreground", "{title}" }
            div { class: "flex min-h-[200px] w-full items-center justify-center rounded-lg border p-6 md:p-10 {class}",
                {children}
            }
        }
    }
}

#[component]
fn NotPorted() -> Element {
    rsx! {
        Example { title: "Not ported yet",
            p { class: "text-sm text-muted-foreground", "The examples for this page are not ported yet." }
        }
    }
}
