//! Avatar: an image element with a fallback for representing the user.
//!
//! Installed by oniko-cn. This file is yours: edit it freely.

use dioxus::prelude::*;
use oniko_cn::{cn, take_class};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum AvatarSize {
    #[default]
    Default,
    Sm,
    Lg,
}

impl AvatarSize {
    /// Value of the `data-size` attribute that the styles and child parts key off.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Default => "default",
            Self::Sm => "sm",
            Self::Lg => "lg",
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
enum ImageState {
    #[default]
    Loading,
    Loaded,
    Failed,
}

#[derive(Clone, Copy)]
struct AvatarContext {
    image: Signal<ImageState>,
}

#[component]
pub fn Avatar(
    #[props(default)] size: AvatarSize,
    #[props(extends = GlobalAttributes, extends = span)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let image = use_signal(ImageState::default);
    use_context_provider(|| AvatarContext { image });

    let mut attributes = attributes;
    let class = cn!(
        "size-8 rounded-full after:rounded-full data-[size=lg]:size-10 data-[size=sm]:size-6 group/avatar relative flex shrink-0 select-none after:absolute after:inset-0 after:border after:border-border after:mix-blend-darken dark:after:mix-blend-lighten",
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        span {
            "data-slot": "avatar",
            "data-size": size.as_str(),
            class: class,
            ..attributes,
            {children}
        }
    }
}

/// Stays hidden until the image has loaded, so `AvatarFallback` shows meanwhile and on error.
#[component]
pub fn AvatarImage(
    #[props(extends = img)] attributes: Vec<Attribute>,
    onload: Option<EventHandler<Event<ImageData>>>,
    onerror: Option<EventHandler<Event<ImageData>>>,
) -> Element {
    let mut state = use_context::<AvatarContext>().image;
    let mut attributes = attributes;
    let class = cn!(
        "rounded-full aspect-square size-full object-cover",
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        img {
            "data-slot": "avatar-image",
            class: class,
            hidden: state() != ImageState::Loaded,
            onload: move |event| {
                state.set(ImageState::Loaded);
                if let Some(handler) = onload {
                    handler.call(event);
                }
            },
            onerror: move |event| {
                state.set(ImageState::Failed);
                if let Some(handler) = onerror {
                    handler.call(event);
                }
            },
            ..attributes,
        }
    }
}

/// Renders until the sibling `AvatarImage` has loaded.
#[component]
pub fn AvatarFallback(
    #[props(extends = GlobalAttributes, extends = span)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let state = use_context::<AvatarContext>().image;
    let mut attributes = attributes;
    let class = cn!(
        "bg-muted text-muted-foreground rounded-full flex size-full items-center justify-center text-sm group-data-[size=sm]/avatar:text-xs",
        take_class(&mut attributes).as_deref()
    );

    if state() == ImageState::Loaded {
        return rsx! {};
    }

    rsx! {
        span {
            "data-slot": "avatar-fallback",
            class: class,
            ..attributes,
            {children}
        }
    }
}

#[component]
pub fn AvatarBadge(
    #[props(extends = GlobalAttributes, extends = span)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(
        "bg-primary text-primary-foreground ring-background absolute right-0 bottom-0 z-10 inline-flex items-center justify-center rounded-full bg-blend-color ring-2 select-none",
        "group-data-[size=sm]/avatar:size-2 group-data-[size=sm]/avatar:[&>svg]:hidden",
        "group-data-[size=default]/avatar:size-2.5 group-data-[size=default]/avatar:[&>svg]:size-2",
        "group-data-[size=lg]/avatar:size-3 group-data-[size=lg]/avatar:[&>svg]:size-2",
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        span {
            "data-slot": "avatar-badge",
            class: class,
            ..attributes,
            {children}
        }
    }
}

#[component]
pub fn AvatarGroup(
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(
        "group/avatar-group flex -space-x-2 *:data-[slot=avatar]:ring-2 *:data-[slot=avatar]:ring-background",
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        div {
            "data-slot": "avatar-group",
            class: class,
            ..attributes,
            {children}
        }
    }
}

#[component]
pub fn AvatarGroupCount(
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut attributes = attributes;
    let class = cn!(
        "bg-muted text-muted-foreground size-8 rounded-full text-sm group-has-data-[size=lg]/avatar-group:size-10 group-has-data-[size=sm]/avatar-group:size-6 [&>svg]:size-4 group-has-data-[size=lg]/avatar-group:[&>svg]:size-5 group-has-data-[size=sm]/avatar-group:[&>svg]:size-3 relative flex shrink-0 items-center justify-center ring-2 ring-background",
        take_class(&mut attributes).as_deref()
    );

    rsx! {
        div {
            "data-slot": "avatar-group-count",
            class: class,
            ..attributes,
            {children}
        }
    }
}
