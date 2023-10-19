use dioxus::prelude::*;

use crate::states::{DialogState, MainState};
use super::show_key_value;

pub fn render_dialog(cx: Scope) -> Element {
    let main_state = use_shared_state::<MainState>(cx).unwrap();

    let dialog = main_state.read();

    let dialog = match dialog.get_dialog_state() {
        None => None,
        Some(dialog_state)=>  {

            let  dialog_class = "modal-dialog";
            //dialog_class = "modal-dialog-narrow";
  
            let (dialog_content, header) = match dialog_state {
                DialogState::ShowKeyValue { the_key, value } => {
                    let result = rsx! { show_key_value { value: value.clone() } };
                    (result, format!("Show Value for Key {}", the_key.as_str()))
                }
            };
     
            rsx! {
                div { id: "dialog-pad",

                    div { class: "{dialog_class}",
                        div { class: "modal-content",
                            div { class: "modal-header",
                                h5 { class: "modal-title", "{header}" }
                                button {
                                    r#type: "button",
                                    class: "btn btn-default btn-sm",
                                    onclick: move |_| {
                                        use_shared_state::<MainState>(cx).unwrap().write().hide_dialog();
                                    },
                                    "X"
                                }
                            }
                            dialog_content
                        }
                    }
                }
            }
        }
        .into(),
    };

    render!(dialog)
}
