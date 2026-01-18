use dioxus::prelude::*;

#[component]
pub fn Problems() -> Element {
    rsx! {
        h1 { "Задачи" }
        div { class: "divider" }
        h1 { class: "italic", "Будет доступно в версии 0.2" }
    }
}
