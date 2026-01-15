#![cfg(feature = "server")]

use uuid::Uuid;

use crate::models::{
    post::{DatabasePost, Post},
    user::{DatabaseUser, User},
};
use std::sync::Arc;

pub mod postgres;

pub trait Database: Send + Sync + Clone {
    async fn new(url: String) -> anyhow::Result<Arc<Self>>;
    async fn create_user(&self, user: DatabaseUser) -> anyhow::Result<()>;
    async fn get_user_by_id(&self, id: Uuid) -> anyhow::Result<User>;
    async fn get_user_by_username(&self, username: &str) -> anyhow::Result<User>;
    async fn create_post(&self, post: DatabasePost) -> anyhow::Result<()>;
    async fn get_k_posts(&self, k: u32) -> anyhow::Result<Vec<Post>>;
}
