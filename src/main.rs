use crate::route::Route;

#[cfg(feature = "server")]
use crate::{app_state::AppState, database::postgres::PostgresDatabase};

use anyhow::Ok;
use dioxus::prelude::*;
use dotenvy::dotenv;

#[cfg(feature = "server")]
mod api;

#[cfg(feature = "server")]
mod app_state;

#[cfg(feature = "server")]
mod crypt;

#[cfg(feature = "server")]
mod database;

#[cfg(feature = "server")]
mod jwt;

#[cfg(feature = "server")]
mod middleware;

#[cfg(feature = "server")]
pub mod models;

mod footer;
mod navbar;
mod route;
mod tabs;
mod wrapper;

#[cfg(feature = "server")]
pub type AppStateType = AppState<PostgresDatabase>;

const MAIN_CSS: Asset = asset!("/assets/tailwind.css");

#[allow(unreachable_code)]
fn main() -> anyhow::Result<()> {
    dotenv()?;
    #[cfg(not(feature = "server"))]
    dioxus::launch(App);
    #[cfg(feature = "server")]
    dioxus::serve(|| async move {
        use crate::database::Database;
        use dioxus::server::axum::Extension;

        let db_url = dotenvy::var("DB_URL").expect("DB_URL must be set!");
        let db = PostgresDatabase::new(db_url)
            .await
            .expect("Failed to connect to database");
        let state = AppStateType { db };
        let router = dioxus::server::router(App);
        let router = dioxus::server::router(App)
            .nest("/api", router)
            .layer(Extension(state));
        Ok(router)
    });

    Ok(())
}

#[component]
fn App() -> Element {
    rsx! {
        document::Stylesheet { href: MAIN_CSS }
        Router::<Route> {}
    }
}
