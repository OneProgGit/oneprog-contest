#[cfg(feature = "server")]
use crate::middleware::auth::Auth;

use crate::models::user::PublicUser;
use dioxus::prelude::*;

#[get("/api/u/me", Auth(user): Auth)]
async fn get_my_data() -> Result<PublicUser, ServerFnError> {
    let public_user = PublicUser {
        id: user.id,
        username: user.username,
        admin: user.admin,
    };

    Ok(public_user)
}
