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
    let response = crate::APP_CTX
        .grpc_client
        .get_by_process_id(GetByProcessIdRequest { process_id })
        .await;

    match response {
        Ok(response) => match response {
            Some(response) => Ok(response),
            None => Ok(vec![]),
        },
        Err(err) => Err(format!("{:?}", err)),
    }
}
