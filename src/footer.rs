use dioxus::prelude::*;

#[component]
pub fn Footer() -> Element {
    rsx! {
        div { class: "footer footer-center bg-base-200 p-4", id: "footer",
            p { "© 2026 OneProg" }
        }
    }
}
