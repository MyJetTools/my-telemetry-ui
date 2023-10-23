#![allow(non_snake_case)]

#[cfg(feature = "ssr")]
use app_ctx::AppCtx;
use dioxus::prelude::*;
use dioxus_fullstack::prelude::*;

#[cfg(feature = "ssr")]
mod api_client;
#[cfg(feature = "ssr")]
mod app_ctx;
#[cfg(feature = "ssr")]
mod grpc_client;

#[cfg(feature = "ssr")]
mod settings;
//mod http_server;

mod router;
mod states;
mod utils;
mod views;

use dioxus_router::prelude::Router;
use views::*;

use crate::states::*;

use crate::router::*;
use crate::utils::from_base_64;

#[cfg(feature = "ssr")]
pub mod reader_grpc {
    tonic::include_proto!("reader");
}

#[cfg(feature = "ssr")]
lazy_static::lazy_static! {
    pub static ref APP_CTX: AppCtx = {
        AppCtx::new(settings::SettingsModel.into())
    };
}

fn main() {
    let config = LaunchBuilder::<FullstackRouterConfig<AppRoute>>::router();

    //let config = LaunchBuilder::new(app);
    /*
       #[cfg(feature = "ssr")]
       config
           .incremental(
               IncrementalRendererConfig::default()
                   .invalidate_after(std::time::Duration::from_secs(120)),
           )
           .launch();
    */
    //#[cfg(not(feature = "ssr"))]
    config.launch();
}

fn Home(cx: Scope) -> Element {
    println!("Home page");
    render! { my_layout {} }
}

#[inline_props]
fn Actions(cx: Scope, service: String) -> Element {
    println!("Actions: {}", service);
    render! { my_layout {} }
}

#[inline_props]
fn LastEvents(cx: Scope, service: String, action: String) -> Element {
    println!("LastEvents: {}/{}", service, action);
    render! { my_layout {} }
}

#[inline_props]
fn Process(cx: Scope, service: String, action: String, id: i64) -> Element {
    println!("LastEvents: {}/{}", service, action);
    render! { my_layout {} }
}

pub fn my_layout(cx: Scope) -> Element {
    let route: AppRoute = dioxus_router::hooks::use_route(&cx).unwrap();

    println!("Route: {:?}", route);

    match route {
        AppRoute::Home => {
            println!("Creating provider for Home");
            use_shared_state_provider(cx, || MainState::new());
        }
        AppRoute::Actions { service } => {
            println!("Creating provider for Actions");
            use_shared_state_provider(cx, || MainState::new_with_selected_service(service))
        }

        AppRoute::LastEvents { service, action } => {
            println!("Creating provider for Actions");
            use_shared_state_provider(cx, || {
                MainState::new_with_selected_action(service, from_base_64(action.as_str()))
            })
        }
        AppRoute::Process {
            service,
            action,
            id,
        } => use_shared_state_provider(cx, || {
            MainState::new_with_selected_process(service, from_base_64(action.as_str()), id)
        }),
    }

    render! {
        div { id: "layout",
            div { id: "left-panel", left_panel {} }
            div { id: "right-panel", right_panel {} }
            dialog::render_dialog {}
        }
    }
}
