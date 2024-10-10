pub mod api_client;
mod app_ctx;
mod grpc_client;
mod settings;

pub mod reader_grpc {
    tonic::include_proto!("reader");
}

#[cfg(feature = "server")]
lazy_static::lazy_static! {
    pub static ref APP_CTX: crate::server::app_ctx::AppCtx = {
        crate::server::app_ctx::AppCtx::new()
    };
}
