use dioxus::prelude::*;

use crate::app_state::ClientAppState;

#[component]
pub fn Account() -> Element {
    let state = use_context::<ClientAppState>();
    let user = state.user.read();

    rsx! {
        div { class: "flex flex-col gap-2",
            if let Some(user) = user.as_ref() {
                h1 { class: "text-sm italic", "{user.username}" }
                h1 {
                    if user.admin {
                        "Администратор"
                    } else {
                        "Не администратор"
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
