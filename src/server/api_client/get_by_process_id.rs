use crate::server::reader_grpc::{GetByProcessIdRequest, MetricEventGrpcModel};

pub async fn get_by_process_id(
    env: &str,
    hours_ago: i64,
    process_id: i64,
) -> Result<Vec<MetricEventGrpcModel>, String> {
    let response = crate::server::APP_CTX
        .get_grpc_client(env)
        .await
        .get_by_process_id(GetByProcessIdRequest {
            process_id,
            hour_key: super::calc_hour_key(hours_ago),
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
