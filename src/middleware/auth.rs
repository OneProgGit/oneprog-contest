use crate::{AppStateType, database::Database, jwt::decode_jwt, models::user::FullUser};
use dioxus::{
    fullstack::extract::FromRef,
    prelude::{HttpError, OrHttpError},
    server::axum::{extract::FromRequestParts, http::request::Parts},
};

pub struct Auth(pub FullUser);

impl<S> FromRequestParts<S> for Auth
where
    AppStateType: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = HttpError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let state = AppStateType::from_ref(state);

        let token = parts
            .headers
            .get("Authorization")
            .and_then(|v| v.to_str().ok())
            .and_then(|s| s.strip_prefix("Bearer "))
            .or_unauthorized("Отсутствует аутентификационный ключ")?;

        let secret = dotenvy::var("JWT_SECRET").or_internal_server_error(
            "Не удалось проверить аутентификационный ключ на валидность",
        )?;

        let claims =
            decode_jwt(token, &secret).or_unauthorized("Невалидный аутентификационный ключ")?;

        let user = state
            .db
            .get_user_by_id(claims.id)
            .await
            .or_unauthorized("Невалидный аутентификационный ключ")?;

        Ok(Self(user))
    }
}
