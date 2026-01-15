use dioxus::prelude::*;

#[component]
pub fn Contest(id: usize) -> Element {
    rsx! {
        h1 { "Контест" }
        div { class: "divider" }

        h1 { class: "italic", "Будет доступно в версии 0.3" }
    }
}
