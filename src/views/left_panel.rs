use std::{collections::BTreeMap, rc::Rc, time::Duration};

use dioxus::prelude::*;

use crate::{states::MainState, AppRoute, DataState};

use crate::components::*;

#[component]
pub fn LeftPanel() -> Element {
    let mut filter = use_signal(|| "".to_string());

    rsx! {
        div {
            EnvsSelector {}
            input {
                id: "search-input",
                class: "form-control",
                placeholder: "Search",
                oninput: move |cx| {
                    let new_value = cx.value().trim().to_string();
                    filter.set(new_value);
                }
            }
        }

        div { id: "left-panel-content",
            LeftPanelContent { filter: filter.read().clone() }
        }
    }
}

#[component]
fn LeftPanelContent(filter: String) -> Element {
    let mut main_state = consume_context::<Signal<MainState>>();

    let main_state_read_model = main_state.read();

    let hours_ago = main_state_read_model.get_hours_ago();

    let services = match main_state_read_model.left_panel.as_ref() {
        DataState::None => {
            spawn(async move {
                {
                    main_state.write().left_panel = DataState::Loading;
                }

                let env = crate::storage::selected_env::get();
                let response = crate::load_service_overview(env, hours_ago).await;
                let response = match response {
                    Ok(response) => response,
                    Err(err) => {
                        main_state.write().left_panel = DataState::Error(err.to_string());
                        return;
                    }
                };

                let mut services = BTreeMap::new();

                for service in response {
                    services.insert(Rc::new(service.id.clone()), service);
                }
                main_state.write().left_panel = DataState::Loaded(Rc::new(services));
            });

            return rsx! {
                {"Loading..."}
            };
        }

        DataState::Loading => {
            return rsx! {
                {"Loading..."}
            }
        }

        DataState::Loaded(data) => data,
        DataState::Error(err) => {
            return rsx! {
                div { style: "color:red", {err.as_str()} }
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

        let badge_type = if service.amount < 50000 {
            "text-bg-light"
        } else if service.amount < 100000 {
            "text-bg-info"
        } else if service.amount < 500000 {
            "text-bg-warning"
        } else {
            "text-bg-danger"
        };

        let duration_line = (service.avg as f64 / max_duration) * 100.0;

        let duration_line = rsx! {
            div { style: "width:100%",
                div { style: "width:{duration_line}%; height: 2px; background-color:blue" }
            }
        };

        if let Some(selected) = main_state_read_model.selected_service.as_ref() {
            if selected.as_str() == service.id.as_str() {
                elements.push(rsx! {
                    button {
                        r#type: "button",
                        class: "btn btn-primary btn-sm",
                        style: "width: 100%; text-align: left;",
                        table { style: "width:100%",
                            tr {
                                td { "{service.id}" }
                                td { style: "text-align:right",
                                    span { class: "badge {badge_type}", {duration} }
                                }
                            }
                        }
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

            Link {
                class: "btn btn-light btn-sm",
                style: "width: 100%; text-align: left;",
                onclick: move |_| {
                    let mut write_access = main_state.write();
                    write_access.set_selected(service_id_cloned.clone());
                },
                to: AppRoute::Actions {
                    service: service.id.clone(),
                },

                table { style: "width:100%",
                    tr {
                        td { {service.id.as_str()} }
                        td { style: "text-align:right",
                            span { class: "badge {badge_type}", {duration} }
                        }
                    }
                }

                {duration_line}
            }
        });
    }

    rsx! {
        {elements.into_iter()}
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct ServiceOverviewApiModel {
    pub id: String,
    pub amount: i64,
    pub avg: i64,
}

impl ServiceOverviewApiModel {
    pub fn get_avg_duration(&self) -> Duration {
        Duration::from_micros(self.avg as u64)
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
pub async fn load_service_overview(
    env: String,
    hours_ago: i64,
) -> Result<Vec<ServiceOverviewApiModel>, ServerFnError> {
    let response = crate::server::api_client::get_list_of_services(env.as_str(), hours_ago)
        .await
        .unwrap();

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
