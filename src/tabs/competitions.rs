use dioxus::prelude::*;

#[component]
pub fn Competitions() -> Element {
    rsx! {
        h1 { "Соревнования" }
        div { class: "divider" }
        h1 { class: "italic", "Будет доступно в версии 0.4" }
    }
}
