use std::collections::{BTreeMap, HashMap};

use my_ssh::ssh_settings::*;
use serde::*;

use super::app_ctx::GrpcLogSettings;

#[derive(Serialize, Deserialize)]
pub struct SettingsModel {
    pub envs: BTreeMap<String, String>,
    pub ssh_credentials: Option<HashMap<String, SshPrivateKeySettingsModel>>,
}

impl SettingsModel {
    pub fn get_envs(&self) -> Vec<String> {
        self.envs.keys().cloned().collect()
    }
    pub fn get_grpc_url(&self, env: &str) -> GrpcLogSettings {
        if let Some(result) = self.envs.get(env) {
            return GrpcLogSettings::new(result.to_string());
        }

        panic!("Can not get settings for env: '{}'", env);
    }
}
