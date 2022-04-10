use aws_sdk_lambda::model::InvocationType;
use aws_sdk_lambda::types::Blob;
use serde::de::DeserializeOwned;
use serde::Deserialize;
use serde::Serialize;

use super::GraphQLError;

#[derive(Serialize)]
pub struct GatewayGraphQLRequestBody<V: Serialize> {
    pub query: String,
    pub variables: V,
    #[serde(rename = "userPool")]
    pub userpool_id: String,
}

#[derive(Deserialize)]
#[serde(untagged)]
enum GatewayGraphQLResponse {
    Data { body: serde_json::Value },
    Error { errors: serde_json::Value },
}

/// Invokes a graphql query against an the gateway AWS lambda, i.e. ms-graphql-gateway.
///
/// **Note**: Do not use this method for querying the internal-facing lambdas e.g. ms-graphql-devices-entry
// Implementation based on https://github.com/BlackbirdHQ/cloud-services/blob/ca6fce3e0ec2d1d5744f074330d3b52b090eb340/ms-graphql-export/src/helpers/blackbird-api.ts#L18
pub async fn gateway_graphql_request<V: Serialize, R: DeserializeOwned>(
    lambda: &aws_sdk_lambda::Client,
    graphql: &GatewayGraphQLRequestBody<V>,
    gateway_lambda_function_name: String,
) -> Result<R, GraphQLError> {
    let response = lambda
        .invoke()
        .function_name(gateway_lambda_function_name)
        .invocation_type(InvocationType::RequestResponse)
        .payload(Blob::new(serde_json::to_vec(&graphql)?))
        .send()
        .await?;

    if let Some(err) = response.function_error {
        return Err(GraphQLError::LambdaFunctionError(err));
    }
    if response.status_code != 200 {
        return Err(GraphQLError::LambdaFunctionBadStatusCode {
            payload: format!("{:?}", response.payload),
            status_code: response.status_code,
        });
    }

    // Try to parse the GraphQL result
    let res: GatewayGraphQLResponse = serde_json::from_slice(
        response
            .payload
            .ok_or(GraphQLError::NoResponsePayload)?
            .as_ref(),
    )?;

    match res {
        GatewayGraphQLResponse::Data { body } => Ok(serde_json::from_str(
            body.as_str().expect("GraphQL body must be string"),
        )?),
        GatewayGraphQLResponse::Error { errors } => {
            Err(GraphQLError::InternalGraphQLError(errors.to_string()))
        }
    }
}
