use std::time::Duration;

use rust_extensions::date_time::DateTimeAsMicroseconds;

use crate::reader_grpc::{AppDataGrpcModel, GetAppEventsByActionRequest};

impl AppDataGrpcModel {
    pub fn get_started(&self) -> DateTimeAsMicroseconds {
        DateTimeAsMicroseconds::new(self.started)
    }

    pub fn get_duration(&self) -> Duration {
        Duration::from_micros(self.duration as u64)
    }
}

pub async fn get_by_service_data(
    service_id: String,
    data: String,
) -> Result<Vec<AppDataGrpcModel>, String> {
    let result = tokio::spawn(async move {
        let grpc_client = crate::APP_CTX.get_telemetry_reader_grpc_client().await;

        let response = grpc_client
            .get_app_events_by_action(GetAppEventsByActionRequest {
                app_id: service_id,
                data,
            })
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
