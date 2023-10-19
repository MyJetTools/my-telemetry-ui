use std::rc::Rc;

use dioxus::prelude::*;

#[inline_props]
pub fn show_key_value(cx: Scope, value: Rc<String>) -> Element {
    render! {
        div { value.as_str() }
    }
}
