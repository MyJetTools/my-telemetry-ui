use std::time::Duration;

use flurl::IntoFlUrl;
use rust_extensions::date_time::DateTimeAsMicroseconds;
#[derive(serde::Deserialize)]
pub struct ProcessMetricsContract {
    pub metrics: Vec<ProcessMetrics>,
}

#[derive(serde::Deserialize, Clone)]
pub struct ProcessMetrics {
    pub id: String,
    pub data: String,
    pub started: i64,
    pub duration: i64,
    pub success: Option<String>,
    pub error: Option<String>,
    pub ip: Option<String>,
}

impl ProcessMetrics {
    pub fn get_started(&self) -> DateTimeAsMicroseconds {
        DateTimeAsMicroseconds::new(self.started)
    }

    pub fn get_duration(&self) -> Duration {
        Duration::from_micros(self.duration as u64)
    }
}

pub async fn get_by_process_id(process_id: i64) -> Result<Vec<ProcessMetrics>, String> {
    let result = tokio::spawn(async move {
        let settings_reader = crate::APP_CTX.get_settings_reader().await;

        let url = settings_reader.get_url().await;

        let response = url
            .append_path_segment("ui")
            .append_path_segment("GetByProcessId")
            .append_query_param("processId", Some(process_id.to_string()))
            .get()
            .await;

        match response {
            Ok(mut response) => {
                let response: ProcessMetricsContract = response.get_json().await.unwrap();
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
