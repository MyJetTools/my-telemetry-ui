#[cfg(feature = "server")]
use app_ctx::AppCtx;
use dioxus::prelude::*;

#[cfg(feature = "server")]
mod api_client;
#[cfg(feature = "server")]
mod app_ctx;
#[cfg(feature = "server")]
mod grpc_client;

#[cfg(feature = "server")]
mod settings;
//mod http_server;

mod router;
mod states;
mod utils;
mod views;

use views::*;

use crate::states::*;

use crate::router::*;
use crate::utils::from_base_64;

#[cfg(feature = "server")]
pub mod reader_grpc {
    tonic::include_proto!("reader");
}

#[cfg(feature = "server")]
lazy_static::lazy_static! {
    pub static ref APP_CTX: AppCtx = {
        AppCtx::new(settings::SettingsModel.into())
    };
}

fn main() {
    let cfg = dioxus::fullstack::Config::new();

    #[cfg(feature = "server")]
    let cfg = cfg.addr(([0, 0, 0, 0], 9001));
    //let config = LaunchBuilder::new(app);
    /*
       #[cfg(feature = "server")]
       config
           .incremental(
               IncrementalRendererConfig::default()
                   .invalidate_after(std::time::Duration::from_secs(120)),
           )
           .launch();
    */
    //#[cfg(not(feature = "server"))]
    LaunchBuilder::fullstack().with_cfg(cfg).launch(|| {
        rsx! {
            Router::<AppRoute> {}
        }
    })
}
#[component]
fn Home() -> Element {
    use_context_provider(|| Signal::new(MainState::new()));
    rsx! {
        MyLayout {}
    }
}

#[component]
fn Actions(service: String) -> Element {
    println!("Actions: {}", service);
    use_context_provider(|| Signal::new(MainState::new_with_selected_service(service)));
    rsx! {
        MyLayout {}
    }
}

#[component]
fn LastEvents(service: String, action: String) -> Element {
    println!("LastEvents: {}/{}", service, action);
    use_context_provider(|| {
        Signal::new(MainState::new_with_selected_action(
            service,
            from_base_64(action.as_str()),
        ))
    });
    rsx! {
        MyLayout {}
    }
}

#[component]
fn Process(service: String, action: String, id: i64) -> Element {
    println!("LastEvents: {}/{}/{}", service, action, id);
    use_context_provider(|| {
        Signal::new(MainState::new_with_selected_process(
            service,
            from_base_64(action.as_str()),
            id,
        ))
    });
    rsx! {
        MyLayout {}
    }
}

#[component]
pub fn MyLayout() -> Element {
    rsx! {
        div { id: "layout",
            div { id: "left-panel", LeftPanel {} }
            div { id: "right-panel", RightPanel {} }
            dialog::RenderDialog {}
        }
    }
}
