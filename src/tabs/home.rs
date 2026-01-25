use dioxus::prelude::*;
use uuid::Uuid;

use crate::api::posts::get_posts;

#[component]
pub fn Home() -> Element {
    let posts_res = use_resource(move || async move { get_posts().await }).suspend()?;

    rsx! {
        button { class: "btn btn-primary w-auto", "+ Добавить пост" }
        div { class: "divider" }
        SuspenseBoundary {
            fallback: |_ctx| {
                rsx! {
                    h1 { class: "italic opacity-70", "Загрузка постов..." }
                }
            },
            match posts_res.read().as_ref() {
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
        div { class: "card bg-base-300 w-auto shadow-sm",
            div { class: "card-body",
                h1 { class: "card-title", {title} }
                div { class: "flex gap-1",
                    h3 { class: "font-medium", "Автор:" }
                    h3 { class: "font-bold", {author_id.to_string()} }
                }
                p { {content} }
            }
        }
    }
}
