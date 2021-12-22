use rusoto_core::RusotoError;

use rusoto_lambda::InvokeError;

use crate::misc::CompressError;
use crate::misc::DecompressError;

use thiserror::Error;

mod gateway;
mod internal;

pub use gateway::{gateway_graphql_request, GatewayGraphQLRequestBody};
pub use internal::{internal_graphql_request, GraphQLRequestBody};

#[derive(Error, Debug)]
pub enum GraphQLError {
    #[error("invalid input query: {0}")]
    InvalidInputQuery(#[from] serde_json::Error),
    #[error("decompress error: {0}")]
    DecompressError(#[from] DecompressError),
    #[error("failed invoking lambda: {0}")]
    LambdaInvoke(#[from] RusotoError<InvokeError>),
    #[error("lambda function error: {0}")]
    LambdaFunctionError(String),
    #[error("lambda function bad status code {status_code:?} with payload: {payload}")]
    LambdaFunctionBadStatusCode {
        status_code: Option<i64>,
        payload: String,
    },
    #[error("no response payload")]
    NoResponsePayload,
    #[error("bad json response. Error: {0}")]
    UnexpectedJsonResponse(serde_json::Error),
    #[error("internal graphql error: {0}")]
    InternalGraphQLError(String),
    #[error("bad format: {0}")]
    BadFormat(#[from] CompressError),
}
