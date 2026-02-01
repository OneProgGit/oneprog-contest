use crate::{app_state::ClientAppState, route::Route};
use dioxus::prelude::*;

#[component]
pub fn Navbar() -> Element {
    const VERSION: &str = env!("CARGO_PKG_VERSION");

    let state = use_context::<ClientAppState>();
    let user = state.user.read();

    rsx! {
        div {
            class: "navbar bg-base-200 flex flex-col gap-5 items-start justify-start",
            id: "navbar",

            h1 { class: "text-4xl font-bold", "OneProg Контест v{VERSION}" }
        }

        div { class: "flex w-full gap-8 w-auto items-center justify-start",
            div {
                role: "tablist",
                class: "tabs w-full tabs-box w-auto gap-2 bg-base-300",
                Link { role: "tab", class: "tab flex-1", to: Route::Home {}, "Главная" }
                Link {
                    role: "tab",
                    class: "tab flex-1",
                    to: Route::Contests {},
                    "Контесты"
                }

                Link { role: "tab", class: "tab flex-1", to: Route::Account {},
                    {
                        format!(
                            "Аккаунт ({})",
                            if let Some(user) = user.as_ref() {
                                user.username.clone()
                            } else {
                                "нет входа в аккаунт".into()
                            },
                        )
                    }
                }
            }
        }
    }
}
