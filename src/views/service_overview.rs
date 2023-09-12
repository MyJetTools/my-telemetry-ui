use std::rc::Rc;

use dioxus::prelude::*;

use crate::{api_client::ServiceOverview, states::MainState};

pub struct ServicesOverviewState {
    pub services: Option<Vec<ServiceOverview>>,
    pub service_id: Option<String>,
}

impl ServicesOverviewState {
    pub fn new() -> Self {
        Self {
            services: None,
            service_id: None,
        }
    }
}

#[derive(Props, PartialEq, Eq)]
pub struct ServicesOverviewProps {
    pub service_id: Rc<String>,
}

pub fn services_overview<'s>(cx: Scope<'s, ServicesOverviewProps>) -> Element {
    let services = use_state(cx, || ServicesOverviewState::new());

    match services.service_id.as_ref() {
        Some(service_id) => {
            if service_id != cx.props.service_id.as_ref() {
                load_services(&cx, &cx.props.service_id.as_ref(), &services);
            }
        }
        None => {
            load_services(&cx, &cx.props.service_id.as_ref(), &services);
        }
    }

    let mut result = Vec::new();
    match services.get().services.as_ref() {
        Some(services) => {
            let max_duration = get_max(services);

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

                    services.iter().map(|service|{
                      let min = format!("{:?}", service.get_min_duration());
                      let max = format!("{:?}", service.get_max_duration()) ;
                      let avg = format!("{:?}", service.get_avg_duration()) ;

                      let bar_max = (service.max as f64 / max_duration) * 100.0;
                      let bar_min = (service.min as f64 / max_duration) * 100.0;
                      let bar_avg = (service.avg as f64 / max_duration) * 100.0;

                      let service_data = Rc::new(service.data.clone());

                       rsx! { tr { class:"table-line",
                            td{
                                "{service.data}"
                                div{ style: "width:100%; padding:0",
                                    div{ style:"width: {bar_max}%; height: 2px; background-color:green"}
                                    div{ style:"width: {bar_avg}%; height: 2px; background-color:orange"}
                                    div{ style:"width: {bar_min}%; height: 2px; background-color:red"}

                                }
                            }
                            td{
                                min
                            }
                            td{
                                avg
                            }
                            td{
                                max
                            }
                            td{
                                "{service.success}"
                            }
                            td{
                                "{service.error}"
                            }
                            td{
                                "{service.total}"
                            }
                            td{
                                button{class:"btn btn-sm btn-primary", style:"padding: 2px 5px;", onclick: move |_|{
                                    let right_panel_state =  use_shared_state::<MainState>(cx).unwrap();
                                    right_panel_state.write().set_selected_data(cx.props.service_id.clone(), service_data.clone());

                                }, "Expand"}
                            }
                          }
                        }
                    })
                }
            });
        }
        None => {
            result.push(rsx! { div { "Loading..." } });
        }
    }

    render!(result.into_iter())
}

fn get_max(services: &[ServiceOverview]) -> f64 {
    let mut result = 0;

    for srv in services {
        if srv.max > result {
            result = srv.max;
        }
    }

    result as f64
}

fn load_services<'s>(
    cx: &Scope<'s, ServicesOverviewProps>,
    service_id: &str,
    state: &UseState<ServicesOverviewState>,
) {
    let state = state.to_owned();

    let service_id = service_id.to_string();
    cx.spawn(async move {
        let response = crate::api_client::get_services_overview(service_id.clone())
            .await
            .unwrap();

        state.set(ServicesOverviewState {
            services: Some(response),
            service_id: Some(service_id),
        });
    });
}
