//use dioxus_router_macro::Routable;
use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

use crate::{Actions, Home, LastEvents, Process};

#[derive(dioxus_router_macro::Routable, Clone, Debug, PartialEq, Serialize, Deserialize)]
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
}
