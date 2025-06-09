use std::{collections::HashMap, sync::Arc};

use my_settings_reader::SettingsReader;
use tokio::sync::Mutex;

use crate::server::{grpc_client::*, settings::SettingsModel};

//pub const APP_VERSION: &'static str = env!("CARGO_PKG_VERSION");
//pub const APP_NAME: &'static str = env!("CARGO_PKG_NAME");
pub struct AppCtx {
    pub settings_reader: SettingsReader<SettingsModel>,
    pub clients_cache: Mutex<HashMap<String, Arc<TelemetryReaderGrpcClient>>>,
}

impl AppCtx {
    pub fn new() -> Self {
        Self {
            settings_reader: SettingsReader::new("~/.my-telemetry-ui"),
            clients_cache: Mutex::new(HashMap::new()),
        }
    }

    pub async fn get_grpc_client(&self, env: &str) -> Arc<TelemetryReaderGrpcClient> {
        let mut clients_cache = self.clients_cache.lock().await;

        if let Some(result) = clients_cache.get(env).cloned() {
            return result;
        }

        let settings = self.settings_reader.get_settings().await;
        let grpc_settings = settings.get_grpc_url(env);

        let grpc_settings = Arc::new(grpc_settings);

        let grpc_client = TelemetryReaderGrpcClient::new(grpc_settings);
        let grpc_client = Arc::new(grpc_client);

        clients_cache.insert(env.to_string(), grpc_client.clone());

        grpc_client
    }
}
