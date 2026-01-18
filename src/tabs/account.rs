use dioxus::prelude::*;

#[component]
pub fn Account() -> Element {
    rsx! {
        h1 { "Аккаунт" }
        div { class: "divider" }
        h1 { class: "italic", "Будет доступно в версии 0.1" }
    }
}
