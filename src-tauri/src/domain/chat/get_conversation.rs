use crate::config::graphql_url;
use get_conversation_query::GetConversationQueryUserConversationByPageIdMessages;
use graphql_client::{GraphQLQuery, Response};
use serde::{Deserialize, Serialize};
use specta::Type;
use tauri_plugin_http::reqwest;

use super::Message;

type UUID = String;

#[derive(GraphQLQuery, Serialize, Deserialize)]
#[graphql(
    schema_path = "../../web/server/schema.graphql",
    query_path = "src/domain/chat/queries/get_conversation_query.graphql",
    response_derives = "Debug,Serialize,Deserialize"
)]
pub struct GetConversationQuery;

#[derive(Deserialize, Serialize, Type, Debug, Clone)]
pub struct GetConversationInput {
    pub page_id: String,
}

impl From<GetConversationInput> for get_conversation_query::Variables {
    fn from(variables: GetConversationInput) -> get_conversation_query::Variables {
        get_conversation_query::Variables {
            page_id: variables.page_id,
        }
    }
}

impl From<GetConversationQueryUserConversationByPageIdMessages> for Message {
    fn from(variables: GetConversationQueryUserConversationByPageIdMessages) -> Message {
        Message {
            id: variables.id,
            is_user: variables.is_user,
            message: variables.text,
        }
    }
}

pub async fn perform_query(
    client: &reqwest::Client,
    variables: get_conversation_query::Variables,
) -> Result<get_conversation_query::GetConversationQueryUserConversationByPageId, String> {
    let request_body = GetConversationQuery::build_query(variables);

    let res = client.post(graphql_url()).json(&request_body).send().await;
    let response_body: Response<get_conversation_query::ResponseData> = res
        .map_err(|_| "Unexpected Error".to_string())?
        .json()
        .await
        .map_err(|_| "Unexpected Error".to_string())?;

    response_body
        .data
        .ok_or("An unexpected error occurred")?
        .user
        .ok_or("Not logged in")?
        .conversation_by_page_id
        .ok_or("Conversation not found".to_string())
}
