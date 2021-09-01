use std::str::FromStr;

use serde_json::Value;
use serde::Serialize;
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

use rusoto_lambda::{InvocationRequest, Lambda};

use crate::misc::{compress, decompress};


pub async fn internal_graphql_request<R: DeserializeOwned, L: Lambda>(
    lambda: &L,
    graphql: GraphQLRequest,
    lambda_function_name: String,
) -> R {
    let payload = serde_json::to_string(&graphql).unwrap();
    let payload = compress(payload);
    let payload = base64::encode(payload);
    let input = InvocationRequest {
        function_name: lambda_function_name,
        invocation_type: Some("RequestResponse".to_string()),
        payload: Some(payload.into()),
        ..Default::default()
    };

    let response = lambda.invoke(input).await.unwrap();
    if let Some(err) = response.function_error {
        panic!("Function error: {}", err);
    }
    if response.status_code != Some(200) {
        panic!(
            "Got wrong status code ({:?}) from lambda: {:?}",
            response.status_code, response.payload
        )
    }

    // Try to parse the GraphQL result
    let res = decompress(&response.payload.unwrap());
    let res = String::from_utf8(res).unwrap();
    let json_value = serde_json::Value::from_str(&res).unwrap();
    let first_result = &json_value.as_array().unwrap()[0];
    if let Some(errors) = first_result.get("errors") {
        panic!("API ERROR: {:?}", errors);
    } else {
        return serde_json::from_value(first_result.get("data").unwrap().clone()).unwrap();
    }
}