use dioxus::prelude::*;

#[component]
pub fn Competitions() -> Element {
    rsx! {
        h1 { class: "italic",
            "Соревнования будут доступны в версии 0.4.0"
        }
        div { class: "divider" }
    }
}
