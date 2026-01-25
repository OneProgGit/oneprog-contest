use dioxus::prelude::*;

#[component]
pub fn Business() -> Element {
    rsx! {
        h1 { class: "italic",
            "Бизнес-план будет доступен в версии 1.0.0"
        }
        div { class: "divider" }
    }
}
