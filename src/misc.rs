use lazy_static::lazy_static;
use std::io::Write;

use flate2::{
    write::{GzDecoder, GzEncoder},
    Compression,
};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use thiserror::Error;

lazy_static! {
    static ref AWS_LAMBDA_RUNTIME_API: Option<String> =
        std::env::var("AWS_LAMBDA_RUNTIME_API").ok();
}

/// Helper macro until the Try block syntax gets stable https://github.com/rust-lang/rust/issues/31436
#[macro_export]
macro_rules! try_block {
    { $($token:tt)* } => {{
        let l = || {
            $($token)*
        };
        l()
    }}
}

pub fn setup_aws_lambda_logging() {
    env_logger::builder()
        .format(|buf, record| {
            // AWS Cloudwatch logs show a new line for each '\n'
            // so replace that with '\r'

            let message = record.args().to_string();

            let reshaped_message = if AWS_LAMBDA_RUNTIME_API.is_some() {
                message.replace("\n\r", "\r").replace('\n', "\r")
            } else {
                message
            };

            writeln!(
                buf,
                "{} - {}: {}",
                record.target(),
                record.level(),
                reshaped_message
            )
        })
        .init();
}

/// Serialized as `gzip(toJson(data))`
/// Derialized as `gunzip(fromJson(data))`
#[derive(Clone, Debug)]
pub struct GzippedJSON<T>(pub T);

impl<'de, T: DeserializeOwned> Deserialize<'de> for GzippedJSON<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let bytes = <serde_bytes::ByteBuf>::deserialize(deserializer)?;
        Ok(GzippedJSON(
            decompress(&bytes).map_err(serde::de::Error::custom)?,
        ))
    }
}

impl<T: Serialize> Serialize for GzippedJSON<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let bytes = compress(&self.0).map_err(serde::ser::Error::custom)?;
        serializer.serialize_bytes(&bytes)
    }
}

#[derive(Error, Debug)]
pub enum CompressError {
    #[error("GZ encoder error: {0}")]
    EncoderWriterError(#[from] std::io::Error),
    #[error("bad json response. Error: {0}")]
    UnexpectedJsonResponse(#[from] serde_json::Error),
}

/// Gzip compress that is typically used together with base64 encoding to minimize data sent/stored
pub fn compress<T: Serialize>(input: T) -> Result<Vec<u8>, CompressError> {
    let mut e = GzEncoder::new(Vec::new(), Compression::default());
    e.write_all(serde_json::to_string(&input)?.as_bytes())?;
    Ok(e.finish()?)
}

#[derive(Error, Debug)]
pub enum DecompressError {
    #[error("GZ decoder error: {0}")]
    DecoderWriterError(#[from] std::io::Error),
    #[error("bad json response. Error: {0}")]
    UnexpectedJsonResponse(#[from] serde_json::Error),
}

/// Gzip decompress that is typically used together with base64 encoding to minimize data sent/stored
pub fn decompress<T: DeserializeOwned>(input: &[u8]) -> Result<T, DecompressError> {
    log::trace!("About to decompress: {:?}", input);
    let mut writer = Vec::new();
    let mut decoder = GzDecoder::new(writer);

    // try to base64 decode, and if that fails, then just try to proceed.
    // If base64 encoded the input includes explicit '"' which we want to remove
    decoder.write_all(input)?;
    writer = decoder.finish()?;
    Ok(serde_json::from_slice(&writer)?)
}

#[cfg(test)]
mod tests {
    use aws_sdk_dynamodb::model::AttributeValue;
    use serde_dynamo::{from_attribute_value, to_attribute_value};

    use crate::misc::compress;

    use super::{decompress, GzippedJSON};

    #[test]
    fn test_compress_decompress() {
        let input = "hejhej";
        let compressed = compress(input).unwrap();
        let decompressed: String = decompress(&compressed).unwrap();
        assert_eq!(input, decompressed)
    }

    #[test]
    fn test_decompress() {
        let input = [
            72, 52, 115, 73, 65, 65, 65, 65, 65, 65, 65, 65, 65, 52, 117, 117, 86, 107, 112, 74,
            76, 69, 108, 85, 115, 113, 112, 87, 75, 105, 48, 65, 115, 108, 73, 100, 99, 120, 75,
            76, 99, 107, 72, 99, 122, 66, 81, 108, 75, 121, 85, 76, 115, 120, 81, 68, 83, 49, 78,
            106, 65, 49, 50, 68, 82, 70, 77, 84, 88, 85, 80, 68, 49, 71, 84, 100, 74, 68, 80, 76,
            78, 78, 51, 107, 78, 75, 79, 48, 112, 66, 82, 106, 111, 53, 82, 85, 99, 119, 79, 108,
            50, 116, 114, 97, 87, 65, 65, 57, 116, 83, 56, 84, 83, 65, 65, 65, 65, 65, 61, 61,
        ];
        let x: serde_json::Value = decompress(&base64::decode(input).unwrap()).unwrap();
        let id = x.as_array().unwrap()[0]
            .get("data")
            .unwrap()
            .get("updateAlarm")
            .unwrap()
            .get("id")
            .unwrap()
            .as_str()
            .unwrap();
        assert_eq!(id, "86d09530-0a54-11ec-b69f-cf2fbd32de70")
    }

    #[test]
    fn test_gzip_wrapper() {
        let gzipped = GzippedJSON("hej".to_string());
        let attr_value: AttributeValue = to_attribute_value(gzipped.clone()).unwrap();
        assert!(attr_value.is_b());
        let back: GzippedJSON<String> = from_attribute_value(attr_value).unwrap();
        assert_eq!(gzipped.0, back.0)
    }
}
