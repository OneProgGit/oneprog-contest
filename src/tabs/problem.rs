use dioxus::prelude::*;

#[component]
pub fn Problem(id: usize) -> Element {
    rsx! {
        h1 { "Задача" }
        div { class: "divider" }

        h1 { class: "italic", "Будет доступно в версии 0.2" }
    }
}
