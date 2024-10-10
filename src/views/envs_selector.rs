use dioxus::prelude::*;

use crate::{states::*, AppRoute};

#[component]
pub fn EnvsSelector() -> Element {
    let main_state = consume_context::<Signal<MainState>>();

    let main_state_read_access = main_state.read();

    let envs_options = main_state_read_access.envs.iter().map(|env| {
        if main_state_read_access.envs.is_selected(env.as_str()) {
            rsx! {
                option { selected: true, {env.as_str() } }
            }
        } else {
            rsx! {
                option { {env.as_str() } }
            }
        }
    });

    rsx! {
        select {
            class: "form-select",
            style: "background-color: white;",

            value: main_state_read_access.envs.get_selected().as_str(),

            oninput: move |ctx| {
                let value = ctx.value();
                crate::storage::selected_env::set(value.as_str());
                consume_context::<Signal<MainState>>()
                    .write()
                    .envs
                    .set_active_env(value.as_str());
                navigator().push(AppRoute::Home {});
            },
            {envs_options}
        }
    }
}
