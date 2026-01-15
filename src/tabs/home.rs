use dioxus::prelude::*;

#[component]
fn BlogPost(author: String, title: String, content: String) -> Element {
    rsx! {
        div { class: "card bg-base-300 w-96 shadow-sm",
            div { class: "card-body",
                h1 { class: "card-title", {title} }

                h3 { class: "font-medium", {author} }

                p { {content} }
            }
        }
    }
}

#[component]
pub fn Home() -> Element {
    rsx! {
        h1 { "Блог" }
        div { class: "divider" }

        h1 { class: "italic", "Будет доступно в версии 0.1" }

        BlogPost {
            author: "OneProg",
            title: "Привет",
            content: "Привет, мирочек!",
        }
    }
}
