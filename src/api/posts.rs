use crate::{
    AppStateType,
    database::Database,
    middleware::auth::Auth,
    models::post::{CreatePostRequest, DatabasePost, Post},
};
use dioxus::prelude::*;
use dioxus::server::axum::extract::{Json, State};
use serde_json::{Value, json};

#[post("/api/posts", State(state): State<AppStateType>, Auth(user): Auth)]
async fn create_post(Json(post): Json<CreatePostRequest>) -> Result<Value, HttpError> {
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

    Ok(json!({ "message" : "Пост создан успешно" }))
}

#[get("/api/posts", State(state): State<AppStateType>)]
pub async fn get_posts() -> Result<Vec<Post>, HttpError> {
    let posts = state
        .db
        .get_posts()
        .await
        .or_internal_server_error("Ошибка сервера при получении постов")?;
    Ok(posts)
}
