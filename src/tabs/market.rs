use dioxus::prelude::*;

#[component]
pub fn Market() -> Element {
    rsx! {
        h1 {
            "Рынок"
        }
        div { class: "divider" }

        h1 { class: "italic",
            "Будет доступно в версии 0.6"
        }
    }
}
