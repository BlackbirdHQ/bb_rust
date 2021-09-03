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
    log::trace!("About to decompress: {:?}", input);
    let mut writer = Vec::new();
    let mut decoder = GzDecoder::new(writer);
    // the input is base64 encoded, but with explicit '"', so remove those
    let base64_decoded = base64::decode(&input[1..input.len()-1]).unwrap();
    decoder.write_all(&base64_decoded).unwrap();
    writer = decoder.finish().unwrap();
    serde_json::from_slice(&writer).unwrap()
}

#[cfg(test)]
mod tests {
    use super::decompress;

    #[test]
    fn test_decompress(){
        let input = [34, 72, 52, 115, 73, 65, 65, 65, 65, 65, 65, 65, 65, 65, 122, 51, 78, 118, 119, 114, 67, 77, 66, 67, 65, 56, 86, 99, 53, 98, 114, 71, 70, 68, 67, 113, 52, 90, 72, 72, 121, 67, 99, 83, 112, 100, 106, 105, 84, 111, 119, 97, 83, 79, 56, 48, 102, 97, 67, 108, 57, 100, 122, 117, 53, 102, 118, 68, 106, 71, 49, 90, 77, 88, 65, 112, 78, 106, 66, 98, 118, 105, 49, 83, 97, 52, 90, 97, 122, 90, 103, 115, 80, 52, 102, 110, 68, 114, 114, 75, 72, 69, 109, 83, 75, 68, 78, 43, 109, 108, 99, 71, 57, 75, 100, 79, 101, 77, 51, 83, 72, 51, 111, 65, 80, 72, 104, 90, 116, 107, 74, 103, 69, 113, 107, 73, 114, 68, 65, 82, 101, 50, 43, 115, 118, 117, 105, 102, 50, 86, 122, 81, 89, 49, 86, 69, 78, 75, 103, 88, 116, 115, 71, 73, 77, 115, 107, 57, 80, 82, 52, 78, 79, 89, 48, 117, 67, 57, 110, 122, 90, 120, 109, 51, 56, 65, 88, 57, 77, 99, 57, 71, 84, 65, 65, 65, 65, 34];
        let _: serde_json::Value = decompress(&input);
    }
}