use crate::{AppStateType, middleware::auth::Auth, models::user::PublicUser};
use dioxus::prelude::*;
use dioxus::server::axum::extract::State;
use serde_json::{Value, json};

#[get("/u/me", State(_): State<AppStateType>, Auth(user): Auth)]
async fn get_my_data() -> Result<Value> {
    let public_user = PublicUser {
        id: user.id,
        username: user.username,
        admin: user.admin,
    };

    Ok(json!(public_user))
}
