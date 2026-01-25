use dioxus::prelude::*;

#[component]
pub fn Support() -> Element {
    rsx! {
        h1 { class: "italic", "Поддержать можно будет в версии 0.5.0" }
        div { class: "divider" }
    }
}
