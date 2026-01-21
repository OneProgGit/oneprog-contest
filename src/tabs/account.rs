use dioxus::prelude::*;

#[component]
pub fn Account() -> Element {
    rsx! {
        h1 { "Аккаунт" }
        div { class: "divider" }
    }
}
