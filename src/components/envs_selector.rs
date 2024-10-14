use dioxus::prelude::*;

use crate::{states::*, AppRoute};

#[component]
pub fn EnvsSelector() -> Element {
    let mut main_state = consume_context::<Signal<MainState>>();

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
            class: "form-select-sm",
            style: "background-color: white; width:100%;border-color: lightgray;",

            value: main_state_read_access.envs.get_selected().as_str(),

            oninput: move |ctx| {
                let value = ctx.value();
                crate::storage::selected_env::set(value.as_str());
                {
                    let mut main_state = main_state.write();
                    main_state.envs.set_active_env(value.as_str());
                    main_state.env_updated();
                }
                navigator().push(AppRoute::SelectEnv { env: value });
            },
            {envs_options}
        }
    }
}
