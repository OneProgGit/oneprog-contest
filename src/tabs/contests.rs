use dioxus::prelude::*;

#[component]
pub fn Contests() -> Element {
    rsx! {
        h1 { "Контесты" }
        div { class: "divider" }
        h1 { class: "italic", "Будет доступно в версии 0.3" }
    }
}
