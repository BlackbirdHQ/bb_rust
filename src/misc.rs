use std::io::Write;

use flate2::{Compression, write::{GzDecoder, GzEncoder}};
use serde::{Serialize, de::DeserializeOwned};

pub fn setup_aws_lambda_logging() {
    env_logger::builder()
        .format(|buf, record| {
            // AWS Cloudwatch logs show a new line for each '\n'
            // so replace that with '\r'
            let stripped = record
                .args()
                .to_string()
                .replace("\n\r", "\r")
                .replace('\n', "\r");

            writeln!(buf, "{}: {}", record.level(), stripped)
        })
        .init();
}

/// Gzip compress that is typically used together with base64 encoding to minimize data sent/stored
pub fn compress<T: Serialize>(input: T) -> Vec<u8> {
    let mut e = GzEncoder::new(Vec::new(), Compression::default());
    e.write_all(serde_json::to_string(&input).unwrap().as_bytes()).unwrap();
    e.finish().unwrap()
}

/// Gzip decompress that is typically used together with base64 encoding to minimize data sent/stored
pub fn decompress<T: DeserializeOwned>(input: &[u8]) -> T {
    let mut writer = Vec::new();
    let mut decoder = GzDecoder::new(writer);
    decoder.write_all(input).unwrap();
    writer = decoder.finish().unwrap();
    serde_json::from_slice(&writer).unwrap()
}