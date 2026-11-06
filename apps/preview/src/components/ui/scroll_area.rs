//! ScrollArea: a scrollable region with a styled viewport and custom scrollbars.
//! ScrollBar: a scrollbar with a draggable thumb. `ScrollArea` renders a vertical one; add
//! `ScrollBar { orientation: ScrollBarOrientation::Horizontal }` inside it for horizontal scrolling.
//!
//! The viewport is a native scroll container. A small script keeps the thumbs in sync with it.
//!
//! Installed by oniko-cn. This file is yours: edit it freely.

use std::sync::atomic::{AtomicUsize, Ordering};

use dioxus::prelude::*;
use oniko_cn::{cn, take_class};

static NEXT_ID: AtomicUsize = AtomicUsize::new(0);

/// Sizes and moves the thumbs, toggles the overflow, scrolling and hovering attributes, and
/// handles thumb drags and clicks on the track. `__ID__` is replaced before it runs.
const DRIVER_SCRIPT: &str = r#"(() => {
  const root = document.getElementById("__ID__");
  if (!root) return;
  const viewport = root.querySelector('[data-slot="scroll-area-viewport"]');
  if (!viewport) return;
  const bars = () => Array.from(root.querySelectorAll('[data-slot="scroll-area-scrollbar"]'));
  const flag = (el, name, on) => (on ? el.setAttribute(name, "") : el.removeAttribute(name));
  const rtl = () => getComputedStyle(root).direction === "rtl";
  const metrics = (bar) => {
    const horizontal = bar.dataset.orientation === "horizontal";
    const client = horizontal ? viewport.clientWidth : viewport.clientHeight;
    const total = horizontal ? viewport.scrollWidth : viewport.scrollHeight;
    const raw = horizontal ? viewport.scrollLeft : viewport.scrollTop;
    const track = horizontal ? bar.clientWidth : bar.clientHeight;
    return { horizontal, client, total, offset: Math.abs(raw), track };
  };
  const update = () => {
    const overflowX = viewport.scrollWidth > viewport.clientWidth + 1;
    const overflowY = viewport.scrollHeight > viewport.clientHeight + 1;
    flag(root, "data-has-overflow-x", overflowX);
    flag(root, "data-has-overflow-y", overflowY);
    let visibleX = false;
    let visibleY = false;
    let barWidth = 0;
    let barHeight = 0;
    for (const bar of bars()) {
      const m = metrics(bar);
      const overflow = m.horizontal ? overflowX : overflowY;
      bar.hidden = !overflow && !bar.hasAttribute("data-keep-mounted");
      flag(bar, "data-has-overflow-x", overflowX);
      flag(bar, "data-has-overflow-y", overflowY);
      const thumb = bar.querySelector('[data-slot="scroll-area-thumb"]');
      if (!overflow || !thumb) continue;
      if (m.horizontal) { visibleX = true; barHeight = bar.offsetHeight; }
      else { visibleY = true; barWidth = bar.offsetWidth; }
      const size = Math.max(16, (m.track * m.client) / m.total);
      const travel = Math.max(0, m.track - size);
      const range = Math.max(1, m.total - m.client);
      const pos = (m.offset / range) * travel;
      if (m.horizontal) {
        thumb.style.width = size + "px";
        thumb.style.height = "";
        thumb.style.transform = "translate3d(" + (rtl() ? -pos : pos) + "px,0,0)";
      } else {
        thumb.style.height = size + "px";
        thumb.style.width = "";
        thumb.style.transform = "translate3d(0," + pos + "px,0)";
      }
    }
    const both = visibleX && visibleY;
    root.style.setProperty("--scroll-area-corner-width", both ? barWidth + "px" : "0px");
    root.style.setProperty("--scroll-area-corner-height", both ? barHeight + "px" : "0px");
  };

  let timer = 0;
  viewport.addEventListener("scroll", () => {
    flag(root, "data-scrolling", true);
    for (const bar of bars()) flag(bar, "data-scrolling", true);
    clearTimeout(timer);
    timer = setTimeout(() => {
      flag(root, "data-scrolling", false);
      for (const bar of bars()) flag(bar, "data-scrolling", false);
    }, 500);
    update();
  }, { passive: true });
  root.addEventListener("pointerenter", () => { flag(root, "data-hovering", true); for (const bar of bars()) flag(bar, "data-hovering", true); });
  root.addEventListener("pointerleave", () => { flag(root, "data-hovering", false); for (const bar of bars()) flag(bar, "data-hovering", false); });

  root.addEventListener("pointerdown", (event) => {
    const bar = event.target.closest && event.target.closest('[data-slot="scroll-area-scrollbar"]');
    if (!bar || !root.contains(bar) || event.button !== 0) return;
    const m = metrics(bar);
    const thumb = bar.querySelector('[data-slot="scroll-area-thumb"]');
    if (!thumb) return;
    const sign = m.horizontal && rtl() ? -1 : 1;
    const axis = (e) => (m.horizontal ? e.clientX : e.clientY);
    const thumbSize = m.horizontal ? thumb.offsetWidth : thumb.offsetHeight;
    const scrollTo = (value) => {
      if (m.horizontal) viewport.scrollLeft = sign * value;
      else viewport.scrollTop = value;
    };
    const range = Math.max(1, m.total - m.client);
    const ratio = range / Math.max(1, m.track - thumbSize);
    if (event.target !== thumb) {
      const rect = bar.getBoundingClientRect();
      const start = m.horizontal ? (rtl() ? rect.right - axis(event) : axis(event) - rect.left) : axis(event) - rect.top;
      scrollTo((start - thumbSize / 2) * ratio);
    }
    const origin = axis(event);
    const startOffset = Math.abs(m.horizontal ? viewport.scrollLeft : viewport.scrollTop);
    const direction = m.horizontal && rtl() ? -1 : 1;
    const move = (e) => scrollTo(startOffset + (axis(e) - origin) * direction * ratio);
    const stop = () => {
      window.removeEventListener("pointermove", move);
      window.removeEventListener("pointerup", stop);
    };
    window.addEventListener("pointermove", move);
    window.addEventListener("pointerup", stop);
    event.preventDefault();
  });

  const observer = new ResizeObserver(update);
  observer.observe(viewport);
  const watch = () => { for (const child of viewport.children) observer.observe(child); };
  watch();
  new MutationObserver(() => { watch(); update(); }).observe(viewport, { childList: true, subtree: true });
  update();
})();"#;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ScrollBarOrientation {
    #[default]
    Vertical,
    Horizontal,
}

impl ScrollBarOrientation {
    pub fn name(self) -> &'static str {
        match self {
            Self::Vertical => "vertical",
            Self::Horizontal => "horizontal",
        }
    }

    /// Base UI positions the scrollbar itself, leaving room for the corner.
    fn style(self) -> &'static str {
        match self {
            Self::Vertical => {
                "position: absolute; top: 0; bottom: var(--scroll-area-corner-height, 0px); inset-inline-end: 0;"
            }
            Self::Horizontal => {
                "position: absolute; bottom: 0; inset-inline-start: 0; inset-inline-end: var(--scroll-area-corner-width, 0px);"
            }
        }
    }
}

#[component]
pub fn ScrollArea(
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!("relative", take_class(&mut attributes));
    let id = use_hook(|| NEXT_ID.fetch_add(1, Ordering::Relaxed));
    let dom_id = format!("scroll-area-{id}");
    {
        let dom_id = dom_id.clone();
        use_effect(move || {
            let _ = document::eval(&DRIVER_SCRIPT.replace("__ID__", &dom_id));
        });
    }

    rsx! {
        div {
            "data-slot": "scroll-area",
            id: dom_id,
            role: "presentation",
            style: "position: relative; overflow: hidden;",
            class: class,
            ..attributes,
            div {
                "data-slot": "scroll-area-viewport",
                class: "no-scrollbar size-full rounded-[inherit] transition-[color,box-shadow] outline-none focus-visible:ring-[3px] focus-visible:ring-ring/50 focus-visible:outline-1",
                style: "overflow: scroll;",
                tabindex: "0",
                {children}
            }
            ScrollBar {}
            div {
                style: "position: absolute; bottom: 0; inset-inline-end: 0; width: var(--scroll-area-corner-width, 0px); height: var(--scroll-area-corner-height, 0px);",
            }
        }
    }
}

#[component]
pub fn ScrollBar(
    #[props(default)] orientation: ScrollBarOrientation,
    /// Keep the scrollbar in the layout even when there is nothing to scroll.
    #[props(default)]
    keep_mounted: bool,
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(
        "data-horizontal:h-2.5 data-horizontal:flex-col data-horizontal:border-t data-horizontal:border-t-transparent data-vertical:h-full data-vertical:w-2.5 data-vertical:border-l data-vertical:border-l-transparent flex touch-none p-px transition-colors select-none",
        take_class(&mut attributes)
    );

    rsx! {
        div {
            "data-slot": "scroll-area-scrollbar",
            "data-orientation": orientation.name(),
            "data-keep-mounted": keep_mounted.then_some(""),
            hidden: !keep_mounted,
            style: orientation.style(),
            class: class,
            ..attributes,
            div {
                "data-slot": "scroll-area-thumb",
                class: "rounded-full relative flex-1 bg-border",
            }
        }
    }
}
