use std::time::Duration;

use rust_extensions::base64::*;

pub fn unix_microseconds_to_string(src: i64) -> String {
    let d = std::time::UNIX_EPOCH + Duration::from_micros(src as u64);
    let dt = chrono::DateTime::<chrono::Utc>::from(d);
    dt.to_rfc3339()
}

pub fn to_base_64(src: &str) -> String {
    src.as_bytes().into_base64()
}

pub fn from_base_64(src: &str) -> String {
    let result = src.from_base64().unwrap();
    String::from_utf8(result).unwrap()
}
