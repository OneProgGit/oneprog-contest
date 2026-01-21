use crate::models::user::PublicUser;
use dioxus::signals::Signal;

#[cfg(feature = "server")]
use crate::{AppStateType, database::Database};

#[cfg(feature = "server")]
use dioxus::fullstack::{FullstackContext, extract::FromRef};

#[cfg(feature = "server")]
use std::sync::Arc;

#[derive(Clone)]
#[cfg(feature = "server")]
pub struct AppState<Db: Database> {
    pub db: Arc<Db>,
}

#[cfg(feature = "server")]
impl FromRef<FullstackContext> for AppStateType {
    fn from_ref(state: &FullstackContext) -> Self {
        state
            .extension::<AppStateType>()
            .expect("Failed to get state")
    }
}

#[derive(Clone)]
pub struct ClientAppState {
    pub user: Signal<Option<PublicUser>>,
}
