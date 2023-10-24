use std::{collections::BTreeMap, rc::Rc, sync::Arc, time::Duration};

use dioxus::prelude::*;
use dioxus_router::prelude::*;

use crate::{states::MainState, AppRoute};
use dioxus_fullstack::prelude::*;

pub fn left_panel(cx: Scope) -> Element {
    let filter = use_state(cx, || "".to_string());

    render! {
        input {
            id: "search-input",

            class: "form-control",

            placeholder: "Search",

            oninput: move |cx| {
                let new_value = cx.value.trim().to_string();
                filter.set(new_value);
            }
        }
        div { id: "left-panel-content", left_panel_content { filter: filter.get().clone() } }
    }
}

#[derive(Props, PartialEq, Eq)]
pub struct LeftPanelContentProps {
    pub filter: String,
}

fn left_panel_content<'s>(cx: Scope<'s, LeftPanelContentProps>) -> Element {
    let left_panel_state = use_shared_state::<MainState>(cx).unwrap();

    let left_panel_state_owned = left_panel_state.to_owned();

    let _future = use_future(cx, (), |_| async move {
        let response = crate::load_service_overview().await.unwrap();

        let mut services = BTreeMap::new();

        for service in response {
            services.insert(Rc::new(service.id.clone()), service);
        }

        let mut left_panel = left_panel_state_owned.write();
        left_panel.services = Some(Arc::new(services));
    });

    let left_panel = left_panel_state.read();

    let mut elements = Vec::new();

    match left_panel.services.as_ref() {
        Some(services) => {
            let max_duration = get_max_duration(services.values());
            for (_, service) in services.as_ref() {
                let duration = format!(
                    "{}/{:?}",
                    format_amount(service.amount),
                    service.get_avg_duration()
                );

                let duration_line = (service.avg as f64 / max_duration) * 100.0;

                let duration_line = rsx! {
                    div { style: "width:100%", div { style: "width:{duration_line}%; height: 2px; background-color:blue" } }
                };

                if let Some(selected) = left_panel.get_selected() {
                    if selected.as_str() == service.id.as_str() {
                        elements.push(rsx! {
                            button {
                                r#type: "button",
                                class: "btn btn-primary btn-sm",
                                style: "width: 100%; text-align: left;",
                                "{service.id} "
                                span { class: "badge text-bg-secondary", duration }
                                duration_line
                            }
                        });
                        continue;
                    }
                }

                if cx.props.filter.len() > 0 && !service.id.contains(cx.props.filter.as_str()) {
                    continue;
                }

                let service_id_cloned = Rc::new(service.id.clone());

                elements.push(rsx! {
                    button {
                        r#type: "button",
                        class: "btn btn-light btn-sm",
                        style: "width: 100%; text-align: left;",

                        Link {
                            onclick: move |_| {
                                println!("Clicked on {}", service_id_cloned);
                                let left_panel_state = use_shared_state::<MainState>(cx).unwrap();
                                left_panel_state.write().set_selected(service_id_cloned.clone());
                            },
                            to: AppRoute::Actions {
    service: service.id.clone(),
},
                            "{service.id} "
                        }
                        span { class: "badge text-bg-secondary", duration }
                        duration_line
                    }
                });
            }
        }
        None => {
            elements.push(rsx! { h4 { "Loading..." } });
        }
    }

    render!(elements.into_iter())
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct ServiceOverviewApiModel {
    pub id: String,
    pub amount: i64,
    pub avg: i64,
}

impl ServiceOverviewApiModel {
    pub fn get_avg_duration(&self) -> Duration {
        Duration::from_millis(self.avg as u64)
    }
}

fn get_max_duration<'s>(services: impl Iterator<Item = &'s ServiceOverviewApiModel>) -> f64 {
    let mut result = 0;

    for srv in services {
        if srv.avg > result {
            result = srv.avg;
        }
    }

    result as f64
}

fn format_amount(value: i64) -> String {
    if value < 1024 {
        return format!("{}", value);
    }

    let value = value / 1024;

    if value < 1024 {
        return format!("{}K", value);
    }

    let value = value / 1024;

    return format!("{}M", value);
}

#[server]
pub async fn load_service_overview() -> Result<Vec<ServiceOverviewApiModel>, ServerFnError> {
    let response = crate::api_client::get_list_of_services().await.unwrap();

    let result = response
        .into_iter()
        .map(|service| ServiceOverviewApiModel {
            id: service.id,
            amount: service.amount,
            avg: service.avg,
        })
        .collect();

    Ok(result)
}
