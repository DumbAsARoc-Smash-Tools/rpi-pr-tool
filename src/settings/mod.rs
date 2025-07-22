use oauth2::{EmptyExtraTokenFields, StandardTokenResponse};
use oauth2::basic::BasicTokenType;

type TokenType = StandardTokenResponse<EmptyExtraTokenFields, BasicTokenType>;
pub const DEFAULT_SETTINGS_LOC: &str = "rpipr.settings";

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct RPIPRSettings {
    token: Option<TokenType>,
}

impl RPIPRSettings {

    pub fn new() -> Self {
        Self {
            token: None
        }
    }

    pub fn set_token(&self, token: Option<TokenType>) -> Self {
        Self {
            token,
            ..*self
        }
    }
}
