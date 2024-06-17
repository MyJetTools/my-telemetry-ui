use std::{collections::BTreeMap, rc::Rc, time::Duration};

use dioxus::prelude::*;

use crate::{states::MainState, AppRoute};

#[component]
pub fn LeftPanel() -> Element {
    let mut filter = use_signal(|| "".to_string());

    rsx! {
        input {
            id: "search-input",

            class: "form-control",

            placeholder: "Search",

            oninput: move |cx| {
                let new_value = cx.value().trim().to_string();
                filter.set(new_value);
            }
        }
        div { id: "left-panel-content",
            LeftPanelContent { filter: filter.read().clone() }
        }
    }
}

#[component]
fn LeftPanelContent(filter: String) -> Element {
    let left_panel_state = consume_context::<Signal<MainState>>();

    let future = use_resource(|| async move {
        let response = crate::load_service_overview().await;
        let response = match response {
            Ok(response) => response,
            Err(err) => {
                return Err(err);
            }
        };

        let mut services = BTreeMap::new();

        for service in response {
            services.insert(Rc::new(service.id.clone()), service);
        }

        Ok(services)

        //let mut left_panel = left_panel_state_owned.write();
        //left_panel.services = Some(Arc::new(services));
    });

    let widget_data = future.read_unchecked();

    let services = match &*widget_data {
        Some(services) => match services {
            Ok(services) => services,
            Err(err) => {
                let err = format!("{}", err.to_string());
                return rsx! {
                    h4 { {err} }
                };
            }
        },
        None => {
            return rsx! {
                h4 { "Loading..." }
            };
        }
    };

    let mut elements = Vec::new();
    let max_duration = get_max_duration(services.values());
    for service in services.values() {
        let duration = format!(
            "{}/{:?}",
            format_amount(service.amount),
            service.get_avg_duration()
        );

        let duration_line = (service.avg as f64 / max_duration) * 100.0;

        let duration_line = rsx! {
            div { style: "width:100%",
                div { style: "width:{duration_line}%; height: 2px; background-color:blue" }
            }
        };

        if let Some(selected) = left_panel_state.read().get_selected() {
            if selected.as_str() == service.id.as_str() {
                elements.push(rsx! {
                    button {
                        r#type: "button",
                        class: "btn btn-primary btn-sm",
                        style: "width: 100%; text-align: left;",
                        "{service.id} "
                        span { class: "badge text-bg-secondary", {duration} }
                        {duration_line}
                    }
                });
                continue;
            }
        }

        if filter.len() > 0 && !service.id.contains(filter.as_str()) {
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
                        consume_context::<Signal<MainState>>()
                            .write()
                            .set_selected(service_id_cloned.clone());
                    },
                    to: AppRoute::Actions {
                        service: service.id.clone(),
                    },
                    "{service.id} "
                }
                span { class: "badge text-bg-secondary", {duration} }
                {duration_line}
            }
        });
    }

    rsx! {
        {elements.into_iter()}
    }

    /*
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
                    div { style: "width:100%",
                        div { style: "width:{duration_line}%; height: 2px; background-color:blue" }
                    }
                };

                if let Some(selected) = left_panel.get_selected() {
                    if selected.as_str() == service.id.as_str() {
                        elements.push(rsx! {
                            button {
                                r#type: "button",
                                class: "btn btn-primary btn-sm",
                                style: "width: 100%; text-align: left;",
                                "{service.id} "
                                span { class: "badge text-bg-secondary", {duration} }
                                {duration_line}
                            }
                        });
                        continue;
                    }
                }

                if filter.len() > 0 && !service.id.contains(filter.as_str()) {
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
                                consume_context::<Signal<MainState>>()
                                    .write()
                                    .set_selected(service_id_cloned.clone());
                            },
                            to: AppRoute::Actions {
                                service: service.id.clone(),
                            },
                            "{service.id} "
                        }
                        span { class: "badge text-bg-secondary", {duration} }
                        {duration_line}
                    }
                });
            }
        }
        None => {
            elements.push(rsx! {
                h4 { "Loading..." }
            });
        }
    }

    rsx! {
        {elements.into_iter()}
    }
     */
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
