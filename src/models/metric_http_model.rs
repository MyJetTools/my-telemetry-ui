use std::time::Duration;

use crate::models::*;
#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct MetricEventApiModel {
    pub started: i64,
    pub duration: i64,
    pub name: String,
    pub data: String,
    pub tags: Vec<TagApiModel>,
    pub success: Option<String>,
    pub fail: Option<String>,
}

impl MetricEventApiModel {
    pub fn get_started(&self) -> String {
        crate::utils::unix_microseconds_to_string(self.started)
    }

    pub fn get_duration(&self) -> Duration {
        Duration::from_micros(self.duration as u64)
    }
}
