#[cfg(feature = "server")]
use crate::{AppStateType, database::Database, middleware::auth::Auth, models::post::DatabasePost};

#[cfg(feature = "server")]
use dioxus::server::axum::extract::State;

use crate::models::post::{CreatePostRequest, Post};
use dioxus::prelude::*;

#[post("/api/posts", State(state): State<AppStateType>, Auth(user): Auth)]
async fn create_post(post: CreatePostRequest) -> Result<(), ServerFnError> {
    (user.admin).or_forbidden("Для создания поста необходимы привилегии администратора")?;

    let db_new_post = DatabasePost {
        author_id: user.id,
        title: post.title,
        content: post.content,
    };

    state
        .db
        .create_post(db_new_post)
        .await
        .or_internal_server_error("Не удалось создать пост")?;

    Ok(())
}

#[get("/api/posts", State(state): State<AppStateType>)]
pub async fn get_posts() -> Result<Vec<Post>, ServerFnError> {
    let posts = state
        .db
        .get_posts()
        .await
        .or_internal_server_error("Ошибка сервера при получении постов")?;
    Ok(posts)
}
