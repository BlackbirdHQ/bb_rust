use crate::misc::CompressError;
use crate::misc::DecompressError;

use thiserror::Error;

mod gateway;
mod internal;

pub use gateway::{gateway_graphql_request, GatewayGraphQLRequestBody};
pub use internal::{
    batch_internal_graphql_request, internal_graphql_request, GraphQLRequestBody, GraphqlContext,
};

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
    #[error("bad format: {0}")]
    BadFormat(#[from] CompressError),
}
