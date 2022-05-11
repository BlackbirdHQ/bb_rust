use bb_rust::graphql::{
    batch_internal_graphql_request, internal_graphql_request, GraphQLRequestBody, GraphqlContext,
};
use serde_json::json;

async fn single() -> anyhow::Result<()> {
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
        context: GraphqlContext::new("eu-west-1_lu59lbvt7".to_string())
            .set_default_language("en".to_string()),
    };

    let lambda = aws_sdk_lambda::Client::new(&aws_config::load_from_env().await);

    let raw_resp =
        internal_graphql_request::<_, serde_json::Value>(&lambda, graphql, function_name).await?;
    println!("{:?}", &raw_resp);
    Ok(())
}

async fn batch() -> anyhow::Result<()> {
    let function_name =
        "arn:aws:lambda:eu-west-1:789153103247:function:prod-ms-graphql-iam-entry".to_string();
    let query1 = "query test {
        company {
          id
        }
      }
      ";
    let request1 = GraphQLRequestBody {
        query: query1.to_string(),
        variables: json!(null),
        context: GraphqlContext::new("eu-west-1_lu59lbvt7".to_string())
            .set_default_language("en".to_string()),
    };

    let query2 = "query test {
        company {
          name
        }
      }
      ";
    let request2 = GraphQLRequestBody {
        query: query2.to_string(),
        variables: json!(null),
        context: GraphqlContext::new("eu-west-1_lu59lbvt7".to_string())
            .set_default_language("en".to_string()),
    };

    let lambda = aws_sdk_lambda::Client::new(&aws_config::load_from_env().await);

    let raw_resp = batch_internal_graphql_request::<_, serde_json::Value>(
        &lambda,
        vec![request1, request2],
        function_name,
    )
    .await?;
    println!("{:?}", &raw_resp);
    Ok(())
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    single().await?;
    batch().await
}
