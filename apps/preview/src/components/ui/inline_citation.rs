//! InlineCitation: a source badge inside running text that opens a hover card, with an optional carousel for several sources.
//!
//! The badge shows the hostname of the first source and a `+N` count for the rest. The carousel keeps its own
//! index, because the `Carousel` component does not expose its state.
//!
//! Installed by oniko-cn. This file is yours: edit it freely.

use std::sync::atomic::{AtomicUsize, Ordering};

use dioxus::prelude::*;
use oniko_cn::{cn, take_class};

use super::badge::{Badge, BadgeVariant};
use super::hover_card::{HoverCard, HoverCardContent, HoverCardTrigger};

static NEXT_ITEM_ID: AtomicUsize = AtomicUsize::new(0);

/// Cuts the host out of a URL by hand, because the registry has no URL parser.
fn hostname(url: &str) -> &str {
    let rest = url.split_once("://").map_or(url, |(_, rest)| rest);
    let authority = rest.split(['/', '?', '#']).next().unwrap_or(rest);
    let host = authority
        .rsplit_once('@')
        .map_or(authority, |(_, host)| host);
    host.split(':').next().unwrap_or(host)
}

fn source_label(sources: &[String]) -> String {
    match sources {
        [] => "unknown".to_string(),
        [first] => hostname(first).to_string(),
        [first, rest @ ..] => format!("{} +{}", hostname(first), rest.len()),
    }
}

#[derive(Clone, Copy)]
struct CarouselState {
    index: Signal<usize>,
    items: Signal<Vec<usize>>,
}

impl CarouselState {
    fn count(self) -> usize {
        self.items.read().len()
    }

    fn current(self) -> usize {
        (*self.index.read()).min(self.count().saturating_sub(1))
    }

    fn go_to(mut self, index: usize) {
        let last = self.count().saturating_sub(1);
        self.index.set(index.min(last));
    }

    fn prev(self) {
        let current = self.current();
        if current > 0 {
            self.go_to(current - 1);
        }
    }

    fn next(self) {
        self.go_to(self.current() + 1);
    }
}

/// Wraps the phrase that a citation refers to and the badge that follows it.
#[component]
pub fn InlineCitation(
    #[props(extends = GlobalAttributes, extends = span)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!("group inline", take_class(&mut attributes).as_deref());

    rsx! {
        span {
            "data-slot": "inline-citation",
            class: class,
            ..attributes,
            {children}
        }
    }
}

/// The cited phrase. It is highlighted while the citation is hovered.
#[component]
pub fn InlineCitationText(
    #[props(extends = GlobalAttributes, extends = span)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(
        "transition-colors group-hover:bg-accent",
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        span {
            "data-slot": "inline-citation-text",
            class: class,
            ..attributes,
            {children}
        }
    }
}

/// Owns the hover card for one citation. It opens and closes without delay.
///
/// It takes no HTML attributes: Dioxus 0.7 cannot forward them to another component.
#[component]
pub fn InlineCitationCard(
    /// Controlled open state. When set, `on_open_change` is the only way to change it.
    open: Option<bool>,
    /// Open state at first render when `open` is not controlled.
    #[props(default)]
    default_open: bool,
    on_open_change: Option<EventHandler<bool>>,
    children: Element,
) -> Element {
    rsx! {
        HoverCard {
            open: open,
            default_open: default_open,
            on_open_change: on_open_change,
            open_delay: 0,
            close_delay: 0,
            {children}
        }
    }
}

/// The badge that shows the first source's hostname, or `+N` for the rest. It opens the card.
///
/// It takes no HTML attributes, for the same reason as `InlineCitationCard`.
#[component]
pub fn InlineCitationCardTrigger(sources: Vec<String>) -> Element {
    let label = source_label(&sources);

    rsx! {
        HoverCardTrigger {
            Badge {
                variant: BadgeVariant::Secondary,
                class: "ml-1 rounded-full",
                "{label}"
            }
        }
    }
}

/// The floating panel with the sources. Shown only while the card is open.
///
/// It takes no HTML attributes, for the same reason as `InlineCitationCard`.
#[component]
pub fn InlineCitationCardBody(children: Element) -> Element {
    rsx! {
        HoverCardContent {
            class: "w-80 p-0",
            {children}
        }
    }
}

/// Shares the index and the registered slides between the carousel parts.
#[derive(Clone, Copy)]
struct CarouselContext(CarouselState);

#[component]
pub fn InlineCitationCarousel(
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!("relative w-full", take_class(&mut attributes).as_deref());
    let index = use_signal(|| 0usize);
    let items = use_signal(Vec::new);
    let state = CarouselState { index, items };
    use_context_provider(|| CarouselContext(state));

    rsx! {
        div {
            "data-slot": "inline-citation-carousel",
            role: "region",
            aria_roledescription: "carousel",
            class: class,
            onkeydown: move |event| {
                if event.key() == Key::ArrowLeft {
                    event.prevent_default();
                    state.prev();
                } else if event.key() == Key::ArrowRight {
                    event.prevent_default();
                    state.next();
                }
            },
            ..attributes,
            {children}
        }
    }
}

/// The row above the slides, holding the index and the previous and next buttons.
#[component]
pub fn InlineCitationCarouselHeader(
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(
        "flex items-center justify-between gap-2 rounded-t-md bg-secondary p-2",
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        div {
            "data-slot": "inline-citation-carousel-header",
            class: class,
            ..attributes,
            {children}
        }
    }
}

/// Holds the slides. Only the current slide is visible.
#[component]
pub fn InlineCitationCarouselContent(
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!("overflow-hidden", take_class(&mut attributes).as_deref());

    rsx! {
        div {
            "data-slot": "inline-citation-carousel-content",
            class: class,
            ..attributes,
            {children}
        }
    }
}

/// One slide. Slides register in the order they mount.
#[component]
pub fn InlineCitationCarouselItem(
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let CarouselContext(root) = use_context::<CarouselContext>();
    let id = use_hook(|| NEXT_ITEM_ID.fetch_add(1, Ordering::Relaxed));
    let mut items = root.items;
    use_effect(move || {
        if !items.peek().contains(&id) {
            items.write().push(id);
        }
    });
    use_drop(move || {
        if let Ok(mut list) = items.try_write() {
            list.retain(|v| *v != id);
        }
    });

    let position = root.items.read().iter().position(|v| *v == id);
    let hidden = (position != Some(root.current())).then_some("hidden");
    let class = cn!(
        "w-full space-y-2 p-4 pl-8",
        hidden,
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        div {
            "data-slot": "inline-citation-carousel-item",
            class: class,
            ..attributes,
            {children}
        }
    }
}

/// Shows `current/total`. Pass children to replace the default text.
#[component]
pub fn InlineCitationCarouselIndex(
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Option<Element>,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(
        "flex flex-1 items-center justify-end px-3 py-1 text-xs text-muted-foreground",
        take_class(&mut attributes).as_deref()
    );
    let CarouselContext(root) = use_context::<CarouselContext>();
    let count = root.count();
    let label = if count == 0 {
        "0/0".to_string()
    } else {
        format!("{}/{count}", root.current() + 1)
    };

    rsx! {
        div {
            "data-slot": "inline-citation-carousel-index",
            class: class,
            ..attributes,
            {children.unwrap_or_else(|| rsx! { "{label}" })}
        }
    }
}

fn arrow_icon(path: &'static str) -> Element {
    rsx! {
        svg {
            "data-slot": "inline-citation-carousel-icon",
            class: "size-4 text-muted-foreground",
            view_box: "0 0 24 24",
            fill: "none",
            stroke: "currentColor",
            stroke_width: "2",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            "aria-hidden": "true",
            path { d: path }
        }
    }
}

/// Goes to the previous slide. Disabled on the first one.
#[component]
pub fn InlineCitationCarouselPrev(
    onclick: Option<EventHandler<MouseEvent>>,
    #[props(extends = GlobalAttributes, extends = button)] attributes: Vec<Attribute>,
) -> Element {
    let mut attributes = attributes;
    let class = cn!("shrink-0", take_class(&mut attributes).as_deref());
    let CarouselContext(root) = use_context::<CarouselContext>();
    let disabled = root.current() == 0;

    rsx! {
        button {
            "data-slot": "inline-citation-carousel-prev",
            r#type: "button",
            aria_label: "Previous",
            disabled: disabled,
            class: class,
            onclick: move |event| {
                root.prev();
                if let Some(handler) = onclick {
                    handler.call(event);
                }
            },
            ..attributes,
            {arrow_icon("M19 12H5 M12 19l-7-7 7-7")}
        }
    }
}

/// Goes to the next slide. Disabled on the last one.
#[component]
pub fn InlineCitationCarouselNext(
    onclick: Option<EventHandler<MouseEvent>>,
    #[props(extends = GlobalAttributes, extends = button)] attributes: Vec<Attribute>,
) -> Element {
    let mut attributes = attributes;
    let class = cn!("shrink-0", take_class(&mut attributes).as_deref());
    let CarouselContext(root) = use_context::<CarouselContext>();
    let disabled = root.current() + 1 >= root.count();

    rsx! {
        button {
            "data-slot": "inline-citation-carousel-next",
            r#type: "button",
            aria_label: "Next",
            disabled: disabled,
            class: class,
            onclick: move |event| {
                root.next();
                if let Some(handler) = onclick {
                    handler.call(event);
                }
            },
            ..attributes,
            {arrow_icon("M5 12h14 M12 5l7 7-7 7")}
        }
    }
}

/// A source: its title, URL and description. Children are rendered after them.
#[component]
pub fn InlineCitationSource(
    title: Option<String>,
    url: Option<String>,
    description: Option<String>,
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Option<Element>,
) -> Element {
    let mut attributes = attributes;
    let class = cn!("space-y-1", take_class(&mut attributes).as_deref());

    rsx! {
        div {
            "data-slot": "inline-citation-source",
            class: class,
            ..attributes,
            if let Some(title) = title {
                h4 {
                    "data-slot": "inline-citation-source-title",
                    class: "truncate text-sm font-medium leading-tight",
                    "{title}"
                }
            }
            if let Some(url) = url {
                p {
                    "data-slot": "inline-citation-source-url",
                    class: "truncate text-xs break-all text-muted-foreground",
                    "{url}"
                }
            }
            if let Some(description) = description {
                p {
                    "data-slot": "inline-citation-source-description",
                    class: "line-clamp-3 text-sm leading-relaxed text-muted-foreground",
                    "{description}"
                }
            }
            {children.unwrap_or_else(|| rsx! {})}
        }
    }
}

/// A quoted passage from a source.
#[component]
pub fn InlineCitationQuote(
    #[props(extends = GlobalAttributes, extends = blockquote)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(
        "border-muted border-l-2 pl-3 text-sm italic text-muted-foreground",
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        blockquote {
            "data-slot": "inline-citation-quote",
            class: class,
            ..attributes,
            {children}
        }
    }
}
