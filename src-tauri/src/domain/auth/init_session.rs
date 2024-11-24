use crate::config::graphql_url;
use graphql_client::{GraphQLQuery, Response};
use serde::{Deserialize, Serialize};
use specta::Type;
use tauri_plugin_http::reqwest;

#[allow(dead_code)]
type UUID = String;

#[derive(GraphQLQuery, Serialize, Deserialize)]
#[graphql(
    schema_path = "../../web/server/schema.graphql",
    query_path = "src/domain/auth/queries/init_session_query.graphql",
    response_derives = "Debug,Serialize,Deserialize"
)]
pub struct InitSessionQuery;

#[derive(Deserialize, Serialize, Type, Debug, Clone)]
pub struct InitSessionInput {
    pub token: String,
}

impl From<InitSessionInput> for init_session_query::Variables {
    fn from(variables: InitSessionInput) -> init_session_query::Variables {
        init_session_query::Variables {
            token: variables.token,
        }
    }
}

pub async fn perform_query(
    client: &reqwest::Client,
    token: String,
) -> Result<init_session_query::InitSessionQueryInitSessionUser, String> {
    let variables = init_session_query::Variables { token };
    let request_body = InitSessionQuery::build_query(variables);

    let res = client.post(graphql_url()).json(&request_body).send().await;
    let response_body: Response<init_session_query::ResponseData> = res
        .map_err(|_| "Unexpected Error".to_string())?
        .json()
        .await
        .map_err(|_| "Unexpected Error".to_string())?;

    let user = response_body
        .data
        .ok_or("error".to_string())?
        .init_session
        .ok_or("Login failed".to_string())?
        .user;

    Ok(user)
}
