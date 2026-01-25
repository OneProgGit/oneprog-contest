use dioxus::prelude::*;

use crate::{footer::Footer, navbar::Navbar, route::Route};

#[component]
pub fn Wrapper() -> Element {
    rsx! {
        div { class: "flex flex-col gap-4 m-2",
            Navbar {}
            Outlet::<Route> {}
            Footer {}
        }
    }
}
