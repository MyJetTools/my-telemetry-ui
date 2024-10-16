use std::{rc::Rc, time::Duration};

use dioxus::prelude::*;

use crate::{
    states::{DialogState, MainState},
    AppRoute,
};

#[component]
pub fn ServicesOverview() -> Element {
    let main_state = consume_context::<Signal<MainState>>();

    let main_resource = use_resource(use_reactive!(|(main_state,)| async move {
        let (env, hours_ago, service_id) = {
            let main_state_read_access = main_state.read();
            let service_id = main_state_read_access.get_selected_service().unwrap();
            let env = crate::storage::selected_env::get();
            let hours_ago = main_state_read_access.get_hours_ago();

            (env, hours_ago, service_id)
        };

        load_services(env, hours_ago, service_id.to_string())
            .await
            .unwrap()
    },));

    let widget_data = main_resource.read_unchecked();

    let services = match &*widget_data {
        Some(services) => services,
        None => {
            return rsx! {
                div { "Loading..." }
            };
        }
    };

    let mut result = Vec::new();

    let max_duration = get_max(services);

    let services_to_render = services.iter().map(|service| {
        let service_id = main_state.read().get_selected_service().unwrap();
        let service_id_2 = service_id.clone();

        let action_base_64 = crate::utils::to_base_64(service.data.as_str());
        let min = format!("{:?}", service.get_min_duration());
        let max = format!("{:?}", service.get_max_duration());
        let avg = format!("{:?}", service.get_avg_duration());

        let bar_max = (service.max as f64 / max_duration) * 100.0;
        let bar_min = (service.min as f64 / max_duration) * 100.0;
        let bar_avg = (service.avg as f64 / max_duration) * 100.0;

        let service_data = Rc::new(service.data.clone());
        let service_data_expand = service_data.clone();
        let service_data_to_show = if service_data.as_str().len() > 64 {
            rsx! {
                span {
                    button {
                        class: "btn btn-sm btn-light",
                        onclick: move |_| {
                            use_context::<Signal<MainState>>()
                                .write()
                                .show_dialog(DialogState::ShowKeyValue {
                                    the_key: Rc::new("Expanding service data".to_string()),
                                    value: service_data_expand.clone(),
                                });
                        },
                        "{&service_data_expand[..64]}..."
                    }
                }
            }
        } else {
            rsx! {
                span { "{service_data_expand.as_str()}" }
            }
        };

        rsx! {
            tr { class: "table-line",
                td {
                    {service_data_to_show},
                    div { style: "width:100%; padding:0",
                        div { style: "width: {bar_max}%; height: 2px; background-color:green" }
                        div { style: "width: {bar_avg}%; height: 2px; background-color:orange" }
                        div { style: "width: {bar_min}%; height: 2px; background-color:red" }
                    }
                }
                td { {min} }
                td { {avg} }
                td { {max} }
                td { "{service.success}" }
                td { "{service.error}" }
                td { "{service.total}" }
                td {
                    button {
                        class: "btn btn-sm btn-primary",
                        style: "padding: 2px 5px;",
                        Link {
                            onclick: move |_| {
                                consume_context::<Signal<MainState>>()
                                    .write()
                                    .set_selected_data(service_id.clone(), service_data.clone());
                            },
                            to: AppRoute::LastEvents {
                                service: service_id_2.to_string(),
                                action: action_base_64,
                            },
                            "Expand"
                        }
                    }
                }
            }
        }
    });

    result.push(rsx! {
        table { class: "table table-striped", style: "text-align: left;",
            tr {
                th { "Data" }
                th { "Min" }
                th { "Avg" }
                th { "Max" }
                th { "Success" }
                th { "Error" }
                th { "Total" }
                th {}
            }
            {services_to_render}
        }
    });

    rsx! {
        {result.into_iter()}
    }
}

fn get_max(services: &[ServiceApiModel]) -> f64 {
    let mut result = 0;

    for srv in services {
        if srv.max > result {
            result = srv.max;
        }
    }

    result as f64
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct ServiceApiModel {
    pub data: String,
    pub max: i64,
    pub min: i64,
    pub avg: i64,
    pub success: i64,
    pub error: i64,
    pub total: i64,
}

impl ServiceApiModel {
    pub fn get_min_duration(&self) -> Duration {
        Duration::from_micros(self.min as u64)
    }

    pub fn get_max_duration(&self) -> Duration {
        Duration::from_micros(self.max as u64)
    }

    pub fn get_avg_duration(&self) -> Duration {
        Duration::from_micros(self.avg as u64)
    }
}

#[server]
async fn load_services(
    env: String,
    hours_ago: i64,
    service_id: String,
) -> Result<Vec<ServiceApiModel>, ServerFnError> {
    let apps = crate::server::api_client::get_services_overview(
        env.as_str(),
        hours_ago,
        service_id.clone(),
    )
    .await;

    let services: Vec<ServiceApiModel> = apps
        .into_iter()
        .map(|src| ServiceApiModel {
            data: src.data,
            max: src.max,
            min: src.min,
            avg: src.avg,
            success: src.success,
            error: src.error,
            total: src.total,
        })
        .collect();

    Ok(services)
}
