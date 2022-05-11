use std::collections::HashSet;

use aws_sdk_lambda::model::InvocationType;
use aws_sdk_lambda::types::Blob;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use serde_with::serde_as;

use crate::misc::{compress, decompress};
use crate::types::peripheral_id::PeripheralId;

use super::GraphQLError;
#[derive(Serialize)]
pub struct GraphQLRequestBody<V> {
    pub query: String,
    pub variables: V,
    pub context: GraphqlContext,
}

#[derive(Serialize)]
/// This struct purely exists to hide the weird extra `graphqlContext` layer in context that the API expects
struct GraphQLRequestBodyToSend<V> {
    pub query: String,
    pub variables: V,
    pub context: GraphqlContextWrapper,
}

#[derive(Serialize)]
struct GraphqlContextWrapper {
    #[serde(rename = "graphqlContext")]
    pub graphql_context: GraphqlContext,
}

#[serde_as]
#[derive(Serialize, Debug)]
struct PayloadToSend<T: Serialize> {
    #[serde_as(as = "serde_with::json::JsonString")]
    body: T,
}

/// Invokes a graphql query against an *internal* AWS lambda, e.g. ms-graphql-devices.
///
/// **Note**: Do not use this method for querying the public-facing ms-graphql-gateway.
pub async fn internal_graphql_request<V: Serialize, R: DeserializeOwned>(
    lambda: &aws_sdk_lambda::client::Client,
    graphql: GraphQLRequestBody<V>,
    lambda_function_name: String,
) -> Result<graphql_client::Response<R>, GraphQLError> {
    let graphql = GraphQLRequestBodyToSend {
        query: graphql.query,
        variables: graphql.variables,
        context: GraphqlContextWrapper {
            graphql_context: graphql.context,
        },
    };
    let payload = PayloadToSend { body: graphql };
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
    let payload = response.payload.ok_or(GraphQLError::NoResponsePayload)?;

    // The format of the payload received is "<base64>" (the quotation marks are included in the payload).
    // We "parse" the string by removing the quotation marks, and then base64 decode it, before decompressing it.
    let base64_decoded = base64::decode(&payload.as_ref()[1..payload.as_ref().len() - 1]).unwrap();
    let [r]: [graphql_client::Response<R>; 1] = decompress(&base64_decoded)?;
    Ok(r)
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

    pub fn allow_line_id(mut self, line_id: String) -> Self {
        self.line_ids.insert(line_id);
        self
    }

    pub fn disallow_line_id(mut self, line_id: String) -> Self {
        self.line_ids.remove(&line_id);
        self
    }

    pub fn line_access_allowed(&self, line_id: &str) -> bool {
        self.line_ids.contains(line_id)
    }

    pub fn allow_peripheral_id(mut self, peripheral_id: PeripheralId) -> Self {
        self.peripheral_ids.insert(peripheral_id);
        self
    }

    pub fn disallow_peripheral_id(mut self, peripheral_id: PeripheralId) -> Self {
        self.peripheral_ids.remove(&peripheral_id);
        self
    }

    pub fn peripheral_access_allowed(&self, peripheral_id: &PeripheralId) -> bool {
        self.peripheral_ids.contains(peripheral_id)
    }

    // TODO extend with accessor methods as neccessary

    /// Set the graphql context's default language.
    pub fn set_default_language(mut self, default_language: String) -> Self {
        self.default_language = default_language;
        self
    }
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
