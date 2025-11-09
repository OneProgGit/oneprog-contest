use dioxus::prelude::*;

use crate::route::Route;

#[component]
pub fn Navbar() -> Element {
    rsx! {
        div { class: "navbar bg-base-200 flex flex-col gap-5 items-start justify-start",
            id: "navbar",

            h1 { class: "text-4xl font-bold",
                "OneProg Контест"
            }

            div { role: "tablist", class: "tabs tabs-box gap-2 bg-base-300",
                Link { role: "tab", class: "tab",
                    to: Route::Home {},
                    "Главная"
                }

                Link { role: "tab", class: "tab",
                    to: Route::Problems {},
                    "Задачи"
                }

                Link { role: "tab", class: "tab",
                    to: Route::Contests {},
                    "Контесты"
                }

                Link { role: "tab", class: "tab",
                    to: Route::Competitions {},
                    "Соревнования"
                }

                Link { role: "tab", class: "tab",
                    to: Route::Market {},
                    "Рынок"
                }

                Link { role: "tab", class: "tab",
                    to: Route::Business {},
                    "Бизнес"
                }

                Link { role: "tab", class: "tab",
                    to: Route::Support {},
                    "Поддержать"
                },

                Link { role: "tab", class: "tab",
                    to: Route::Account {},
                    "Аккаунт"
                }
            }
        }

        div { class: "m-2",
            Outlet::<Route> { }
        }
    }
}
