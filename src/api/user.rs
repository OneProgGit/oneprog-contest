use crate::{AppStateType, middleware::auth::Auth};
use dioxus::prelude::*;
use dioxus::server::axum::extract::State;
use serde_json::{Value, json};

#[get("/u/me", State(_): State<AppStateType>, Auth(user): Auth)]
async fn get_user_data() -> Result<Value> {
    Ok(json!(user))
}
