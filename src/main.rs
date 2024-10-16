use dioxus::prelude::*;

//mod http_server;

mod components;
mod models;
mod states;
mod storage;
mod utils;
mod views;

#[cfg(feature = "server")]
mod server;
use serde::*;
use views::*;

use crate::states::*;

use crate::utils::from_base_64;

#[derive(Routable, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum AppRoute {
    #[route("/")]
    Home,

    #[route("/env/:env")]
    SelectEnv { env: String },

    #[route("/actions/:service")]
    Actions { service: String },

    #[route("/last/:service/:action")]
    LastEvents { service: String, action: String },

    #[route("/process/:service/:action/:id")]
    Process {
        service: String,
        action: String,
        id: i64,
    },

    #[route("/:..segments")]
    NotFound { segments: Vec<String> },
}

#[component]
fn NotFound(segments: Vec<String>) -> Element {
    rsx! { "404: Not Found" }
}

fn main() {
    let cfg = dioxus::fullstack::Config::new();

    #[cfg(feature = "server")]
    let cfg = cfg.addr(([0, 0, 0, 0], 9001));

    LaunchBuilder::fullstack().with_cfg(cfg).launch(|| {
        rsx! {
            Router::<AppRoute> {}
        }
    })
}

#[component]
fn Home() -> Element {
    use_context_provider(|| Signal::new(MainState::new()));
    rsx! {
        MyLayout {}
    }
}

#[component]
fn SelectEnv(env: String) -> Element {
    use_context_provider(|| Signal::new(MainState::new()));
    rsx! {
        MyLayout {}
    }
}

#[component]
fn Actions(service: String) -> Element {
    use_context_provider(|| Signal::new(MainState::new_with_selected_service(service)));
    rsx! {
        MyLayout {}
    }
}

#[component]
fn LastEvents(service: String, action: String) -> Element {
    println!("LastEvents: {}/{}", service, action);
    use_context_provider(|| {
        Signal::new(MainState::new_with_selected_action(
            service,
            from_base_64(action.as_str()),
        ))
    });
    rsx! {
        MyLayout {}
    }
}

#[component]
fn Process(service: String, action: String, id: i64) -> Element {
    println!("LastEvents: {}/{}/{}", service, action, id);
    use_context_provider(|| {
        Signal::new(MainState::new_with_selected_process(
            service,
            from_base_64(action.as_str()),
            id,
        ))
    });
    rsx! {
        MyLayout {}
    }
}

#[component]
pub fn MyLayout() -> Element {
    let mut main_state = consume_context::<Signal<MainState>>();
    let main_state_read_access = main_state.read();

    if main_state_read_access.envs.initialized() && main_state_read_access.files.initialized() {
        return rsx! {
            div { id: "layout",
                div { id: "left-panel", LeftPanel {} }

                div { id: "right-panel", RightPanel {} }
                div { id: "top-panel", RenderTopPanel {} }
                div { id: "bottom-panel", RenderBottomPanel {} }
                dialog::RenderDialog {}
            }
        };
    }

    let mut loading_envs_state = use_signal(|| DataState::None);
    let loading_envs_state_read_access = loading_envs_state.read();

    match loading_envs_state_read_access.as_ref() {
        DataState::None => {
            spawn(async move {
                loading_envs_state.set(DataState::Loading);

                let (envs_initialized, files_initialized) = {
                    let read_access = main_state.read();
                    (
                        read_access.envs.initialized(),
                        read_access.files.initialized(),
                    )
                };

                if !envs_initialized {
                    let envs = get_envs().await;
                    match envs {
                        Ok(envs) => {
                            main_state.write().envs.set_envs(envs);
                        }
                        Err(err) => {
                            loading_envs_state.set(DataState::Error(err.to_string()));
                            return;
                        }
                    }
                }

                if !files_initialized {
                    let selected_env = { main_state.read().envs.get_selected().to_string() };

                    let envs = get_available_files_to_read(selected_env).await;
                    match envs {
                        Ok(envs) => {
                            main_state.write().files.set_files(envs);
                        }
                        Err(err) => {
                            loading_envs_state.set(DataState::Error(err.to_string()));
                            return;
                        }
                    }
                }

                loading_envs_state.set(DataState::Loaded(()));
            });
            return render_loading_environments();
        }

        DataState::Loading => {
            return render_loading_environments();
        }

        DataState::Loaded(_) => {
            return render_loading_environments();
        }

        DataState::Error(err) => return rsx! { "Error loading environments: {err}" },
    }
}

fn render_loading_environments() -> Element {
    rsx! { "Loading environments..." }
}

#[server]
pub async fn get_envs() -> Result<Vec<String>, ServerFnError> {
    let result = crate::server::APP_CTX
        .settings_reader
        .get_settings()
        .await
        .get_envs();

    Ok(result)
}

#[server]
pub async fn get_available_files_to_read(
    env: String,
) -> Result<Vec<crate::models::MetricFileHttpModel>, ServerFnError> {
    let grpc_client = crate::server::APP_CTX.get_grpc_client(env.as_str()).await;

    let items = grpc_client
        .get_available_hours_ago(())
        .await
        .unwrap()
        .unwrap_or_default();

    let result = items
        .into_iter()
        .map(|itm| crate::models::MetricFileHttpModel {
            hours_ago: itm.hour_ago,
            file_size: itm.file_size,
        })
        .collect();

    Ok(result)
}
