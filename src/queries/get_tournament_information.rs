//! Gets tournament information including entrant count,
//! tournament name, and event name.

use super::STARTGG_ENDPOINT;

use graphql_client::{GraphQLQuery, Response};
use reqwest::Client;

#[derive(Debug, Clone)]
pub struct TournamentInformationQueryResult {
    tournament_name: String,
    event_name: String,
    num_entrants: i64,
}

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "api/schema.json",
    query_path = "api/tournament_info_query.graphql",
    response_derives = "Debug"
)]
pub struct TournamentInformationQuery;

pub async fn get_tournament_and_event_info<S, T>(
    token: S,
    event_slug: T,
) -> anyhow::Result<TournamentInformationQueryResult>
where
    S: ToString,
    T: ToString,
{
    let request_body =
        TournamentInformationQuery::build_query(tournament_information_query::Variables {
            event_slug: Some(event_slug.to_string()),
        });

    let client = Client::new();

    #[allow(unused_variables)]
    let resp = client
        .post(STARTGG_ENDPOINT)
        .bearer_auth(token.to_string())
        .json(&request_body)
        .send()
        .await?;

    let resp: Response<tournament_information_query::ResponseData> = resp.json().await?;

    let respdata = match resp.data {
        Some(d) => d,
        None => {
            return Err(anyhow::anyhow!("`data` does not exist in query.",));
        }
    };

    let event = match respdata.event {
        Some(e) => e,
        None => return Err(anyhow::anyhow!("`event` does not exist in query.")),
    };

    let tournament = match event.tournament {
        Some(t) => t,
        None => return Err(anyhow::anyhow!("`tournament` does not exist in query.")),
    };

    Ok(TournamentInformationQueryResult {
        tournament_name: tournament.name.unwrap().clone(),
        event_name: event.name.unwrap().clone(),
        num_entrants: event.num_entrants.unwrap(),
    })
}
