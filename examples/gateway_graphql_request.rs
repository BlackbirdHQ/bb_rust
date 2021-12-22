use bb_rust::graphql::{gateway_graphql_request, GatewayGraphQLRequestBody};
use rusoto_core::Region;
use rusoto_lambda::LambdaClient;
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let function_name =
        "arn:aws:lambda:eu-west-1:930089190179:function:staging-ms-graphql-gateway-entry"
            .to_string();
    let query = "query me {
        features {
          userPool {
            id
            feature {
              Q
              M
              key
            }
          }
          group {
            id
            feature {
              Q
              M
              key
            }
          }
          line {
            id
            feature {
              Q
              M
              key
            }
          }
          peripheral {
            id
            feature {
              Q
              M
              key
            }
          }
        }
      }
      ";
    let graphql = GatewayGraphQLRequestBody {
        query: query.to_string(),
        variables: json!(null),
        userpool_id: "eu-west-1_aBWI4ripT".to_string(),
    };

    let lambda = LambdaClient::new(Region::EuWest1);

    let raw_resp =
        gateway_graphql_request::<serde_json::Value, _>(&lambda, &graphql, function_name).await?;
    println!("{}", serde_json::to_string(&raw_resp)?);
    Ok(())
}
