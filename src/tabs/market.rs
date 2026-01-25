use dioxus::prelude::*;

#[component]
pub fn Market() -> Element {
    rsx! {
        h1 { class: "italic", "Рынок будет доступен в версии 0.6.0" }
        div { class: "divider" }
    }
}
