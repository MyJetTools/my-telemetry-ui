use std::time::Duration;

use rust_extensions::date_time::DateTimeAsMicroseconds;

use crate::models::*;

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct ServiceDataApiModel {
    pub process_id: i64,
    pub started: i64,
    pub duration: u64,
    pub success: Option<String>,
    pub fail: Option<String>,
    pub tags: Vec<TagApiModel>,
}

impl ServiceDataApiModel {
    pub fn get_started(&self) -> DateTimeAsMicroseconds {
        DateTimeAsMicroseconds::new(self.started)
    }

    pub fn get_duration(&self) -> Duration {
        Duration::from_micros(self.duration)
    }
}
