use dioxus::prelude::*;

#[component]
pub fn Business() -> Element {
    rsx! {
        h1 { "Бизнес" }
        div { class: "divider" }
        h1 { class: "italic", "Будет доступно в версии 1.0" }
    }
}
