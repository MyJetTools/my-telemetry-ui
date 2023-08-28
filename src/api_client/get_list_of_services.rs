use std::time::Duration;

use flurl::IntoFlUrl;
#[derive(serde::Deserialize)]
pub struct ServicesContract {
    pub services: Vec<ServiceModel>,
}

#[derive(serde::Deserialize, Clone)]
pub struct ServiceModel {
    pub id: String,
    avg: i64,
}

impl ServiceModel {
    pub fn get_avg_duration(&self) -> Duration {
        Duration::from_micros(self.avg as u64)
    }
}

pub async fn get_list_of_services() -> Result<Vec<ServiceModel>, String> {
    let result = tokio::spawn(async move {
        let settings_reader = crate::APP_CTX.get_settings_reader().await;

        let url = settings_reader.get_url().await;

        let response = url
            .append_path_segment("ui")
            .append_path_segment("GetServices")
            .get()
            .await;

        match response {
            Ok(mut response) => {
                let response: ServicesContract = response.get_json().await.unwrap();
                Ok(response.services)
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
