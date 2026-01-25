use dioxus::prelude::*;
use uuid::Uuid;

use crate::api::posts::get_posts;

#[component]
pub fn Home() -> Element {
    let posts_res = use_resource(move || async move { get_posts().await }).suspend()?;

    rsx! {
        h1 { "Блог" }
        div { class: "divider" }
        SuspenseBoundary {
            fallback: |_ctx| {
                rsx! {
                    h1 { class: "italic opacity-70", "Загрузка постов..." }
                }
            },
            match &*posts_res.read() {
                Ok(posts) => rsx! {
                    for post in posts {
                        BlogPost {
                            author_id: post.author_id,
                            title: post.title.clone(),
                            content: post.content.clone(),
                        }
                    }
                },
                Err(err) => rsx! {
                    h1 { class: "italic text-red-500", "Ошибка при загрузке постов: {err}" }
                },
            }
        }
    }
}

#[component]
fn BlogPost(author_id: Uuid, title: String, content: String) -> Element {
    rsx! {
        div { class: "card bg-base-300 w-96 shadow-sm",
            div { class: "card-body",
                h1 { class: "card-title", {title} }
                h3 { class: "font-medium", {author_id.to_string()} }
                p { {content} }
            }
        }
    }
}
