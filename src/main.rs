use dioxus::prelude::*;

use crate::route::Route;

mod navbar;
mod route;
mod tabs;

const MAIN_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Stylesheet { href: MAIN_CSS }
        Router::<Route> {}
    }
}
