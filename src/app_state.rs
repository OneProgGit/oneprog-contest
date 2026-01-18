use crate::{AppStateType, database::Database};
use dioxus::fullstack::{FullstackContext, extract::FromRef};
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState<Db: Database> {
    pub db: Arc<Db>,
}

impl FromRef<FullstackContext> for AppStateType {
    fn from_ref(state: &FullstackContext) -> Self {
        state
            .extension::<AppStateType>()
            .expect("Failed to get state")
    }
}
