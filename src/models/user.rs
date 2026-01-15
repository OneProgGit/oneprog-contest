use serde::{Deserialize, Serialize};
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
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub hashed_password: String,
    pub admin: bool,
}
