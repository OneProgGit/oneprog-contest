use dioxus::prelude::*;

#[component]
pub fn PageNotFound(segments: Vec<String>) -> Element {
    rsx! {
        h1 { class: "text-2xl", "404" }
        div { class: "divider" }
        h1 { class: "italic",
            "Данная страница не найдена. Проверьте адрес на корректность"
        }
    }
}
