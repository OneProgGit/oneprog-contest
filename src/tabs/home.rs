use dioxus::prelude::*;

#[cfg(feature = "server")]
use crate::api::posts::get_posts;

#[component]
pub fn Home() -> Element {
    let posts = use_resource(|| async {
        get_posts().await;
    });

    rsx! {
        h1 { "Блог" }
        div { class: "divider" }
    }
}

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
