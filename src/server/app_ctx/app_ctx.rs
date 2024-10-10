use std::{collections::HashMap, sync::Arc};

use my_settings_reader::SettingsReader;
use my_ssh::SshSessionsPool;
use tokio::sync::Mutex;

use crate::server::{grpc_client::*, settings::SettingsModel};

use super::GrpcLogSettings;

//pub const APP_VERSION: &'static str = env!("CARGO_PKG_VERSION");
//pub const APP_NAME: &'static str = env!("CARGO_PKG_NAME");
pub struct AppCtx {
    pub settings_reader: SettingsReader<SettingsModel>,
    pub clients_cache: Mutex<HashMap<String, Arc<TelemetryReaderGrpcClient>>>,
    pub ssh_sessions_pool: Arc<SshSessionsPool>,
}

impl AppCtx {
    pub fn new() -> Self {
        Self {
            settings_reader: SettingsReader::new("~/.my-telemetry-ui"),
            clients_cache: Mutex::new(HashMap::new()),
            ssh_sessions_pool: SshSessionsPool::new().into(),
        }
    }

    pub async fn get_grpc_client(&self, env: &str) -> Arc<TelemetryReaderGrpcClient> {
        let mut clients_cache = self.clients_cache.lock().await;

        if let Some(result) = clients_cache.get(env).cloned() {
            return result;
        }

        let settings = self.settings_reader.get_settings().await;
        let over_ssh_connection = settings.get_env_url(env).await;

        let grpc_client = TelemetryReaderGrpcClient::new(Arc::new(GrpcLogSettings::new(
            over_ssh_connection.remote_resource_string,
        )));

        if let Some(value) = over_ssh_connection.ssh_credentials {
            grpc_client.set_ssh_credentials(Arc::new(value)).await;
            grpc_client
                .set_ssh_sessions_pool(self.ssh_sessions_pool.clone())
                .await;
        };

        let grpc_client = Arc::new(grpc_client);

        clients_cache.insert(env.to_string(), grpc_client.clone());

        grpc_client
    }
}
