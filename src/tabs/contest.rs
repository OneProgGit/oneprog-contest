use dioxus::prelude::*;

#[component]
pub fn Contest(id: usize) -> Element {
    rsx! {
        h1 { class: "italic", "Контесты будут доступны в версии 0.3.0" }
        div { class: "divider" }
    }
}
