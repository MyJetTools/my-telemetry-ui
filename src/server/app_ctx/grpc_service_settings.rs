use crate::server::grpc_client::*;

pub struct GrpcLogSettings(String);

impl GrpcLogSettings {
    pub fn new(host: String) -> Self {
        Self(host)
    }
}

#[async_trait::async_trait]
impl my_grpc_extensions::GrpcClientSettings for GrpcLogSettings {
    async fn get_grpc_url(&self, name: &'static str) -> String {
        if name == TelemetryReaderGrpcClient::get_service_name() {
            return self.0.to_string();
        }

        panic!("Unknown service name: {}", name);
    }
}
