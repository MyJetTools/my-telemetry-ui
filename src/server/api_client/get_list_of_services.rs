use std::time::Duration;

use crate::server::reader_grpc::{GetAppsRequest, ServiceGrpcModel};

impl ServiceGrpcModel {
    pub fn get_avg_duration(&self) -> Duration {
        Duration::from_micros(self.avg as u64)
    }
}

pub async fn get_list_of_services(
    env: &str,
    hours_ago: i64,
) -> Result<Vec<ServiceGrpcModel>, String> {
    let response = crate::server::APP_CTX
        .get_grpc_client(env)
        .await
        .get_apps(GetAppsRequest {
            hour_key: super::calc_hour_key(hours_ago),
        })
        .await;

    match response {
        Ok(result) => Ok(result.unwrap_or_default()),
        Err(err) => Err(format!("{:?}", err)),
    }
}
