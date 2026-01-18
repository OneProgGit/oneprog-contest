use dioxus::prelude::*;

use crate::{footer::Footer, navbar::Navbar, route::Route};

#[component]
pub fn Wrapper() -> Element {
    rsx! {
        Navbar {}
        Outlet::<Route> {}
        Footer {}
    }
}
