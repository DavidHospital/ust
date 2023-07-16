use rspotify::{prelude::*, scopes, AuthCodePkceSpotify, Credentials, OAuth, Config};

use crate::redirect_uri::redirect_uri_web_server;

mod redirect_uri;

#[tokio::main]
async fn main() {
    // You can use any logger for debugging.
    env_logger::init();

    // Set RSPOTIFY_CLIENT_ID and RSPOTIFY_CLIENT_SECRET in an .env file (after
    // enabling the `env-file` feature) or export them manually:
    //
    // export RSPOTIFY_CLIENT_ID="your client_id"
    //
    // It will then be read with `from_env`.
    //
    // Otherwise, set client_id explictly:
    //
    // ```
    // let creds = Credentials::new_pkce("my-client-id");
    // ```
    let creds = Credentials::from_env().unwrap();

    // Same for RSPOTIFY_REDIRECT_URI. You can also set it explictly:
    //
    // ```
    // let oauth = OAuth {
    //     redirect_uri: "http://localhost:8888/callback".to_string(),
    //     scopes: scopes!("user-read-recently-played"),
    //     ..Default::default(),
    // };
    // ```
    let oauth = OAuth::from_env(scopes!("user-read-playback-state")).unwrap();

    let mut spotify = AuthCodePkceSpotify::with_config(creds.clone(), oauth.clone(), Config {token_cached: true, .. Default::default()});

    // Obtaining the access token
    let url = spotify.get_authorize_url(None).unwrap();

    // This function requires the `cli` feature enabled.
    match spotify.read_token_cache(false).await {
        Ok(Some(token)) => {
            // if token.is_expired() {
                spotify.refresh_token().await.unwrap();
            // }

            *spotify.token.lock().await.unwrap() = Some(token.clone());
        }
        _ => {
            match redirect_uri_web_server(8888, &url) {
                Ok(url) => {
                    let code = spotify.parse_response_code(&url).unwrap();
                    spotify.request_token(&code).await.unwrap();
                },
                Err(()) => {},
            };
        }
    };

    // Running the requests
    let history = spotify.current_playback(None, None::<Vec<_>>).await.unwrap().expect("error fetching history");
    println!("{:?}", history.item.unwrap());
}
