use std::time::Duration;

use crate::reader_grpc::{AppActionGrpcModel, GetByAppRequest};

impl AppActionGrpcModel {
    pub fn get_avg_duration(&self) -> Duration {
        Duration::from_micros(self.avg as u64)
    }

    pub fn get_min_duration(&self) -> Duration {
        Duration::from_micros(self.min as u64)
    }

    pub fn get_max_duration(&self) -> Duration {
        Duration::from_micros(self.max as u64)
    }
}

pub async fn get_services_overview(service_id: String) -> Result<Vec<AppActionGrpcModel>, String> {
    let result = tokio::spawn(async move {
        let grpc_client = crate::APP_CTX.get_telemetry_reader_grpc_client().await;

        let response = grpc_client
            .get_app_actions(GetByAppRequest { app_id: service_id })
            .await;

        match response {
            Ok(response) => match response {
                Some(response) => Ok(response),
                None => Ok(vec![]),
            },
            Err(err) => Err(format!("{:?}", err)),
        }
    })
    .await;

    match result {
        Ok(result) => result,
        Err(err) => Err(format!("{:?}", err)),
    }
}
