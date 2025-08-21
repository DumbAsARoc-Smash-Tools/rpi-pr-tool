mod get_tournament_information;

/// The API endpoint to which we send GraphQL
/// queries.
const STARTGG_ENDPOINT: &str = "https://api.start.gg/gql/alpha";

/// Separate types module to reexport the structs
/// our queries return.
pub mod types {
    pub use super::get_tournament_information::TournamentInformationQueryResult;
}

pub use get_tournament_information::get_tournament_and_event_info;
