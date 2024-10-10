use crate::server::reader_grpc::{AppActionGrpcModel, GetByAppRequest};

pub async fn get_services_overview(
    env: &str,
    service_id: String,
) -> Result<Vec<AppActionGrpcModel>, String> {
    let response = crate::server::APP_CTX
        .get_grpc_client(env)
        .await
        .get_app_actions(GetByAppRequest { app_id: service_id })
        .await;

    match response {
        Ok(response) => match response {
            Some(response) => Ok(response),
            None => Ok(vec![]),
        },
        Err(err) => Err(format!("{:?}", err)),
    }
}
