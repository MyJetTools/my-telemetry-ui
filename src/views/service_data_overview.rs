use std::{rc::Rc, time::Duration};

use crate::{
    router::AppRoute,
    states::{DialogState, MainState},
    utils::to_base_64,
};
use dioxus::prelude::*;
use dioxus_fullstack::prelude::*;
use dioxus_router::prelude::Link;

pub struct ServiceDataOverviewState {
    data: Option<Vec<ServiceDataApiModel>>,
}

impl ServiceDataOverviewState {
    pub fn new() -> Self {
        Self { data: None }
    }
}

#[derive(Props, PartialEq, Eq)]
pub struct ServiceDataOverviewProps {
    pub service_id: Rc<String>,
    pub data: Rc<String>,
}

pub fn service_data_overview<'s>(cx: Scope<'s, ServiceDataOverviewProps>) -> Element {
    let widget_state = use_state(cx, || ServiceDataOverviewState::new());

    match widget_state.get().data.as_ref() {
        Some(data) => {
            let max_duration = get_max(data);
            let items = data.iter().map(|data| {
                let started = data.get_started();
                let duration = format!("{:?}", data.get_duration());

                let bar_duration = (data.duration as f64 / max_duration) * 100.0;

                let (message, color) = match &data.success {
                    Some(success) => (success.as_str(), "green"),
                    None => match &data.fail {
                        Some(error) => (error.as_str(), "red"),
                        None => ("", "black"),
                    },
                };

                let tags = data.tags.iter().map(|tag| {
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
                                        use_shared_state::<MainState>(cx)
                                            .unwrap()
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
                            span { style: "color:black", tag.value.as_str() }
                        }
                    };
                    rsx! {
                        div { style: "padding:0; color:gray;", " {key.as_str()}: ", value }
                    }
                });

                let process_id = data.process_id;
                let service_id_2 = cx.props.service_id.to_string();
                let action_base_64 = to_base_64( cx.props.data.as_str());

                rsx! {
                    tr { class: "table-line",
                        td {
                            started,
                            div { style: "width:100%;padding:0", div { style: "width: {bar_duration}%; height: 2px; background-color:green" } }
                        }
                        td { duration }
                        td { style: "color: {color}", message }
                        td { tags }
                        td {
                            button {
                                class: "btn btn-sm btn-primary",
                                style: "padding: 2px 5px;",
                                Link {
                                    onclick: move |_| {
                                        let right_panel_state = use_shared_state::<MainState>(cx).unwrap();
                                        right_panel_state
                                            .write()
                                            .set_show_process(
                                                cx.props.service_id.clone(),
                                                cx.props.data.clone(),
                                                process_id,
                                            );
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

            render! {
                div { style: "text-align: left;",
                    b { "{cx.props.data}" }
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
                    items
                }
            }
        }
        None => {
            let service_id = cx.props.service_id.as_ref().to_string();
            let service_data = cx.props.data.as_ref().to_string();
            let widget_state = widget_state.to_owned();
            cx.spawn(async move {
                let data = load_services_data(service_id, service_data).await.unwrap();
                widget_state.set(ServiceDataOverviewState { data: Some(data) })
            });
            render! { h1 { "Loading" } }
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

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct ServiceDataApiModel {
    pub process_id: i64,
    pub started: i64,
    pub duration: u64,
    pub success: Option<String>,
    pub fail: Option<String>,
    pub tags: Vec<TagApiModel>,
}

impl ServiceDataApiModel {
    pub fn get_started(&self) -> String {
        crate::utils::unix_microseconds_to_string(self.started)
    }

    pub fn get_duration(&self) -> Duration {
        Duration::from_micros(self.duration)
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct TagApiModel {
    pub key: String,
    pub value: String,
}

#[server]
async fn load_services_data(
    service_id: String,
    service_data: String,
) -> Result<Vec<ServiceDataApiModel>, ServerFnError> {
    let mut response = crate::api_client::get_by_service_data(service_id, service_data)
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
