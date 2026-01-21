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

        div { class: "flex gap-8 items-center justify-start",
            div { role: "tablist", class: "tabs tabs-box gap-2 bg-base-300",
                Link { role: "tab", class: "tab", to: Route::Home {}, "Главная" }
                Link { role: "tab", class: "tab", to: Route::Problems {}, "Задачи" }
                Link { role: "tab", class: "tab", to: Route::Contests {}, "Контесты" }

                Link { role: "tab", class: "tab", to: Route::Competitions {},
                    "Соревнования"
                }

                Link { role: "tab", class: "tab", to: Route::Market {}, "Рынок" }
                Link { role: "tab", class: "tab", to: Route::Business {}, "Бизнес" }
                Link { role: "tab", class: "tab", to: Route::Support {}, "Поддержать" }
                Link { role: "tab", class: "tab", to: Route::Account {}, "Аккаунт" }
            }
            h1 { class: "font-extrabold",
                if let Some(user) = user.as_ref() {
                    "Залогинен как {user.username}"
                } else {
                    "Не залогинен"
                }
            }
        }
    }
}
