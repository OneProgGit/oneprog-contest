use crate::{tabs, wrapper::Wrapper};
use dioxus::prelude::*;
use tabs::{
    account::Account, contest::Contest, contests::Contests, home::Home,
    page_not_found::PageNotFound, user::User,
};

#[derive(Debug, Clone, Routable, PartialEq)]
pub enum Route {
    #[layout(Wrapper)]
    #[route("/")]
    Home {},

    #[route("/contests")]
    Contests {},

    #[route("/c/:id")]
    Contest { id: usize },

    #[route("/u/me")]
    Account {},

    #[route("/u/:id")]
    User { id: usize },

    #[route("/:..segments")]
    PageNotFound { segments: Vec<String> },
}
