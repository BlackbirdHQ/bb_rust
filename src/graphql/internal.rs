use std::collections::HashSet;

use aws_sdk_lambda::model::InvocationType;
use aws_sdk_lambda::types::Blob;
use serde::de::DeserializeOwned;
use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;

use crate::misc::{compress, decompress};
use crate::types::PeripheralId;
use serde_json::json;

use super::GraphQLError;
#[derive(Serialize)]
pub struct GraphQLRequestBody {
    pub query: String,
    pub variables: Value,
    pub context: GraphqlContext,
}

#[derive(Serialize)]
/// This struct purely exists to hide the weird extra `graphqlContext` layer in context that the API expects
struct GraphQLRequestBodyToSend {
    pub query: String,
    pub variables: Value,
    pub context: GraphqlContextWrapper,
}

#[derive(Serialize)]
struct GraphqlContextWrapper {
    #[serde(rename = "graphqlContext")]
    pub graphql_context: GraphqlContext,
}

/// Invokes a graphql query against an *internal* AWS lambda, e.g. ms-graphql-devices.
///
/// **Note**: Do not use this method for querying the public-facing ms-graphql-gateway.
pub async fn internal_graphql_request<R: DeserializeOwned>(
    lambda: &aws_sdk_lambda::client::Client,
    graphql: GraphQLRequestBody,
    lambda_function_name: String,
) -> Result<R, GraphQLError> {
    let graphql = GraphQLRequestBodyToSend {
        query: graphql.query,
        variables: graphql.variables,
        context: GraphqlContextWrapper {
            graphql_context: graphql.context,
        },
    };
    let body = serde_json::to_string(&graphql)?;
    let payload = vec![json!({ "body": body })];
    let payload = compress(payload)?;
    let payload = format!("\"{}\"", base64::encode(payload));

    let response = lambda
        .invoke()
        .function_name(lambda_function_name)
        .invocation_type(InvocationType::RequestResponse)
        .payload(Blob::new(payload))
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
    let res: [InternalGraphQLResponse<R>; 1] = decompress(
        response
            .payload
            .ok_or(GraphQLError::NoResponsePayload)?
            .as_ref(),
    )?;

    let r = res.into_iter().next().unwrap();
    if let Some(errors) = r.errors {
        Err(GraphQLError::InternalGraphQLError(errors.to_string()))
    } else {
        Ok(r.data.expect("GraphQL result did not have data field"))
    }
}

#[derive(Deserialize)]
struct InternalGraphQLResponse<T> {
    data: Option<T>,
    errors: Option<serde_json::Value>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
/// Based on https://github.com/BlackbirdHQ/module-graphql-service/blob/a096efdd573396a6cfa869bb9c44df968d941f4b/src/types.ts#L24
pub struct GraphqlContext {
    #[serde(rename = "defaultLanguage")]
    default_language: String,
    language: String,
    #[serde(rename = "groupIds")]
    group_ids: HashSet<String>,
    #[serde(rename = "lineIds")]
    line_ids: HashSet<String>,
    #[serde(rename = "peripheralIds")]
    peripheral_ids: HashSet<PeripheralId>,
    #[serde(rename = "userPools")]
    user_pools: Vec<String>,
    #[serde(rename = "userSub")]
    user_sub: String,
    #[serde(rename = "userPool")]
    user_pool: String,
    #[serde(rename = "requiredBy")]
    required_by: Option<Required>,
    #[serde(rename = "requires")]
    requires: Option<Required>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
struct Required {
    #[serde(rename = "lineIds")]
    line_ids: HashSet<String>,
    #[serde(rename = "peripheralIds")]
    peripheral_ids: HashSet<PeripheralId>,
}

impl GraphqlContext {
    pub fn new(user_pool: String) -> Self {
        GraphqlContext {
            default_language: Default::default(),
            language: Default::default(),
            group_ids: Default::default(),
            line_ids: Default::default(),
            peripheral_ids: Default::default(),
            user_pools: Default::default(),
            user_sub: Default::default(),
            user_pool,
            required_by: Default::default(),
            requires: Default::default(),
        }
    }

    /// Get a reference to the graphql context's user pool.
    pub fn user_pool(&self) -> &str {
        self.user_pool.as_ref()
    }

    pub fn allow_line_id(&mut self, line_id: String) {
        self.line_ids.insert(line_id);
    }

    pub fn disallow_line_id(&mut self, line_id: String) {
        self.line_ids.remove(&line_id);
    }

    pub fn line_access_allowed(&self, line_id: &str) -> bool {
        self.line_ids.contains(line_id)
    }

    // TODO extend with accessor methods as neccessary
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::GraphqlContext;

    #[test]
    fn deserialize_graphql_context() {
        let json = r#"{
            "lineIds": ["1", "2", "1"], 
            "userPool":"asd", 
            "defaultLanguage": "en",
            "language": "de",
            "groupIds": ["asd"],
            "peripheralIds": ["1-2"],
            "userPools": ["a"],
            "userSub": "asd"
        }"#;
        let c: GraphqlContext = serde_json::from_str(json).unwrap();
        assert_eq!(
            c.line_ids,
            HashSet::from_iter(["1".to_string(), "2".to_string()])
        )
    }
}
