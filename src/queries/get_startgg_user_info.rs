//! The query we use to ensure that we're
//! successfully logged in to Start.GG
//! (and also get some basic user information).

use super::STARTGG_ENDPOINT;

use anyhow::anyhow;
use graphql_client::{GraphQLQuery, Response};
use reqwest::Client;

#[derive(Debug, Clone)]
pub struct GetStartGGUserInfoResult {
    pub username: String,
    pub pfp_url: Option<String>,
}

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "api/schema.json",
    query_path = "api/user-login.graphql",
    response_derives = "Debug"
)]
pub struct UserLoggedInStartGGQuery;

pub async fn get_startgg_user_and_profile_icon<S>(
    token: S,
) -> anyhow::Result<GetStartGGUserInfoResult>
where
    S: ToString,
{
    let request_body =
        UserLoggedInStartGGQuery::build_query(user_logged_in_start_gg_query::Variables);

    let client = Client::new();
    let resp = client
        .post(STARTGG_ENDPOINT)
        .bearer_auth(token.to_string())
        .json(&request_body)
        .send()
        .await?;

    let resp: Response<user_logged_in_start_gg_query::ResponseData> = resp.json().await?;

    let respdata = match resp.data {
        Some(d) => match d.current_user {
            Some(usr) => usr,
            None => return Err(anyhow!("`currentUser` does not exist in query.")),
        },
        None => return Err(anyhow!("`data` does not exist in query.".to_string())),
    };

    let url_string = match respdata.images.unwrap()[0].as_ref() {
        Some(img) => Some(img.url.as_ref().unwrap().clone()),
        None => None,
    };

    Ok(GetStartGGUserInfoResult {
        username: respdata.player.unwrap().gamer_tag.unwrap().clone(),
        pfp_url: url_string,
    })
}
