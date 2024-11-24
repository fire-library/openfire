use crate::config::graphql_url;
use graphql_client::{GraphQLQuery, Response};
use serde::{Deserialize, Serialize};
use specta::Type;
use tauri_plugin_http::reqwest;

type UUID = String;

#[derive(GraphQLQuery, Serialize, Deserialize)]
#[graphql(
    schema_path = "../../web/server/schema.graphql",
    query_path = "src/domain/chat/queries/ask_mutation.graphql",
    response_derives = "Debug,Serialize,Deserialize"
)]
pub struct AskMutation;

#[derive(Deserialize, Serialize, Type, Debug, Clone)]
pub struct AskInput {
    pub conv_id: String,
    pub question: String,
    pub page: String,
    pub page_id: String,
}

impl From<AskInput> for ask_mutation::Variables {
    fn from(variables: AskInput) -> ask_mutation::Variables {
        ask_mutation::Variables {
            conv_id: variables.conv_id,
            question: variables.question,
            page: variables.page,
            page_id: variables.page_id,
        }
    }
}

pub async fn perform_query(
    client: &reqwest::Client,
    variables: ask_mutation::Variables,
) -> Result<ask_mutation::AskMutationAsk, String> {
    let request_body = AskMutation::build_query(variables);

    let res = client.post(graphql_url()).json(&request_body).send().await;
    let response_body: Response<ask_mutation::ResponseData> = res
        .map_err(|_| "Unexpected Error".to_string())?
        .json()
        .await
        .map_err(|_| "Unexpected Error".to_string())?;

    response_body.data.map(|data| data.ask).ok_or_else(|| {
        response_body.errors.map_or_else(
            || "An unexpected error occurred".to_string(),
            |errors| errors[0].message.to_string(),
        )
    })
}
