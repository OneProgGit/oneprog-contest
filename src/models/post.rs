use serde::{Deserialize, Serialize};

#[cfg(feature = "server")]
use sqlx::prelude::FromRow;

use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct CreatePostRequest {
    pub title: String,
    pub content: String,
}

#[cfg(feature = "server")]
pub struct DatabasePost {
    pub author_id: Uuid,
    pub title: String,
    pub content: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "server", derive(FromRow))]
pub struct Post {
    pub id: Uuid,
    pub author_id: Uuid,
    pub title: String,
    pub content: String,
}
