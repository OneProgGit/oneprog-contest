use dioxus::prelude::*;

use crate::route::Route;

#[component]
pub fn Navbar() -> Element {
    rsx! {
        div { class: "navbar bg-base-200 flex flex-col gap-5 items-start justify-start",
            id: "navbar",

            h1 { class: "text-4xl",
                "OneProg Контест"
            }

            div { role: "tablist", class: "tabs tabs-box gap-2 bg-base-300",
                Link {
                    to: Route::Home {},
                    button { role: "tab", class: "tab",
                        "Главная"
                    }
                }

                Link {
                    to: Route::Problems {},
                    button { role: "tab", class: "tab",
                        "Задачи"
                    }
                }

                Link {
                    to: Route::Contests {},
                    button { role: "tab", class: "tab",
                        "Контесты"
                    }
                }

                Link {
                    to: Route::Competitions {},
                    button { role: "tab", class: "tab",
                        "Соревнования"
                    }
                }

                Link {
                    to: Route::Market {},
                    button { role: "tab", class: "tab",
                        "Рынок"
                    }
                }

                Link {
                    to: Route::Business {},
                    button { role: "tab", class: "tab",
                        "Бизнес"
                    }
                }

                Link {
                    to: Route::Support {},
                    button { role: "tab", class: "tab",
                        "Поддержать"
                    }
                }

                Link {
                    to: Route::Account {},
                    button { role: "tab", class: "tab",
                        "Аккаунт"
                    }
                }
            }
        }

        div { class: "m-2",
            Outlet::<Route> { }
        }
    }
}
