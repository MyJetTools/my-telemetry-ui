use dioxus::prelude::*;
use serde::*;

use crate::DataState;
#[component]
pub fn RenderBottomPanel() -> Element {
    let mut component_state = use_signal(|| DataState::None);

    let component_state_read_access = component_state.read();

    let data = match component_state_read_access.as_ref() {
        DataState::None => {
            spawn(async move {
                let env = crate::storage::selected_env::get();
                let result = load_tech_info(env).await;
                match result {
                    Ok(data) => {
                        component_state.set(DataState::Loaded(data));
                    }
                    Err(err) => {
                        component_state.set(DataState::Error(err.to_string()));
                    }
                }
            });
            return rsx! { "Loading..." };
        }
        DataState::Loading => {
            return rsx! { "Loading..." };
        }
        DataState::Loaded(data) => data,
        DataState::Error(err) => {
            return rsx! { "{err.as_str()}" };
        }
    };

    rsx! {
        table {
            tr {

                td { "Q by process:" }
                td {
                    {data.queue_by_process_size.to_string()},
                    "/"
                    {data.queue_by_process_capacity.to_string()}
                }

                td {
                    div { class: "separator" }
                }

                td { "AppData Size:" }
                td {
                    {data.app_data_size.to_string()},
                    "/"
                    {data.app_data_capacity.to_string()}
                }

                td {
                    div { class: "separator" }
                }

                td { "AppData Q hours size:" }
                td { {data.app_data_hours_size.to_string()} }
                td {
                    div { class: "separator" }
                }

                td { "AppData Q hours to persist:" }
                td {
                    td { {data.app_data_to_persist_hours_size.to_string()} }
                }
                td {
                    div { class: "separator" }
                }
                td { "Q size:" }
                td {
                    {data.queue_size.to_string()},
                    "/"
                    {data.queue_capacity.to_string()}
                }
                td {
                    div { class: "separator" }
                }

                td { "UserId links size:" }
                td {
                    {data.user_id_links_size.to_string()},
                    "/"
                    {data.user_id_links_capacity.to_string()}
                }
            }
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BottomPanelHttpModel {
    pub app_data_hours_size: u64,
    pub app_data_to_persist_hours_size: u64,
    pub queue_size: u64,
    pub queue_capacity: u64,
    pub queue_by_process_size: u64,
    pub queue_by_process_capacity: u64,
    pub user_id_links_size: u64,
    pub user_id_links_capacity: u64,

    pub app_data_size: u64,
    pub app_data_capacity: u64,
}

#[server]
pub async fn load_tech_info(env: String) -> Result<BottomPanelHttpModel, ServerFnError> {
    let grpc_client = crate::server::APP_CTX.get_grpc_client(env.as_str()).await;

    let response = grpc_client.get_tech_metrics(()).await.unwrap();

    let result = BottomPanelHttpModel {
        app_data_hours_size: response.app_data_hours_size,
        app_data_to_persist_hours_size: response.app_data_to_persist_hours_size,
        queue_size: response.queue_size,
        queue_capacity: response.queue_capacity,

        queue_by_process_size: response.queue_by_process_size,
        queue_by_process_capacity: response.queue_by_process_capacity,
        user_id_links_size: response.user_id_links_size,
        user_id_links_capacity: response.user_id_links_capacity,

        app_data_size: response.app_data_size,
        app_data_capacity: response.app_data_capacity,
    };
    Ok(result)
}
