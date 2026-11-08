//! Collapsible: an interactive component which expands and collapses a panel.
//!
//! The panel keeps Base UI's animation hooks: `--collapsible-panel-height`, `data-starting-style`
//! and `data-ending-style`, so a class such as `h-(--collapsible-panel-height)` with
//! `data-starting-style:h-0 data-ending-style:h-0` animates the height.
//!
//! Installed by oniko-cn. This file is yours: edit it freely.

use std::sync::atomic::{AtomicUsize, Ordering};

use dioxus::prelude::*;
use oniko_cn::{cn, take_class};

use super::button::{ButtonSize, ButtonVariant, button_variants};

static NEXT_ID: AtomicUsize = AtomicUsize::new(0);

/// Waits (up to 40 frames) until `ok(el)` holds for the element with `__ID__`, then runs `__BODY__`.
const WAIT_SCRIPT: &str = r#"(async () => {
  const ok = (el) => __COND__;
  let el = null;
  for (let n = 0; n < 40; n++) {
    el = document.getElementById("__ID__");
    if (ok(el)) break;
    await new Promise((resolve) => requestAnimationFrame(resolve));
  }
  __BODY__
})();"#;

const SETTLE_BODY: &str = "if (el) { await Promise.all(el.getAnimations({ subtree: true }).map((a) => a.finished.catch(() => {}))); } dioxus.send(1);";

const MEASURE_BODY: &str = "dioxus.send(el ? el.scrollHeight : 0);";

fn script(id: &str, cond: &str, body: &str) -> String {
    WAIT_SCRIPT
        .replace("__COND__", cond)
        .replace("__BODY__", body)
        .replace("__ID__", id)
}

async fn run(js: String) -> Option<f64> {
    let mut eval = document::eval(&js);
    eval.recv::<f64>().await.ok()
}

fn put<T: 'static>(mut signal: Signal<T>, value: T) {
    if let Ok(mut slot) = signal.try_write() {
        *slot = value;
    }
}

/// A signal that follows a prop, so context shared with children never goes stale.
pub fn use_prop<T: Clone + PartialEq + 'static>(value: T) -> Signal<T> {
    let mut signal = use_signal(|| value.clone());
    if *signal.peek() != value {
        signal.set(value);
    }
    signal
}

/// Presence and animation state of a collapsible panel, shared with `AccordionContent`.
#[derive(Clone, Copy)]
pub struct PanelTransition {
    mounted: Signal<bool>,
    shown: Signal<bool>,
    starting: Signal<bool>,
    ending: Signal<bool>,
    height: Signal<Option<f64>>,
}

impl PanelTransition {
    /// False once the exit animation has finished.
    pub fn mounted(self) -> bool {
        (self.mounted)()
    }

    /// Drives `data-open` / `data-closed`; lags `open` while the panel is being measured.
    pub fn shown(self) -> bool {
        (self.shown)()
    }

    pub fn starting(self) -> bool {
        (self.starting)()
    }

    pub fn ending(self) -> bool {
        (self.ending)()
    }

    /// Value for the panel height custom property.
    pub fn height_css(self) -> String {
        match (self.height)() {
            Some(height) => format!("{height}px"),
            None => "auto".to_string(),
        }
    }
}

/// Runs the open and close animation of the element `dom_id` whenever `open` changes.
/// `var` is the name of the panel height custom property the element carries.
pub fn use_panel_transition(dom_id: String, var: &'static str, open: bool) -> PanelTransition {
    let state = PanelTransition {
        mounted: use_signal(|| open),
        shown: use_signal(|| open),
        starting: use_signal(|| false),
        ending: use_signal(|| false),
        height: use_signal(|| None),
    };
    let target = use_prop(open);
    let id = use_prop(dom_id);
    let epoch = use_signal(|| 0u64);
    let mut first = use_signal(|| true);

    use_effect(move || {
        let open = target();
        if *first.peek() {
            first.set(false);
            return;
        }
        let id = id.peek().clone();
        let mut epoch = epoch;
        let mine = *epoch.peek() + 1;
        epoch.set(mine);
        let alive = move || epoch.try_peek().map(|e| *e == mine).unwrap_or(false);

        spawn(async move {
            if open {
                put(state.mounted, true);
                put(state.shown, true);
                put(state.ending, false);
                put(state.height, None);
                put(state.starting, true);
                let measure = script(
                    &id,
                    "el && el.hasAttribute('data-starting-style')",
                    MEASURE_BODY,
                );
                let height = run(measure).await.unwrap_or(0.0);
                if !alive() {
                    return;
                }
                put(state.height, Some(height));
                put(state.starting, false);
                let settled = script(
                    &id,
                    "!el || !el.hasAttribute('data-starting-style')",
                    SETTLE_BODY,
                );
                let _ = run(settled).await;
                if alive() {
                    put(state.height, None);
                }
            } else {
                let height = run(script(&id, "true", MEASURE_BODY)).await.unwrap_or(0.0);
                if !alive() {
                    return;
                }
                put(state.height, Some(height));
                let applied = script(
                    &id,
                    &format!("!el || parseFloat(el.style.getPropertyValue('{var}')) === {height}"),
                    "dioxus.send(1);",
                );
                let _ = run(applied).await;
                if !alive() {
                    return;
                }
                put(state.shown, false);
                put(state.ending, true);
                let settled = script(
                    &id,
                    "!el || el.hasAttribute('data-ending-style')",
                    SETTLE_BODY,
                );
                let _ = run(settled).await;
                if alive() {
                    put(state.mounted, false);
                    put(state.ending, false);
                    put(state.height, None);
                }
            }
        });
    });

    state
}

/// Calls `on_match` when the browser finds text inside the hidden panel with find-in-page.
pub fn use_before_match(dom_id: String, enabled: bool, on_match: EventHandler<()>) {
    use_effect(move || {
        if !enabled {
            return;
        }
        let js = format!(
            r#"(() => {{
  const el = document.getElementById("{dom_id}");
  if (el) el.addEventListener("beforematch", () => dioxus.send(1));
}})();"#
        );
        spawn(async move {
            let mut eval = document::eval(&js);
            while eval.recv::<f64>().await.is_ok() {
                on_match.call(());
            }
        });
    });
}

#[derive(Clone, Copy)]
struct CollapsibleContext {
    internal: Signal<bool>,
    controlled: Signal<Option<bool>>,
    disabled: Signal<bool>,
    on_open_change: Option<EventHandler<bool>>,
    id: usize,
}

impl CollapsibleContext {
    fn is_open(self) -> bool {
        (self.controlled)().unwrap_or_else(|| (self.internal)())
    }

    fn set_open(mut self, next: bool) {
        if self.is_open() == next {
            return;
        }
        if self.controlled.peek().is_none() {
            self.internal.set(next);
        }
        if let Some(handler) = self.on_open_change {
            handler.call(next);
        }
    }
}

fn state(open: bool) -> &'static str {
    if open { "open" } else { "closed" }
}

#[component]
pub fn Collapsible(
    /// Controlled open state. When set, `on_open_change` is the only way to change it.
    open: Option<bool>,
    /// Open state at first render when `open` is not controlled.
    #[props(default)]
    default_open: bool,
    #[props(default)] disabled: bool,
    on_open_change: Option<EventHandler<bool>>,
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(take_class(&mut attributes));
    let internal = use_signal(|| default_open);
    let controlled = use_prop(open);
    let disabled = use_prop(disabled);
    let id = use_hook(|| NEXT_ID.fetch_add(1, Ordering::Relaxed));
    let context = use_context_provider(|| CollapsibleContext {
        internal,
        controlled,
        disabled,
        on_open_change,
        id,
    });

    let open = context.is_open();
    rsx! {
        div {
            "data-slot": "collapsible",
            "data-state": state(open),
            "data-open": open.then_some(""),
            "data-closed": (!open).then_some(""),
            "data-disabled": disabled().then_some(""),
            class: class,
            ..attributes,
            {children}
        }
    }
}

/// The toggle button. Set `variant` or `size` to give it the look of a `Button`,
/// which is what upstream does with `render={<Button />}`.
#[component]
pub fn CollapsibleTrigger(
    variant: Option<ButtonVariant>,
    size: Option<ButtonSize>,
    onclick: Option<EventHandler<MouseEvent>>,
    #[props(extends = GlobalAttributes, extends = button)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let taken = take_class(&mut attributes);
    let class = if variant.is_some() || size.is_some() {
        button_variants(
            variant.unwrap_or_default(),
            size.unwrap_or_default(),
            taken.as_deref(),
        )
    } else {
        cn!(taken)
    };
    let context = use_context::<CollapsibleContext>();
    let open = context.is_open();
    let disabled = (context.disabled)();
    let id = context.id;

    rsx! {
        button {
            "data-slot": "collapsible-trigger",
            "data-state": state(open),
            "data-panel-open": open.then_some(""),
            "data-disabled": disabled.then_some(""),
            r#type: "button",
            aria_controls: "collapsible-content-{id}",
            aria_expanded: open,
            disabled: disabled,
            class: class,
            onclick: move |event| {
                if disabled {
                    return;
                }
                if let Some(handler) = onclick {
                    handler.call(event);
                }
                context.set_open(!open);
            },
            ..attributes,
            {children}
        }
    }
}

#[component]
pub fn CollapsibleContent(
    /// Keep the panel in the DOM, hidden, while it is closed.
    #[props(default)]
    keep_mounted: bool,
    /// Hide the panel with `hidden="until-found"` so find-in-page can open it.
    #[props(default)]
    hidden_until_found: bool,
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(take_class(&mut attributes));
    let context = use_context::<CollapsibleContext>();
    let open = context.is_open();
    let dom_id = format!("collapsible-content-{}", context.id);
    let transition = use_panel_transition(dom_id.clone(), "--collapsible-panel-height", open);
    use_before_match(
        dom_id.clone(),
        hidden_until_found,
        EventHandler::new(move |_| context.set_open(true)),
    );

    let mounted = transition.mounted();
    if !mounted && !keep_mounted && !hidden_until_found {
        return rsx! {};
    }
    let shown = transition.shown();
    let hidden = if mounted {
        None
    } else if hidden_until_found {
        Some("until-found")
    } else {
        Some("true")
    };
    let height = transition.height_css();

    rsx! {
        div {
            "data-slot": "collapsible-content",
            "data-state": state(shown),
            "data-open": shown.then_some(""),
            "data-closed": (!shown).then_some(""),
            "data-starting-style": transition.starting().then_some(""),
            "data-ending-style": transition.ending().then_some(""),
            "data-disabled": (context.disabled)().then_some(""),
            id: dom_id,
            hidden: hidden,
            style: "--collapsible-panel-height: {height}",
            class: class,
            ..attributes,
            {children}
        }
    }
}
