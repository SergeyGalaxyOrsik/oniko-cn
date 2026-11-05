mod chain_of_thought;
mod chat;
mod checkpoint;
mod confirmation;
mod context;
mod file_tree;
mod inline_citation;
mod message;
mod model_selector;
mod plan;
mod prompt_input;
mod queue;
mod reasoning;
mod sources;
mod stack_trace;
mod subagent;
mod suggestion;
mod task;
mod terminal;
mod test_results;
mod tool;

use dioxus::prelude::*;

#[component]
pub fn AiDemos() -> Element {
    rsx! {
        h2 { class: "text-lg font-semibold", "AI Elements" }
        chain_of_thought::ChainOfThoughtDemo {}
        chat::ChatDemo {}
        checkpoint::CheckpointDemo {}
        confirmation::ConfirmationDemo {}
        context::ContextDemo {}
        file_tree::FileTreeDemo {}
        inline_citation::InlineCitationDemo {}
        message::MessageDemo {}
        model_selector::ModelSelectorDemo {}
        plan::PlanDemo {}
        prompt_input::PromptInputDemo {}
        queue::QueueDemo {}
        reasoning::ReasoningDemo {}
        sources::SourcesDemo {}
        stack_trace::StackTraceDemo {}
        subagent::SubagentDemo {}
        suggestion::SuggestionDemo {}
        task::TaskDemo {}
        terminal::TerminalDemo {}
        test_results::TestResultsDemo {}
        tool::ToolDemo {}
    }
}
