use crate::{
    database::Database,
    models::{
        post::{DatabasePost, Post},
        user::{DatabaseUser, FullUser},
    },
};
use sqlx::{PgPool, postgres::PgPoolOptions};
use std::sync::Arc;
use uuid::Uuid;

#[derive(Clone)]
pub struct PostgresDatabase {
    pool: PgPool,
}

impl Database for PostgresDatabase {
    async fn new(url: String) -> anyhow::Result<Arc<Self>> {
        let pool = PgPoolOptions::new().connect(&url).await?;
        Ok(Arc::new(Self { pool }))
    }

    async fn create_user(&self, user: DatabaseUser) -> anyhow::Result<()> {
        sqlx::query(
            "
            insert into users (username, hashed_password)
            values ($1, $2)
            ",
        )
        .bind(user.username)
        .bind(user.hashed_password)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    async fn get_user_by_id(&self, id: Uuid) -> anyhow::Result<FullUser> {
        let user = sqlx::query_as("select * from users where id=$1")
            .bind(id)
            .fetch_one(&self.pool)
            .await?;
        Ok(user)
    }

    async fn get_user_by_username(&self, username: &str) -> anyhow::Result<FullUser> {
        let user = sqlx::query_as("select * from users where username=$1")
            .bind(username)
            .fetch_one(&self.pool)
            .await?;
        Ok(user)
    }

    async fn create_post(&self, post: DatabasePost) -> anyhow::Result<()> {
        sqlx::query(
            "
            insert into posts (author_id, title, content)
            values ($1, $2, $3)
            ",
        )
        .bind(post.author_id)
        .bind(post.title)
        .bind(post.content)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    async fn get_posts(&self) -> anyhow::Result<Vec<Post>> {
        let posts = sqlx::query_as("select * from posts")
            .fetch_all(&self.pool)
            .await?;
        Ok(posts)
    }
}
