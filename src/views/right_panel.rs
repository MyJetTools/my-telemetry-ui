use dioxus::prelude::*;

use crate::{states::*, views::*};

pub fn right_panel(cx: Scope) -> Element {
    let main_state = use_shared_state::<MainState>(cx).unwrap();

    match main_state.read().get_right_panel() {
        Some(state) => match state {
            RightPanelState::ShowServiceOverview(_) => {
                render!(services_overview {})
            }
            RightPanelState::ShowServiceDataOverview(service_id, data) => {
                render! { service_data_overview { service_id: service_id.clone(), data: data.clone() } }
            }
            RightPanelState::ShowProcess(service_id, data, process_id) => {
                render! {process_overview { service_id: service_id.clone(), data: data.clone(), process_id: *process_id }}
            }
        },
        None => {
            render!("Nothing selected")
        }
    }
}
