use crate::server::reader_grpc::{AppDataGrpcModel, GetAppEventsByActionRequest};

pub async fn get_by_service_data(
    env: &str,
    service_id: String,
    data: String,
) -> Result<Vec<AppDataGrpcModel>, String> {
    let response = crate::server::APP_CTX
        .get_grpc_client(env)
        .await
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
