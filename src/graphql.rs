use rusoto_core::RusotoError;
use serde_json::Value;
use serde::Serialize;
use serde::Deserialize;
use serde::de::DeserializeOwned;


#[derive(Serialize)]
pub struct GraphQLRequest {
    pub body: GraphQLRequestBody,
}

#[derive(Serialize)]
pub struct GraphQLRequestBody {
    pub query: String,
    pub variables: Value,
    pub context: Value,
}

use rusoto_lambda::{InvocationRequest, InvokeError, Lambda};

use crate::misc::{compress, decompress};

use thiserror::Error;

/// Invokes a graphql query against an *internal* AWS lambda, e.g. ms-graphql-devices.
///
/// **Note**: Do not use this method for querying the public-facing ms-graphql-gateway.
pub async fn internal_graphql_request<R: DeserializeOwned + Clone, L: Lambda>(
    lambda: &L,
    graphql: GraphQLRequest,
    lambda_function_name: String,
) -> Result<R, GraphQLError> {
    let payload = serde_json::to_string(&graphql)?;
    let payload = compress(payload);
    let payload = base64::encode(payload);
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
        return Err(GraphQLError::LambdaFunctionBadStatusCode{
            payload: format!("{:?}", response.payload),
            status_code: response.status_code
        });
    }

    // Try to parse the GraphQL result
    let res = decompress(&response.payload.ok_or(GraphQLError::NoResponsePayload)?);
    let parsed_response: [InternalGraphQLResponse<R>; 1] = serde_json::from_slice(&res).map_err(|e|GraphQLError::UnexpectedJsonResponse(e))?;
    
    
    let first_result = &parsed_response[0];
    if let Some(errors) = &first_result.errors {
        return Err(GraphQLError::InternalGraphQLError(errors.to_string()));
    } else {
        return Ok(first_result.clone().data.unwrap());
    }
}

#[derive(Deserialize, Clone)]
struct InternalGraphQLResponse<T: Clone>{
    data: Option<T>,
    errors: Option<serde_json::Value>
}

#[derive(Error, Debug)]
pub enum GraphQLError {
    #[error("invalid input query")]
    InvalidInputQuery(#[from] serde_json::Error),
    #[error("failed invoking lambda")]
    LambdaInvoke(#[from] RusotoError<InvokeError>),
    #[error("lambda function error: {0}")]
    LambdaFunctionError(String),
    #[error("lambda function bad status code {status_code:?} with payload: {payload}")]
    LambdaFunctionBadStatusCode{
        status_code: Option<i64>,
        payload: String
    },
    #[error("no response payload")]
    NoResponsePayload,
    #[error("bad json response. Error: {0}")]
    UnexpectedJsonResponse(serde_json::Error),
    #[error("internal graphql error: {0}")]
    InternalGraphQLError(String)
}