use crate::misc::CompressError;
use crate::misc::DecompressError;

use serde::Deserialize;
use thiserror::Error;

mod gateway;
mod internal;

pub use gateway::{gateway_graphql_request, GatewayGraphQLRequestBody};
pub use internal::{internal_graphql_request, GraphQLRequestBody};

#[allow(clippy::large_enum_variant)]
#[derive(Error, Debug)]
pub enum GraphQLError {
    #[error("invalid input query: {0}")]
    InvalidInputQuery(#[from] serde_json::Error),
    #[error("decompress error: {0}")]
    DecompressError(#[from] DecompressError),
    #[error("failed invoking lambda: {0}")]
    LambdaInvoke(#[from] aws_sdk_lambda::types::SdkError<aws_sdk_lambda::error::InvokeError>),
    #[error("lambda function error: {0}")]
    LambdaFunctionError(String),
    #[error("lambda function bad status code {status_code} with payload: {payload}")]
    LambdaFunctionBadStatusCode { status_code: i32, payload: String },
    #[error("no response payload")]
    NoResponsePayload,
    #[error("bad json response. Error: {0}")]
    UnexpectedJsonResponse(serde_json::Error),
    #[error("internal graphql error: {0:?}")]
    InternalGraphQLError(Vec<InnerGraphQLError>),
    #[error("bad format: {0}")]
    BadFormat(#[from] CompressError),
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
enum GraphQLResponse {
    // Order between variants is important, as we want to catch errors before data
    Error { errors: Vec<InnerGraphQLError> },
    Data { data: serde_json::Value },
}

#[derive(Deserialize, Debug)]
pub struct InnerGraphQLError {
    pub locations: Vec<GraphQLErrorLocation>,
    pub message: String,
}

#[derive(Deserialize, Debug)]
pub struct GraphQLErrorLocation {
    pub column: u32,
    pub line: u32,
}
