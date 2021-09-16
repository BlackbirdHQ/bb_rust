use rusoto_core::RusotoError;
use serde::de::DeserializeOwned;
use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;

#[derive(Serialize)]
pub struct GraphQLRequestBody {
    pub query: String,
    pub variables: Value,
    pub context: Value,
}

use rusoto_lambda::{InvocationRequest, InvokeError, Lambda};
use serde_json::json;

use crate::misc::DecompressError;
use crate::misc::{compress, decompress};

use thiserror::Error;

/// Invokes a graphql query against an *internal* AWS lambda, e.g. ms-graphql-devices.
///
/// **Note**: Do not use this method for querying the public-facing ms-graphql-gateway.
pub async fn internal_graphql_request<R: DeserializeOwned + Clone, L: Lambda>(
    lambda: &L,
    graphql: GraphQLRequestBody,
    lambda_function_name: String,
) -> Result<R, GraphQLError> {
    let body = serde_json::to_string(&graphql)?;
    let payload = vec![json!({ "body": body })];
    let payload = compress(payload).unwrap();
    let payload = format!("\"{}\"", base64::encode(payload));
    let input = InvocationRequest {
        function_name: lambda_function_name,
        invocation_type: Some("RequestResponse".to_string()),
        payload: Some(payload.into()),
        ..Default::default()
    };

    let response = lambda.invoke(input).await?;
    if let Some(err) = response.function_error {
        return Err(GraphQLError::LambdaFunctionError(err));
    }
    if response.status_code != Some(200) {
        return Err(GraphQLError::LambdaFunctionBadStatusCode {
            payload: format!("{:?}", response.payload),
            status_code: response.status_code,
        });
    }

    // Try to parse the GraphQL result
    let res: [InternalGraphQLResponse<R>; 1] =
        decompress(&response.payload.ok_or(GraphQLError::NoResponsePayload)?)?;

    let first_result = &res[0];
    if let Some(errors) = &first_result.errors {
        Err(GraphQLError::InternalGraphQLError(errors.to_string()))
    } else {
        Ok(first_result.clone().data.unwrap())
    }
}

#[derive(Deserialize, Clone)]
struct InternalGraphQLResponse<T: Clone> {
    data: Option<T>,
    errors: Option<serde_json::Value>,
}

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
}
