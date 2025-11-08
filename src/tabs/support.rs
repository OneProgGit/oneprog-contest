use dioxus::prelude::*;

#[component]
pub fn Support() -> Element {
    rsx! {
        h1 {
            "Поддержать"
        }
        div { class: "divider" }

        h1 { class: "italic",
            "Будет доступно в версии 0.5"
        }
    }
}
