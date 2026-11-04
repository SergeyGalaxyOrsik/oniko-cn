mod components;
mod demos;
mod icons;
mod layout;
mod pages;
mod registry;
mod widgets;

use dioxus::prelude::*;

use layout::{DocsLayout, SiteLayout};
use pages::{ComponentPage, Home, Installation, Introduction, NotFound};

const TAILWIND: Asset = asset!("/assets/tailwind.css");
const LOGO: Asset = asset!("/assets/oniko.svg");

pub const GITHUB_URL: &str = "https://github.com/SergeyGalaxyOrsik/oniko-cn";
pub const APP_URL: &str = "https://oniko.orsik.site";

#[derive(Routable, Clone, PartialEq, Debug)]
#[rustfmt::skip]
pub enum Route {
    #[layout(SiteLayout)]
        #[route("/")]
        Home {},
        #[nest("/docs")]
            #[layout(DocsLayout)]
                #[route("/")]
                Introduction {},
                #[route("/installation")]
                Installation {},
                #[route("/components/:name")]
                ComponentPage { name: String },
            #[end_layout]
        #[end_nest]
        #[route("/:..segments")]
        NotFound { segments: Vec<String> },
}

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Stylesheet { href: TAILWIND }
        document::Link { rel: "icon", href: LOGO }
        Router::<Route> {}
    }
}
