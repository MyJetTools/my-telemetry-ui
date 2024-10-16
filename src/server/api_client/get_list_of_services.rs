use crate::server::reader_grpc::{GetAppsRequest, ServiceGrpcModel};

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
