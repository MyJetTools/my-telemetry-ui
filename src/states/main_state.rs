use std::rc::Rc;

use super::{DialogState, RightPanelState};

pub struct MainState {
    pub right_panel_state: Option<RightPanelState>,
    pub dialog: Option<DialogState>,
}
impl MainState {
    pub fn new() -> Self {
        Self {
            right_panel_state: None,
            dialog: None,
        }
    }

    pub fn new_with_selected_service(service_name: String) -> Self {
        Self {
            right_panel_state: Some(RightPanelState::ShowServiceOverview(Rc::new(service_name))),
            dialog: None,
        }
    }

    pub fn new_with_selected_action(service_name: String, action: String) -> Self {
        Self {
            right_panel_state: Some(RightPanelState::ShowServiceDataOverview(
                Rc::new(service_name),
                Rc::new(action),
            )),
            dialog: None,
        }
    }

    pub fn new_with_selected_process(
        service_name: String,
        action: String,
        process_id: i64,
    ) -> Self {
        Self {
            right_panel_state: Some(RightPanelState::ShowProcess(
                Rc::new(service_name),
                Rc::new(action),
                process_id,
            )),
            dialog: None,
        }
    }

    pub fn set_selected(&mut self, selected: Rc<String>) {
        println!("Selected: {}", selected);
        self.right_panel_state = Some(RightPanelState::ShowServiceOverview(selected));
    }

    pub fn get_selected(&self) -> Option<Rc<String>> {
        match self.right_panel_state.as_ref()? {
            RightPanelState::ShowServiceOverview(id) => Some(id.clone()),
            RightPanelState::ShowServiceDataOverview(id, _) => Some(id.clone()),
            RightPanelState::ShowProcess(id, _, _) => Some(id.clone()),
        }
    }

    pub fn set_selected_data(&mut self, service_id: Rc<String>, data: Rc<String>) {
        self.right_panel_state = Some(RightPanelState::ShowServiceDataOverview(service_id, data));
    }

    pub fn set_show_process(&mut self, service_id: Rc<String>, data: Rc<String>, process_id: i64) {
        self.right_panel_state = Some(RightPanelState::ShowProcess(service_id, data, process_id));
    }

    pub fn get_right_panel(&self) -> Option<RightPanelState> {
        self.right_panel_state.clone()
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
