use dioxus::prelude::*;

#[component]
pub fn User(id: usize) -> Element {
    rsx! {
        h1 { "Пользователь" }
        div { class: "divider" }
        h1 { class: "italic", "Будет доступно в версии 0.1" }
    }
}
