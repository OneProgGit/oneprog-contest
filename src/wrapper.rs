use dioxus::prelude::*;

use crate::{footer::Footer, navbar::Navbar, route::Route};

#[component]
pub fn Wrapper() -> Element {
    rsx! {
        div { class: "flex flex-col min-h-screen gap-4 mx-2 items-start justify-start",
            Navbar {}
            div { class: "flex-1", Outlet::<Route> {} }
            Footer {}
        }
    }
}
