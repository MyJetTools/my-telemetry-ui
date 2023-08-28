use std::rc::Rc;

pub enum RightPanelState {
    ShowServiceOverview(Rc<String>),
}
