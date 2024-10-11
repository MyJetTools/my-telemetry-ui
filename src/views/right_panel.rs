use dioxus::prelude::*;

use crate::{states::*, views::*};

#[component]
pub fn RightPanel() -> Element {
    let main_state = consume_context::<Signal<MainState>>();

    let right_panel_state = main_state.read().get_right_panel();
    match right_panel_state {
        Some(state) => match state {
            RightPanelState::ShowServiceOverview => ServicesOverview(),
            RightPanelState::ShowServiceDataOverview(service_id, data) => {
                rsx! {
                    ServiceDataOverview { service_id: service_id.clone(), data: data.clone() }
                }
            }
            RightPanelState::ShowProcess(service_id, data, process_id) => {
                rsx! {
                    ProcessOverview { service_id, data, process_id }
                }
            }
        },
        None => {
            rsx!("Nothing selected")
        }
    }
}
