use std::sync::Arc;

use tokio::sync::RwLock;

use crate::{grpc_client::TelemetryReaderGrpcClient, settings::SettingsReader};

pub struct AppCtxInner {
    settings_reader: Arc<SettingsReader>,
    telemetry_reader_grpc_client: Arc<TelemetryReaderGrpcClient>,
}

//pub const APP_VERSION: &'static str = env!("CARGO_PKG_VERSION");
//pub const APP_NAME: &'static str = env!("CARGO_PKG_NAME");
pub struct AppCtx {
    inner: RwLock<Option<Arc<AppCtxInner>>>,
}

impl AppCtx {
    pub fn new() -> Self {
        Self {
            inner: RwLock::new(None),
        }
    }

    pub async fn inject_settings(&self, settings_reader: Arc<SettingsReader>) {
        let mut write_access = self.inner.write().await;

        write_access.replace(Arc::new(AppCtxInner {
            telemetry_reader_grpc_client: Arc::new(TelemetryReaderGrpcClient::new(
                settings_reader.clone(),
            )),
            settings_reader: settings_reader,
        }));
    }

    pub async fn get_settings_reader(&self) -> Arc<SettingsReader> {
        let read_access = self.inner.read().await;
        read_access.as_ref().unwrap().settings_reader.clone()
    }

    pub async fn get_telemetry_reader_grpc_client(&self) -> Arc<TelemetryReaderGrpcClient> {
        let read_access = self.inner.read().await;
        read_access
            .as_ref()
            .unwrap()
            .telemetry_reader_grpc_client
            .clone()
    }
}
