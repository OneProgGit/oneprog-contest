#[cfg(feature = "server")]
use crate::{
    AppStateType, crypt::verify_password, database::Database, jwt::create_jwt,
    models::jwt::JwtClaims,
};

#[cfg(feature = "server")]
use chrono::{Duration, Utc};

#[cfg(feature = "server")]
use dioxus::server::axum::extract::State;

use crate::models::user::AuthRequest;
use dioxus::{fullstack::Json, prelude::*};

#[post("/api/u/login", State(state): State<AppStateType>)]
async fn login(Json(user): Json<AuthRequest>) -> Result<String, HttpError> {
    let expected_user = state
        .db
        .get_user_by_username(&user.username)
        .await
        .or_bad_request("Неверный логин или пароль")?;

    let is_valid_password = verify_password(&expected_user.hashed_password, &user.password)
        .or_internal_server_error("Не удалось проверить пароль на валидность")?;

    (is_valid_password).or_bad_request("Неверный логин или пароль")?;

    let claims = JwtClaims {
        id: expected_user.id,
        exp: (Utc::now() + Duration::hours(24)).timestamp() as usize,
    };

    let secret = dotenvy::var("JWT_SECRET")
        .or_internal_server_error("Не удалось создать аутентификационный ключ")?;
    let token = create_jwt(&claims, &secret)
        .or_internal_server_error("Не удалось создать аутентификационный ключ")?;

    Ok(token)
}
