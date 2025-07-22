//! OAuth handling

use oauth2::*;
use oauth2::basic::BasicTokenType;
use oauth2::basic::BasicClient;

use tokio::io::{AsyncBufReadExt, BufReader, AsyncWriteExt};

// use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpListener;
use url::Url;

const STARTGG_AUTH_URL: &str = "https://start.gg/oauth/authorize";
const STARTGG_TOKEN_URL: &str = "https://api.start.gg/oauth/access_token";
const STARTGG_AUTH_SCOPES: [&str; 1] = [ "user.identity" ];
const STARTGG_REDIRECT_URI: &str = "http://localhost";

pub struct StartGGOAuth;

impl StartGGOAuth {

    /// General structure of code borrowed from oauth2 example at
    /// https://github.com/ramosbugs/oauth2-rs/blob/main/examples/github_async.rs
    pub async fn get_oauth_token(redirect_uri_port: u16) -> Option<StandardTokenResponse<EmptyExtraTokenFields, BasicTokenType>> {
        let startgg_client_id = ClientId::new(
            env!("CLIENT_ID").to_string()
        );
        
        let startgg_client_secret = ClientSecret::new(
            env!("CLIENT_SECRET").to_string()
        );
        
        let startgg_auth_url = AuthUrl::new(
            STARTGG_AUTH_URL.to_string()
        ).expect("Could not create authentication URL.");
        
        let startgg_token_url = TokenUrl::new(
            STARTGG_TOKEN_URL.to_string()
        ).expect("Could not create token URL.");

        let client = BasicClient::new(startgg_client_id)
            .set_client_secret(startgg_client_secret)
            .set_auth_uri(startgg_auth_url)
            .set_token_uri(startgg_token_url)
            .set_redirect_uri(
                RedirectUrl::new(
                    format!("{}:{}", STARTGG_REDIRECT_URI, redirect_uri_port)
                ).expect("Invalid redirect URL")
            );
        
        let http_client = oauth2::reqwest::ClientBuilder::new()
            .redirect(oauth2::reqwest::redirect::Policy::none())
            .build()
            .expect("Failed to build HTTP client!");

        // @TODO Go back to example on SSD and see
        // why that works and this doesn't.
        let (auth_url, csrf_state) = client
            .authorize_url(CsrfToken::new_random)
            .add_scopes(STARTGG_AUTH_SCOPES.map(|s| Scope::new(s.to_string())))
            .url();

        open::that(format!("{auth_url}"));

        let (code, state) = {
            // A very naive implementation of the redirect server.
            let listener = TcpListener::bind(
                format!("127.0.0.1:{}", redirect_uri_port)
            ).await.unwrap();

            loop {
                if let Ok((mut stream, _)) = listener.accept().await {
                    let mut reader = BufReader::new(&mut stream);

                    let mut request_line = String::new();
                    reader.read_line(&mut request_line).await.unwrap();

                    let redirect_url = request_line.split_whitespace().nth(1).unwrap();
                    let url = Url::parse(&("http://localhost".to_string() + redirect_url)).unwrap();

                    let code = url
                        .query_pairs()
                        .find(|(key, _)| key == "code")
                        .map(|(_, code)| AuthorizationCode::new(code.into_owned()))
                        .unwrap();

                    let state = url
                        .query_pairs()
                        .find(|(key, _)| key == "state")
                        .map(|(_, state)| CsrfToken::new(state.into_owned()))
                        .unwrap();

                    let message = "Go back to your terminal :)";
                    let response = format!(
                        "HTTP/1.1 200 OK\r\ncontent-length: {}\r\n\r\n{}",
                        message.len(),
                        message
                    );
                    stream.write_all(response.as_bytes()).await.unwrap();

                    // The server will terminate itself after collecting the first code.
                    break (code, state);
                }
            }
        };

        if state.secret() != csrf_state.secret() {
            println!("Tampering involved!");
            println!("Expected: {}", csrf_state.secret());
            println!("Got:      {}", state.secret());
        }

        // get code via already created
        // http client
        let token_res = client.exchange_code(code).request_async(&http_client).await;

        if let Ok(token) = token_res {
            let scopes = if let Some(scopes_vec) = token.scopes() {
                scopes_vec
                    .iter()
                    .flat_map(|comma_separated| comma_separated.split(','))
                    .collect::<Vec<_>>()
            } else {
                Vec::new()
            };
            println!("StartGG returned the following scopes:\n{scopes:?}\n");
            println!("Token: {}", token.access_token().secret());
            println!("Expires in: {:?}", token.expires_in().unwrap_or(
                std::time::Duration::from_secs(0)
            ));

            return Some(token);
        }

        return None;
    }
}