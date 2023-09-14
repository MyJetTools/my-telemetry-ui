use std::{cmp::Ordering, rc::Rc, time::Duration};

use dioxus::prelude::*;

use crate::{reader_grpc::MetricEventGrpcModel, states::MainState};
#[derive(Props, PartialEq, Eq)]
pub struct ProcessOverviewProps {
    pub service_id: Rc<String>,
    pub data: Rc<String>,
    pub process_id: i64,
}

struct ProcessOverviewState {
    data: Option<Vec<MetricEventGrpcModel>>,
}

pub fn process_overview<'s>(cx: Scope<'s, ProcessOverviewProps>) -> Element {
    let widget_state = use_state(cx, || ProcessOverviewState { data: None });

    let content = match widget_state.get().data.as_ref() {
        Some(items) => {
            if items.len() == 0 {
                load_data(&cx, widget_state);
                rsx! { h1 { "No Data" } }
            } else {
                let mut to_render = Vec::new();

                let min_max = get_min_max(items);

                for item in items {
                    let started = item.get_started().to_rfc3339();

                    let bg_color = if &item.data == cx.props.data.as_str() {
                        "lightgray"
                    } else {
                        "white"
                    };

                    let (message, color) = match &item.fail {
                        Some(error) => (error.as_str(), "red"),
                        None => match &item.success {
                            Some(success) => (success.as_str(), "green"),
                            None => ("", "black"),
                        },
                    };

                    let tags = item.tags.iter().map(|tag| {
                        let key = tag.key.as_str();
                        let value = tag.value.as_str();
                        rsx! {
                            div { style: "padding:0; color:gray;",
                                " {key}: "
                                span { style: "color:black", value }
                            }
                        }
                    });

                    let duration = format!("{:?}", item.get_duration());

                    let delay = item.started - min_max.start;

                    let duration_line_left = ((item.started - min_max.start) as f64
                        / min_max.max_duration as f64)
                        * 100.0;

                    let duration_line_width =
                        (item.duration as f64 / min_max.max_duration as f64) * 100.0;

                    let delay = if delay > 0 {
                        format!("{:?}", Duration::from_micros(delay as u64))
                    } else {
                        "".to_string()
                    };

                    to_render.push(rsx! {
                        tr { class: "table-line", style: "background-color:{bg_color}",
                            td { started }
                            td {
                                div { style: "padding:0", "{item.name}" }
                                div { style: "padding:0; font-size:10px", "{item.data}" }
                            }
                            td { duration }
                            td { style: "color:{color}", message }
                            td { tags }
                        }
                    });

                    to_render.push(rsx! {
                        tr {
                            td { colspan: 5,
                                div { style: "padding:0; width:{duration_line_left}%; font-size:8px;text-align: right;",
                                    delay
                                }
                                div { style: "padding:0; margin-left:{duration_line_left}%; width:{duration_line_width}%; background-color:blue; height:2px" }
                            }
                        }
                    })
                }

                rsx! {
                    table { class: "table", style: "text-align: left;",
                        tr {
                            td { "Started" }
                            td { "Name" }
                            td { "Duration" }
                            td { "Message" }
                            td { "Tags" }
                        }
                        to_render.into_iter()
                    }
                }
            }
        }
        None => {
            load_data(&cx, widget_state);
            rsx! { h1 { "Loading..." } }
        }
    };

    render! {
        div { style: "text-align: left;",
            button {
                class: "btn btn-sm btn-primary",
                style: "padding: 2px 5px;",
                onclick: move |_| {
                    let main_state = use_shared_state::<MainState>(cx).unwrap();
                    main_state
                        .write()
                        .set_selected_data(cx.props.service_id.clone(), cx.props.data.clone());
                },
                "Back"
            }
            b { "{cx.props.process_id}" }
            hr {}
        }
        content
    }
}

pub struct MinMax {
    start: i64,
    finished: i64,
    max_duration: i64,
}

fn get_min_max(items: &[MetricEventGrpcModel]) -> MinMax {
    let mut result = {
        let first = items.first().unwrap();
        MinMax {
            start: first.started,
            finished: first.started,
            max_duration: 0,
        }
    };

    for item in items {
        if result.start > item.started {
            result.start = item.started;
        }

        let finished = item.started + item.duration;

        if result.finished < finished {
            result.finished = finished;
        }
    }

    result.max_duration = result.finished - result.start;

    result
}

fn load_data<'s>(cx: &Scope<'s, ProcessOverviewProps>, state: &UseState<ProcessOverviewState>) {
    let state = state.to_owned();

    let process_id = cx.props.process_id;

    cx.spawn(async move {
        let mut response = crate::api_client::get_by_process_id(process_id)
            .await
            .unwrap();

        response.sort_by(|i1, i2| {
            if i1.started > i2.started {
                Ordering::Greater
            } else if i1.started < i2.started {
                Ordering::Less
            } else {
                Ordering::Equal
            }
        });

        state.set(ProcessOverviewState {
            data: Some(response),
        });
    });
}
