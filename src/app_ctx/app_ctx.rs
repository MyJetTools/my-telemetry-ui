use std::sync::Arc;

use crate::{grpc_client::TelemetryReaderGrpcClient, settings::SettingsModel};

//pub const APP_VERSION: &'static str = env!("CARGO_PKG_VERSION");
//pub const APP_NAME: &'static str = env!("CARGO_PKG_NAME");
pub struct AppCtx {
    pub grpc_client: Arc<TelemetryReaderGrpcClient>,
}

impl AppCtx {
    pub fn new(settings_model: Arc<SettingsModel>) -> Self {
        Self {
            grpc_client: Arc::new(TelemetryReaderGrpcClient::new(settings_model)),
        }
    }
}
