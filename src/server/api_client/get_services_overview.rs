use crate::server::reader_grpc::*;

pub async fn get_services_overview(
    env: &str,
    hours_ago: i64,
    service_id: String,
) -> Vec<AppActionGrpcModel> {
    let grpc_client = crate::server::APP_CTX.get_grpc_client(env).await;
    let apps = grpc_client
        .get_app_actions(GetByAppRequest {
            app_id: service_id,
            hour_key: super::calc_hour_key(hours_ago),
        })
        .await
        .unwrap()
        .unwrap_or_default();

    apps
}
