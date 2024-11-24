use crate::config::graphql_url;
use graphql_client::{GraphQLQuery, Response};
use serde::{Deserialize, Serialize};
use tauri_plugin_http::reqwest;

#[allow(dead_code)]
type UUID = String;

#[derive(GraphQLQuery, Serialize, Deserialize)]
#[graphql(
    schema_path = "../../web/server/schema.graphql",
    query_path = "src/domain/auth/queries/get_user_query.graphql",
    response_derives = "Debug,Serialize,Deserialize"
)]
pub struct GetUserQuery;

pub async fn perform_query(
    client: &reqwest::Client,
) -> Result<Option<get_user_query::GetUserQueryUser>, String> {
    let request_body = GetUserQuery::build_query(get_user_query::Variables {});
    let res = client.post(graphql_url()).json(&request_body).send().await;

    let response_body: Response<get_user_query::ResponseData> = res
        .map_err(|_| "Unexpected Error".to_string())?
        .json()
        .await
        .map_err(|_| "Unexpected Error".to_string())?;

    let user = response_body.data.ok_or("error".to_string())?.user;

    Ok(user)
}
