use std::{rc::Rc, time::Duration};

use dioxus::prelude::*;

use crate::{
    models::*,
    states::{DialogState, MainState},
    DataState,
};

struct ProcessOverviewState {
    data: DataState<Vec<Rc<MetricEventApiModel>>>,
}

impl ProcessOverviewState {
    pub fn new() -> Self {
        Self {
            data: DataState::None,
        }
    }
}

#[component]
pub fn ProcessOverview(data: Rc<String>, process_id: i64) -> Element {
    let mut widget_state = use_signal(|| ProcessOverviewState::new());

    let widget_state_read_access = widget_state.read();

    let items = match widget_state_read_access.data.as_ref() {
        DataState::None => {
            spawn(async move {
                widget_state.write().data = DataState::Loading;

                let env = crate::storage::selected_env::get();

                let hours_ago = consume_context::<Signal<MainState>>()
                    .read()
                    .get_hours_ago();
                let response = load_metric_events(env, hours_ago, process_id).await;

                match response {
                    Ok(response) => {
                        let data: Vec<_> = response.into_iter().map(|itm| Rc::new(itm)).collect();
                        widget_state.write().data = data.into();
                    }
                    Err(err) => {
                        widget_state.write().data = DataState::Error(err.to_string());
                    }
                }
            });
            return rsx! {
                h1 { "Loading..." }
            };
        }
        DataState::Loading => {
            return rsx! {
                h1 { "Loading..." }
            }
        }

        DataState::Loaded(data) => data,

        DataState::Error(err) => {
            return rsx! {
                h1 { {err.as_str()} }
            }
        }
    };

    let to_render = if items.len() == 0 {
        rsx! {
            h1 { "No Data" }
        }
    } else {
        let mut to_render = Vec::new();

        let min_max = get_min_max(&items);

        for item in items {
            let started = item.get_started().to_rfc3339();
            let started = &started[..26];

            let bg_color = if &item.data == data.as_str() {
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

            let duration = format!("{:?}", item.get_duration());

            let delay = item.started - min_max.start;

            let duration_line_left =
                ((item.started - min_max.start) as f64 / min_max.max_duration as f64) * 100.0;

            let duration_line_width = (item.duration as f64 / min_max.max_duration as f64) * 100.0;

            let delay = if delay > 0 {
                format!("{:?}", Duration::from_micros(delay as u64))
            } else {
                "".to_string()
            };

            let data = Rc::new(item.data.to_string());

            let data_to_expand = data.clone();

            let item_spawned = item.clone();

            let data_to_render = if item.data.len() > 64 {
                rsx! {
                    span {
                        button {
                            class: "btn btn-sm btn-light",
                            onclick: move |_| {
                                consume_context::<Signal<MainState>>()
                                    .write()
                                    .show_dialog(DialogState::ShowKeyValue {
                                        the_key: Rc::new(format!("Expanding data for {}", item_spawned.name)),
                                        value: data_to_expand.clone(),
                                    });
                            },
                            "{&data_to_expand[..64]}..."
                        }
                    }
                }
            } else {
                rsx! {
                    span { "{item.data}" }
                }
            };

            to_render.push(rsx! {
                tr { class: "table-line", style: "background-color:{bg_color}",
                    td { {started} }
                    td {
                        div { style: "padding:0", "{item.name}" }
                        div { style: "padding:0; font-size:10px", {data_to_render} }
                    }
                    td { {duration} }
                    td { style: "color:{color}", {message} }
                    td { {tags} }
                }
            });

            to_render.push(rsx! {
                tr {
                    td { colspan: 5,
                        div { style: "padding:0; width:{duration_line_left}%; font-size:8px;text-align: right;",
                            {delay}
                        }
                        div { style: "padding:0; margin-left:{duration_line_left}%; width:{duration_line_width}%; background-color:blue; height:2px" }
                    }
                }
            })
        }

        rsx! {
            table {
                class: "table",
                style: "text-align: left;margin-bottom: 50px;",
                tr {
                    td { "Started" }
                    td { "Name" }
                    td { "Duration" }
                    td { "Message" }
                    td { "Tags" }
                }
                {to_render.into_iter()}
            }
        }
    };

    rsx! {
        div { class: "table_top_label",
            b { "{process_id}" }
            hr {}
        }
        {to_render}
    }
}

pub struct MinMax {
    start: i64,
    finished: i64,
    max_duration: i64,
}

fn get_min_max(items: &[Rc<MetricEventApiModel>]) -> MinMax {
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

#[server]
async fn load_metric_events(
    env: String,
    hours_ago: i64,
    process_id: i64,
) -> Result<Vec<MetricEventApiModel>, ServerFnError> {
    let mut response =
        crate::server::api_client::get_by_process_id(env.as_str(), hours_ago, process_id)
            .await
            .unwrap();

    response.sort_by(|i1, i2| {
        if i1.started > i2.started {
            std::cmp::Ordering::Greater
        } else if i1.started < i2.started {
            std::cmp::Ordering::Less
        } else {
            std::cmp::Ordering::Equal
        }
    });

    let result: Vec<_> = response
        .into_iter()
        .map(|itm| MetricEventApiModel {
            started: itm.started,
            duration: itm.duration,
            name: itm.name,
            data: itm.data,
            tags: itm
                .tags
                .into_iter()
                .map(|tag| TagApiModel {
                    key: tag.key,
                    value: tag.value,
                })
                .collect(),
            success: itm.success,
            fail: itm.fail,
        })
        .collect();

    Ok(result)
}
