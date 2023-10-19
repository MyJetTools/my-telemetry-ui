use std::{collections::BTreeMap, rc::Rc, sync::Arc};

use crate::reader_grpc::ServiceGrpcModel;

use super::{DialogState, RightPanelState};

pub struct MainState {
    pub services: Option<Arc<BTreeMap<Rc<String>, ServiceGrpcModel>>>,
    pub selected: Option<Rc<String>>,
    pub right_panel_state: Option<RightPanelState>,
    pub dialog: Option<DialogState>,
}
impl MainState {
    pub fn new() -> Self {
        Self {
            services: None,
            selected: None,
            right_panel_state: None,
            dialog: None,
        }
    }

    pub fn set_selected(&mut self, selected: Rc<String>) {
        self.right_panel_state = Some(RightPanelState::ShowServiceOverview(selected.clone()));
        self.selected = selected.into();
    }

    pub fn set_selected_data(&mut self, service_id: Rc<String>, data: Rc<String>) {
        self.right_panel_state = Some(RightPanelState::ShowServiceDataOverview(service_id, data));
    }

    pub fn set_show_process(&mut self, service_id: Rc<String>, data: Rc<String>, process_id: i64) {
        self.right_panel_state = Some(RightPanelState::ShowProcess(service_id, data, process_id));
    }

    pub fn get_right_panel(&self) -> Option<&RightPanelState> {
        self.right_panel_state.as_ref()
    }

    pub fn get_dialog_state(&self) -> Option<&DialogState> {
        self.dialog.as_ref()
    }

    pub fn show_dialog(&mut self, dialog_state: DialogState) {
        self.dialog = Some(dialog_state);
    }

    pub fn hide_dialog(&mut self) {
        self.dialog = None;
    }
}
