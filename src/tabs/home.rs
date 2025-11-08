use dioxus::prelude::*;

#[component]
pub fn Home() -> Element {
    rsx! {
        h1 {
            "Новости"
        }
        div { class: "divider" }

        h1 { class: "italic",
            "Будет доступно в версии 0.1"
        }
    }
}
