use crate::{AppStateType, middleware::auth::Auth, models::user::PublicUser};
use dioxus::prelude::*;
use dioxus::server::axum::extract::State;

#[get("/api/u/me", State(_): State<AppStateType>, Auth(user): Auth)]
async fn get_my_data() -> Result<PublicUser> {
    let public_user = PublicUser {
        id: user.id,
        username: user.username,
        admin: user.admin,
    };

    Ok(public_user)
}
