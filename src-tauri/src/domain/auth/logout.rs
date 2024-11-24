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
    query_path = "src/domain/auth/queries/logout_query.graphql",
    response_derives = "Debug,Serialize,Deserialize"
)]
pub struct Logout;

#[derive(Deserialize, Serialize, Type, Debug, Clone)]
pub struct InitSessionInput {
    pub token: String,
}

pub async fn perform_query(client: &reqwest::Client) -> Result<logout::LogoutLogout, String> {
    let variables = logout::Variables {};
    let request_body = Logout::build_query(variables);

    let res = client.post(graphql_url()).json(&request_body).send().await;
    let response_body: Response<logout::ResponseData> = res
        .map_err(|_| "Unexpected Error".to_string())?
        .json()
        .await
        .map_err(|_| "Unexpected Error".to_string())?;

    let success = response_body
        .data
        .ok_or("error".to_string())?
        .logout
        .ok_or("Logout failed".to_string())?;

    Ok(success)
}
