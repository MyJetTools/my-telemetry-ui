use std::rc::Rc;

pub enum RightPanelState {
    ShowServiceOverview(Rc<String>),
    ShowServiceDataOverview(Rc<String>, Rc<String>),
    ShowProcess(Rc<String>, Rc<String>, i64),
}
