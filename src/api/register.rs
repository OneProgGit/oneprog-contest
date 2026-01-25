#[cfg(feature = "server")]
use crate::{AppStateType, crypt::hash_password, database::Database, models::user::DatabaseUser};

#[cfg(feature = "server")]
use dioxus::server::axum::extract::State;

#[cfg(feature = "server")]
use regex::Regex;

use crate::models::user::AuthRequest;
use dioxus::{fullstack::Json, prelude::*};

#[post("/api/u/register", State(state): State<AppStateType>)]
pub async fn register(Json(user): Json<AuthRequest>) -> Result<(), HttpError> {
    (user.username.len() >= 4 && user.username.len() <= 16)
        .or_bad_request("Логин может содержать минимум 4 и максимум 16 символов")?;
    (user.password.len() >= 8 && user.password.len() <= 32)
        .or_bad_request("Пароль может содержать минимум 8 и максимум 32 символа")?;

    let username_regex =
        Regex::new("^\\w+$").or_internal_server_error("OneProg допустил баг в прод :)")?;

    (username_regex.is_match(&user.username))
        .or_bad_request(
            "Логин может содержать только заглавные и строчные латинские буквы, а также нижние подчёркивания",
        )?;

    let hashed_password = hash_password(&user.password)
        .or_internal_server_error("Не удалось зарегистрировать пользователя")?;

    let new_user = DatabaseUser {
        username: user.username,
        hashed_password,
    };

    state
        .db
        .create_user(new_user)
        .await
        .or_internal_server_error(
            "Не удалось зарегистрировать пользователя. Возможно, пользователь с таким логином уже существует",
        )?;

    Ok(())
}
