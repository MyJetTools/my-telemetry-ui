use std::time::Duration;

use crate::reader_grpc::ServiceGrpcModel;

impl ServiceGrpcModel {
    pub fn get_avg_duration(&self) -> Duration {
        Duration::from_micros(self.avg as u64)
    }
}

pub async fn get_list_of_services() -> Result<Vec<ServiceGrpcModel>, String> {
    let result = tokio::spawn(async move {
        let response = crate::APP_CTX.grpc_client.get_apps(()).await;
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
