use std::rc::Rc;

#[derive(Clone, Debug)]
pub enum RightPanelState {
    ShowServiceOverview,
    ShowServiceDataOverview(Rc<String>, Rc<String>),
    ShowProcess(Rc<String>, Rc<String>, i64),
}
