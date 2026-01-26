use dioxus::prelude::*;

use crate::app_state::ClientAppState;

#[component]
pub fn Account() -> Element {
    let state = use_context::<ClientAppState>();
    let user = state.user.read();

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
                button { class: "btn btn-primary w-auto", "Зарегистрироваться" }
                button { class: "btn btn-secondary w-auto", "Войти" }
            }
        }
        div { class: "divider" }
    }
}
