use std::io::Write;

use flate2::{Compression, write::{GzDecoder, GzEncoder}};

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

            writeln!(buf, "{}: {:?}", record.level(), stripped)
        })
        .init();
}

/// Gzip compress that is typically used together with base64 encoding to minimize data sent/stored
pub fn compress(input: String) -> Vec<u8> {
    let mut e = GzEncoder::new(Vec::new(), Compression::default());
    e.write_all(input.as_bytes()).unwrap();
    e.finish().unwrap()
}

/// Gzip decompress that is typically used together with base64 encoding to minimize data sent/stored
pub fn decompress(input: &[u8]) -> Vec<u8> {
    let mut writer = Vec::new();
    let mut decoder = GzDecoder::new(writer);
    decoder.write_all(input).unwrap();
    writer = decoder.finish().unwrap();
    writer
}