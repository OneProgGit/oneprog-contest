use dioxus::prelude::*;

#[component]
pub fn User(id: usize) -> Element {
    rsx! {
        h1 { class: "italic",
            "Пользователи будут доступны в версии 0.1.0"
        }
        div { class: "divider" }
    }
}
