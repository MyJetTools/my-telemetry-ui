use std::time::Duration;

use flurl::IntoFlUrl;
#[derive(serde::Deserialize)]
pub struct ServicesOverviewContract {
    pub data: Vec<ServiceOverview>,
}

#[derive(serde::Deserialize, Clone)]
pub struct ServiceOverview {
    pub data: String,
    pub min: i64,
    pub max: i64,
    pub avg: i64,
    pub success: usize,
    pub error: usize,
    pub total: usize,
}

impl ServiceOverview {
    pub fn get_avg_duration(&self) -> Duration {
        Duration::from_micros(self.avg as u64)
    }

    pub fn get_min_duration(&self) -> Duration {
        Duration::from_micros(self.min as u64)
    }

    pub fn get_max_duration(&self) -> Duration {
        Duration::from_micros(self.max as u64)
    }
}

pub async fn get_services_overview(service_id: String) -> Result<Vec<ServiceOverview>, String> {
    let result = tokio::spawn(async move {
        let settings_reader = crate::APP_CTX.get_settings_reader().await;

        let url = settings_reader.get_url().await;

        let response = url
            .append_path_segment("ui")
            .append_path_segment("GetServiceOverview")
            .append_query_param("id", Some(service_id))
            .get()
            .await;

        match response {
            Ok(mut response) => {
                let response: ServicesOverviewContract = response.get_json().await.unwrap();
                Ok(response.data)
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
