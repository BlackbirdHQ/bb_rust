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
            
            writeln!(buf, "{} - {}: {}", record.target(), record.level(), stripped)
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
        let input = [34, 72, 52, 115, 73, 65, 65, 65, 65, 65, 65, 65, 65, 65, 52, 117, 117, 86, 107, 112, 74, 76, 69, 108, 85, 115, 113, 112, 87, 75, 105, 48, 65, 115, 108, 73, 100, 99, 120, 75, 76, 99, 107, 72, 99, 122, 66, 81, 108, 75, 121, 85, 76, 115, 120, 81, 68, 83, 49, 78, 106, 65, 49, 50, 68, 82, 70, 77, 84, 88, 85, 80, 68, 49, 71, 84, 100, 74, 68, 80, 76, 78, 78, 51, 107, 78, 75, 79, 48, 112, 66, 82, 106, 111, 53, 82, 85, 99, 119, 79, 108, 50, 116, 114, 97, 87, 65, 65, 57, 116, 83, 56, 84, 83, 65, 65, 65, 65, 65, 61, 61, 34];
        let x: serde_json::Value = decompress(&input);
        let id = x.as_array().unwrap()[0].get("data").unwrap().get("updateAlarm").unwrap().get("id").unwrap().as_str().unwrap();
        assert_eq!(id, "86d09530-0a54-11ec-b69f-cf2fbd32de70")
    }
}