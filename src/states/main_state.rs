use std::{collections::BTreeMap, rc::Rc, sync::Arc};

use crate::api_client::ServiceModel;

use super::RightPanelState;

pub struct MainState {
    pub services: Option<Arc<BTreeMap<Rc<String>, ServiceModel>>>,
    pub selected: Option<Rc<String>>,
    pub right_panel_state: Option<RightPanelState>,
}
impl MainState {
    pub fn new() -> Self {
        Self {
            services: None,
            selected: None,
            right_panel_state: None,
        }
    }

    pub fn set_selected(&mut self, selected: Rc<String>) {
        self.right_panel_state = Some(RightPanelState::ShowServiceOverview(selected.clone()));
        self.selected = selected.into();
    }

    pub fn get_right_panel(&self) -> Option<&RightPanelState> {
        self.right_panel_state.as_ref()
    }

    pub fn get_selected_service_id(&self) -> Rc<String> {
        self.selected.as_ref().unwrap().clone()
    }
}
