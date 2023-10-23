use std::{rc::Rc, time::Duration};

use dioxus::prelude::*;
use dioxus_fullstack::prelude::*;
use dioxus_router::prelude::*;

use crate::{states::{DialogState, MainState}, router::AppRoute};

pub struct ServicesOverviewState {
    pub services: Option<Vec<ServiceApiModel>>,
    pub service_id: Option<String>,
    pub service_id_in_refresh: bool
}

impl ServicesOverviewState {
    pub fn new() -> Self {
        Self {
            services: None,
            service_id: None,
            service_id_in_refresh: false,
        }
    }

    pub fn new_as_service_id_in_refresh(service_id: Option<String>) -> Self {
        Self {
            services: None,
            service_id,
            service_id_in_refresh: true,
        }
    }

    pub fn has_to_be_refreshed(&self, service_id:&str)->bool{
        match self.service_id.as_ref() {
            Some(id) => id != service_id,
            None => true,
        }
    }

}

pub fn services_overview<'s>(cx: Scope) -> Element {
    let main_state = use_shared_state::<MainState>(cx).unwrap();
    let widget_state = use_state(cx, || ServicesOverviewState::new());


    let widget_state_owned: UseState<ServicesOverviewState> = widget_state.to_owned();
    let service_id = main_state.read().get_selected().unwrap();

    let service_id_owned = service_id.clone();


    let future = use_future(cx, (), |_| async move {
        if widget_state_owned.has_to_be_refreshed(service_id_owned.as_str()) {

        let result = load_services(service_id_owned.as_str().to_string()).await.unwrap();
        widget_state_owned.set(ServicesOverviewState{
            services: Some(result),
            service_id: Some(service_id_owned.as_str().to_string()),
            service_id_in_refresh: false,
        });
        }
   
    });


    if !widget_state.get().service_id_in_refresh{
        if widget_state.get().has_to_be_refreshed(service_id.as_str()) {
            let service_id = widget_state.service_id.clone();
            widget_state.set(ServicesOverviewState::new_as_service_id_in_refresh(service_id));
            future.restart();
        } 
    }




    let mut result = Vec::new();
    match widget_state.get().services.as_ref() {
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
                      let service_id = service_id.clone();
                      let service_id_2 = service_id.clone();

                      let action_base_64 = crate::utils::to_base_64(service.data.as_str());
                      let min = format!("{:?}", service.get_min_duration());
                      let max = format!("{:?}", service.get_max_duration()) ;
                      let avg = format!("{:?}", service.get_avg_duration()) ;

                      let bar_max = (service.max as f64 / max_duration) * 100.0;
                      let bar_min = (service.min as f64 / max_duration) * 100.0;
                      let bar_avg = (service.avg as f64 / max_duration) * 100.0;

                      let service_data = Rc::new(service.data.clone());
                      let service_data_expand = service_data.clone();
                      let service_data_to_show = if service_data.as_str().len()>64{
                 
                        rsx! {
                            span {
                                button {
                                    class: "btn btn-sm btn-light",
                                 
                                    onclick: move |_| {
                                        use_shared_state::<MainState>(cx)
                                            .unwrap()
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
                      }else{
                        rsx!{ span{"{service_data_expand.as_str()}"}}
                      };

                       rsx! { tr { class:"table-line",
                            td{
                                service_data_to_show,
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
                                button{class:"btn btn-sm btn-primary", style:"padding: 2px 5px;",
                                Link {
                                    onclick: move |_| {
                                        let right_panel_state =  use_shared_state::<MainState>(cx).unwrap();
                                        right_panel_state.write().set_selected_data(service_id.clone(), service_data.clone());
    
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
pub struct ServiceApiModel{
    pub data: String,
    pub max: i64,
    pub min: i64,
    pub avg: i64,
    pub success: i64,
    pub error: i64,
    pub total: i64,
}


impl ServiceApiModel{
    pub fn get_min_duration(&self)->Duration{
        Duration::from_millis(self.min as u64)
    }

    pub fn get_max_duration(&self)->Duration{
        Duration::from_millis(self.max as u64)
    }

    pub fn get_avg_duration(&self)->Duration{
        Duration::from_millis(self.avg as u64)
    }
}

#[server]
async fn load_services(
    service_id: String,
)-> Result<Vec<ServiceApiModel>, ServerFnError>
 {

    let response = crate::api_client::get_services_overview(service_id.clone())
    .await
    .unwrap();

    let result: Vec<ServiceApiModel> = response.into_iter().map(|src|ServiceApiModel{ data: src.data, max: src.max, min: src.min, avg: src.avg, success: src.success, error: src.error, total: src.total }).collect();

    Ok(result)

}
