use crate::reader_grpc::{AppDataGrpcModel, GetAppEventsByActionRequest};

pub async fn get_by_service_data(
    service_id: String,
    data: String,
) -> Result<Vec<AppDataGrpcModel>, String> {
    let response = crate::APP_CTX
        .grpc_client
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
}
