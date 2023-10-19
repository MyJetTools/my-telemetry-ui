use std::{cmp::Ordering, rc::Rc};

use dioxus::prelude::*;

use crate::{reader_grpc::AppDataGrpcModel, states::MainState};

pub struct ServiceDataOverviewState {
    data: Option<Vec<AppDataGrpcModel>>,
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
                let started = data.get_started().to_rfc3339();
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
                    let key = tag.key.as_str();
                    let value = if tag.value.len() > 40 {
                        rsx! {
                            span { button { class: "btn btn-sm btn-primary", "Show value" } }
                        }
                    } else {
                        rsx! {
                            span { style: "color:black", tag.value.as_str() }
                        }
                    };
                    rsx! {
                        div { style: "padding:0; color:gray;", " {key}: ", value }
                    }
                });

                let process_id = data.process_id;

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
                                "Show"
                            }
                        }
                    }
                }
            });

            render! {
                div { style: "text-align: left;",
                    button {
                        class: "btn btn-sm btn-primary",
                        style: "padding: 2px 5px;",
                        onclick: move |_| {
                            let main_state = use_shared_state::<MainState>(cx).unwrap();
                            main_state.write().set_selected(cx.props.service_id.clone());
                        },
                        "Back"
                    }

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
            load_services_data(&cx, widget_state);
            render! { h1 { "Loading" } }
        }
    }
}

fn get_max(services: &[AppDataGrpcModel]) -> f64 {
    let mut result = 0;

    for srv in services {
        if srv.duration > result {
            result = srv.duration;
        }
    }

    result as f64
}

fn load_services_data<'s>(
    cx: &Scope<'s, ServiceDataOverviewProps>,
    state: &UseState<ServiceDataOverviewState>,
) {
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
}
