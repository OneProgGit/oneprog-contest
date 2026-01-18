use crate::{tabs, wrapper::Wrapper};
use dioxus::prelude::*;
use tabs::{
    account::Account, business::Business, competitions::Competitions, contest::Contest,
    contests::Contests, home::Home, market::Market, page_not_found::PageNotFound, problem::Problem,
    problems::Problems, support::Support, user::User, users::Users,
};

#[derive(Debug, Clone, Routable, PartialEq)]
pub enum Route {
    #[layout(Wrapper)]
    #[route("/")]
    Home {},

    #[route("/problems")]
    Problems {},

    #[route("/p/:id")]
    Problem { id: usize },

    #[route("/contests")]
    Contests {},

    #[route("/c/:id")]
    Contest { id: usize },

    #[route("/competitions")]
    Competitions {},

    #[route("/market")]
    Market {},

    #[route("/business")]
    Business {},

    #[route("/support")]
    Support {},

    #[route("/users")]
    Users {},

    #[route("/u/me")]
    Account {},

    #[route("/u/:id")]
    User { id: usize },

    #[route("/:..segments")]
    PageNotFound { segments: Vec<String> },
}
