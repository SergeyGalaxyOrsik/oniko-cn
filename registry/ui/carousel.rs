//! Carousel: a slideshow with motion and swipe built using native scroll snapping.
//!
//! Vertical carousels need a height on `CarouselContent` (for example `h-[270px]`).
//! Read the state of the carousel, or move it, through the `CarouselApi` that `set_api` hands out.
//!
//! Installed by oniko-cn. This file is yours: edit it freely.

use std::sync::atomic::{AtomicUsize, Ordering};

use dioxus::prelude::*;
use oniko_cn::{cn, take_class};

use super::button::{Button, ButtonSize, ButtonVariant};

static NEXT_ID: AtomicUsize = AtomicUsize::new(0);

/// Computes the snap points, reports `[selected, count, canPrev, canNext]` to Rust on every change
/// and runs the commands Rust sends (`prev`, `next`, `to:<index>`, `stop`, `reset`).
/// `__ID__`, `__VERTICAL__`, `__ALIGN__`, `__LOOP__`, `__START__`, `__DELAY__`, `__STOP_ON_INTERACTION__`
/// and `__STOP_ON_ENTER__` are replaced before it runs.
const DRIVER_SCRIPT: &str = r#"(async () => {
  const root = document.getElementById("__ID__");
  if (!root) return;
  const view = root.querySelector('[data-slot="carousel-content"]');
  if (!view) return;
  const vertical = __VERTICAL__;
  const align = "__ALIGN__";
  const loop = __LOOP__;
  const delay = __DELAY__;
  const rtl = () => !vertical && getComputedStyle(root).direction === "rtl";
  const slides = () => Array.from(view.firstElementChild ? view.firstElementChild.children : []);
  const scrollOf = () => Math.abs(vertical ? view.scrollTop : view.scrollLeft);
  const snaps = () => {
    const box = view.getBoundingClientRect();
    const viewSize = vertical ? view.clientHeight : view.clientWidth;
    const max = Math.max(0, (vertical ? view.scrollHeight : view.scrollWidth) - viewSize);
    const out = [];
    for (const slide of slides()) {
      const r = slide.getBoundingClientRect();
      const size = vertical ? r.height : r.width;
      let start;
      if (vertical) start = r.top - box.top + view.scrollTop;
      else if (rtl()) start = box.right - r.right + Math.abs(view.scrollLeft);
      else start = r.left - box.left + view.scrollLeft;
      const shift = align === "start" ? 0 : align === "end" ? viewSize - size : (viewSize - size) / 2;
      const target = Math.min(max, Math.max(0, start - shift));
      if (out.length === 0 || Math.abs(out[out.length - 1] - target) > 1) out.push(target);
    }
    return out;
  };
  const selected = (list) => {
    const at = scrollOf();
    let best = 0;
    list.forEach((s, i) => { if (Math.abs(s - at) < Math.abs(list[best] - at)) best = i; });
    return best;
  };
  const report = () => {
    const list = snaps();
    const index = selected(list);
    dioxus.send([index, list.length, loop ? (list.length > 1 ? 1 : 0) : (index > 0 ? 1 : 0), loop ? (list.length > 1 ? 1 : 0) : (index < list.length - 1 ? 1 : 0)]);
  };
  const go = (index) => {
    const list = snaps();
    if (list.length === 0) return;
    let i = index;
    if (loop) i = (i + list.length) % list.length;
    i = Math.min(list.length - 1, Math.max(0, i));
    const to = list[i];
    const top = vertical ? to : 0;
    const left = vertical ? 0 : (rtl() ? -to : to);
    view.scrollTo({ top, left, behavior: "smooth" });
  };
  const initial = snaps();
  if (__START__ > 0 && initial.length > 0) {
    const to = initial[Math.min(__START__, initial.length - 1)];
    view.scrollTo({ top: vertical ? to : 0, left: vertical ? 0 : (rtl() ? -to : to), behavior: "instant" });
  }
  let frame = 0;
  view.addEventListener("scroll", () => {
    cancelAnimationFrame(frame);
    frame = requestAnimationFrame(report);
  }, { passive: true });
  const observer = new ResizeObserver(report);
  observer.observe(view);
  for (const slide of slides()) observer.observe(slide);
  new MutationObserver(() => { for (const slide of slides()) observer.observe(slide); report(); })
    .observe(view, { childList: true, subtree: true });
  report();

  let timer = 0;
  let stopped = false;
  const tick = () => {
    const list = snaps();
    const index = selected(list);
    go(index >= list.length - 1 ? 0 : index + 1);
  };
  const start = () => { clearInterval(timer); if (delay > 0 && !stopped) timer = setInterval(tick, delay); };
  const halt = () => { clearInterval(timer); timer = 0; };
  if (delay > 0) {
    start();
    if (__STOP_ON_INTERACTION__) {
      root.addEventListener("pointerdown", () => { stopped = true; halt(); });
      root.addEventListener("keydown", () => { stopped = true; halt(); });
    }
    if (__STOP_ON_ENTER__) {
      root.addEventListener("mouseenter", halt);
      root.addEventListener("mouseleave", start);
    }
  }

  while (true) {
    const command = await dioxus.recv();
    if (command === "prev") go(selected(snaps()) - 1);
    else if (command === "next") go(selected(snaps()) + 1);
    else if (command === "stop") { stopped = true; halt(); }
    else if (command === "reset") { stopped = false; start(); }
    else if (typeof command === "string" && command.startsWith("to:")) go(parseInt(command.slice(3), 10));
  }
})();"#;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum CarouselOrientation {
    #[default]
    Horizontal,
    Vertical,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum CarouselAlign {
    Start,
    #[default]
    Center,
    End,
}

impl CarouselAlign {
    fn name(self) -> &'static str {
        match self {
            Self::Start => "start",
            Self::Center => "center",
            Self::End => "end",
        }
    }
}

/// Options for the carousel, a subset of Embla's.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct CarouselOptions {
    /// Which edge of the viewport a slide snaps to. Defaults to `Center`.
    pub align: CarouselAlign,
    /// Wrap from the last slide to the first and back.
    pub r#loop: bool,
    /// Index of the slide shown first.
    pub start_index: usize,
}

/// Moves to the next slide on a timer, like Embla's autoplay plugin.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CarouselAutoplay {
    pub delay_ms: u32,
    /// Stop for good after the user presses a key or a pointer inside the carousel.
    pub stop_on_interaction: bool,
    /// Pause while the pointer is over the carousel.
    pub stop_on_mouse_enter: bool,
}

impl Default for CarouselAutoplay {
    fn default() -> Self {
        Self {
            delay_ms: 4000,
            stop_on_interaction: true,
            stop_on_mouse_enter: false,
        }
    }
}

#[derive(Clone, Copy)]
struct CarouselContext {
    selected: Signal<usize>,
    count: Signal<usize>,
    can_prev: Signal<bool>,
    can_next: Signal<bool>,
    commands: Signal<Option<document::Eval>>,
    orientation: Signal<CarouselOrientation>,
    align: Signal<CarouselAlign>,
}

impl CarouselContext {
    fn send(self, command: &str) {
        if let Some(eval) = self.commands.peek().as_ref() {
            let _ = eval.send(command);
        }
    }
}

/// Handle to a running carousel. Reading its values from a component subscribes to changes.
#[derive(Clone, Copy)]
pub struct CarouselApi {
    context: CarouselContext,
}

impl CarouselApi {
    pub fn can_scroll_prev(&self) -> bool {
        (self.context.can_prev)()
    }

    pub fn can_scroll_next(&self) -> bool {
        (self.context.can_next)()
    }

    /// Index of the snap point nearest to the current position.
    pub fn selected_scroll_snap(&self) -> usize {
        (self.context.selected)()
    }

    /// Number of snap points.
    pub fn scroll_snap_count(&self) -> usize {
        (self.context.count)()
    }

    pub fn scroll_prev(&self) {
        self.context.send("prev");
    }

    pub fn scroll_next(&self) {
        self.context.send("next");
    }

    pub fn scroll_to(&self, index: usize) {
        self.context.send(&format!("to:{index}"));
    }
}

/// The state of the enclosing `Carousel`.
pub fn use_carousel() -> CarouselApi {
    CarouselApi {
        context: use_context::<CarouselContext>(),
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
    #[props(default)] opts: CarouselOptions,
    /// Called once with a handle to the carousel.
    set_api: Option<EventHandler<CarouselApi>>,
    autoplay: Option<CarouselAutoplay>,
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!("relative", take_class(&mut attributes));
    let id = use_hook(|| NEXT_ID.fetch_add(1, Ordering::Relaxed));
    let dom_id = format!("carousel-{id}");
    let selected = use_signal(|| 0usize);
    let count = use_signal(|| 0usize);
    let can_prev = use_signal(|| false);
    let can_next = use_signal(|| false);
    let commands = use_signal(|| None::<document::Eval>);
    let orientation = use_prop(orientation);
    let align = use_prop(opts.align);
    let context = use_context_provider(|| CarouselContext {
        selected,
        count,
        can_prev,
        can_next,
        commands,
        orientation,
        align,
    });

    {
        let dom_id = dom_id.clone();
        use_effect(move || {
            let autoplay = autoplay;
            let js = DRIVER_SCRIPT
                .replace("__ID__", &dom_id)
                .replace(
                    "__VERTICAL__",
                    if *orientation.peek() == CarouselOrientation::Vertical {
                        "true"
                    } else {
                        "false"
                    },
                )
                .replace("__ALIGN__", align.peek().name())
                .replace("__LOOP__", if opts.r#loop { "true" } else { "false" })
                .replace("__START__", &opts.start_index.to_string())
                .replace(
                    "__DELAY__",
                    &autoplay.map(|a| a.delay_ms).unwrap_or(0).to_string(),
                )
                .replace(
                    "__STOP_ON_INTERACTION__",
                    if autoplay.is_some_and(|a| a.stop_on_interaction) {
                        "true"
                    } else {
                        "false"
                    },
                )
                .replace(
                    "__STOP_ON_ENTER__",
                    if autoplay.is_some_and(|a| a.stop_on_mouse_enter) {
                        "true"
                    } else {
                        "false"
                    },
                );
            let mut eval = document::eval(&js);
            let mut commands = commands;
            commands.set(Some(eval));
            if let Some(handler) = set_api {
                handler.call(CarouselApi { context });
            }
            spawn(async move {
                let mut selected = selected;
                let mut count = count;
                let mut can_prev = can_prev;
                let mut can_next = can_next;
                while let Ok(state) = eval.recv::<Vec<f64>>().await {
                    if let [index, total, prev, next] = state[..] {
                        selected.set(index as usize);
                        count.set(total as usize);
                        can_prev.set(prev > 0.0);
                        can_next.set(next > 0.0);
                    }
                }
            });
        });
    }

    rsx! {
        div {
            id: dom_id,
            role: "region",
            aria_roledescription: "carousel",
            "data-slot": "carousel",
            class: class,
            onkeydown: move |event| {
                let (prev, next) = match *orientation.peek() {
                    CarouselOrientation::Horizontal => (Key::ArrowLeft, Key::ArrowRight),
                    CarouselOrientation::Vertical => (Key::ArrowUp, Key::ArrowDown),
                };
                if event.key() == prev {
                    event.prevent_default();
                    context.send("prev");
                } else if event.key() == next {
                    event.prevent_default();
                    context.send("next");
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
    let vertical = (root.orientation)() == CarouselOrientation::Vertical;
    let class = cn!(
        "flex",
        if vertical { "-mt-4 flex-col" } else { "-ml-4" },
        take_class(&mut attributes)
    );
    let snap = if vertical {
        "overflow-x: hidden; overflow-y: auto; scroll-snap-type: y mandatory;"
    } else {
        "overflow-x: auto; overflow-y: hidden; scroll-snap-type: x mandatory;"
    };

    rsx! {
        div {
            class: "no-scrollbar overflow-hidden",
            style: snap,
            "data-slot": "carousel-content",
            div { class: class, ..attributes, {children} }
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
    let snap = match (root.align)() {
        CarouselAlign::Start => "snap-start",
        CarouselAlign::Center => "snap-center",
        CarouselAlign::End => "snap-end",
    };
    let class = cn!(
        "min-w-0 shrink-0 grow-0 basis-full",
        if (root.orientation)() == CarouselOrientation::Vertical {
            "pt-4"
        } else {
            "pl-4"
        },
        snap,
        take_class(&mut attributes)
    );

    rsx! {
        div {
            role: "group",
            aria_roledescription: "slide",
            "data-slot": "carousel-item",
            class: class,
            ..attributes,
            {children}
        }
    }
}

#[component]
pub fn CarouselPrevious(
    #[props(default = ButtonVariant::Outline)] variant: ButtonVariant,
    #[props(default = ButtonSize::IconSm)] size: ButtonSize,
    onclick: Option<EventHandler<MouseEvent>>,
    #[props(extends = GlobalAttributes, extends = button)] attributes: Vec<Attribute>,
) -> Element {
    let mut attributes = attributes;
    let root = use_context::<CarouselContext>();
    let position = if (root.orientation)() == CarouselOrientation::Vertical {
        "-top-12 left-1/2 -translate-x-1/2 rotate-90"
    } else {
        "inset-y-0 -left-12 my-auto"
    };
    let class = cn!(
        "cn-carousel-previous absolute touch-manipulation",
        position,
        take_class(&mut attributes)
    );
    attributes.insert(0, Attribute::new("data-slot", "carousel-previous", None, false));
    attributes.insert(1, Attribute::new("class", class, None, false));

    rsx! {
        Button {
            variant: variant,
            size: size,
            disabled: !(root.can_prev)(),
            onclick: move |event| {
                root.send("prev");
                if let Some(handler) = onclick {
                    handler.call(event);
                }
            },
            attributes: attributes,
            svg {
                class: "cn-rtl-flip",
                width: "24",
                height: "24",
                view_box: "0 0 24 24",
                fill: "none",
                stroke: "currentColor",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
                path { d: "m15 18-6-6 6-6" }
            }
            span { class: "sr-only", "Previous slide" }
        }
    }
}

#[component]
pub fn CarouselNext(
    #[props(default = ButtonVariant::Outline)] variant: ButtonVariant,
    #[props(default = ButtonSize::IconSm)] size: ButtonSize,
    onclick: Option<EventHandler<MouseEvent>>,
    #[props(extends = GlobalAttributes, extends = button)] attributes: Vec<Attribute>,
) -> Element {
    let mut attributes = attributes;
    let root = use_context::<CarouselContext>();
    let position = if (root.orientation)() == CarouselOrientation::Vertical {
        "-bottom-12 left-1/2 -translate-x-1/2 rotate-90"
    } else {
        "inset-y-0 -right-12 my-auto"
    };
    let class = cn!(
        "cn-carousel-next absolute touch-manipulation",
        position,
        take_class(&mut attributes)
    );
    attributes.insert(0, Attribute::new("data-slot", "carousel-next", None, false));
    attributes.insert(1, Attribute::new("class", class, None, false));

    rsx! {
        Button {
            variant: variant,
            size: size,
            disabled: !(root.can_next)(),
            onclick: move |event| {
                root.send("next");
                if let Some(handler) = onclick {
                    handler.call(event);
                }
            },
            attributes: attributes,
            svg {
                class: "cn-rtl-flip",
                width: "24",
                height: "24",
                view_box: "0 0 24 24",
                fill: "none",
                stroke: "currentColor",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
                path { d: "m9 18 6-6-6-6" }
            }
            span { class: "sr-only", "Next slide" }
        }
    }
}
