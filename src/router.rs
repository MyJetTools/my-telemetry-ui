//use dioxus_router_macro::Routable;
use crate::{Actions, Home, LastEvents, Process};
use dioxus::prelude::*;

use serde::{Deserialize, Serialize};

#[derive(Routable, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum AppRoute {
    #[route("/")]
    Home,
    #[route("/actions/:service")]
    Actions { service: String },

    #[route("/last/:service/:action")]
    LastEvents { service: String, action: String },

    #[route("/process/:service/:action/:id")]
    Process {
        service: String,
        action: String,
        id: i64,
    },

    #[route("/:..segments")]
    NotFound { segments: Vec<String> },
}

#[component]
fn NotFound(segments: Vec<String>) -> Element {
    rsx! { "404: Not Found" }
}
