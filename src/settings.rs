use crate::grpc_client::TelemetryReaderGrpcClient;

pub struct SettingsModel;

#[async_trait::async_trait]
impl my_grpc_extensions::GrpcClientSettings for SettingsModel {
    async fn get_grpc_url(&self, name: &'static str) -> String {
        if name == TelemetryReaderGrpcClient::get_service_name() {
            match std::env::var("TELEMETRY_READER_GRPC_URL") {
                Ok(url) => url,
                Err(_) => panic!("TELEMETRY_READER_GRPC_URL is not set"),
            }
        }

        panic!("Unknown grpc service name: {}", name)
    }
}
