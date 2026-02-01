use dioxus::{fullstack::Form, logger::tracing, prelude::*};

use crate::{api::register::register, app_state::ClientAppState, models::user::AuthRequest};

#[component]
pub fn Account() -> Element {
    let state = use_context::<ClientAppState>();
    let user = state.user.read();

    let mut is_register_modal_open = use_signal(|| false);

    rsx! {
        div { class: "flex flex-col gap-2",
            if let Some(user) = user.as_ref() {
                div { class: "flex gap-2 items-center justify-center",
                    h1 { class: "text-sm italic", "{user.username}" }
                    if user.admin {
                        div { class: "badge badge-soft badge-success", "Администратор" }
                    } else {
                        div { class: "badge badge-soft badge-error", "Администратор" }
                    }
                }
                button { class: "btn btn-error w-auto", "Выйти" }
            } else {
                button {
                    class: "btn btn-primary w-auto",
                    onclick: move |_| {
                        tracing::info!("Test");
                        is_register_modal_open.set(true);
                    },
                    "Зарегистрироваться"
                }
                button { class: "btn btn-secondary w-auto", "Войти" }

                if is_register_modal_open() {
                    dialog { class: "modal", open: true,
                        div { class: "modal-box",
                            form {
                                input {
                                    name: "username",
                                    r#type: "text",
                                    id: "username",
                                    class: "input",
                                    placeholder: "Введите логин...",
                                }
                                input {
                                    name: "password",
                                    r#type: "password",
                                    id: "password",
                                    class: "input",
                                    placeholder: "Введите пароль...",
                                }
                                button { class: "btn btn-primary", "Зарегистрироваться" }
                            }
                        }
                    }
                }
            }
        }
        div { class: "divider" }
    }
}
