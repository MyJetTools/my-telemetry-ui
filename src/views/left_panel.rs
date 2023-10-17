use std::{collections::BTreeMap, rc::Rc, sync::Arc};

use dioxus::prelude::*;

use crate::{reader_grpc::ServiceGrpcModel, states::MainState};

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

    let left_panel = left_panel_state.read();

    let left_panel_owned = left_panel_state.to_owned();
    let left_panel_owned = Rc::new(left_panel_owned);

    let mut elements = Vec::new();

    match left_panel.services.as_ref() {
        Some(services) => {
            let max_duration = get_max_duration(services.values());
            for (service_id, service) in services.as_ref() {
                let duration = format!("{}/{:?}", service.amount, service.get_avg_duration());

                let duration_line = (service.avg as f64 / max_duration) * 100.0;

                let duration_line = rsx! {
                    div { style: "width:100%", div { style: "width:{duration_line}%; height: 2px; background-color:blue" } }
                };

                if let Some(selected) = left_panel.selected.as_ref() {
                    if selected.as_ref() == &service.id {
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

                let left_panel_owned = left_panel_owned.clone();

                let service_id = service_id.clone();
                elements.push(rsx! {
                    button {
                        r#type: "button",
                        class: "btn btn-light btn-sm",
                        style: "width: 100%; text-align: left;",
                        onclick: move |_| {
                            left_panel_owned.write().set_selected(service_id.clone());
                        },
                        "{service.id} "
                        span { class: "badge text-bg-secondary", duration }
                        duration_line
                    }
                });
            }
        }
        None => {
            elements.push(rsx! { h4 { "Loading..." } });
            load_services(&cx, &left_panel_state);
        }
    }

    render!(elements.into_iter())
}

fn load_services<'s>(
    cx: &Scope<'s, LeftPanelContentProps>,
    left_panel_state: &UseSharedState<MainState>,
) {
    let left_panel_state = left_panel_state.to_owned();
    cx.spawn(async move {
        let response = crate::api_client::get_list_of_services().await.unwrap();

        let mut services = BTreeMap::new();

        for service in response {
            services.insert(Rc::new(service.id.clone()), service);
        }

        let mut left_panel = left_panel_state.write();

        left_panel.services = Some(Arc::new(services));
    });
}

fn get_max_duration<'s>(services: impl Iterator<Item = &'s ServiceGrpcModel>) -> f64 {
    let mut result = 0;

    for srv in services {
        if srv.avg > result {
            result = srv.avg;
        }
    }

    result as f64
}
