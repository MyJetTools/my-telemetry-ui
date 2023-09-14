use std::time::Duration;

use rust_extensions::date_time::DateTimeAsMicroseconds;

use crate::reader_grpc::{GetByProcessIdRequest, MetricEventGrpcModel};

impl MetricEventGrpcModel {
    pub fn get_started(&self) -> DateTimeAsMicroseconds {
        DateTimeAsMicroseconds::new(self.started)
    }

    pub fn get_duration(&self) -> Duration {
        Duration::from_micros(self.duration as u64)
    }
}

pub async fn get_by_process_id(process_id: i64) -> Result<Vec<MetricEventGrpcModel>, String> {
    let result = tokio::spawn(async move {
        let grpc_client = crate::APP_CTX.get_telemetry_reader_grpc_client().await;

        let response = grpc_client
            .get_by_process_id(GetByProcessIdRequest { process_id })
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
