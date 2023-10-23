use crate::reader_grpc::{AppActionGrpcModel, GetByAppRequest};

pub async fn get_services_overview(service_id: String) -> Result<Vec<AppActionGrpcModel>, String> {
    let response = crate::APP_CTX
        .grpc_client
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
