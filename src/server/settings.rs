use std::collections::{BTreeMap, HashMap};

use serde::*;

use my_ssh::SshCredentialsSettingsModel;

#[derive(Serialize, Deserialize)]
pub struct SettingsModel {
    pub envs: BTreeMap<String, String>,
    pub ssh_credentials: Option<HashMap<String, SshCredentialsSettingsModel>>,
}

impl SettingsModel {
    pub fn get_envs(&self) -> Vec<String> {
        self.envs.keys().cloned().collect()
    }
    pub async fn get_env_url(&self, env: &str) -> my_ssh::OverSshConnectionSettings {
        if let Some(result) = self.envs.get(env) {
            return my_ssh::OverSshConnectionSettings::parse(result, self.ssh_credentials.as_ref())
                .await;
        }

        panic!("Can not get settings for env: '{}'", env);
    }
}
/*
#[async_trait::async_trait]
impl my_grpc_extensions::GrpcClientSettings for SettingsModel {
    async fn get_grpc_url(&self, name: &'static str) -> String {
        if name == TelemetryReaderGrpcClient::get_service_name() {
            match std::env::var("TELEMETRY_READER_GRPC_URL") {
                Ok(url) => return url,
                Err(_) => panic!("TELEMETRY_READER_GRPC_URL is not set"),
            }
        }

        panic!("Unknown grpc service name: {}", name)
    }
}

*/
