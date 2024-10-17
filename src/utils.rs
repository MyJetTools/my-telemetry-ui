use rust_extensions::base64::*;

pub fn to_base_64(src: &str) -> String {
    src.as_bytes().into_base64()
}

pub fn from_base_64(src: &str) -> String {
    let result = src.from_base64().unwrap();
    String::from_utf8(result).unwrap()
}
