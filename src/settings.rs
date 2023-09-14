use serde::*;

use crate::grpc_client::TelemetryReaderGrpcClient;

#[derive(my_settings_reader::SettingsModel, Serialize, Deserialize, Debug, Clone)]
pub struct SettingsModel {
    pub url: String,
}

#[async_trait::async_trait]
impl my_grpc_extensions::GrpcClientSettings for SettingsReader {
    async fn get_grpc_url(&self, name: &'static str) -> String {
        if name == TelemetryReaderGrpcClient::get_service_name() {
            let read_access = self.settings.read().await;
            return read_access.url.clone();
        }

        panic!("Unknown grpc service name: {}", name)
    }
}
