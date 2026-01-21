use serde::{Deserialize, Serialize};

#[cfg(feature = "server")]
use sqlx::prelude::FromRow;

use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct AuthRequest {
    pub username: String,
    pub password: String,
}

pub struct DatabaseUser {
    pub username: String,
    pub hashed_password: String,
}

#[derive(Serialize, Deserialize, Debug, FromRow)]
#[cfg(feature = "server")]
pub struct FullUser {
    pub id: Uuid,
    pub username: String,
    pub hashed_password: String,
    pub admin: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PublicUser {
    pub id: Uuid,
    pub username: String,
    pub admin: bool,
}
