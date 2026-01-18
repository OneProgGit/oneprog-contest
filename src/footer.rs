use dioxus::prelude::*;

#[component]
pub fn Footer() -> Element {
    rsx! {
        div {
            class: "footer sm:footer-horizontal footer-center bg-base-200 text-base-content p-4",
            id: "footer",

            p { "© 2026 OneProg" }
        }
    }
}
