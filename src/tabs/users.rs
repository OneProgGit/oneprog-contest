use dioxus::prelude::*;

#[component]
pub fn Users() -> Element {
    rsx! {
        h1 {
            "Пользователи"
        }
        div { class: "divider" }

        h1 { class: "italic",
            "Будет доступно в версии 0.1"
        }
    }
}
