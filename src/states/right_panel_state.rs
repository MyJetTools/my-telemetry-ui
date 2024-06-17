use std::rc::Rc;

#[derive(Clone, Debug)]
pub enum RightPanelState {
    ShowServiceOverview(Rc<String>),
    ShowServiceDataOverview(Rc<String>, Rc<String>),
    ShowProcess(Rc<String>, Rc<String>, i64),
}
