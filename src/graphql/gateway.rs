use serde::de::DeserializeOwned;
use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;

use rusoto_lambda::{InvocationRequest, Lambda};

use super::GraphQLError;

#[derive(Serialize)]
pub struct GatewayGraphQLRequestBody {
    pub query: String,
    pub variables: Value,
    #[serde(rename = "userPool")]
    pub userpool_id: String,
}

#[derive(Deserialize, Clone)]
struct GatewayGraphQLResponse {
    body: Option<serde_json::Value>,
    errors: Option<serde_json::Value>,
}

/// Invokes a graphql query against an the gateway AWS lambda, i.e. ms-graphql-gateway.
///
/// **Note**: Do not use this method for querying the internal-facing lambdas e.g. ms-graphql-devices-entry
// Implementation based on https://github.com/BlackbirdHQ/cloud-services/blob/ca6fce3e0ec2d1d5744f074330d3b52b090eb340/ms-graphql-export/src/helpers/blackbird-api.ts#L18
pub async fn gateway_graphql_request<R: DeserializeOwned + Clone, L: Lambda>(
    lambda: &L,
    graphql: &GatewayGraphQLRequestBody,
    gateway_lambda_function_name: String,
) -> Result<R, GraphQLError> {
    let input = InvocationRequest {
        function_name: gateway_lambda_function_name,
        invocation_type: Some("RequestResponse".to_string()),
        payload: Some(serde_json::to_string(&graphql)?.into()),
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
    let res: GatewayGraphQLResponse =
        serde_json::from_slice(&response.payload.ok_or(GraphQLError::NoResponsePayload)?)?;

    if let Some(errors) = &res.errors {
        Err(GraphQLError::InternalGraphQLError(errors.to_string()))
    } else {
        Ok(serde_json::from_str(
            res.body
                .expect("GraphQL result did not have data field")
                .as_str()
                .expect("GraphQL body must be string"),
        )?)
    }
}
