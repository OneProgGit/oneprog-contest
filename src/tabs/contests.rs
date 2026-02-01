use dioxus::prelude::*;

#[component]
pub fn Contests() -> Element {
    rsx! {
        h1 { class: "italic",
            "Контесты и задачи будут доступны в версии 0.2.0"
        }
        div { class: "divider" }
    }
}
