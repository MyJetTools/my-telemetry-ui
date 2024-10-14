use serde::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct MetricFileHttpModel {
    pub hours_ago: i64,
    pub file_size: u64,
}
