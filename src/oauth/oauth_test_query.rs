//! The query we use to ensure that we're
//! successfully logged in to Start.GG
//! (and also get some basic user information).


const STARTGG_ENDPOINT: &str = "https://api.start.gg/gql/alpha";

#[derive(Debug)]
struct UserLoggedInStartGGQueryError(String);

impl std::fmt::Display for UserLoggedInStartGGQueryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for UserLoggedInStartGGQueryError {}


use graphql_client::{GraphQLQuery, Response};
use reqwest::Client;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "api/schema.json",
    query_path = "api/user-login.graphql",
    response_derives = "Debug",
)]
pub struct UserLoggedInStartGGQuery;

pub async fn get_startgg_user_and_profile_icon<S>(
    token: S,
) -> Result<(String, String), Box<dyn std::error::Error>> where S: ToString {

    let request_body = UserLoggedInStartGGQuery::build_query(
        user_logged_in_start_gg_query::Variables
    );

    let client = Client::new();
    let resp = client.post(STARTGG_ENDPOINT)
        .bearer_auth(token.to_string())
        .json(&request_body)
        .send().await?;

    let resp: Response<user_logged_in_start_gg_query::ResponseData> = resp.json().await?;

    let respdata = match resp.data {
        Some(d) => {
            match d.current_user {
                Some(usr) => usr,
                None => {
                    return Err(Box::new(
                        UserLoggedInStartGGQueryError(
                            "`currentUser` does not exist in query.".to_string()
                        )
                    ));
                }
            }
        },
        None => {
            return Err(Box::new(
                UserLoggedInStartGGQueryError(
                    "`data` does not exist in query.".to_string()
                )
            ));
        }
    };

    Ok((
        respdata.player.unwrap().gamer_tag.unwrap().clone(),
        respdata.images.unwrap()[0].as_ref().unwrap().url.as_ref().unwrap().clone()
    ))
}
