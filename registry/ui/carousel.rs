//! Carousel: a horizontal or vertical slideshow that moves through a set of items.
//!
//! Vertical carousels need a height on `CarouselContent` (for example `h-[200px]`),
//! because the items fill the visible height of the content.
//!
//! Installed by oniko-cn. This file is yours: edit it freely.

use std::sync::atomic::{AtomicUsize, Ordering};

use dioxus::prelude::*;
use oniko_cn::{cn, take_class};

use super::button::{ButtonSize, ButtonVariant, button_variants};

static NEXT_ID: AtomicUsize = AtomicUsize::new(0);

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum CarouselOrientation {
    #[default]
    Horizontal,
    Vertical,
}

impl CarouselOrientation {
    fn name(self) -> &'static str {
        match self {
            Self::Horizontal => "horizontal",
            Self::Vertical => "vertical",
        }
    }
}

#[derive(Clone, Copy)]
struct CarouselContext {
    index: Signal<usize>,
    items: Signal<Vec<usize>>,
    orientation: Signal<CarouselOrientation>,
}

impl CarouselContext {
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

/// A signal that follows a prop, so context shared with children never goes stale.
fn use_prop<T: Clone + PartialEq + 'static>(value: T) -> Signal<T> {
    let mut signal = use_signal(|| value.clone());
    if *signal.peek() != value {
        signal.set(value);
    }
    signal
}

#[component]
pub fn Carousel(
    #[props(default)] orientation: CarouselOrientation,
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!("relative w-full", take_class(&mut attributes));
    let index = use_signal(|| 0usize);
    let items = use_signal(Vec::new);
    let orientation = use_prop(orientation);
    let context = CarouselContext {
        index,
        items,
        orientation,
    };
    use_context_provider(|| context);

    rsx! {
        div {
            "data-slot": "carousel",
            role: "region",
            aria_roledescription: "carousel",
            "data-orientation": orientation().name(),
            class: class,
            onkeydown: move |event| {
                let (prev, next) = match *orientation.peek() {
                    CarouselOrientation::Horizontal => (Key::ArrowLeft, Key::ArrowRight),
                    CarouselOrientation::Vertical => (Key::ArrowUp, Key::ArrowDown),
                };
                if event.key() == prev {
                    event.prevent_default();
                    context.prev();
                } else if event.key() == next {
                    event.prevent_default();
                    context.next();
                }
            },
            ..attributes,
            {children}
        }
    }
}

#[component]
pub fn CarouselContent(
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let root = use_context::<CarouselContext>();
    let orientation = (root.orientation)();
    let current = root.current();

    // The track is one viewport wider (or taller) than the content, so each item
    // carries its own gutter as padding and the first item still sits flush.
    let (track_class, transform) = match orientation {
        CarouselOrientation::Horizontal => (
            "-ml-4 flex",
            format!("translate3d(calc(-{current} * 100%), 0, 0)"),
        ),
        CarouselOrientation::Vertical => (
            "-mt-4 flex h-[calc(100%_+_1rem)] flex-col",
            format!("translate3d(0, calc(-{current} * 100%), 0)"),
        ),
    };
    let class = cn!("overflow-hidden", take_class(&mut attributes));
    let track = format!("{track_class} transition-transform duration-300 ease-out");

    rsx! {
        div {
            "data-slot": "carousel-content",
            "data-orientation": orientation.name(),
            class: class,
            ..attributes,
            div {
                "data-slot": "carousel-track",
                class: track,
                style: "transform: {transform}",
                {children}
            }
        }
    }
}

#[component]
pub fn CarouselItem(
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let root = use_context::<CarouselContext>();
    let id = use_hook(|| NEXT_ID.fetch_add(1, Ordering::Relaxed));
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

    let padding = match (root.orientation)() {
        CarouselOrientation::Horizontal => "pl-4",
        CarouselOrientation::Vertical => "pt-4",
    };
    let class = cn!(
        "min-w-0 shrink-0 grow-0 basis-full",
        padding,
        take_class(&mut attributes)
    );

    rsx! {
        div {
            "data-slot": "carousel-item",
            role: "group",
            aria_roledescription: "slide",
            class: class,
            ..attributes,
            {children}
        }
    }
}

#[component]
pub fn CarouselPrevious(
    onclick: Option<EventHandler<MouseEvent>>,
    #[props(extends = GlobalAttributes, extends = button)] attributes: Vec<Attribute>,
) -> Element {
    let mut attributes = attributes;
    let root = use_context::<CarouselContext>();
    let vertical = (root.orientation)() == CarouselOrientation::Vertical;
    let position = if vertical {
        "-top-12 left-1/2 -translate-x-1/2"
    } else {
        "top-1/2 -left-12 -translate-y-1/2"
    };
    let class = cn!(
        button_variants(ButtonVariant::Outline, ButtonSize::Icon, None),
        "cn-carousel-previous absolute",
        position,
        take_class(&mut attributes)
    );
    let disabled = root.current() == 0;
    let icon = if vertical {
        ("M12 19V5", "m5 12 7-7 7 7")
    } else {
        ("M19 12H5", "m12 19-7-7 7-7")
    };

    rsx! {
        button {
            "data-slot": "carousel-previous",
            r#type: "button",
            disabled: disabled,
            class: class,
            onclick: move |event| {
                root.prev();
                if let Some(handler) = onclick {
                    handler.call(event);
                }
            },
            ..attributes,
            ArrowIcon { shaft: icon.0, head: icon.1 }
        }
    }
}

#[component]
pub fn CarouselNext(
    onclick: Option<EventHandler<MouseEvent>>,
    #[props(extends = GlobalAttributes, extends = button)] attributes: Vec<Attribute>,
) -> Element {
    let mut attributes = attributes;
    let root = use_context::<CarouselContext>();
    let vertical = (root.orientation)() == CarouselOrientation::Vertical;
    let position = if vertical {
        "-bottom-12 left-1/2 -translate-x-1/2"
    } else {
        "top-1/2 -right-12 -translate-y-1/2"
    };
    let class = cn!(
        button_variants(ButtonVariant::Outline, ButtonSize::Icon, None),
        "cn-carousel-next absolute",
        position,
        take_class(&mut attributes)
    );
    let disabled = root.current() + 1 >= root.count();
    let icon = if vertical {
        ("M12 5v14", "m19 12-7 7-7-7")
    } else {
        ("M5 12h14", "m12 5 7 7-7 7")
    };

    rsx! {
        button {
            "data-slot": "carousel-next",
            r#type: "button",
            disabled: disabled,
            class: class,
            onclick: move |event| {
                root.next();
                if let Some(handler) = onclick {
                    handler.call(event);
                }
            },
            ..attributes,
            ArrowIcon { shaft: icon.0, head: icon.1 }
        }
    }
}

#[component]
fn ArrowIcon(shaft: &'static str, head: &'static str) -> Element {
    rsx! {
        svg {
            class: "pointer-events-none size-4 shrink-0",
            view_box: "0 0 24 24",
            fill: "none",
            stroke: "currentColor",
            stroke_width: "2",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            path { d: shaft }
            path { d: head }
        }
    }
}
