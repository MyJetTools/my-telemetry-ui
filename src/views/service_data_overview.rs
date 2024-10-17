use std::rc::Rc;

use crate::{
    models::*,
    states::{DialogState, MainState},
    utils::to_base_64,
    AppRoute,
};
use dioxus::prelude::*;
use dioxus_router::prelude::Link;

#[component]
pub fn ServiceDataOverview(data: Rc<String>) -> Element {
    let mut main_state = consume_context::<Signal<MainState>>();
    let main_state_read_access = main_state.read();

    let hours_ago = main_state_read_access.get_hours_ago();

    let service_id = main_state_read_access
        .get_selected_service()
        .clone()
        .unwrap();

    let service_data = match &main_state_read_access.server_data_overview {
        crate::DataState::None => {
            spawn(async move {
                main_state.write().server_data_overview = crate::DataState::Loading;
                let service_id = service_id.to_string();
                let service_data = data.to_string();
                let client_id = crate::storage::client_id::get();
                let env = crate::storage::selected_env::get();
                let seconds_from = crate::storage::from_time::get();
                let data = load_services_data(
                    env,
                    hours_ago,
                    service_id,
                    service_data,
                    client_id,
                    seconds_from.to_seconds_within_hour(),
                )
                .await;

                match data {
                    Ok(data) => {
                        main_state.write().server_data_overview =
                            crate::DataState::Loaded(Rc::new(data));
                    }
                    Err(err) => {
                        main_state.write().server_data_overview =
                            crate::DataState::Error(err.to_string());
                    }
                }
            });

            return rsx! { "Loading..." };
        }
        crate::DataState::Loading => return rsx! { "Loading..." },
        crate::DataState::Loaded(data) => data,
        crate::DataState::Error(err) => {
            return rsx! {
                div {
                    h1 { "Error" }
                    p { {err.as_str()} }
                }
            }
        }
    };
    let max_duration = get_max(service_data);
    let items = service_data.iter().map(|service_data| {
        let started = service_data.get_started().to_rfc3339();
        let started = &started[..26];
        
        

        let duration = format!("{:?}", service_data.get_duration());

        let bar_duration = (service_data.duration as f64 / max_duration) * 100.0;

        let (message, color) = match &service_data.success {
            Some(success) => (success.as_str(), "green"),
            None => match &service_data.fail {
                Some(error) => (error.as_str(), "red"),
                None => ("", "black"),
            },
        };

        let tags = service_data.tags.iter().map(|tag| {
            let key = Rc::new(tag.key.to_string());
            let key_show_dialog = key.clone();
            let value = Rc::new(tag.value.to_string());
            let value_show_dialog = value.clone();

            let value = if tag.value.len() > 40 {
                rsx! {
                    span {
                        button {
                            class: "btn btn-sm btn-primary",
                            onclick: move |_| {
                                consume_context::<Signal<MainState>>()
                                    .write()
                                    .show_dialog(DialogState::ShowKeyValue {
                                        the_key: key_show_dialog.clone(),
                                        value: value_show_dialog.clone(),
                                    });
                            },
                            "Show value"
                        }
                    }
                }
            } else {
                rsx! {
                    span { style: "color:black", {tag.value.as_str()} }
                }
            };
            rsx! {
                div { style: "padding:0; color:gray;",
                    " {key.as_str()}: "
                    {value}
                }
            }
        });

        let process_id = service_data.process_id;


        let service_id_1 = service_id.clone();
        let service_id_2 = service_id_1.clone();
        let action_base_64 = to_base_64(data.as_str());

        let data_cloned = data.clone();

        rsx! {
            tr { class: "table-line",
                td {
                    {started},
                    div { style: "width:100%;padding:0",
                        div { style: "width: {bar_duration}%; height: 2px; background-color:green" }
                    }
                }
                td { {duration} }
                td { style: "color: {color}", {message} }
                td { {tags} }
                td {
                    button {
                        class: "btn btn-sm btn-primary",
                        style: "padding: 2px 5px;",
                        Link {
                            onclick: move |_| {
                                consume_context::<Signal<MainState>>()
                                    .write()
                                    .set_show_process(service_id_1.clone(), data_cloned.clone(), process_id);
                            },
                            to: AppRoute::Process {
                                service: service_id_2.to_string(),
                                action: action_base_64,
                                id: process_id,
                            },
                            "Show"
                        }
                    }
                }
            }
        }
    });

    rsx! {
        div { class: "table_top_label",
            b { "{data}" }
            hr {}
        }
        table { class: "table", style: "text-align: left;",
            tr {
                th { "Started" }
                th { "Duration" }
                th { "Message" }
                th { "Tags" }
                th {}
            }
            {items}
        }
    }
}

fn get_max(services: &[ServiceDataApiModel]) -> f64 {
    let mut result = 0;

    for srv in services {
        if srv.duration > result {
            result = srv.duration;
        }
    }

    result as f64
}

#[server]
async fn load_services_data(
    env: String,
    hours_ago: i64,
    service_id: String,
    service_data: String,
    client_id: String,
    from_sec_within_hour: i64,
) -> Result<Vec<ServiceDataApiModel>, ServerFnError> {
    let mut response = crate::server::api_client::get_by_service_data(
        env.as_str(),
        hours_ago,
        service_id,
        service_data,
        client_id,
        from_sec_within_hour,
    )
    .await
    .unwrap();

    response.sort_by(|i1, i2| {
        if i1.started < i2.started {
            std::cmp::Ordering::Greater
        } else if i1.started > i2.started {
            std::cmp::Ordering::Less
        } else {
            std::cmp::Ordering::Equal
        }
    });

    let result: Vec<ServiceDataApiModel> = response
        .into_iter()
        .map(|src| ServiceDataApiModel {
            process_id: src.process_id,
            started: src.started,
            duration: src.duration as u64,
            success: src.success,
            fail: src.fail,
            tags: src
                .tags
                .into_iter()
                .map(|tag| TagApiModel {
                    key: tag.key,
                    value: tag.value,
                })
                .collect(),
        })
        .collect();

    Ok(result)

    /*
    let state = state.to_owned();

    let service_id = cx.props.service_id.as_str().to_string();
    let service_data = cx.props.data.as_str().to_string();
    cx.spawn(async move {
        let mut response = crate::api_client::get_by_service_data(service_id.clone(), service_data)
            .await
            .unwrap();

        response.sort_by(|i1, i2| {
            if i1.started < i2.started {
                Ordering::Greater
            } else if i1.started > i2.started {
                Ordering::Less
            } else {
                Ordering::Equal
            }
        });
        state.set(ServiceDataOverviewState {
            data: Some(response),
        });
    });
     */
}
