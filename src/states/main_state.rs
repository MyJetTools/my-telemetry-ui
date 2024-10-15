use std::{collections::BTreeMap, rc::Rc};

use crate::models::*;
use crate::ServiceOverviewApiModel;

use super::{AvailableFiles, DataState, DialogState, Envs, RightPanelState};

pub struct MainState {
    pub envs: Envs,
    pub left_panel: DataState<Rc<BTreeMap<Rc<String>, ServiceOverviewApiModel>>>,
    pub selected_service: Option<Rc<String>>,
    pub right_panel_state: Option<RightPanelState>,
    pub server_data_overview: DataState<Rc<Vec<ServiceDataApiModel>>>,
    pub dialog: Option<DialogState>,
    pub files: AvailableFiles,
    pub client_id: String,
    pub from_time: TimeModel,
}
impl MainState {
    pub fn new() -> Self {
        Self {
            right_panel_state: None,
            dialog: None,
            envs: Envs::new(),
            left_panel: DataState::None,
            server_data_overview: DataState::None,
            selected_service: None,
            files: AvailableFiles::new(),
            client_id: crate::storage::client_id::get(),
            from_time: crate::storage::from_time::get(),
        }
    }

    pub fn new_with_selected_service(service_name: String) -> Self {
        Self {
            right_panel_state: Some(RightPanelState::ShowServiceOverview),
            dialog: None,
            envs: Envs::new(),
            left_panel: DataState::None,
            selected_service: Some(Rc::new(service_name)),
            files: AvailableFiles::new(),
            client_id: crate::storage::client_id::get(),
            server_data_overview: DataState::None,
            from_time: crate::storage::from_time::get(),
        }
    }

    pub fn new_with_selected_action(service_name: String, action: String) -> Self {
        Self {
            right_panel_state: Some(RightPanelState::ShowServiceDataOverview(Rc::new(action))),
            dialog: None,
            envs: Envs::new(),
            left_panel: DataState::None,
            selected_service: Rc::new(service_name).into(),
            files: AvailableFiles::new(),
            client_id: crate::storage::client_id::get(),
            server_data_overview: DataState::None,
            from_time: crate::storage::from_time::get(),
        }
    }

    pub fn new_with_selected_process(
        service_name: String,
        action: String,
        process_id: i64,
    ) -> Self {
        Self {
            right_panel_state: Some(RightPanelState::ShowProcess(Rc::new(action), process_id)),
            dialog: None,
            envs: Envs::new(),
            left_panel: DataState::None,
            selected_service: Some(Rc::new(service_name)),
            files: AvailableFiles::new(),
            client_id: crate::storage::client_id::get(),
            server_data_overview: DataState::None,
            from_time: crate::storage::from_time::get(),
        }
    }

    pub fn set_selected(&mut self, selected: Rc<String>) {
        println!("Selected: {}", selected);
        self.selected_service = Some(selected.clone());
        self.right_panel_state = Some(RightPanelState::ShowServiceOverview);
    }

    pub fn get_selected_service(&self) -> Option<Rc<String>> {
        self.selected_service.clone()
    }

    pub fn set_selected_data(&mut self, service_id: Rc<String>, data: Rc<String>) {
        self.right_panel_state = Some(RightPanelState::ShowServiceDataOverview(data));
        self.selected_service = Some(service_id);
    }

    pub fn set_show_process(&mut self, service_id: Rc<String>, data: Rc<String>, process_id: i64) {
        self.right_panel_state = Some(RightPanelState::ShowProcess(data, process_id));
        self.selected_service = Some(service_id);
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

    pub fn env_updated(&mut self) {
        self.left_panel = DataState::None;
        self.right_panel_state = None;
        self.dialog = None;
        self.selected_service = None;
        self.files.reset();
    }

    pub fn set_hours_ago(&mut self, hours_ago: i64) {
        crate::storage::hours_ago::set(hours_ago);
        self.right_panel_state = None;
        self.dialog = None;
        self.selected_service = None;
        self.left_panel = DataState::None;
    }

    pub fn try_get_hours_ago(&self) -> Option<i64> {
        let result = crate::storage::hours_ago::get();

        self.files.get_available_hours_ago(result)
    }

    pub fn get_hours_ago(&self) -> i64 {
        let result = crate::storage::hours_ago::get();

        self.files
            .get_available_hours_ago(result)
            .unwrap_or_default()
    }

    pub fn apply_client_id(&mut self) {
        crate::storage::client_id::set(&self.client_id);
        crate::storage::from_time::set(&self.from_time);
        self.server_data_overview = DataState::None;
    }

    pub fn reset_client_id(&mut self) {
        self.client_id = "".to_string();
        self.from_time = TimeModel::default();
        crate::storage::client_id::set(&self.client_id);
        crate::storage::from_time::set(&self.from_time);
        self.server_data_overview = DataState::None;
    }
}
