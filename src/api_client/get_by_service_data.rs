use std::time::Duration;

use flurl::IntoFlUrl;
use rust_extensions::date_time::DateTimeAsMicroseconds;
#[derive(serde::Deserialize)]
pub struct ServiceDataContract {
    pub metrics: Vec<ServiceDataMetrics>,
}

#[derive(serde::Deserialize, Clone)]
pub struct ServiceDataMetrics {
    pub id: i64,
    pub started: i64,
    pub duration: i64,

    pub success: Option<String>,
    pub error: Option<String>,
    pub ip: Option<String>,
}

impl ServiceDataMetrics {
    pub fn get_started(&self) -> DateTimeAsMicroseconds {
        DateTimeAsMicroseconds::new(self.started)
    }

    pub fn get_duration(&self) -> Duration {
        Duration::from_micros(self.duration as u64)
    }
}

pub async fn get_by_service_data(
    service_id: String,
    data: String,
) -> Result<Vec<ServiceDataMetrics>, String> {
    let result = tokio::spawn(async move {
        let settings_reader = crate::APP_CTX.get_settings_reader().await;

        let url = settings_reader.get_url().await;

        let response = url
            .append_path_segment("ui")
            .append_path_segment("GetByServiceData")
            .append_query_param("id", Some(service_id))
            .append_query_param("data", Some(data))
            .get()
            .await;

        match response {
            Ok(mut response) => {
                let response: ServiceDataContract = response.get_json().await.unwrap();
                Ok(response.metrics)
            }
            Err(err) => Err(format!("{:?}", err)),
        }
    })
    .await;

    match result {
        Ok(result) => result,
        Err(err) => Err(format!("{:?}", err)),
    }
}
