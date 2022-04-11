use bb_rust::graphql::{internal_graphql_request, GraphQLRequestBody};
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let function_name =
        "arn:aws:lambda:eu-west-1:789153103247:function:prod-ms-graphql-iam-entry".to_string();
    let query = "query test {
        company {
          id
        }
      }
      ";
    let graphql = GraphQLRequestBody {
        query: query.to_string(),
        variables: json!(null),
        context: json!({
            "graphqlContext": {
                "peripheralIds": [],
                "defaultLanguage": "en",
                "userPool": "eu-west-1_lu59lbvt7"
            }
        }),
    };

    let lambda = aws_sdk_lambda::Client::new(&aws_config::load_from_env().await);

    let raw_resp =
        internal_graphql_request::<_, serde_json::Value>(&lambda, graphql, function_name).await?;
    println!("{:?}", &raw_resp);
    Ok(())
}
