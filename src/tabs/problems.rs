use dioxus::prelude::*;

#[component]
pub fn Problems() -> Element {
    rsx! {
        h1 { class: "italic", "Задачи будут доступны в версии 0.2.0" }
        div { class: "divider" }
    }
}
