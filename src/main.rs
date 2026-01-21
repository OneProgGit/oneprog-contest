use crate::app_state::ClientAppState;
use crate::route::Route;
use anyhow::Ok;
use dioxus::prelude::*;
use dotenvy::dotenv;

#[cfg(feature = "server")]
use crate::{app_state::AppState, database::postgres::PostgresDatabase};

mod api;

#[cfg(feature = "server")]
mod crypt;

#[cfg(feature = "server")]
mod database;

#[cfg(feature = "server")]
mod jwt;

#[cfg(feature = "server")]
mod middleware;

mod app_state;
mod footer;
pub mod models;
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
        let router = dioxus::server::router(App).layer(Extension(state));
        Ok(router)
    });

    Ok(())
}

#[component]
fn App() -> Element {
    let state = ClientAppState {
        user: Signal::new(None),
    };

    use_context_provider(|| state);

    rsx! {
        document::Stylesheet { href: MAIN_CSS }
        Router::<Route> {}
    }
}
